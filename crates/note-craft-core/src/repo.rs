//! Note Craft Repository Management System
//!
//! Ce module fournit un système de gestion de repository pour Note Craft.
//!
//! Phase 1 - Étape 1.6: Repository Management

use crate::config::{Config, ConfigManager};
use crate::fs::FileSystem;
use crate::page::PageSystem;
use crate::state::StateManager;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use thiserror::Error;

// ============================================================================
// TYPES DE REPOSITORY
// ============================================================================

/// Identifiant unique pour un repository
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RepoId(pub u64);

impl Default for RepoId {
    fn default() -> Self {
        Self::new()
    }
}

impl RepoId {
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        RepoId(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}

impl fmt::Display for RepoId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RepoId({})", self.0)
    }
}

/// État d'un repository
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RepoStatus {
    /// Non initialisé
    Uninitialized,
    /// Ouvert
    Open,
    /// Fermé
    Closed,
    /// Corrompu
    Corrupted,
    /// En cours de synchronisation
    Syncing,
}

impl fmt::Display for RepoStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepoStatus::Uninitialized => write!(f, "Uninitialized"),
            RepoStatus::Open => write!(f, "Open"),
            RepoStatus::Closed => write!(f, "Closed"),
            RepoStatus::Corrupted => write!(f, "Corrupted"),
            RepoStatus::Syncing => write!(f, "Syncing"),
        }
    }
}

/// Type de repository
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RepoType {
    /// Local (sur le filesystem)
    Local,
    /// Distant (non implémenté pour l'instant)
    Remote,
}

impl fmt::Display for RepoType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepoType::Local => write!(f, "Local"),
            RepoType::Remote => write!(f, "Remote"),
        }
    }
}

/// Métadonnées d'un repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoMetadata {
    /// Nom du repository
    pub name: String,
    /// Description
    pub description: Option<String>,
    /// Version de Note Craft
    pub note_craft_version: Option<String>,
    /// Date de création
    pub created_at: u64,
    /// Date de modification
    pub updated_at: u64,
    /// Dernière ouverture
    pub last_opened: Option<u64>,
    /// Auteur
    pub author: Option<String>,
}

impl RepoMetadata {
    pub fn new(name: String) -> Self {
        RepoMetadata {
            name,
            description: None,
            note_craft_version: None,
            created_at: Self::current_timestamp(),
            updated_at: Self::current_timestamp(),
            last_opened: None,
            author: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
        self.updated_at = Self::current_timestamp();
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
        self.updated_at = Self::current_timestamp();
    }

    fn current_timestamp() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }
}

/// Configuration spécifique à un repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoConfig {
    /// Configuration générale
    pub config: Config,
    /// Chemins spécifiques au repository
    pub paths: RepoPaths,
}

impl Default for RepoConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl RepoConfig {
    pub fn new() -> Self {
        RepoConfig {
            config: Config::new(),
            paths: RepoPaths::new(),
        }
    }
}

/// Chemins spécifiques à un repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoPaths {
    /// Chemin du dossier des blocs
    pub blocks_dir: PathBuf,
    /// Chemin du dossier des pages
    pub pages_dir: PathBuf,
    /// Chemin du dossier de configuration
    pub config_dir: PathBuf,
    /// Chemin du dossier de cache
    pub cache_dir: PathBuf,
    /// Chemin du dossier des assets
    pub assets_dir: PathBuf,
}

impl RepoPaths {
    pub fn new() -> Self {
        RepoPaths {
            blocks_dir: PathBuf::from("blocks"),
            pages_dir: PathBuf::from("pages"),
            config_dir: PathBuf::from("config"),
            cache_dir: PathBuf::from(".cache"),
            assets_dir: PathBuf::from("assets"),
        }
    }
}

impl Default for RepoPaths {
    fn default() -> Self {
        Self::new()
    }
}

impl RepoPaths {
    /// Crée les chemins basés sur un chemin de base
    pub fn with_base<P: AsRef<Path>>(base: P) -> Self {
        let base = base.as_ref();
        RepoPaths {
            blocks_dir: base.join("blocks"),
            pages_dir: base.join("pages"),
            config_dir: base.join("config"),
            cache_dir: base.join(".cache"),
            assets_dir: base.join("assets"),
        }
    }

