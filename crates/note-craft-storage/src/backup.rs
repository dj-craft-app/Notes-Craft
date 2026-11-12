//!
//! Système de sauvegarde automatique de la base SQLite.
//!
//! Ce module fournit:
//! - Création de sauvegardes autonomes de la base via `VACUUM INTO`
//! - Sauvegardes automatiques déclenchées par l'appelant ([`BackupManager::auto_backup`])
//! - Rétention automatique (seules les N plus récentes sont conservées)
//! - Listing et restauration des sauvegardes
//!
//! Il suit les conventions de `exporter.rs` / `importer.rs` : les opérations
//! passent par la `Connection` fournie par l'appelant (entrée canonique de la
//! base : `schema::Schema::initialize_database`, schema.rs:481). Le `Storage`
//! de `lib.rs` (vestige) n'est jamais utilisé ici.
//!
//! Source auditée : feature Logseq `data_backup` — « Système de sauvegarde
//! automatique », namespace `frontend.worker.db.backup`
//! (docs/audits/logseq_audit_results.json:618-637).
//!
//! Arbitrages tranchés par l'issue #108 (documentés aux points concernés):
//! 1. Copie par `VACUUM INTO '<chemin>'` via la `Connection` : base autonome,
//!    SQL pur, zéro nouvelle dépendance.
//! 2. `BackupConfig` avec `Default` = (true, 5, false), miroir des clés de
//!    configuration `storage.backup_enabled`, `storage.backup_count`,
//!    `storage.compress_backups` (config.rs:912-914). Le module ne lit PAS
//!    note-craft-core : la configuration est fournie par l'appelant, sans
//!    couplage à la crate core.
//! 3. Nommage : dossier `<dossier_de_la_db>/backups/`, fichiers
//!    `note_craft_YYYYMMDD_HHMMSS.db` (horodatage chrono local). Collision
//!    dans la même seconde → suffixe `_2`, `_3`, ...
//! 4. Rétention automatique : tri par nom (les noms étant horodatés, tri par
//!    nom = tri chronologique), seules les `max_backups` plus récentes sont
//!    conservées.
//! 5. `auto_backup(&self, conn)` : si `!enabled` → `Ok(None)` ; sinon création
//!    + purge. Appelée par l'appelant aux points de sauvegarde/fermeture.
//! 6. `compress = true` → sauvegarde NON compressée + `log::warn!` : aucune
//!    crate de compression n'est autorisée dans le workspace.
//!
//! Phase 4 - Étape 4.5 (Backup System) — docs/roadmap/roadmap_v3.md:816
//! Critère d'acceptation : « Backup System: Sauvegardes automatiques »
//! (docs/roadmap/roadmap_v3.md:831)

use chrono::{DateTime, Local};
use rusqlite::Connection;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

// ============================================================================
// ERREURS
// ============================================================================

/// Erreurs de sauvegarde (miroir de `ExportError`, exporter.rs:27).
#[derive(Error, Debug)]
pub enum BackupError {
    #[error("Erreur IO: {0}")]
    IoError(String),
    #[error("Erreur de stockage: {0}")]
    StorageError(String),
    #[error("Sauvegarde non trouvée: {0}")]
    BackupNotFound(String),
    #[error("Nom de sauvegarde invalide: {0}")]
    InvalidName(String),
}

impl From<std::io::Error> for BackupError {
    fn from(err: std::io::Error) -> Self {
        BackupError::IoError(err.to_string())
    }
}

impl From<rusqlite::Error> for BackupError {
    fn from(err: rusqlite::Error) -> Self {
        BackupError::StorageError(err.to_string())
    }
}

// ============================================================================
// CONFIGURATION
// ============================================================================

/// Configuration du système de sauvegarde.
///
/// `Default` = (enabled: true, max_backups: 5, compress: false), miroir des
/// clés de configuration `storage.backup_enabled`, `storage.backup_count`,
/// `storage.compress_backups` (config.rs:912-914). Arbitrage #108.2 : le
/// module n'est PAS couplé à note-craft-core, l'appelant fournit la
/// configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackupConfig {
    /// Sauvegardes automatiques activées (`storage.backup_enabled`).
    pub enabled: bool,
    /// Nombre de sauvegardes conservées au maximum (`storage.backup_count`).
    pub max_backups: usize,
    /// Compression demandée (`storage.compress_backups`). Arbitrage #108.6 :
    /// conservé pour parité avec la config, mais aucune crate de compression
    /// n'existe dans le workspace → sauvegarde NON compressée + `log::warn!`.
    pub compress: bool,
}

