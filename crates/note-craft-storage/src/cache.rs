//!
//! Système de cache multi-niveaux pour les requêtes fréquentes.
//!
//! Ce module fournit:
//! - **L1** : [`MemoryCache`] — cache mémoire par thread/processus, avec TTL
//!   optionnel par entrée et éviction LRU par capacité
//! - **L2** : [`SqliteCache`] — cache persistant adossé à une table SQLite
//!   (`cache_entries`), valeurs sérialisées en JSON, TTL stocké en epoch
//!   millisecondes, purge des entrées expirées
//! - [`MultiLevelCache`] : orchestration L1 → L2 (lecture : L1, sinon L2 avec
//!   promotion vers L1 ; écriture : les deux niveaux), invalidation par clé,
//!   par table (namespace) et globale
//! - Statistiques hit/miss par niveau ([`CacheStats`])
//!
//! Conventions : mêmes règles que `migrations.rs` (migrations.rs:1) — les
//! opérations L2 passent par la `Connection` fournie par l'appelant ; le
//! `Storage` de `lib.rs` (vestige) n'est jamais utilisé ici.
//!
//! Choix de design (sans nouvelle dépendance, Rust uniquement) :
//! - L1 protégée par `RwLock` (lecture concurrente, écriture exclusive) ;
//! - LRU maison via compteur d'accès (pas de crate `lru`) ;
//! - L2 protégée par `Mutex<Connection>` car `rusqlite::Connection` n'est
//!   pas `Sync` [À VÉRIFIER : version rusqlite du workspace, voir
//!   Cargo.lock] ;
//! - TTL L1 basé sur `std::time::Instant` (monotone), TTL L2 basé sur
//!   `chrono::Utc` (persistable entre sessions).
//!
//! Source auditée : feature Logseq `data_cache` — « Système de cache pour les
//! requêtes fréquentes », fichiers `src/main/frontend/common/cache.cljs` et
//! `src/main/frontend/worker/render_affected_keys.cljs`
//! (docs/audits/logseq_audit_results.json:637).
//!
//! Phase 4 - Étape 4.7 (Cache System) — docs/roadmap/roadmap_v3.md:818
//! Critère d'acceptation : Cache System: Cache multi-niveaux
//! (docs/roadmap/roadmap_v3.md:832).

use rusqlite::{params, Connection, OptionalExtension};
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, RwLock};
use std::time::{Duration, Instant};
use thiserror::Error;

// ============================================================================
// ERREURS
// ============================================================================

/// Erreurs du système de cache (miroir de `MigrationError`, migrations.rs:33).
#[derive(Error, Debug)]
pub enum CacheError {
    #[error("Erreur SQLite (cache L2): {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("Erreur de sérialisation (cache L2): {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Verrou du cache empoisonné: {0}")]
    Poisoned(String),
}

// ============================================================================
// CLÉS ET CONFIGURATION
// ============================================================================

/// Clé de cache, préfixée par la table (namespace) pour permettre
/// l'invalidation par table.
///
/// Format canonique : `{table}::{id}`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CacheKey {
    table: String,
    id: String,
}

impl CacheKey {
    /// Crée une clé pour une table et un identifiant donnés.
    pub fn new(table: &str, id: &str) -> Self {
        CacheKey {
            table: table.to_string(),
            id: id.to_string(),
        }
    }

    /// Représentation canonique plate (clé primaire L2, clé HashMap L1).
    pub fn as_flat(&self) -> String {
        format!("{}::{}", self.table, self.id)
    }

    /// Table (namespace) de la clé, pour l'invalidation par table.
    pub fn table(&self) -> &str {
        &self.table
    }
}

/// Configuration du cache multi-niveaux.
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Capacité maximale du L1 mémoire (entrées). Au-delà : éviction LRU.
    pub l1_capacity: usize,
    /// TTL par défaut (None = pas d'expiration).
    pub default_ttl: Option<Duration>,
}

impl Default for CacheConfig {
    fn default() -> Self {
        CacheConfig {
            l1_capacity: 1024,
            default_ttl: None,
        }
    }
}

// ============================================================================
// STATISTIQUES
// ============================================================================

/// Statistiques hit/miss d'un niveau de cache.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub inserts: u64,
    pub invalidations: u64,
}