    /// Vérifie que tous les chemins existent, crée les dossiers si nécessaire
    pub fn ensure_exists(&self) -> std::io::Result<()> {
        use std::fs;

        fs::create_dir_all(&self.blocks_dir)?;
        fs::create_dir_all(&self.pages_dir)?;
        fs::create_dir_all(&self.config_dir)?;
        fs::create_dir_all(&self.cache_dir)?;
        fs::create_dir_all(&self.assets_dir)?;

        Ok(())
    }
}

/// Informations sur un repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoInfo {
    /// ID du repository
    pub id: RepoId,
    /// Métadonnées
    pub metadata: RepoMetadata,
    /// Type de repository
    pub repo_type: RepoType,
    /// Chemin du repository
    pub path: PathBuf,
    /// Statut
    pub status: RepoStatus,
}

impl RepoInfo {
    pub fn new<P: AsRef<Path>>(path: P, name: String, repo_type: RepoType) -> Self {
        RepoInfo {
            id: RepoId::new(),
            metadata: RepoMetadata::new(name),
            repo_type,
            path: path.as_ref().to_path_buf(),
            status: RepoStatus::Uninitialized,
        }
    }

    /// Obtient le nom
    pub fn name(&self) -> &str {
        &self.metadata.name
    }

    /// Définit le statut
    pub fn set_status(&mut self, status: RepoStatus) {
        self.status = status;
    }
}

// ============================================================================
// ERREURS DE REPOSITORY
// ============================================================================

#[derive(Error, Debug)]
pub enum RepoError {
    #[error("Erreur IO: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Repository introuvable: {0}")]
    NotFound(String),
    #[error("Repository déjà ouvert: {0}")]
    AlreadyOpen(String),
    #[error("Repository déjà existe: {0}")]
    AlreadyExists(String),
    #[error("Repository corrompu: {0}")]
    Corrupted(String),
    #[error("Chemin invalide: {0}")]
    InvalidPath(String),
    #[error("Nom de repository invalide")]
    InvalidName,
    #[error("Repository verrouillé: {0}")]
    Locked(String),
}

// ============================================================================
// ÉVÉNEMENTS DE REPOSITORY
// ============================================================================

/// Type d'événement de repository
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepoEventType {
    /// Repository créé
    Created,
    /// Repository ouvert
    Opened,
    /// Repository fermé
    Closed,
    /// Repository supprimé
    Deleted,
    /// Statut changé
    StatusChanged,
    /// Configuration changée
    ConfigChanged,
}

/// Événement de repository
#[derive(Debug, Clone)]
pub struct RepoEvent {
    /// Type d'événement
    pub event_type: RepoEventType,
    /// ID du repository
    pub repo_id: Option<RepoId>,
    /// Nom du repository
    pub repo_name: Option<String>,
    /// Chemin du repository
    pub repo_path: Option<PathBuf>,
    /// Ancien statut
    pub old_status: Option<RepoStatus>,
    /// Nouveau statut
    pub new_status: Option<RepoStatus>,
    /// Timestamp
    pub timestamp: u64,
}

impl RepoEvent {
    pub fn new(event_type: RepoEventType) -> Self {
        RepoEvent {
            event_type,
            repo_id: None,
            repo_name: None,
            repo_path: None,
            old_status: None,
            new_status: None,
            timestamp: Self::current_timestamp(),
        }
    }

    pub fn with_repo(mut self, info: &RepoInfo) -> Self {
        self.repo_id = Some(info.id);
        self.repo_name = Some(info.name().to_string());
        self.repo_path = Some(info.path.clone());
        self
    }

    pub fn with_status_change(mut self, old_status: RepoStatus, new_status: RepoStatus) -> Self {
        self.old_status = Some(old_status);
        self.new_status = Some(new_status);
        self
    }

    fn current_timestamp() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }
}

/// Trait pour écouter les événements de repository
pub trait RepoEventListener: Send + Sync {
    /// Appelé lorsqu'un événement se produit
    fn on_repo_event(&self, event: &RepoEvent);
}

