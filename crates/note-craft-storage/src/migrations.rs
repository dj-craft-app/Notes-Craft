//!
//! Système de migration du schéma SQLite, versionné et transactionnel.
//!
//! Ce module fournit:
//! - Un registre de migrations versionnées ([`Migration`], [`default_migrations`])
//! - Un suivi de version via `PRAGMA user_version` (source de vérité du
//!   système de migration)
//! - Application transactionnelle des migrations en attente : chaque
//!   migration s'exécute dans SA transaction ; un échec annule cette
//!   migration (rollback) et interrompt la chaîne
//! - Un backup automatique AVANT toute migration (réutilise le module
//!   `backup` de l'étape 4.5) — mitigation du risque R6
//!   (docs/roadmap/roadmap_v3.md:1031)
//! - Une validation post-migration ([`validate`]) : intégrité SQLite,
//!   contraintes de clés étrangères, orphelins, tables requises — inspirée
//!   de `validate.cljs` (docs/audits/logseq_audit_results.json:1202)
//!
//! Conventions : mêmes règles que `backup.rs` — les opérations passent par
//! la `Connection` fournie par l'appelant ; le `Storage` de `lib.rs`
//! (vestige) n'est jamais utilisé ici.
//!
//! Choix de versionnement : `PRAGMA user_version` plutôt que la table
//! `metadata.db_version` de `schema.rs` (schema.rs:544). Motif : `user_version`
//! est atomique avec la transaction de migration (rollback = version
//! restaurée), sans dépendance à une table qui n'existe pas encore sur une
//! base vide. La table `metadata` reste gérée par `schema.rs` pour son propre
//! usage ; les deux compteurs sont indépendants et documentés comme tels.
//!
//! Source auditée : feature Logseq `worker_migration` — « Système de
//! migration des données », fichiers `migrate.cljs` + `validate.cljs`
//! (docs/audits/logseq_audit_results.json:1199-1215).
//! Choix SQLite : docs/audits/rust_architecture_choices.json:171.
//!
//! Phase 4 - Étape 4.6 (Migration System) — docs/roadmap/roadmap_v3.md:817
//! Critère d'acceptation : migrations testées, backup avant migration
//! (mitigation R6, docs/roadmap/roadmap_v3.md:1031).

use crate::backup::{BackupConfig, BackupInfo, BackupManager};
use rusqlite::Connection;
use std::path::{Path, PathBuf};
use thiserror::Error;

// ============================================================================
// ERREURS
// ============================================================================

/// Erreurs du système de migration (miroir de `BackupError`, backup.rs:54).
#[derive(Error, Debug)]
pub enum MigrationError {
    #[error("Erreur de stockage: {0}")]
    StorageError(String),
    #[error("Backup pré-migration échoué: {0}")]
    BackupFailed(String),
    #[error("Migration {version} ({name}) échouée: {cause}")]
    MigrationFailed {
        version: u32,
        name: String,
        cause: String,
    },
}

impl From<rusqlite::Error> for MigrationError {
    fn from(err: rusqlite::Error) -> Self {
        MigrationError::StorageError(err.to_string())
    }
}

impl From<crate::backup::BackupError> for MigrationError {
    fn from(err: crate::backup::BackupError) -> Self {
        MigrationError::BackupFailed(err.to_string())
    }
}

// ============================================================================
// MIGRATIONS
// ============================================================================

/// Une migration versionnée du schéma.
///
/// `up_sql` est appliqué dans une transaction dédiée ; il DOIT être
/// idempotent par construction (la transaction garantit l'atomicité, le
/// suivi de version garantit qu'il n'est appliqué qu'une fois).
#[derive(Debug, Clone)]
pub struct Migration {
    /// Version cible atteinte après application (strictement croissantes).
    pub version: u32,
    /// Nom lisible (traceabilité dans les rapports et les logs).
    pub name: String,
    /// SQL à appliquer (peut contenir plusieurs instructions).
    pub up_sql: String,
}