impl Default for BackupConfig {
    fn default() -> Self {
        BackupConfig {
            enabled: true,
            max_backups: 5,
            compress: false,
        }
    }
}

// ============================================================================
// STRUCTURES
// ============================================================================

/// Informations sur une sauvegarde existante.
#[derive(Debug, Clone)]
pub struct BackupInfo {
    /// Nom du fichier (ex. `note_craft_20240106_143002.db`).
    pub name: String,
    /// Chemin complet du fichier de sauvegarde.
    pub path: PathBuf,
    /// Date de création (mtime du fichier, heure locale).
    pub created_at: DateTime<Local>,
    /// Taille du fichier en octets.
    pub size_bytes: u64,
}

/// Gestionnaire de sauvegardes de la base SQLite.
///
/// Les sauvegardes vivent dans `<dossier_de_la_db>/backups/` (arbitrage
/// #108.3) et sont créées via `VACUUM INTO` sur la `Connection` fournie par
/// l'appelant (arbitrage #108.1 : base autonome, SQL pur, zéro dépendance).
pub struct BackupManager {
    /// Chemin du fichier de base de données à sauvegarder.
    db_path: PathBuf,
    /// Configuration (rétention, activation, compression).
    config: BackupConfig,
}

impl BackupManager {
    /// Crée un gestionnaire de sauvegardes pour la base `db_path`.
    pub fn new(db_path: &Path, config: BackupConfig) -> Self {
        BackupManager {
            db_path: db_path.to_path_buf(),
            config,
        }
    }

    /// Dossier des sauvegardes : `<dossier_de_la_db>/backups/`.
    pub fn backup_dir(&self) -> PathBuf {
        let parent = self
            .db_path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));
        parent.join("backups")
    }

    /// Crée une sauvegarde autonome de la base via `VACUUM INTO`
    /// (arbitrage #108.1) et renvoie ses informations.
    ///
    /// Le fichier produit est une base SQLite complète et autonome : `VACUUM
    /// INTO` produit une copie compactée, utilisable sans les fichiers
    /// `-wal`/`-shm` de la base d'origine.
    pub fn create_backup(&self, conn: &Connection) -> Result<BackupInfo, BackupError> {
        if self.config.compress {
            // Arbitrage #108.6 : aucune crate de compression dans le workspace.
            log::warn!(
                "note-craft-storage: compress_backups=true, mais aucune crate de \
                 compression n'existe dans le workspace (issue #108, arbitrage 6) ; \
                 la sauvegarde n'est PAS compressée"
            );
        }

        let dir = self.backup_dir();
        fs::create_dir_all(&dir)?;

        let now = Local::now();
        let path = unique_backup_path(&dir, now)?;

        // VACUUM INTO '<chemin>' : les quotes simples du chemin sont échappées
        // selon les règles des littéraux SQL ('' pour ').
        let escaped = path.to_string_lossy().replace('\'', "''");
        conn.execute_batch(&format!("VACUUM INTO '{}'", escaped))?;

        let meta = fs::metadata(&path)?;
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .ok_or_else(|| {
                BackupError::IoError(format!(
                    "chemin de sauvegarde sans nom de fichier: {}",
                    escaped
                ))
            })?;

        Ok(BackupInfo {
            name,
            path,
            created_at: DateTime::<Local>::from(meta.modified()?),
            size_bytes: meta.len(),
        })
    }

    /// Exécute la sauvegarde automatique de l'appelant (points de
    /// sauvegarde / fermeture), puis la purge par rétention.
    ///
    /// Arbitrage #108.5 : si `!enabled` → `Ok(None)` sans aucun accès disque ;
    /// sinon création puis conservation des `max_backups` plus récentes.
    pub fn auto_backup(&self, conn: &Connection) -> Result<Option<BackupInfo>, BackupError> {
        if !self.config.enabled {
            return Ok(None);
        }
        let info = self.create_backup(conn)?;
        self.prune_backups()?;
        Ok(Some(info))
    }

    /// Rétention : ne conserve que les `max_backups` plus récentes
    /// (arbitrage #108.4 : tri par nom = tri chronologique, les noms étant
    /// horodatés).
    fn prune_backups(&self) -> Result<(), BackupError> {
        let mut names = self.backup_names()?;
        while names.len() > self.config.max_backups {
            let oldest = names.remove(0);
            fs::remove_file(self.backup_dir().join(&oldest))?;
        }
        Ok(())
    }

    /// Liste les sauvegardes existantes, triées par nom croissant (donc par
    /// date croissante, arbitrage #108.4).
    pub fn list_backups(&self) -> Result<Vec<BackupInfo>, BackupError> {
        let mut infos = Vec::new();
        for name in self.backup_names()? {
            let path = self.backup_dir().join(&name);
            let meta = fs::metadata(&path)?;
            infos.push(BackupInfo {
                name,
                path,
                created_at: DateTime::<Local>::from(meta.modified()?),
                size_bytes: meta.len(),
            });
        }
        Ok(infos)
    }

    /// Restaure une sauvegarde en copiant son fichier vers `target`.
    ///
    /// `name` doit désigner un nom de fichier du dossier de sauvegardes
    /// (pas de chemin) : tout nom traversant (`../`, séparateur) ou ne
    /// respectant pas le format `note_craft_YYYYMMDD_HHMMSS[_N].db` est
    /// rejeté par [`BackupError::InvalidName`].
    pub fn restore_backup(&self, name: &str, target: &Path) -> Result<(), BackupError> {
        validate_name(name)?;
        let source = self.backup_dir().join(name);
        if !source.is_file() {
            return Err(BackupError::BackupNotFound(name.to_string()));
        }
        if let Some(parent) = target.parent() {
            if !parent.as_os_str().is_empty() && !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        fs::copy(&source, target)?;
        Ok(())
    }

    /// Noms des sauvegardes présentes dans le dossier, triés par nom
    /// croissant (les noms étant horodatés, tri par nom = tri chronologique,
    /// arbitrage #108.4).
    fn backup_names(&self) -> Result<Vec<String>, BackupError> {
        let dir = self.backup_dir();
        let mut names: Vec<String> = Vec::new();
        if !dir.exists() {
            return Ok(names);
        }
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().to_string();
            if is_backup_name(&name) {
                names.push(name);
            }
        }
        names.sort();
        Ok(names)
    }
}