// ============================================================================
// REPOSITORY
// ============================================================================

/// Un repository Note Craft
#[derive(Clone)]
pub struct Repository {
    /// Informations du repository
    info: RepoInfo,
    /// Système de pages
    page_system: Arc<RwLock<PageSystem>>,
    /// Système de fichiers
    fs: Arc<dyn FileSystem>,
    /// Configuration
    config: RepoConfig,
    /// Gestionnaire de configuration
    config_manager: Option<Arc<ConfigManager>>,
    /// Gestionnaire d'état
    state_manager: Option<Arc<StateManager>>,
}

impl Repository {
    /// Crée un nouveau repository
    pub fn new<P: AsRef<Path>>(
        path: P,
        name: String,
        repo_type: RepoType,
        page_system: Arc<RwLock<PageSystem>>,
        fs: Arc<dyn FileSystem>,
    ) -> Self {
        let path_ref = path.as_ref();
        let info = RepoInfo::new(path_ref, name, repo_type);
        let paths = RepoPaths::with_base(path_ref);

        Repository {
            info,
            page_system,
            fs,
            config: RepoConfig {
                config: Config::new(),
                paths,
            },
            config_manager: None,
            state_manager: None,
        }
    }

    /// Initialise le repository (crée les dossiers)
    pub fn init(&mut self) -> Result<(), RepoError> {
        self.config.paths.ensure_exists()?;
        self.info.set_status(RepoStatus::Uninitialized);
        Ok(())
    }

    /// Ouvre le repository
    pub fn open(&mut self) -> Result<(), RepoError> {
        if self.info.status == RepoStatus::Open {
            return Err(RepoError::AlreadyOpen(self.info.name().to_string()));
        }

        // Initialiser si nécessaire
        if self.info.status == RepoStatus::Uninitialized {
            self.init()?;
        }

        self.info.set_status(RepoStatus::Open);
        self.info.metadata.last_opened = Some(RepoMetadata::current_timestamp());

        Ok(())
    }

    /// Ferme le repository
    pub fn close(&mut self) -> Result<(), RepoError> {
        if self.info.status != RepoStatus::Open {
            return Ok(());
        }

        self.info.set_status(RepoStatus::Closed);
        Ok(())
    }

    /// Obtient les informations du repository
    pub fn info(&self) -> &RepoInfo {
        &self.info
    }

    /// Obtient l'ID du repository
    pub fn id(&self) -> RepoId {
        self.info.id
    }

    /// Obtient le nom du repository
    pub fn name(&self) -> &str {
        &self.info.metadata.name
    }

    /// Obtient le chemin du repository
    pub fn path(&self) -> &PathBuf {
        &self.info.path
    }

    /// Obtient le statut du repository
    pub fn status(&self) -> RepoStatus {
        self.info.status
    }

    /// Définit le gestionnaire de configuration
    pub fn set_config_manager(&mut self, config_manager: Arc<ConfigManager>) {
        self.config_manager = Some(config_manager);
    }

    /// Définit le gestionnaire d'état
    pub fn set_state_manager(&mut self, state_manager: Arc<StateManager>) {
        self.state_manager = Some(state_manager);
    }

    /// Obtient le PageSystem
    pub fn page_system(&self) -> Arc<RwLock<PageSystem>> {
        self.page_system.clone()
    }

    /// Obtient le FileSystem
    pub fn fs(&self) -> Arc<dyn FileSystem> {
        self.fs.clone()
    }
}

// ============================================================================
// REPOSITORY MANAGER
// ============================================================================

/// Gestionnaire de repositories
///
/// Ce struct gère tous les repositories Note Craft ouverts.
pub struct RepoManager {
    /// Map des repositories par ID
    repos: RwLock<HashMap<RepoId, Repository>>,
    /// Repository actuel
    current_repo: RwLock<Option<RepoId>>,
    /// Système de fichiers
    fs: Arc<dyn FileSystem>,
    /// Chemin de base pour les repositories
    base_path: PathBuf,
    /// Écouteurs d'événements
    listeners: RwLock<Vec<Arc<dyn RepoEventListener>>>,
}