/// Registre par défaut des migrations du projet.
///
/// - v1 : schéma initial complet (tables + index + triggers de
///   `schema::Schema`, schema.rs:32-478).
/// - v2 : journal des migrations appliquées (`migration_log`), additif.
pub fn default_migrations() -> Vec<Migration> {
    vec![
        Migration {
            version: 1,
            name: "schéma initial (tables, index, triggers)".to_string(),
            up_sql: format!(
                "{};\n{};\n{}",
                crate::schema::Schema::create_all_tables(),
                crate::schema::Schema::create_all_indexes().join(";\n"),
                crate::schema::Schema::create_all_triggers().join(";\n"),
            ),
        },
        Migration {
            version: 2,
            name: "journal des migrations (migration_log)".to_string(),
            up_sql: "CREATE TABLE IF NOT EXISTS migration_log (
                         id INTEGER PRIMARY KEY,
                         version INTEGER NOT NULL UNIQUE,
                         name TEXT NOT NULL,
                         applied_at INTEGER NOT NULL
                     );
                     CREATE INDEX IF NOT EXISTS idx_migration_log_applied_at
                         ON migration_log(applied_at);"
                .to_string(),
        },
    ]
}

/// Dernière version du registre par défaut.
pub fn latest_version() -> u32 {
    default_migrations()
        .last()
        .map(|m| m.version)
        .expect("default_migrations() n'est jamais vide")
}

// ============================================================================
// RAPPORT
// ============================================================================

/// Résultat d'une exécution de [`MigrationManager::migrate`].
#[derive(Debug)]
pub struct MigrationReport {
    /// Version avant migration (`PRAGMA user_version`).
    pub from_version: u32,
    /// Version après migration.
    pub to_version: u32,
    /// Migrations appliquées, dans l'ordre.
    pub applied: Vec<(u32, String)>,
    /// Backup créé avant migration (mitigation R6). `None` si désactivé
    /// par la config ou si aucune migration n'était en attente.
    pub backup: Option<BackupInfo>,
}

// ============================================================================
// GESTIONNAIRE
// ============================================================================

/// Gestionnaire de migrations pour une base SQLite.
///
/// Suit le pattern de `BackupManager` (backup.rs:132) : construit avec le
/// chemin de la base (pour le backup pré-migration) et une configuration.
pub struct MigrationManager {
    /// Chemin du fichier de base de données (pour le backup pré-migration).
    db_path: PathBuf,
    /// Configuration du backup pré-migration (étape 4.5).
    backup_config: BackupConfig,
}

impl MigrationManager {
    /// Crée un gestionnaire pour la base `db_path`.
    pub fn new(db_path: &Path, backup_config: BackupConfig) -> Self {
        MigrationManager {
            db_path: db_path.to_path_buf(),
            backup_config,
        }
    }

    /// Version courante du schéma (`PRAGMA user_version`).
    pub fn current_version(&self, conn: &Connection) -> Result<u32, MigrationError> {
        let v: i64 = conn.query_row("PRAGMA user_version", [], |row| row.get(0))?;
        Ok(u32::try_from(v).unwrap_or(0))
    }