/// Chemin de sauvegarde unique pour l'instant `now` : `note_craft_<stamp>.db`,
/// avec suffixe `_2`, `_3`, ... en cas de collision dans la même seconde
/// (arbitrage #108.3). Plafond à 999 collisions par seconde pour éviter
/// l'ordre lexicographique artificiel de `_10` vs `_2`.
fn unique_backup_path(dir: &Path, now: DateTime<Local>) -> Result<PathBuf, BackupError> {
    let stamp = now.format("%Y%m%d_%H%M%S");
    let mut candidate = dir.join(format!("note_craft_{}.db", stamp));
    let mut n: usize = 1;
    while candidate.exists() {
        n += 1;
        if n > 999 {
            return Err(BackupError::IoError(format!(
                "trop de sauvegardes dans la même seconde (>= 999) dans {}",
                dir.display()
            )));
        }
        candidate = dir.join(format!("note_craft_{}_{}.db", stamp, n));
    }
    Ok(candidate)
}

/// Vérifie qu'un nom de fichier est un nom de sauvegarde valide :
/// `note_craft_<chiffres et '_'>.db` (arbitrage #108.3).
fn is_backup_name(name: &str) -> bool {
    match name
        .strip_prefix("note_craft_")
        .and_then(|s| s.strip_suffix(".db"))
    {
        Some(stem) if !stem.is_empty() => stem.bytes().all(|b| b.is_ascii_digit() || b == b'_'),
        _ => false,
    }
}