impl RepoManager {
    /// Crée un nouveau RepoManager
    pub fn new(fs: Arc<dyn FileSystem>) -> Self {
        RepoManager {
            repos: RwLock::new(HashMap::new()),
            current_repo: RwLock::new(None),
            fs,
            base_path: PathBuf::new(),
            listeners: RwLock::new(Vec::new()),
        }
    }

    /// Crée un RepoManager avec un chemin de base
    pub fn with_base_path<P: AsRef<Path>>(fs: Arc<dyn FileSystem>, base_path: P) -> Self {
        RepoManager {
            repos: RwLock::new(HashMap::new()),
            current_repo: RwLock::new(None),
            fs,
            base_path: base_path.as_ref().to_path_buf(),
            listeners: RwLock::new(Vec::new()),
        }
    }

    /// Crée un nouveau repository
    pub fn create_repo(
        &self,
        name: String,
        path: Option<PathBuf>,
        repo_type: RepoType,
    ) -> Result<RepoId, RepoError> {
        // Valider le nom
        if name.is_empty() || name.contains('/') || name.contains('\\') {
            return Err(RepoError::InvalidName);
        }

        // Déterminer le chemin
        let repo_path = if let Some(p) = path {
            p
        } else {
            self.base_path.join(&name)
        };

        // Vérifier que le chemin n'existe pas déjà
        if repo_path.exists() {
            return Err(RepoError::AlreadyExists(name));
        }

        // Créer le repository
        let page_system = Arc::new(RwLock::new(PageSystem::new()));
        let mut repo = Repository::new(
            repo_path.clone(),
            name.clone(),
            repo_type,
            page_system,
            self.fs.clone(),
        );

        // Initialiser
        repo.init()?;

        // Ajouter au gestionnaire
        let repo_id = repo.id();
        {
            let mut repos = self.repos.write().unwrap();
            repos.insert(repo_id, repo);
        }

        // Notifier
        self.emit_event(RepoEvent::new(RepoEventType::Created).with_repo(&RepoInfo {
            id: repo_id,
            metadata: RepoMetadata::new(name),
            repo_type,
            path: repo_path,
            status: RepoStatus::Uninitialized,
        }));

        Ok(repo_id)
    }

    /// Ouvre un repository
    pub fn open_repo(&self, repo_id: RepoId) -> Result<(), RepoError> {
        let mut repos = self.repos.write().unwrap();
        if let Some(repo) = repos.get_mut(&repo_id) {
            repo.open()?;

            // Définir comme repository actuel
            let mut current = self.current_repo.write().unwrap();
            *current = Some(repo_id);

            // Notifier
            self.emit_event(
                RepoEvent::new(RepoEventType::Opened)
                    .with_repo(repo.info())
                    .with_status_change(RepoStatus::Closed, RepoStatus::Open),
            );

            Ok(())
        } else {
            Err(RepoError::NotFound(format!(
                "Repository {} not found",
                repo_id.0
            )))
        }
    }

    /// Ferme un repository
    pub fn close_repo(&self, repo_id: RepoId) -> Result<(), RepoError> {
        let mut repos = self.repos.write().unwrap();
        if let Some(repo) = repos.get_mut(&repo_id) {
            repo.close()?;

            // Si c'est le repository actuel, le désélectionner
            let mut current = self.current_repo.write().unwrap();
            if *current == Some(repo_id) {
                *current = None;
            }

            // Notifier
            self.emit_event(
                RepoEvent::new(RepoEventType::Closed)
                    .with_repo(repo.info())
                    .with_status_change(RepoStatus::Open, RepoStatus::Closed),
            );

            Ok(())
        } else {
            Err(RepoError::NotFound(format!(
                "Repository {} not found",
                repo_id.0
            )))
        }
    }