impl CacheStats {
    /// Taux de hit en pourcentage (0.0 si aucune requête).
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            (self.hits as f64 / total as f64) * 100.0
        }
    }
}

// ============================================================================
// NIVEAU 1 : CACHE MÉMOIRE (TTL + LRU)
// ============================================================================

/// Entrée du cache mémoire.
#[derive(Debug)]
struct MemoryEntry<T> {
    value: T,
    /// Date d'expiration (None = pas d'expiration).
    expires_at: Option<Instant>,
    /// Compteur d'accès pour l'éviction LRU.
    last_access: u64,
}

/// Cache L1 : mémoire, TTL optionnel par entrée, éviction LRU par capacité.
pub struct MemoryCache<T> {
    entries: RwLock<HashMap<String, MemoryEntry<T>>>,
    capacity: usize,
    access_counter: AtomicU64,
    stats: RwLock<CacheStats>,
}

impl<T> MemoryCache<T> {
    /// Crée un cache mémoire de la capacité donnée.
    pub fn new(capacity: usize) -> Self {
        MemoryCache {
            entries: RwLock::new(HashMap::new()),
            capacity,
            access_counter: AtomicU64::new(0),
            stats: RwLock::new(CacheStats::default()),
        }
    }

    /// Lit une valeur. Retourne `None` sur miss ou entrée expirée
    /// (l'entrée expirée est alors supprimée — lazy expiration).
    pub fn get(&self, key: &CacheKey) -> Option<T>
    where
        T: Clone,
    {
        let now = Instant::now();
        let mut map = self.entries.write().ok()?; // None si poison
        match map.get_mut(&key.as_flat()) {
            Some(entry) => {
                if let Some(exp) = entry.expires_at {
                    if now >= exp {
                        map.remove(&key.as_flat());
                        self.record_miss();
                        return None;
                    }
                }
                entry.last_access = self.access_counter.fetch_add(1, Ordering::Relaxed);
                let value = entry.value.clone();
                self.record_hit();
                Some(value)
            }
            None => {
                self.record_miss();
                None
            }
        }
    }

    /// Insère une valeur avec TTL optionnel (None = pas d'expiration).
    /// Si la capacité est atteinte, l'entrée LRU est évincée.
    pub fn insert(&self, key: &CacheKey, value: T, ttl: Option<Duration>) {
        if let Ok(mut map) = self.entries.write() {
            if map.len() >= self.capacity && !map.contains_key(&key.as_flat()) {
                if let Some(lru_key) = map
                    .iter()
                    .min_by_key(|(_, e)| e.last_access)
                    .map(|(k, _)| k.clone())
                {
                    map.remove(&lru_key);
                }
            }
            map.insert(
                key.as_flat(),
                MemoryEntry {
                    value,
                    expires_at: ttl.map(|d| Instant::now() + d),
                    last_access: self.access_counter.fetch_add(1, Ordering::Relaxed),
                },
            );
            if let Ok(mut s) = self.stats.write() {
                s.inserts += 1;
            }
        }
    }

    /// Invalide une clé précise. Retourne `true` si une entrée existait.
    pub fn invalidate(&self, key: &CacheKey) -> bool {
        match self.entries.write() {
            Ok(mut map) => {
                let removed = map.remove(&key.as_flat()).is_some();
                if removed {
                    if let Ok(mut s) = self.stats.write() {
                        s.invalidations += 1;
                    }
                }
                removed
            }
            Err(_) => false,
        }
    }

    /// Invalide toutes les clés d'une table (namespace). Retourne le nombre
    /// d'entrées supprimées.
    pub fn invalidate_table(&self, table: &str) -> usize {
        match self.entries.write() {
            Ok(mut map) => {
                let prefix = format!("{}::", table);
                let keys: Vec<String> = map
                    .keys()
                    .filter(|k| k.starts_with(&prefix))
                    .cloned()
                    .collect();
                let count = keys.len();
                for k in keys {
                    map.remove(&k);
                }
                if let Ok(mut s) = self.stats.write() {
                    s.invalidations += count as u64;
                }
                count
            }
            Err(_) => 0,
        }
    }

    /// Vide entièrement le cache.
    pub fn clear(&self) {
        if let Ok(mut map) = self.entries.write() {
            map.clear();
        }
    }