/// Valide un nom utilisé par [`BackupManager::restore_backup`] : nom simple,
/// sans séparateur, sans traversée (`../`), au format sauvegarde.
fn validate_name(name: &str) -> Result<(), BackupError> {
    if name.is_empty()
        || name == "."
        || name == ".."
        || name.contains('/')
        || name.contains('\\')
        || name.contains("..")
        || !is_backup_name(name)
    {
        return Err(BackupError::InvalidName(name.to_string()));
    }
    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    /// Initialise une base canonique (schéma 4.1, schema.rs:481) contenant
    /// une page, et renvoie (chemin de la base, Connection).
    fn init_db_with_page(dir: &Path) -> (PathBuf, Connection) {
        let db_path = dir.join("test.db");
        let conn = crate::schema::Schema::initialize_database(&db_path)
            .expect("initialize_database doit réussir");
        conn.execute(
            "INSERT INTO pages (uuid, path, title, filename, created_at, updated_at) \
             VALUES ('uuid-1', '/tmp/page.md', 'Page 1', 'page.md', 1, 1)",
            [],
        )
        .expect("INSERT INTO pages doit réussir");
        (db_path, conn)
    }

    #[test]
    fn test_create_backup_creates_file() {
        let temp = tempfile::tempdir().unwrap();
        let (db_path, conn) = init_db_with_page(temp.path());
        let manager = BackupManager::new(&db_path, BackupConfig::default());

        let info = manager.create_backup(&conn).unwrap();

        assert!(is_backup_name(&info.name), "nom invalide: {}", info.name);
        assert!(
            info.path.starts_with(manager.backup_dir()),
            "la sauvegarde doit vivre dans backups/"
        );
        assert!(info.path.is_file());
        assert!(info.size_bytes > 0);
    }

    #[test]
    fn test_name_collision_suffix() {
        // Arbitrage #108.3 : collision dans la même seconde → _2, _3, ...
        let temp = tempfile::tempdir().unwrap();
        let backups = temp.path().join("backups");
        fs::create_dir_all(&backups).unwrap();
        let now = Local.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
        fs::write(backups.join("note_craft_20240101_120000.db"), b"a").unwrap();
        fs::write(backups.join("note_craft_20240101_120000_2.db"), b"a").unwrap();

        let path = unique_backup_path(&backups, now).unwrap();

        assert_eq!(
            path.file_name().unwrap().to_string_lossy(),
            "note_craft_20240101_120000_3.db"
        );
    }

    #[test]
    fn test_retention_keeps_only_max_backups() {
        let temp = tempfile::tempdir().unwrap();
        let (db_path, conn) = init_db_with_page(temp.path());
        let config = BackupConfig {
            max_backups: 2,
            ..Default::default()
        };
        let manager = BackupManager::new(&db_path, config);

        let first = manager.auto_backup(&conn).unwrap().expect("enabled → Some");
        let _second = manager.auto_backup(&conn).unwrap().expect("enabled → Some");
        let third = manager.auto_backup(&conn).unwrap().expect("enabled → Some");

        let list = manager.list_backups().unwrap();
        assert_eq!(list.len(), 2, "rétention max_backups=2");
        assert!(
            list.iter().any(|b| b.name == third.name),
            "la plus récente doit être conservée"
        );
        assert!(
            !list.iter().any(|b| b.name == first.name),
            "la plus ancienne doit être purgée (tri par nom = chrono, arbitrage 4)"
        );
        assert!(
            !manager.backup_dir().join(&first.name).exists(),
            "le fichier purgé doit être supprimé du disque"
        );
    }

    #[test]
    fn test_disabled_auto_backup_returns_none() {
        let temp = tempfile::tempdir().unwrap();
        let (db_path, conn) = init_db_with_page(temp.path());
        let config = BackupConfig {
            enabled: false,
            ..Default::default()
        };
        let manager = BackupManager::new(&db_path, config);

        let result = manager.auto_backup(&conn).unwrap();

        assert!(result.is_none(), "!enabled → Ok(None), arbitrage 5");
        assert!(manager.list_backups().unwrap().is_empty());
        assert!(
            !manager.backup_dir().exists(),
            "aucun dossier backups/ ne doit être créé"
        );
    }

    #[test]
    fn test_list_backups_sorted_and_filtered() {
        let temp = tempfile::tempdir().unwrap();
        let (db_path, _conn) = init_db_with_page(temp.path());
        let manager = BackupManager::new(&db_path, BackupConfig::default());
        let dir = manager.backup_dir();
        fs::create_dir_all(&dir).unwrap();
        for name in [
            "note_craft_20240101_120001.db",
            "note_craft_20240101_120000.db",
            "note_craft_20240102_090000.db",
        ] {
            fs::write(dir.join(name), b"x").unwrap();
        }
        // Fichiers parasites ignorés.
        fs::write(dir.join("note_craft_bad.db"), b"x").unwrap();
        fs::write(dir.join("other.db"), b"x").unwrap();
        fs::write(dir.join(".hidden.tmp"), b"x").unwrap();

        let list = manager.list_backups().unwrap();

        let names: Vec<&str> = list.iter().map(|b| b.name.as_str()).collect();
        assert_eq!(
            names,
            [
                "note_craft_20240101_120000.db",
                "note_craft_20240101_120001.db",
                "note_craft_20240102_090000.db",
            ],
            "liste triée par nom croissant (chrono), hors fichiers parasites"
        );
    }

    #[test]
    fn test_restore_backup_usable_with_data() {
        let temp = tempfile::tempdir().unwrap();
        let (db_path, conn) = init_db_with_page(temp.path());
        let manager = BackupManager::new(&db_path, BackupConfig::default());
        let info = manager.create_backup(&conn).unwrap();

        let target_dir = tempfile::tempdir().unwrap();
        let target = target_dir.path().join("restored").join("notes.db");
        manager.restore_backup(&info.name, &target).unwrap();

        let restored = Connection::open(&target).unwrap();
        let count: i64 = restored
            .query_row(
                "SELECT COUNT(*) FROM pages WHERE uuid = 'uuid-1'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(count, 1, "la base restaurée doit contenir les données");
    }

    #[test]
    fn test_backup_is_standalone() {
        // `initialize_database` active le mode WAL (schema.rs:505) : le fichier
        // produit par VACUUM INTO doit pourtant s'ouvrir et se vérifier seul,
        // sans les fichiers -wal/-shm de la base d'origine (arbitrage 1).
        let temp = tempfile::tempdir().unwrap();
        let (db_path, conn) = init_db_with_page(temp.path());
        let manager = BackupManager::new(&db_path, BackupConfig::default());
        let info = manager.create_backup(&conn).unwrap();

        let standalone = Connection::open(&info.path).unwrap();
        let integrity: String = standalone
            .query_row("PRAGMA integrity_check", [], |r| r.get(0))
            .unwrap();
        assert_eq!(integrity, "ok");
        let count: i64 = standalone
            .query_row("SELECT COUNT(*) FROM pages", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_restore_not_found() {
        let temp = tempfile::tempdir().unwrap();
        let (db_path, _conn) = init_db_with_page(temp.path());
        let manager = BackupManager::new(&db_path, BackupConfig::default());
        let target_dir = tempfile::tempdir().unwrap();
        let target = target_dir.path().join("out.db");

        let err = manager
            .restore_backup("note_craft_19990101_000000.db", &target)
            .unwrap_err();

        assert!(
            matches!(err, BackupError::BackupNotFound(ref n) if n == "note_craft_19990101_000000.db"),
            "attendu BackupNotFound, obtenu {:?}",
            err
        );
    }

    #[test]
    fn test_restore_rejects_traversal_and_invalid_names() {
        let temp = tempfile::tempdir().unwrap();
        let (db_path, _conn) = init_db_with_page(temp.path());
        let manager = BackupManager::new(&db_path, BackupConfig::default());
        let target_dir = tempfile::tempdir().unwrap();
        let target = target_dir.path().join("out.db");

        for bad in [
            "../note_craft_20240101_000000.db",
            "sub/note_craft_20240101_000000.db",
            "..\\note_craft_20240101_000000.db",
            "..",
            "note_craft_..db",
            "note_craft_20240101_000000.db.bak",
            "note_craft_.db",
            "",
        ] {
            let err = manager.restore_backup(bad, &target).unwrap_err();
            assert!(
                matches!(err, BackupError::InvalidName(_)),
                "attendu InvalidName pour {:?}, obtenu {:?}",
                bad,
                err
            );
        }
    }

    #[test]
    fn test_default_config() {
        let default = BackupConfig::default();
        assert_eq!(
            default,
            BackupConfig {
                enabled: true,
                max_backups: 5,
                compress: false,
            },
            "Default = (true, 5, false), miroir config.rs:912-914"
        );
    }
}