    /// Supprime un repository
    pub fn delete_repo(&self, repo_id: RepoId, force: bool) -> Result<(), RepoError> {
        // Vérifier que le repository n'est pas ouvert
        {
            let repos = self.repos.read().unwrap();
            if let Some(repo) = repos.get(&repo_id) {
                if repo.status() == RepoStatus::Open && !force {
                    return Err(RepoError::Locked(format!(
                        "Repository {} is open",
                        repo.name()
                    )));
                }
            }
        }

        let mut repos = self.repos.write().unwrap();
        if let Some(repo) = repos.remove(&repo_id) {
            // Supprimer le dossier du repository
            if repo.info.repo_type == RepoType::Local {
                std::fs::remove_dir_all(&repo.info.path)?;
            }

            // Si c'est le repository actuel, le désélectionner
            let mut current = self.current_repo.write().unwrap();
            if *current == Some(repo_id) {
                *current = None;
            }

            // Notifier
            self.emit_event(RepoEvent::new(RepoEventType::Deleted).with_repo(repo.info()));

            Ok(())
        } else {
            Err(RepoError::NotFound(format!(
                "Repository {} not found",
                repo_id.0
            )))
        }
    }

    /// Définit le repository actuel
    pub fn set_current_repo(&self, repo_id: RepoId) -> Result<(), RepoError> {
        let repos = self.repos.read().unwrap();
        if !repos.contains_key(&repo_id) {
            return Err(RepoError::NotFound(format!(
                "Repository {} not found",
                repo_id.0
            )));
        }

        let mut current = self.current_repo.write().unwrap();
        *current = Some(repo_id);

        Ok(())
    }

    /// Obtient le repository actuel (clone local, non thread-safe)
    pub fn get_current_repo(&self) -> Option<Repository> {
        let current = self.current_repo.read().unwrap();
        if let Some(repo_id) = *current {
            let repos = self.repos.read().unwrap();
            if let Some(repo) = repos.get(&repo_id) {
                return Some(repo.clone());
            }
        }
        None
    }

    /// Obtient un repository par ID (clone local, non thread-safe)
    pub fn get_repo(&self, repo_id: RepoId) -> Option<Repository> {
        let repos = self.repos.read().unwrap();
        repos.get(&repo_id).cloned()
    }

    /// Liste tous les repositories
    pub fn list_repos(&self) -> Vec<RepoInfo> {
        let repos = self.repos.read().unwrap();
        repos.values().map(|r| r.info.clone()).collect()
    }

    /// Liste les repositories ouverts
    pub fn list_open_repos(&self) -> Vec<RepoInfo> {
        let repos = self.repos.read().unwrap();
        repos
            .values()
            .filter(|r| r.status() == RepoStatus::Open)
            .map(|r| r.info.clone())
            .collect()
    }

    /// Obtient le nombre de repositories
    pub fn count(&self) -> usize {
        let repos = self.repos.read().unwrap();
        repos.len()
    }

    /// Vérifie si un repository existe
    pub fn contains(&self, repo_id: RepoId) -> bool {
        let repos = self.repos.read().unwrap();
        repos.contains_key(&repo_id)
    }

    /// Obtient le chemin de base
    pub fn base_path(&self) -> &PathBuf {
        &self.base_path
    }

    /// Définit le chemin de base
    pub fn set_base_path<P: AsRef<Path>>(&mut self, path: P) {
        self.base_path = path.as_ref().to_path_buf();
    }

    /// Ajoute un écouteur
    pub fn add_listener(&self, listener: Arc<dyn RepoEventListener>) {
        let mut listeners = self.listeners.write().unwrap();
        listeners.push(listener);
    }

    /// Supprime un écouteur
    pub fn remove_listener(&self, listener: &Arc<dyn RepoEventListener>) {
        let mut listeners = self.listeners.write().unwrap();
        listeners.retain(|l| !Arc::ptr_eq(l, listener));
    }

    /// Émet un événement
    fn emit_event(&self, event: RepoEvent) {
        let listeners = self.listeners.read().unwrap();
        for listener in listeners.iter() {
            listener.on_repo_event(&event);
        }
    }