    /// Migrations du registre `migrations` encore non appliquées à `conn`.
    pub fn pending<'a>(
        &self,
        conn: &Connection,
        migrations: &'a [Migration],
    ) -> Result<Vec<&'a Migration>, MigrationError> {
        let current = self.current_version(conn)?;
        Ok(migrations.iter().filter(|m| m.version > current).collect())
    }

    /// Applique toutes les migrations en attente du registre fourni.
    ///
    /// Ordre des opérations (mitigation R6, roadmap_v3.md:1031) :
    /// 1. Si aucune migration en attente → no-op idempotent (aucun backup).
    /// 2. Backup automatique via `BackupManager` (module 4.5). Un échec du
    ///    backup ABORT la migration : on ne migre jamais sans filet.
    /// 3. Chaque migration s'applique dans SA transaction
    ///    (`unchecked_transaction`) : `up_sql` + `PRAGMA user_version` +
    ///    trace dans `migration_log` (si la table existe). Un échec →
    ///    rollback de CETTE migration uniquement, erreur renvoyée, les
    ///    migrations précédentes restent appliquées (chaque étape est
    ///    un point de reprise sûr).
    ///
    /// Base PLUS RÉCENTE que le registre : si `user_version` dépasse la
    /// dernière version du registre (base ouverte par un binaire plus
    /// récent, puis rétrogradée), aucune migration n'est en attente →
    /// no-op idempotent, `to_version == from_version`, aucun backup.
    /// Le système ne tente JAMAIS de « rétrograder » le schéma.
    ///
    /// Concurrence : la gestion de `busy_timeout` (et des verrous SQLite)
    /// est à la charge de l'appelant (cf. `Schema::initialize_database`,
    /// schema.rs:506, qui règle `busy_timeout=5000`). Ce module ne modifie
    /// aucun PRAGMA de concurrence.
    pub fn migrate(
        &self,
        conn: &Connection,
        migrations: &[Migration],
    ) -> Result<MigrationReport, MigrationError> {
        let from_version = self.current_version(conn)?;
        let pending = self.pending(conn, migrations)?;

        if pending.is_empty() {
            return Ok(MigrationReport {
                from_version,
                to_version: from_version,
                applied: Vec::new(),
                backup: None,
            });
        }

        // Mitigation R6 : backup AVANT toute migration.
        let backup = BackupManager::new(&self.db_path, self.backup_config)
            .auto_backup(conn)
            .map_err(MigrationError::from)?;

        let mut applied = Vec::new();
        for migration in pending {
            self.apply_one(conn, migration)?;
            applied.push((migration.version, migration.name.clone()));
        }

        Ok(MigrationReport {
            from_version,
            to_version: self.current_version(conn)?,
            applied,
            backup,
        })
    }

    /// Applique les migrations du registre par défaut ([`default_migrations`]).
    pub fn migrate_default(&self, conn: &Connection) -> Result<MigrationReport, MigrationError> {
        let migrations = default_migrations();
        self.migrate(conn, &migrations)
    }

    /// Applique UNE migration dans une transaction dédiée.
    ///
    /// La transaction encapsule : le SQL de migration, la mise à jour de
    /// `PRAGMA user_version` et la trace dans `migration_log` (si la table
    /// existe — elle est créée par la v2, donc absente pour la v1). Tout
    /// échec → rollback complet de la migration (version incluse).
    fn apply_one(&self, conn: &Connection, migration: &Migration) -> Result<(), MigrationError> {
        let tx = conn
            .unchecked_transaction()
            .map_err(|e| MigrationError::MigrationFailed {
                version: migration.version,
                name: migration.name.clone(),
                cause: format!("ouverture de transaction: {e}"),
            })?;

        tx.execute_batch(&migration.up_sql)
            .map_err(|e| MigrationError::MigrationFailed {
                version: migration.version,
                name: migration.name.clone(),
                cause: e.to_string(),
            })?;

        tx.pragma_update(None, "user_version", migration.version)
            .map_err(|e| MigrationError::MigrationFailed {
                version: migration.version,
                name: migration.name.clone(),
                cause: format!("mise à jour de user_version: {e}"),
            })?;

        // Trace dans migration_log si la table existe (créée par la v2+).
        let has_log: bool = tx
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master
                 WHERE type='table' AND name='migration_log'",
                [],
                |row| row.get::<_, i64>(0),
            )
            .map(|c| c > 0)
            .unwrap_or(false);
        if has_log {
            let now = chrono::Utc::now().timestamp();
            // La trace est secondaire : on ne fait pas échouer une migration
            // saine pour un échec d'écriture de journal, mais on ne le
            // tait PAS (règle « aucune erreur silencieuse »).
            if let Err(e) = tx.execute(
                "INSERT OR REPLACE INTO migration_log (version, name, applied_at)
                 VALUES (?1, ?2, ?3)",
                rusqlite::params![migration.version, migration.name, now],
            ) {
                log::warn!(
                    "note-craft-storage: trace migration_log échouée pour v{} ({}): {e}",
                    migration.version,
                    migration.name
                );
            }
        }

        tx.commit().map_err(|e| MigrationError::MigrationFailed {
            version: migration.version,
            name: migration.name.clone(),
            cause: format!("commit: {e}"),
        })?;

        Ok(())
    }
}

// ============================================================================
// VALIDATION POST-MIGRATION (inspirée de validate.cljs)
// ============================================================================