    /// Nombre d'entrées présentes (y compris potentiellement expirées non
    /// encore lues — expiration lazy).
    pub fn len(&self) -> usize {
        self.entries.read().map(|m| m.len()).unwrap_or(0)
    }

    /// `true` si le cache est vide.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Statistiques courantes.
    pub fn stats(&self) -> CacheStats {
        self.stats.read().map(|s| *s).unwrap_or_default()
    }

    fn record_hit(&self) {
        if let Ok(mut s) = self.stats.write() {
            s.hits += 1;
        }
    }

    fn record_miss(&self) {
        if let Ok(mut s) = self.stats.write() {
            s.misses += 1;
        }
    }
}

// ============================================================================
// NIVEAU 2 : CACHE SQLITE (PERSISTANT)
// ============================================================================

/// Cache L2 : table SQLite `cache_entries`, valeurs JSON, TTL persistant.
///
/// La table est créée idempotemment à l'ouverture ([`SqliteCache::new`]).
/// Schéma : `key TEXT PRIMARY KEY, table_name TEXT, value TEXT (JSON),
/// expires_at INTEGER (epoch ms, NULL = pas d'expiration)`.
pub struct SqliteCache {
    conn: Mutex<Connection>,
    stats: RwLock<CacheStats>,
}

impl SqliteCache {
    /// Crée le cache L2 sur la connexion fournie (table créée si absente).
    pub fn new(conn: Connection) -> Result<Self, CacheError> {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS cache_entries (
                 key         TEXT PRIMARY KEY,
                 table_name  TEXT NOT NULL,
                 value       TEXT NOT NULL,
                 expires_at  INTEGER
             );
             CREATE INDEX IF NOT EXISTS idx_cache_entries_table
                 ON cache_entries(table_name);",
        )?;
        Ok(SqliteCache {
            conn: Mutex::new(conn),
            stats: RwLock::new(CacheStats::default()),
        })
    }

    /// Lit une valeur JSON désérialisée en `T`. Miss si absente ou expirée
    /// (l'entrée expirée est supprimée — lazy expiration).
    pub fn get<T>(&self, key: &CacheKey) -> Result<Option<T>, CacheError>
    where
        T: DeserializeOwned,
    {
        let conn = self
            .conn
            .lock()
            .map_err(|e| CacheError::Poisoned(e.to_string()))?;
        let row: Option<(String, Option<i64>)> = conn
            .query_row(
                "SELECT value, expires_at FROM cache_entries WHERE key = ?1",
                params![key.as_flat()],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()?;

        match row {
            None => {
                self.record_miss();
                Ok(None)
            }
            Some((json, expires_at)) => {
                if let Some(exp) = expires_at {
                    let now_ms = chrono::Utc::now().timestamp_millis();
                    if now_ms >= exp {
                        conn.execute(
                            "DELETE FROM cache_entries WHERE key = ?1",
                            params![key.as_flat()],
                        )?;
                        self.record_miss();
                        return Ok(None);
                    }
                }
                let value: T = serde_json::from_str(&json)?;
                self.record_hit();
                Ok(Some(value))
            }
        }
    }

    /// Insère (upsert) une valeur sérialisée en JSON avec TTL optionnel.
    pub fn insert<T>(
        &self,
        key: &CacheKey,
        value: &T,
        ttl: Option<Duration>,
    ) -> Result<(), CacheError>
    where
        T: Serialize + ?Sized,
    {
        let json = serde_json::to_string(value)?;
        let expires_at = ttl.map(|d| chrono::Utc::now().timestamp_millis() + d.as_millis() as i64);
        let conn = self
            .conn
            .lock()
            .map_err(|e| CacheError::Poisoned(e.to_string()))?;
        conn.execute(
            "INSERT OR REPLACE INTO cache_entries (key, table_name, value, expires_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![key.as_flat(), key.table(), json, expires_at],
        )?;
        if let Ok(mut s) = self.stats.write() {
            s.inserts += 1;
        }
        Ok(())
    }

    /// Invalide une clé précise. Retourne `true` si une entrée existait.
    pub fn invalidate(&self, key: &CacheKey) -> Result<bool, CacheError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| CacheError::Poisoned(e.to_string()))?;
        let n = conn.execute(
            "DELETE FROM cache_entries WHERE key = ?1",
            params![key.as_flat()],
        )?;
        if let Ok(mut s) = self.stats.write() {
            s.invalidations += n as u64;
        }
        Ok(n > 0)
    }

    /// Invalide toutes les clés d'une table. Retourne le nombre supprimé.
    pub fn invalidate_table(&self, table: &str) -> Result<usize, CacheError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| CacheError::Poisoned(e.to_string()))?;
        let n = conn.execute(
            "DELETE FROM cache_entries WHERE table_name = ?1",
            params![table],
        )?;
        if let Ok(mut s) = self.stats.write() {
            s.invalidations += n as u64;
        }
        Ok(n)
    }

    /// Supprime toutes les entrées expirées (expiration proactive).
    /// Retourne le nombre supprimé.
    pub fn purge_expired(&self) -> Result<usize, CacheError> {
        let now_ms = chrono::Utc::now().timestamp_millis();
        let conn = self
            .conn
            .lock()
            .map_err(|e| CacheError::Poisoned(e.to_string()))?;
        let n = conn.execute(
            "DELETE FROM cache_entries WHERE expires_at IS NOT NULL AND expires_at <= ?1",
            params![now_ms],
        )?;
        Ok(n)
    }

    /// Vide entièrement le cache L2.
    pub fn clear(&self) -> Result<(), CacheError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| CacheError::Poisoned(e.to_string()))?;
        conn.execute("DELETE FROM cache_entries", params![])?;
        Ok(())
    }

    /// Nombre d'entrées présentes.
    pub fn len(&self) -> Result<usize, CacheError> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| CacheError::Poisoned(e.to_string()))?;
        let n: i64 = conn.query_row("SELECT COUNT(*) FROM cache_entries", params![], |r| {
            r.get(0)
        })?;
        Ok(n as usize)
    }

    /// `true` si le cache L2 est vide.
    pub fn is_empty(&self) -> Result<bool, CacheError> {
        Ok(self.len()? == 0)
    }

    /// Statistiques courantes.
    pub fn stats(&self) -> CacheStats {
        self.stats.read().map(|s| *s).unwrap_or_default()
    }

    fn record_hit(&self) {
        if let Ok(mut s) = self.stats.write() {
            s.hits += 1;
        }
    }

    fn record_miss(&self) {
        if let Ok(mut s) = self.stats.write() {
            s.misses += 1;
        }
    }
}