    /// Scanne les repositories existants dans le chemin de base
    pub fn scan_repos(&mut self) -> Result<Vec<RepoId>, RepoError> {
        if !self.base_path.exists() {
            std::fs::create_dir_all(&self.base_path)?;
        }

        let mut found_repos = Vec::new();

        for entry in std::fs::read_dir(&self.base_path)? {
            let entry = entry?;
            let path = entry.path();

            // Vérifier si c'est un dossier avec un fichier de configuration Note Craft
            if path.is_dir() {
                let config_path = path.join("note-craft.json");
                if config_path.exists() {
                    // C'est probablement un repository Note Craft
                    let name = entry.file_name().to_string_lossy().to_string();

                    // Créer le repository
                    let page_system = Arc::new(RwLock::new(PageSystem::new()));
                    let repo = Repository::new(
                        path.clone(),
                        name,
                        RepoType::Local,
                        page_system,
                        self.fs.clone(),
                    );
                    let repo_id = repo.id();

                    // Ajouter au gestionnaire
                    {
                        let mut repos = self.repos.write().unwrap();
                        repos.insert(repo_id, repo);
                    }

                    found_repos.push(repo_id);
                }
            }
        }

        Ok(found_repos)
    }
}

impl Default for RepoManager {
    fn default() -> Self {
        use crate::fs::LocalFileSystem;
        Self::new(Arc::new(LocalFileSystem))
    }
}