/// Sévérité d'un problème détecté par [`validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// Incohérence de données (orphelins, clés étrangères violées).
    Data,
    /// Incohérence de schéma (table/index manquant).
    Schema,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Data => write!(f, "DONNÉES"),
            Severity::Schema => write!(f, "SCHÉMA"),
        }
    }
}

/// Un problème détecté par la validation post-migration.
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    /// Sévérité (données ou schéma).
    pub severity: Severity,
    /// Description du problème.
    pub message: String,
}

/// Valide l'intégrité de la base après migration.
///
/// Vérifications (inspirées de `validate.cljs`, logseq_audit_results.json:1202) :
/// 1. `PRAGMA quick_check` : intégrité physique de la base SQLite.
/// 2. `PRAGMA foreign_key_check` : contraintes de clés étrangères violées
///    (détecte les orphelins même si `foreign_keys=OFF` a permis l'insert).
/// 3. Liens orphelins : `links` référençant des `blocks`/`pages` absents
///    (redondance volontaire avec (2) : les FK ne sont pas toujours actives).
/// 4. Tables requises du schéma (délègue à `Schema::validate_schema`,
///    schema.rs:600).
///
/// Renvoie la liste des problèmes (vide = base cohérente). Ne modifie JAMAIS
/// la base : lecture seule.
pub fn validate(conn: &Connection) -> Result<Vec<ValidationIssue>, MigrationError> {
    let mut issues = Vec::new();

    // 1. Intégrité physique.
    let quick: String = conn.query_row("PRAGMA quick_check", [], |row| row.get(0))?;
    if quick != "ok" {
        issues.push(ValidationIssue {
            severity: Severity::Data,
            message: format!("PRAGMA quick_check != ok: {quick}"),
        });
    }

    // 2. Clés étrangères violées.
    let mut stmt = conn.prepare("PRAGMA foreign_key_check")?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let table: String = row.get(0)?;
        let rowid: i64 = row.get(1)?;
        issues.push(ValidationIssue {
            severity: Severity::Data,
            message: format!("clé étrangère violée: {table} ligne {rowid}"),
        });
    }

    // 3. Liens orphelins (source et cible).
    let orphan_sql = "SELECT COUNT(*) FROM links l
        WHERE NOT EXISTS (SELECT 1 FROM blocks b WHERE b.id = l.source_block_id)
           OR NOT EXISTS (SELECT 1 FROM pages p WHERE p.id = l.target_page_id)";
    let has_links: bool = conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='links'",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map(|c| c > 0)
        .unwrap_or(false);
    if has_links {
        let orphans: i64 = conn.query_row(orphan_sql, [], |row| row.get(0))?;
        if orphans > 0 {
            issues.push(ValidationIssue {
                severity: Severity::Data,
                message: format!("{orphans} lien(s) orphelin(s) (source/cible absente)"),
            });
        }
    }

    // 4. Tables requises (délégation à Schema::validate_schema).
    for error in crate::schema::Schema::validate_schema(conn)? {
        issues.push(ValidationIssue {
            severity: Severity::Schema,
            message: error,
        });
    }

    Ok(issues)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    /// Ouvre une base VIDE (aucune table) dans un tempdir.
    fn empty_conn() -> (tempfile::TempDir, Connection) {
        let dir = tempfile::tempdir().unwrap();
        let conn = Connection::open(dir.path().join("test.db")).unwrap();
        (dir, conn)
    }

    /// Ouvre une base via le gestionnaire de migrations (registre par défaut).
    fn migrated_conn() -> (tempfile::TempDir, Connection) {
        let (dir, conn) = empty_conn();
        let manager = MigrationManager::new(
            &dir.path().join("test.db"),
            BackupConfig {
                enabled: false,
                ..BackupConfig::default()
            },
        );
        manager.migrate_default(&conn).unwrap();
        (dir, conn)
    }

    // --- Critère 1+2 : base vide → migration complète, migrations successives

    #[test]
    fn test_empty_db_migrates_to_latest() {
        let (_dir, conn) = empty_conn();
        let manager = MigrationManager::new(
            Path::new("unused.db"),
            BackupConfig {
                enabled: false,
                ..BackupConfig::default()
            },
        );

        let report = manager.migrate_default(&conn).unwrap();

        assert_eq!(report.from_version, 0);
        assert_eq!(report.to_version, latest_version());
        assert_eq!(report.applied.len(), default_migrations().len());
        assert_eq!(manager.current_version(&conn).unwrap(), latest_version());

        // Le schéma est complet : la validation ne signale que ce qui manque
        // éventuellement (rien).
        let issues = validate(&conn).unwrap();
        assert!(issues.is_empty(), "issues: {issues:?}");
    }

    #[test]
    fn test_successive_migrations() {
        let (_dir, conn) = empty_conn();
        let manager = MigrationManager::new(
            Path::new("unused.db"),
            BackupConfig {
                enabled: false,
                ..BackupConfig::default()
            },
        );

        // Applique uniquement la v1.
        let only_v1: Vec<Migration> = default_migrations().into_iter().take(1).collect();
        let report = manager.migrate(&conn, &only_v1).unwrap();
        assert_eq!(report.to_version, 1);
        assert_eq!(manager.current_version(&conn).unwrap(), 1);

        // La table de la v2 n'existe pas encore.
        let has_log: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master
                 WHERE type='table' AND name='migration_log'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(has_log, 0);

        // Puis le registre complet : seule la v2 est appliquée.
        let report = manager.migrate_default(&conn).unwrap();
        assert_eq!(report.from_version, 1);
        assert_eq!(report.to_version, latest_version());
        assert_eq!(report.applied.len(), 1);
        assert_eq!(report.applied[0].0, 2);

        // La trace de la v2 est dans migration_log.
        let logged: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM migration_log WHERE version = 2",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(logged, 1);
    }

    // --- Critère 2 : idempotence

    #[test]
    fn test_idempotence() {
        let (_dir, conn) = migrated_conn();
        let manager = MigrationManager::new(
            Path::new("unused.db"),
            BackupConfig {
                enabled: false,
                ..BackupConfig::default()
            },
        );

        // Deuxième exécution : no-op, aucune migration appliquée.
        let report = manager.migrate_default(&conn).unwrap();
        assert!(report.applied.is_empty());
        assert_eq!(report.from_version, report.to_version);
        assert!(report.backup.is_none(), "aucun backup pour un no-op");
        assert_eq!(manager.current_version(&conn).unwrap(), latest_version());
    }

    // --- Critère 1 : échec → rollback prouvé

    #[test]
    fn test_failure_rolls_back() {
        let (_dir, conn) = empty_conn();
        let manager = MigrationManager::new(
            Path::new("unused.db"),
            BackupConfig {
                enabled: false,
                ..BackupConfig::default()
            },
        );

        // Registre de test : v1 OK, v2 crée une table PUIS échoue.
        let registry = vec![
            Migration {
                version: 1,
                name: "ok".to_string(),
                up_sql: "CREATE TABLE IF NOT EXISTS t_ok (id INTEGER PRIMARY KEY);".to_string(),
            },
            Migration {
                version: 2,
                name: "echec".to_string(),
                up_sql: "CREATE TABLE IF NOT EXISTS t_avant_echec (id INTEGER PRIMARY KEY);
                         INSERT INTO table_inexistante VALUES (1);"
                    .to_string(),
            },
        ];

        let result = manager.migrate(&conn, &registry);
        assert!(result.is_err(), "la v2 doit échouer");

        // Rollback prouvé : user_version reste à 1 (pas 2).
        assert_eq!(manager.current_version(&conn).unwrap(), 1);

        // La table créée AVANT l'échec dans la même migration est absente :
        // la transaction a tout annulé.
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master
                 WHERE type='table' AND name='t_avant_echec'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 0, "t_avant_echec doit être absente (rollback)");

        // La v1 (committée avant l'échec) est toujours là.
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master
                 WHERE type='table' AND name='t_ok'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1, "t_ok doit exister (v1 committée)");
    }

    // --- Critère 3 : backup automatique avant migration

    #[test]
    fn test_backup_created_before_migration() {
        let (dir, conn) = empty_conn();
        let db_path = dir.path().join("test.db");
        let manager = MigrationManager::new(&db_path, BackupConfig::default());

        let report = manager.migrate_default(&conn).unwrap();

        // Le rapport cite le backup, et le fichier existe sur disque.
        let backup = report.backup.expect("backup créé avant migration");
        assert!(
            backup.path.is_file(),
            "fichier backup absent: {:?}",
            backup.path
        );
        assert!(backup.size_bytes > 0, "backup vide");

        // Il est bien dans le dossier de backups du module 4.5.
        let backup_dir = BackupManager::new(&db_path, BackupConfig::default()).backup_dir();
        assert_eq!(backup.path.parent().unwrap(), backup_dir);
    }

    #[test]
    fn test_backup_failure_aborts_migration() {
        let (dir, conn) = empty_conn();
        let db_path = dir.path().join("test.db");

        // BackupManager écrit dans <parent>/backups : on occupe ce chemin
        // par un FICHIER pour faire échouer create_dir_all.
        let poison = dir.path().join("backups");
        std::fs::write(&poison, b"").unwrap();

        let manager = MigrationManager::new(&db_path, BackupConfig::default());
        let result = manager.migrate_default(&conn);

        assert!(result.is_err(), "backup échoué → migration abortée");
        // Aucune migration appliquée : version inchangée, base vide.
        assert_eq!(manager.current_version(&conn).unwrap(), 0);
    }

    // --- Base plus récente que le registre : no-op, jamais de rétrogradage

    #[test]
    fn test_newer_db_is_noop() {
        let (_dir, conn) = empty_conn();
        let manager = MigrationManager::new(
            Path::new("unused.db"),
            BackupConfig {
                enabled: false,
                ..BackupConfig::default()
            },
        );

        // Simule une base ouverte par un binaire plus récent (v99) puis
        // rétrogradée : user_version dépasse le registre (v1/v2).
        conn.pragma_update(None, "user_version", 99).unwrap();
        assert_eq!(manager.current_version(&conn).unwrap(), 99);

        let report = manager.migrate_default(&conn).unwrap();

        // No-op : rien d'appliqué, version inchangée (jamais de downgrade),
        // aucun backup créé.
        assert!(report.applied.is_empty());
        assert_eq!(report.from_version, 99);
        assert_eq!(report.to_version, 99);
        assert!(report.backup.is_none());
        assert_eq!(manager.current_version(&conn).unwrap(), 99);
    }

    // --- Critère 4 : validation détecte une incohérence

    #[test]
    fn test_validation_clean_after_migration() {
        let (_dir, conn) = migrated_conn();
        let issues = validate(&conn).unwrap();
        assert!(
            issues.is_empty(),
            "base migrée cohérente, issues: {issues:?}"
        );
    }

    #[test]
    fn test_validation_detects_orphan_link() {
        let (_dir, conn) = migrated_conn();

        // Insère un lien orphelin en contournant les FK (foreign_keys=OFF,
        // comme le fait le vestige Storage de lib.rs qui n'active pas les FK).
        conn.pragma_update(None, "foreign_keys", "OFF").unwrap();
        let now = chrono::Utc::now().timestamp();
        conn.execute(
            "INSERT INTO links (id, uuid, source_block_id, source_page_id, target_page_id, link_type, created_at, updated_at, is_resolved)
             VALUES (1, 'orphan-uuid', 999, 999, 999, 'ref', ?1, ?1, FALSE)",
            rusqlite::params![now],
        )
        .unwrap();

        let issues = validate(&conn).unwrap();
        assert!(!issues.is_empty(), "l'orphelin doit être détecté");
        assert!(
            issues
                .iter()
                .any(|i| i.severity == Severity::Data && i.message.contains("orphelin")),
            "issues: {issues:?}"
        );
    }

    #[test]
    fn test_validation_detects_missing_table() {
        let (_dir, conn) = empty_conn();
        // Base vide : les tables requises manquent → problèmes de schéma.
        let issues = validate(&conn).unwrap();
        assert!(
            issues
                .iter()
                .any(|i| i.severity == Severity::Schema && i.message.contains("Missing table")),
            "issues: {issues:?}"
        );
    }
}