// ============================================================================
// CACHE MULTI-NIVEAUX (L1 + L2)
// ============================================================================

/// Cache multi-niveaux : L1 mémoire (rapide, volatile) devant L2 SQLite
/// (persistant). Thread-safe, partageable via `Arc`.
///
/// Sémantique de lecture : L1 d'abord ; sur miss L1, L2 ; sur hit L2, la
/// valeur est **promue** dans L1 (lecture suivante servie par L1).
/// Sémantique d'écriture : les deux niveaux sont écrits.
/// Sémantique d'invalidation : les deux niveaux sont invalidés.
pub struct MultiLevelCache<T> {
    l1: MemoryCache<T>,
    l2: SqliteCache,
    default_ttl: Option<Duration>,
}

impl<T> MultiLevelCache<T>
where
    T: Clone + Serialize + DeserializeOwned,
{
    /// Crée le cache multi-niveaux avec la configuration donnée.
    pub fn new(config: &CacheConfig, conn: Connection) -> Result<Self, CacheError> {
        Ok(MultiLevelCache {
            l1: MemoryCache::new(config.l1_capacity),
            l2: SqliteCache::new(conn)?,
            default_ttl: config.default_ttl,
        })
    }

    /// Lit une valeur : L1 → L2 (avec promotion L1) → miss.
    pub fn get(&self, key: &CacheKey) -> Result<Option<T>, CacheError> {
        if let Some(v) = self.l1.get(key) {
            return Ok(Some(v));
        }
        if let Some(v) = self.l2.get::<T>(key)? {
            // Promotion L2 → L1 (sans TTL : la valeur L2 garde son TTL
            // propre ; L1 la sert jusqu'à invalidation).
            self.l1.insert(key, v.clone(), self.default_ttl);
            return Ok(Some(v));
        }
        Ok(None)
    }

    /// Insère une valeur dans les deux niveaux avec TTL optionnel
    /// (None = TTL par défaut de la configuration).
    pub fn insert(
        &self,
        key: &CacheKey,
        value: T,
        ttl: Option<Duration>,
    ) -> Result<(), CacheError> {
        let ttl = ttl.or(self.default_ttl);
        self.l1.insert(key, value.clone(), ttl);
        self.l2.insert(key, &value, ttl)?;
        Ok(())
    }

    /// Invalide une clé dans les deux niveaux.
    pub fn invalidate(&self, key: &CacheKey) -> Result<bool, CacheError> {
        let l1_removed = self.l1.invalidate(key);
        let l2_removed = self.l2.invalidate(key)?;
        Ok(l1_removed || l2_removed)
    }

    /// Invalide une table entière dans les deux niveaux.
    /// Retourne le total d'entrées supprimées.
    pub fn invalidate_table(&self, table: &str) -> Result<usize, CacheError> {
        let n1 = self.l1.invalidate_table(table);
        let n2 = self.l2.invalidate_table(table)?;
        Ok(n1 + n2)
    }

    /// Vide les deux niveaux.
    pub fn clear(&self) -> Result<(), CacheError> {
        self.l1.clear();
        self.l2.clear()?;
        Ok(())
    }

    /// Purge proactive des entrées L2 expirées.
    pub fn purge_expired(&self) -> Result<usize, CacheError> {
        self.l2.purge_expired()
    }

    /// Statistiques L1.
    pub fn l1_stats(&self) -> CacheStats {
        self.l1.stats()
    }

    /// Statistiques L2.
    pub fn l2_stats(&self) -> CacheStats {
        self.l2.stats()
    }

    /// Nombre d'entrées L1.
    pub fn l1_len(&self) -> usize {
        self.l1.len()
    }

    /// Nombre d'entrées L2.
    pub fn l2_len(&self) -> Result<usize, CacheError> {
        self.l2.len()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Barrier};
    use std::thread;

    fn conn() -> Connection {
        Connection::open_in_memory().unwrap()
    }

    fn key(table: &str, id: &str) -> CacheKey {
        CacheKey::new(table, id)
    }

    // ------------------------------------------------------------------
    // CacheKey
    // ------------------------------------------------------------------

    #[test]
    fn test_cache_key_flat_format() {
        let k = key("nodes", "42");
        assert_eq!(k.as_flat(), "nodes::42");
        assert_eq!(k.table(), "nodes");
    }

    #[test]
    fn test_cache_key_equality() {
        assert_eq!(key("nodes", "1"), key("nodes", "1"));
        assert_ne!(key("nodes", "1"), key("links", "1"));
        assert_ne!(key("nodes", "1"), key("nodes", "2"));
    }

    // ------------------------------------------------------------------
    // L1 : hit / miss / stats
    // ------------------------------------------------------------------

    #[test]
    fn test_l1_hit_and_miss() {
        let cache = MemoryCache::<String>::new(10);
        let k = key("nodes", "1");

        assert_eq!(cache.get(&k), None); // miss
        cache.insert(&k, "v1".to_string(), None);
        assert_eq!(cache.get(&k), Some("v1".to_string())); // hit

        let s = cache.stats();
        assert_eq!(s.hits, 1);
        assert_eq!(s.misses, 1);
        assert_eq!(s.inserts, 1);
        assert!((s.hit_rate() - 50.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_l1_overwrite() {
        let cache = MemoryCache::<u64>::new(10);
        let k = key("nodes", "1");
        cache.insert(&k, 1, None);
        cache.insert(&k, 2, None);
        assert_eq!(cache.get(&k), Some(2));
        assert_eq!(cache.len(), 1);
    }

    // ------------------------------------------------------------------
    // L1 : TTL
    // ------------------------------------------------------------------

    #[test]
    fn test_l1_ttl_expiration() {
        let cache = MemoryCache::<String>::new(10);
        let k = key("nodes", "1");
        cache.insert(&k, "v".to_string(), Some(Duration::from_millis(20)));
        assert_eq!(cache.get(&k), Some("v".to_string())); // pas encore expirée
        thread::sleep(Duration::from_millis(30));
        assert_eq!(cache.get(&k), None); // expirée
        assert_eq!(cache.len(), 0); // supprimée (lazy expiration)
    }

    #[test]
    fn test_l1_no_ttl_never_expires() {
        let cache = MemoryCache::<String>::new(10);
        let k = key("nodes", "1");
        cache.insert(&k, "v".to_string(), None);
        thread::sleep(Duration::from_millis(10));
        assert_eq!(cache.get(&k), Some("v".to_string()));
    }

    // ------------------------------------------------------------------
    // L1 : invalidation
    // ------------------------------------------------------------------

    #[test]
    fn test_l1_invalidate_key() {
        let cache = MemoryCache::<String>::new(10);
        let k = key("nodes", "1");
        cache.insert(&k, "v".to_string(), None);
        assert!(cache.invalidate(&k));
        assert_eq!(cache.get(&k), None);
        assert!(!cache.invalidate(&k)); // déjà absente
    }

    #[test]
    fn test_l1_invalidate_table_only_matching() {
        let cache = MemoryCache::<String>::new(10);
        cache.insert(&key("nodes", "1"), "a".to_string(), None);
        cache.insert(&key("nodes", "2"), "b".to_string(), None);
        cache.insert(&key("links", "1"), "c".to_string(), None);

        let removed = cache.invalidate_table("nodes");
        assert_eq!(removed, 2);
        assert_eq!(cache.get(&key("nodes", "1")), None);
        assert_eq!(cache.get(&key("nodes", "2")), None);
        assert_eq!(cache.get(&key("links", "1")), Some("c".to_string())); // épargnée
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_l1_clear() {
        let cache = MemoryCache::<String>::new(10);
        cache.insert(&key("nodes", "1"), "a".to_string(), None);
        cache.insert(&key("links", "1"), "b".to_string(), None);
        cache.clear();
        assert!(cache.is_empty());
    }

    // ------------------------------------------------------------------
    // L1 : éviction LRU
    // ------------------------------------------------------------------

    #[test]
    fn test_l1_lru_eviction() {
        let cache = MemoryCache::<u64>::new(2);
        let k1 = key("nodes", "1");
        let k2 = key("nodes", "2");
        let k3 = key("nodes", "3");

        cache.insert(&k1, 1, None);
        cache.insert(&k2, 2, None);
        assert_eq!(cache.get(&k1), Some(1)); // k1 devient plus récente que k2
        cache.insert(&k3, 3, None); // capacité atteinte → éviction LRU (k2)

        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get(&k1), Some(1)); // conservée (récemment utilisée)
        assert_eq!(cache.get(&k2), None); // évincée
        assert_eq!(cache.get(&k3), Some(3)); // conservée
    }

    // ------------------------------------------------------------------
    // L2 : hit / miss / persistance JSON
    // ------------------------------------------------------------------

    #[test]
    fn test_l2_hit_and_miss() {
        let cache = SqliteCache::new(conn()).unwrap();
        let k = key("nodes", "1");
        assert_eq!(cache.get::<String>(&k).unwrap(), None); // miss
        cache.insert(&k, "v1", None).unwrap();
        assert_eq!(cache.get::<String>(&k).unwrap(), Some("v1".to_string())); // hit

        let s = cache.stats();
        assert_eq!(s.hits, 1);
        assert_eq!(s.misses, 1);
        assert_eq!(s.inserts, 1);
    }

    #[test]
    fn test_l2_struct_serialization() {
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
        struct Node {
            id: u64,
            content: String,
        }
        let cache = SqliteCache::new(conn()).unwrap();
        let k = key("nodes", "7");
        let node = Node {
            id: 7,
            content: "hello".to_string(),
        };
        cache.insert(&k, &node, None).unwrap();
        let loaded: Node = cache.get(&k).unwrap().unwrap();
        assert_eq!(loaded, node);
    }

    #[test]
    fn test_l2_upsert() {
        let cache = SqliteCache::new(conn()).unwrap();
        let k = key("nodes", "1");
        cache.insert(&k, "v1", None).unwrap();
        cache.insert(&k, "v2", None).unwrap();
        assert_eq!(cache.get::<String>(&k).unwrap(), Some("v2".to_string()));
        assert_eq!(cache.len().unwrap(), 1);
    }

    // ------------------------------------------------------------------
    // L2 : TTL
    // ------------------------------------------------------------------

    #[test]
    fn test_l2_ttl_expiration_and_purge() {
        let cache = SqliteCache::new(conn()).unwrap();
        let k = key("nodes", "1");
        cache
            .insert(&k, "v", Some(Duration::from_millis(20)))
            .unwrap();
        assert_eq!(cache.get::<String>(&k).unwrap(), Some("v".to_string()));
        thread::sleep(Duration::from_millis(30));
        assert_eq!(cache.get::<String>(&k).unwrap(), None); // expirée (lazy)
        assert_eq!(cache.len().unwrap(), 0); // supprimée au passage
    }

    #[test]
    fn test_l2_purge_expired_proactive() {
        let cache = SqliteCache::new(conn()).unwrap();
        cache
            .insert(&key("nodes", "1"), "a", Some(Duration::from_millis(20)))
            .unwrap();
        cache.insert(&key("nodes", "2"), "b", None).unwrap();
        thread::sleep(Duration::from_millis(30));
        let purged = cache.purge_expired().unwrap();
        assert_eq!(purged, 1);
        assert_eq!(cache.len().unwrap(), 1); // seule l'entrée sans TTL reste
        assert_eq!(
            cache.get::<String>(&key("nodes", "2")).unwrap(),
            Some("b".to_string())
        );
    }

    // ------------------------------------------------------------------
    // L2 : invalidation
    // ------------------------------------------------------------------

    #[test]
    fn test_l2_invalidate_key() {
        let cache = SqliteCache::new(conn()).unwrap();
        let k = key("nodes", "1");
        cache.insert(&k, "v", None).unwrap();
        assert!(cache.invalidate(&k).unwrap());
        assert!(!cache.invalidate(&k).unwrap()); // déjà absente
        assert_eq!(cache.get::<String>(&k).unwrap(), None);
    }

    #[test]
    fn test_l2_invalidate_table() {
        let cache = SqliteCache::new(conn()).unwrap();
        cache.insert(&key("nodes", "1"), "a", None).unwrap();
        cache.insert(&key("nodes", "2"), "b", None).unwrap();
        cache.insert(&key("links", "1"), "c", None).unwrap();

        let removed = cache.invalidate_table("nodes").unwrap();
        assert_eq!(removed, 2);
        assert_eq!(cache.len().unwrap(), 1);
        assert_eq!(
            cache.get::<String>(&key("links", "1")).unwrap(),
            Some("c".to_string())
        );
    }

    #[test]
    fn test_l2_clear() {
        let cache = SqliteCache::new(conn()).unwrap();
        cache.insert(&key("nodes", "1"), "a", None).unwrap();
        cache.clear().unwrap();
        assert!(cache.is_empty().unwrap());
    }

    // ------------------------------------------------------------------
    // Multi-niveaux
    // ------------------------------------------------------------------

    fn multi() -> MultiLevelCache<String> {
        let config = CacheConfig::default();
        MultiLevelCache::new(&config, conn()).unwrap()
    }

    #[test]
    fn test_multi_miss_then_insert_then_hit() {
        let cache = multi();
        let k = key("nodes", "1");
        assert_eq!(cache.get(&k).unwrap(), None); // miss des deux niveaux

        cache.insert(&k, "v1".to_string(), None).unwrap();
        assert_eq!(cache.get(&k).unwrap(), Some("v1".to_string()));
        assert_eq!(cache.l1_len(), 1);
        assert_eq!(cache.l2_len().unwrap(), 1);
    }

    #[test]
    fn test_multi_l2_promotion_to_l1() {
        let cache = multi();
        let k = key("nodes", "1");
        // Écriture directe en L2 uniquement (simule un autre process).
        cache.l2.insert(&k, "v", None).unwrap();
        assert_eq!(cache.l1_len(), 0);

        // Lecture : miss L1, hit L2 → promotion.
        assert_eq!(cache.get(&k).unwrap(), Some("v".to_string()));
        assert_eq!(cache.l1_len(), 1);
        assert_eq!(cache.l1_stats().hits, 0); // le hit est compté en L2
        assert_eq!(cache.l2_stats().hits, 1);

        // Lecture suivante : servie par L1.
        assert_eq!(cache.get(&k).unwrap(), Some("v".to_string()));
        assert_eq!(cache.l1_stats().hits, 1);
    }

    #[test]
    fn test_multi_invalidate_key_both_levels() {
        let cache = multi();
        let k = key("nodes", "1");
        cache.insert(&k, "v".to_string(), None).unwrap();
        assert!(cache.invalidate(&k).unwrap());
        assert_eq!(cache.l1_len(), 0);
        assert_eq!(cache.l2_len().unwrap(), 0);
        assert_eq!(cache.get(&k).unwrap(), None);
    }

    #[test]
    fn test_multi_invalidate_table_both_levels() {
        let cache = multi();
        cache
            .insert(&key("nodes", "1"), "a".to_string(), None)
            .unwrap();
        cache
            .insert(&key("nodes", "2"), "b".to_string(), None)
            .unwrap();
        cache
            .insert(&key("links", "1"), "c".to_string(), None)
            .unwrap();

        let removed = cache.invalidate_table("nodes").unwrap();
        assert_eq!(removed, 4); // 2 en L1 + 2 en L2
        assert_eq!(cache.get(&key("nodes", "1")).unwrap(), None);
        assert_eq!(
            cache.get(&key("links", "1")).unwrap(),
            Some("c".to_string())
        );
    }

    #[test]
    fn test_multi_ttl_expires_both_levels() {
        let cache = multi();
        let k = key("nodes", "1");
        cache
            .insert(&k, "v".to_string(), Some(Duration::from_millis(20)))
            .unwrap();
        thread::sleep(Duration::from_millis(30));
        assert_eq!(cache.get(&k).unwrap(), None); // expirée dans les deux niveaux
        assert_eq!(cache.l2_len().unwrap(), 0); // L2 purge au passage
    }

    #[test]
    fn test_multi_default_ttl_from_config() {
        let config = CacheConfig {
            l1_capacity: 10,
            default_ttl: Some(Duration::from_millis(20)),
        };
        let cache = MultiLevelCache::<String>::new(&config, conn()).unwrap();
        let k = key("nodes", "1");
        // TTL None → TTL par défaut appliqué.
        cache.insert(&k, "v".to_string(), None).unwrap();
        thread::sleep(Duration::from_millis(30));
        assert_eq!(cache.get(&k).unwrap(), None);
    }

    #[test]
    fn test_multi_clear() {
        let cache = multi();
        cache
            .insert(&key("nodes", "1"), "a".to_string(), None)
            .unwrap();
        cache.clear().unwrap();
        assert_eq!(cache.l1_len(), 0);
        assert_eq!(cache.l2_len().unwrap(), 0);
    }

    // ------------------------------------------------------------------
    // Concurrence
    // ------------------------------------------------------------------

    #[test]
    fn test_l1_concurrent_access() {
        let cache = Arc::new(MemoryCache::<u64>::new(512));
        let barrier = Arc::new(Barrier::new(4));
        let mut handles = Vec::new();

        for t in 0..4u64 {
            let cache = Arc::clone(&cache);
            let barrier = Arc::clone(&barrier);
            handles.push(thread::spawn(move || {
                barrier.wait();
                for i in 0..100u64 {
                    let k = key("nodes", &format!("{}-{}", t, i));
                    cache.insert(&k, i, None);
                    assert_eq!(cache.get(&k), Some(i));
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(cache.len(), 400);
        assert_eq!(cache.stats().hits, 400);
    }

    #[test]
    fn test_multi_concurrent_read_write() {
        let cache = Arc::new(multi());
        let barrier = Arc::new(Barrier::new(4));
        let mut handles = Vec::new();

        for t in 0..4u64 {
            let cache = Arc::clone(&cache);
            let barrier = Arc::clone(&barrier);
            handles.push(thread::spawn(move || {
                barrier.wait();
                for i in 0..50u64 {
                    let k = key("nodes", &format!("{}-{}", t, i));
                    cache.insert(&k, format!("v-{}-{}", t, i), None).unwrap();
                    let got = cache.get(&k).unwrap();
                    assert_eq!(got, Some(format!("v-{}-{}", t, i)));
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(cache.l2_len().unwrap(), 200);
    }
}