impl fmt::Display for RepoManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RepoManager {{ repos: {}, current: {:?} }}",
            self.count(),
            self.current_repo.read().unwrap()
        )
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::LocalFileSystem;
    use std::sync::Arc;
    use tempfile::tempdir;

    // Mock listener pour les tests
    struct TestRepoListener {
        events: RwLock<Vec<RepoEvent>>,
    }

    impl TestRepoListener {
        fn new() -> Arc<Self> {
            Arc::new(Self {
                events: RwLock::new(Vec::new()),
            })
        }

        fn get_events(&self) -> Vec<RepoEvent> {
            let events = self.events.read().unwrap();
            events.clone()
        }
    }

    impl RepoEventListener for TestRepoListener {
        fn on_repo_event(&self, event: &RepoEvent) {
            let mut events = self.events.write().unwrap();
            events.push(event.clone());
        }
    }

    #[test]
    fn test_repo_id_new() {
        let id1 = RepoId::new();
        let id2 = RepoId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_repo_metadata_new() {
        let metadata = RepoMetadata::new("Test Repo".to_string());
        assert_eq!(metadata.name, "Test Repo");
        assert!(metadata.created_at > 0);
    }

    #[test]
    fn test_repo_paths_new() {
        let paths = RepoPaths::new();
        assert_eq!(paths.blocks_dir, PathBuf::from("blocks"));
        assert_eq!(paths.pages_dir, PathBuf::from("pages"));
    }

    #[test]
    fn test_repo_paths_with_base() {
        let paths = RepoPaths::with_base("/test/repo");
        assert_eq!(paths.blocks_dir, PathBuf::from("/test/repo/blocks"));
        assert_eq!(paths.pages_dir, PathBuf::from("/test/repo/pages"));
    }

    #[test]
    fn test_repo_info_new() {
        let info = RepoInfo::new("/test/repo", "Test Repo".to_string(), RepoType::Local);
        assert_eq!(info.name(), "Test Repo");
        assert_eq!(info.repo_type, RepoType::Local);
        assert_eq!(info.path, PathBuf::from("/test/repo"));
        assert_eq!(info.status, RepoStatus::Uninitialized);
    }

    #[test]
    fn test_repo_info_set_status() {
        let mut info = RepoInfo::new("/test/repo", "Test Repo".to_string(), RepoType::Local);
        info.set_status(RepoStatus::Open);
        assert_eq!(info.status, RepoStatus::Open);
    }

    #[test]
    fn test_repository_new() {
        let fs = Arc::new(LocalFileSystem);
        let page_system = Arc::new(RwLock::new(PageSystem::new()));
        let repo = Repository::new(
            "/test/repo",
            "Test Repo".to_string(),
            RepoType::Local,
            page_system,
            fs,
        );

        assert_eq!(repo.name(), "Test Repo");
        assert_eq!(repo.path(), &PathBuf::from("/test/repo"));
        assert_eq!(repo.status(), RepoStatus::Uninitialized);
    }

    #[test]
    fn test_repo_manager_new() {
        let fs = Arc::new(LocalFileSystem);
        let manager = RepoManager::new(fs);
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_repo_manager_create_repo() {
        let dir = tempdir().unwrap();
        let fs = Arc::new(LocalFileSystem);
        let manager = RepoManager::with_base_path(fs, dir.path());

        let repo_id = manager
            .create_repo("Test Repo".to_string(), None, RepoType::Local)
            .unwrap();

        assert_eq!(manager.count(), 1);
        assert!(manager.contains(repo_id));

        let repos = manager.list_repos();
        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].name(), "Test Repo");
    }

    #[test]
    fn test_repo_manager_create_repo_invalid_name() {
        let fs = Arc::new(LocalFileSystem);
        let manager = RepoManager::new(fs);

        let result = manager.create_repo("".to_string(), None, RepoType::Local);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RepoError::InvalidName));
    }

    #[test]
    fn test_repo_manager_create_repo_already_exists() {
        let dir = tempdir().unwrap();
        let fs = Arc::new(LocalFileSystem);
        let manager = RepoManager::with_base_path(fs.clone(), dir.path());

        // Créer un dossier manuellement
        let repo_path = dir.path().join("Existing Repo");
        std::fs::create_dir_all(&repo_path).unwrap();

        let result = manager.create_repo("Existing Repo".to_string(), None, RepoType::Local);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RepoError::AlreadyExists(_)));
    }

    #[test]
    fn test_repo_manager_open_repo() {
        let dir = tempdir().unwrap();
        let fs = Arc::new(LocalFileSystem);
        let manager = RepoManager::with_base_path(fs, dir.path());

        let repo_id = manager
            .create_repo("Test Repo".to_string(), None, RepoType::Local)
            .unwrap();
        manager.open_repo(repo_id).unwrap();

        let repos = manager.list_open_repos();
        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].status, RepoStatus::Open);
    }

    #[test]
    fn test_repo_manager_close_repo() {
        let dir = tempdir().unwrap();
        let fs = Arc::new(LocalFileSystem);
        let manager = RepoManager::with_base_path(fs, dir.path());

        let repo_id = manager
            .create_repo("Test Repo".to_string(), None, RepoType::Local)
            .unwrap();
        manager.open_repo(repo_id).unwrap();
        manager.close_repo(repo_id).unwrap();

        let repos = manager.list_open_repos();
        assert_eq!(repos.len(), 0);
    }

    #[test]
    fn test_repo_manager_set_current_repo() {
        let dir = tempdir().unwrap();
        let fs = Arc::new(LocalFileSystem);
        let manager = RepoManager::with_base_path(fs, dir.path());

        let repo_id = manager
            .create_repo("Test Repo".to_string(), None, RepoType::Local)
            .unwrap();
        manager.set_current_repo(repo_id).unwrap();

        let current = manager.get_current_repo();
        assert!(current.is_some());
        assert_eq!(current.as_ref().unwrap().id(), repo_id);
    }

    #[test]
    fn test_repo_manager_delete_repo() {
        let dir = tempdir().unwrap();
        let fs = Arc::new(LocalFileSystem);
        let manager = RepoManager::with_base_path(fs, dir.path());

        let repo_id = manager
            .create_repo("Test Repo".to_string(), None, RepoType::Local)
            .unwrap();
        manager.delete_repo(repo_id, true).unwrap();

        assert_eq!(manager.count(), 0);
        assert!(!manager.contains(repo_id));
    }

    #[test]
    fn test_repo_manager_delete_open_repo_without_force() {
        let dir = tempdir().unwrap();
        let fs = Arc::new(LocalFileSystem);
        let manager = RepoManager::with_base_path(fs, dir.path());

        let repo_id = manager
            .create_repo("Test Repo".to_string(), None, RepoType::Local)
            .unwrap();
        manager.open_repo(repo_id).unwrap();

        let result = manager.delete_repo(repo_id, false);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RepoError::Locked(_)));
    }

    #[test]
    fn test_repo_manager_event_listener() {
        let dir = tempdir().unwrap();
        let fs = Arc::new(LocalFileSystem);
        let manager = RepoManager::with_base_path(fs, dir.path());
        let listener = TestRepoListener::new();

        manager.add_listener(listener.clone());

        manager
            .create_repo("Test Repo".to_string(), None, RepoType::Local)
            .unwrap();

        let events = listener.get_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, RepoEventType::Created);
    }

    #[test]
    fn test_repo_manager_scan_repos() {
        let dir = tempdir().unwrap();
        let fs = Arc::new(LocalFileSystem);

        // Créer un faux repository
        let repo_path = dir.path().join("Test Repo");
        std::fs::create_dir_all(&repo_path).unwrap();
        std::fs::write(repo_path.join("note-craft.json"), "{}").unwrap();

        let mut manager = RepoManager::with_base_path(fs, dir.path());
        let repos = manager.scan_repos().unwrap();

        assert_eq!(repos.len(), 1);
    }

    #[test]
    fn test_repo_config_new() {
        let config = RepoConfig::new();
        assert!(config.config.is_empty());
    }

    #[test]
    fn test_repo_manager_base_path() {
        let fs = Arc::new(LocalFileSystem);
        let base_path = PathBuf::from("/test/base");
        let manager = RepoManager::with_base_path(fs, &base_path);
        assert_eq!(manager.base_path(), &base_path);
    }

    #[test]
    fn test_repo_status_display() {
        assert_eq!(format!("{}", RepoStatus::Uninitialized), "Uninitialized");
        assert_eq!(format!("{}", RepoStatus::Open), "Open");
        assert_eq!(format!("{}", RepoStatus::Closed), "Closed");
        assert_eq!(format!("{}", RepoStatus::Corrupted), "Corrupted");
        assert_eq!(format!("{}", RepoStatus::Syncing), "Syncing");
    }

    #[test]
    fn test_repo_type_display() {
        assert_eq!(format!("{}", RepoType::Local), "Local");
        assert_eq!(format!("{}", RepoType::Remote), "Remote");
    }

    #[test]
    fn test_repo_error_display() {
        let error = RepoError::NotFound("test".to_string());
        assert_eq!(format!("{}", error), "Repository introuvable: test");
    }

    #[test]
    fn test_repo_event_new() {
        let event = RepoEvent::new(RepoEventType::Created);
        assert_eq!(event.event_type, RepoEventType::Created);
        assert!(event.repo_id.is_none());
        assert!(event.repo_name.is_none());
    }

    #[test]
    fn test_repo_paths_ensure_exists() {
        let dir = tempdir().unwrap();
        let paths = RepoPaths::with_base(dir.path());

        paths.ensure_exists().unwrap();

        assert!(dir.path().join("blocks").exists());
        assert!(dir.path().join("pages").exists());
        assert!(dir.path().join("config").exists());
        assert!(dir.path().join(".cache").exists());
        assert!(dir.path().join("assets").exists());
    }

    #[test]
    fn test_repo_manager_list_open_repos() {
        let dir = tempdir().unwrap();
        let fs = Arc::new(LocalFileSystem);
        let manager = RepoManager::with_base_path(fs, dir.path());

        let repo1 = manager
            .create_repo("Repo 1".to_string(), None, RepoType::Local)
            .unwrap();
        let _repo2 = manager
            .create_repo("Repo 2".to_string(), None, RepoType::Local)
            .unwrap();

        manager.open_repo(repo1).unwrap();

        let open_repos = manager.list_open_repos();
        assert_eq!(open_repos.len(), 1);
        assert_eq!(open_repos[0].id, repo1);
    }

    #[test]
    fn test_repo_manager_get_repo() {
        let dir = tempdir().unwrap();
        let fs = Arc::new(LocalFileSystem);
        let manager = RepoManager::with_base_path(fs, dir.path());

        let repo_id = manager
            .create_repo("Test Repo".to_string(), None, RepoType::Local)
            .unwrap();

        let repo = manager.get_repo(repo_id);
        assert!(repo.is_some());
        assert_eq!(repo.as_ref().unwrap().name(), "Test Repo");
    }
}
