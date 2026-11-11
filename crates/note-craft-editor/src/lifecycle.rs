//! Editor Lifecycle Management
//!
//! Ce module gère le cycle de vie complet de l'éditeur :
//! - Initialisation avec configuration
//! - État opérationnel
//! - Gestion des erreurs centralisée
//! - Fermeture propre avec cleanup des ressources

use std::fmt;
use std::sync::{Arc, Mutex};
use thiserror::Error;

/// États possibles de l'éditeur
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum EditorState {
    /// L'éditeur n'est pas encore initialisé
    #[default]
    Uninitialized,
    /// L'éditeur est en cours d'initialisation
    Initializing,
    /// L'éditeur est prêt à être utilisé
    Ready,
    /// L'éditeur a rencontré une erreur
    Error(String),
    /// L'éditeur est en cours de fermeture
    Closing,
    /// L'éditeur est fermé
    Closed,
}

impl fmt::Display for EditorState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EditorState::Uninitialized => write!(f, "Uninitialized"),
            EditorState::Initializing => write!(f, "Initializing"),
            EditorState::Ready => write!(f, "Ready"),
            EditorState::Error(msg) => write!(f, "Error: {}", msg),
            EditorState::Closing => write!(f, "Closing"),
            EditorState::Closed => write!(f, "Closed"),
        }
    }
}

/// Types d'erreurs possibles pendant le cycle de vie
#[derive(Debug, Error)]
pub enum LifecycleError {
    /// Erreur lors de l'initialisation
    #[error("Initialization failed: {0}")]
    InitializationFailed(String),

    /// Erreur lors de la fermeture
    #[error("Shutdown failed: {0}")]
    ShutdownFailed(String),

    /// L'éditeur n'est pas dans l'état attendu
    #[error("Editor is in invalid state: {0}")]
    InvalidState(String),

    /// Ressource introuvable
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    /// Erreur de configuration
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    /// Erreur interne
    #[error("Internal error: {0}")]
    InternalError(String),
}

/// Événements du cycle de vie
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LifecycleEvent {
    /// L'initialisation a commencé
    InitializationStarted,
    /// L'initialisation est terminée
    InitializationCompleted,
    /// Une erreur est survenue
    ErrorOccurred(String),
    /// La fermeture a commencé
    ShutdownStarted,
    /// La fermeture est terminée
    ShutdownCompleted,
    /// L'éditeur est prêt
    Ready,
}

/// Callback pour les événements du cycle de vie
type LifecycleCallback = Box<dyn FnMut(&LifecycleEvent) + Send + Sync>;

/// Configuration pour l'initialisation de l'éditeur
#[derive(Debug, Clone)]
pub struct EditorConfig {
    /// Nom de l'éditeur
    pub name: String,
    /// Chemin du fichier à éditer (optionnel)
    pub file_path: Option<String>,
    /// Contenu initial (si pas de fichier)
    pub initial_content: Option<String>,
    /// Activer le mode auto-save
    pub auto_save: bool,
    /// Intervalle d'auto-save en secondes
    pub auto_save_interval: u64,
    /// Thème par défaut
    pub theme: String,
    /// Langue par défaut
    pub language: String,
    /// Activer la prévisualisation en temps réel
    pub live_preview: bool,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            file_path: None,
            initial_content: None,
            auto_save: true,
            auto_save_interval: 30,
            theme: "light".to_string(),
            language: "en".to_string(),
            live_preview: true,
        }
    }
}

impl EditorConfig {
    /// Crée une nouvelle configuration avec un nom
    pub fn with_name(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    /// Définit le chemin du fichier
    pub fn file_path(mut self, path: &str) -> Self {
        self.file_path = Some(path.to_string());
        self
    }

    /// Définit le contenu initial
    pub fn initial_content(mut self, content: &str) -> Self {
        self.initial_content = Some(content.to_string());
        self
    }

    /// Active/désactive l'auto-save
    pub fn auto_save(mut self, enabled: bool) -> Self {
        self.auto_save = enabled;
        self
    }

    /// Définit l'intervalle d'auto-save
    pub fn auto_save_interval(mut self, seconds: u64) -> Self {
        self.auto_save_interval = seconds;
        self
    }

    /// Définit le thème
    pub fn theme(mut self, theme: &str) -> Self {
        self.theme = theme.to_string();
        self
    }

    /// Définit la langue
    pub fn language(mut self, language: &str) -> Self {
        self.language = language.to_string();
        self
    }

    /// Active/désactive la prévisualisation en temps réel
    pub fn live_preview(mut self, enabled: bool) -> Self {
        self.live_preview = enabled;
        self
    }

    /// Valide la configuration
    pub fn validate(&self) -> Result<(), LifecycleError> {
        if self.name.is_empty() {
            return Err(LifecycleError::ConfigurationError(
                "Editor name cannot be empty".to_string(),
            ));
        }

        if self.auto_save_interval == 0 && self.auto_save {
            return Err(LifecycleError::ConfigurationError(
                "Auto-save interval cannot be zero".to_string(),
            ));
        }

        Ok(())
    }
}

/// Gestionnaire du cycle de vie de l'éditeur
pub struct LifecycleManager {
    /// État actuel
    state: Arc<Mutex<EditorState>>,
    /// Configuration
    config: EditorConfig,
    /// Callbacks pour les événements
    callbacks: Vec<LifecycleCallback>,
    /// Ressources à nettoyer
    resources: Vec<Box<dyn Resource + Send + Sync>>,
}

/// Trait pour les ressources qui peuvent être nettoyées
pub trait Resource {
    /// Nettoie la ressource
    fn cleanup(&mut self) -> Result<(), String>;
    /// Nom de la ressource
    fn name(&self) -> &str;
}

impl LifecycleManager {
    /// Crée un nouveau gestionnaire avec une configuration
    pub fn new(config: EditorConfig) -> Self {
        Self {
            state: Arc::new(Mutex::new(EditorState::Uninitialized)),
            config,
            callbacks: Vec::new(),
            resources: Vec::new(),
        }
    }

    /// Crée un nouveau gestionnaire avec configuration par défaut
    pub fn with_defaults() -> Self {
        Self::new(EditorConfig::default())
    }

    /// Ajoute un callback pour les événements
    pub fn add_callback<F>(&mut self, callback: F)
    where
        F: FnMut(&LifecycleEvent) + Send + Sync + 'static,
    {
        self.callbacks.push(Box::new(callback));
    }

    /// Ajoute une ressource à gérer
    pub fn add_resource<R>(&mut self, resource: R)
    where
        R: Resource + Send + Sync + 'static,
    {
        self.resources.push(Box::new(resource));
    }

    /// Retourne l'état actuel
    pub fn state(&self) -> EditorState {
        self.state.lock().unwrap().clone()
    }

    /// Vérifie si l'éditeur est prêt
    pub fn is_ready(&self) -> bool {
        matches!(self.state(), EditorState::Ready)
    }

    /// Vérifie si l'éditeur est fermé
    pub fn is_closed(&self) -> bool {
        matches!(self.state(), EditorState::Closed)
    }

    /// Vérifie si une erreur est survenue
    pub fn is_error(&self) -> bool {
        matches!(self.state(), EditorState::Error(_))
    }

    /// Initialise l'éditeur
    pub fn initialize(&mut self) -> Result<(), LifecycleError> {
        // Valider la configuration
        self.config.validate()?;

        // Changer l'état
        {
            let mut state = self.state.lock().unwrap();
            *state = EditorState::Initializing;
        }

        self.emit_event(LifecycleEvent::InitializationStarted);

        // Simuler l'initialisation (dans un vrai scénario, cela initialiserait
        // les composants de l'éditeur)
        // Pour l'instant, nous validons juste que nous pouvons passer à Ready

        // Changer l'état à Ready
        {
            let mut state = self.state.lock().unwrap();
            *state = EditorState::Ready;
        }

        self.emit_event(LifecycleEvent::InitializationCompleted);
        self.emit_event(LifecycleEvent::Ready);

        Ok(())
    }

    /// Initialise l'éditeur avec un contenu initial
    pub fn initialize_with_content(content: &str) -> Result<Self, LifecycleError> {
        let mut manager = Self {
            state: Arc::new(Mutex::new(EditorState::Uninitialized)),
            config: EditorConfig {
                initial_content: Some(content.to_string()),
                ..Default::default()
            },
            callbacks: Vec::new(),
            resources: Vec::new(),
        };

        manager.initialize()?;
        Ok(manager)
    }

    /// Émet un événement à tous les callbacks
    fn emit_event(&mut self, event: LifecycleEvent) {
        for callback in &mut self.callbacks {
            callback(&event);
        }
    }

    /// Ferme l'éditeur proprement
    pub fn shutdown(&mut self) -> Result<(), LifecycleError> {
        let current_state = self.state();

        match current_state {
            EditorState::Closed => {
                return Err(LifecycleError::InvalidState(
                    "Editor is already closed".to_string(),
                ))
            }
            EditorState::Closing => {
                return Err(LifecycleError::InvalidState(
                    "Editor is already shutting down".to_string(),
                ))
            }
            _ => {}
        }

        // Changer l'état
        {
            let mut state = self.state.lock().unwrap();
            *state = EditorState::Closing;
        }

        self.emit_event(LifecycleEvent::ShutdownStarted);

        // Nettoyer les ressources dans l'ordre inverse
        let mut errors: Vec<String> = Vec::new();
        for resource in self.resources.iter_mut().rev() {
            if let Err(err) = resource.cleanup() {
                errors.push(format!("Failed to cleanup {}: {}", resource.name(), err));
            }
        }

        if !errors.is_empty() {
            let error_msg = errors.join("; ");
            self.emit_event(LifecycleEvent::ErrorOccurred(error_msg.clone()));
            {
                let mut state = self.state.lock().unwrap();
                *state = EditorState::Error(error_msg.clone());
            }
            return Err(LifecycleError::ShutdownFailed(error_msg));
        }

        // Changer l'état à Closed
        {
            let mut state = self.state.lock().unwrap();
            *state = EditorState::Closed;
        }

        self.emit_event(LifecycleEvent::ShutdownCompleted);

        Ok(())
    }

    /// Ferme l'éditeur avec un message d'erreur
    pub fn shutdown_with_error(&mut self, error: &str) -> Result<(), LifecycleError> {
        {
            let mut state = self.state.lock().unwrap();
            *state = EditorState::Error(error.to_string());
        }

        self.emit_event(LifecycleEvent::ErrorOccurred(error.to_string()));
        self.emit_event(LifecycleEvent::ShutdownCompleted);

        Ok(())
    }

    /// Force la fermeture (même si une erreur s'est produite)
    pub fn force_shutdown(&mut self) -> Result<(), LifecycleError> {
        // Nettoyer les ressources malgré tout
        for resource in self.resources.iter_mut().rev() {
            let _ = resource.cleanup();
        }

        {
            let mut state = self.state.lock().unwrap();
            *state = EditorState::Closed;
        }

        self.emit_event(LifecycleEvent::ShutdownCompleted);

        Ok(())
    }

    /// Réinitialise l'éditeur
    pub fn reset(&mut self) -> Result<(), LifecycleError> {
        // Fermer proprement
        self.shutdown()?;

        // Réinitialiser l'état
        {
            let mut state = self.state.lock().unwrap();
            *state = EditorState::Uninitialized;
        }

        // Nettoyer les callbacks et ressources
        self.callbacks.clear();
        self.resources.clear();

        Ok(())
    }

    /// Retourne la configuration
    pub fn config(&self) -> &EditorConfig {
        &self.config
    }

    /// Met à jour la configuration
    pub fn set_config(&mut self, config: EditorConfig) -> Result<(), LifecycleError> {
        config.validate()?;
        self.config = config;
        Ok(())
    }

    /// Exécute une fonction uniquement si l'éditeur est dans un état valide
    pub fn with_ready_state<F, R>(&self, func: F) -> Result<R, LifecycleError>
    where
        F: FnOnce() -> R,
    {
        if self.is_ready() {
            Ok(func())
        } else {
            Err(LifecycleError::InvalidState(format!(
                "Expected Ready state, got: {}",
                self.state()
            )))
        }
    }

    /// Attend que l'éditeur soit prêt (pour les scénarios async)
    /// Retourne Ok(()) si prêt, Err si erreur ou timeout
    pub fn wait_for_ready(&self, timeout_ms: u64) -> Result<(), LifecycleError> {
        use std::time::{Duration, Instant};

        let start = Instant::now();
        loop {
            match self.state() {
                EditorState::Ready => return Ok(()),
                EditorState::Error(msg) => {
                    return Err(LifecycleError::InvalidState(format!(
                        "Editor in error state: {}",
                        msg
                    )))
                }
                EditorState::Closed => {
                    return Err(LifecycleError::InvalidState("Editor is closed".to_string()))
                }
                _ => {
                    if start.elapsed() >= Duration::from_millis(timeout_ms) {
                        return Err(LifecycleError::InvalidState(
                            "Timeout waiting for Ready state".to_string(),
                        ));
                    }
                    std::thread::sleep(Duration::from_millis(100));
                }
            }
        }
    }
}

/// Implémentation par défaut de Default pour LifecycleManager
impl Default for LifecycleManager {
    fn default() -> Self {
        Self::with_defaults()
    }
}

/// Builder pour LifecycleManager
pub struct LifecycleManagerBuilder {
    config: EditorConfig,
    callbacks: Vec<LifecycleCallback>,
    resources: Vec<Box<dyn Resource + Send + Sync>>,
}

impl LifecycleManagerBuilder {
    /// Crée un nouveau builder
    pub fn new() -> Self {
        Self {
            config: EditorConfig::default(),
            callbacks: Vec::new(),
            resources: Vec::new(),
        }
    }

    /// Définit la configuration
    pub fn config(mut self, config: EditorConfig) -> Self {
        self.config = config;
        self
    }

    /// Définit le nom de l'éditeur
    pub fn name(mut self, name: &str) -> Self {
        self.config.name = name.to_string();
        self
    }

    /// Définit le chemin du fichier
    pub fn file_path(mut self, path: &str) -> Self {
        self.config.file_path = Some(path.to_string());
        self
    }

    /// Ajoute un callback
    pub fn with_callback<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&LifecycleEvent) + Send + Sync + 'static,
    {
        self.callbacks.push(Box::new(callback));
        self
    }

    /// Ajoute une ressource
    pub fn with_resource<R>(mut self, resource: R) -> Self
    where
        R: Resource + Send + Sync + 'static,
    {
        self.resources.push(Box::new(resource));
        self
    }

    /// Construit le LifecycleManager
    pub fn build(self) -> LifecycleManager {
        LifecycleManager {
            state: Arc::new(Mutex::new(EditorState::Uninitialized)),
            config: self.config,
            callbacks: self.callbacks,
            resources: self.resources,
        }
    }
}

impl Default for LifecycleManagerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Ressource simple pour tester
pub struct SimpleResource {
    name: String,
    cleaned_up: bool,
}

impl SimpleResource {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            cleaned_up: false,
        }
    }
}

impl Resource for SimpleResource {
    fn cleanup(&mut self) -> Result<(), String> {
        self.cleaned_up = true;
        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for SimpleResource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SimpleResource({})", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editor_state_display() {
        assert_eq!(format!("{}", EditorState::Uninitialized), "Uninitialized");
        assert_eq!(format!("{}", EditorState::Ready), "Ready");
        assert_eq!(format!("{}", EditorState::Closed), "Closed");
        assert_eq!(
            format!("{}", EditorState::Error("test".to_string())),
            "Error: test"
        );
    }

    #[test]
    fn test_lifecycle_error_display() {
        let err = LifecycleError::InitializationFailed("test".to_string());
        assert_eq!(format!("{}", err), "Initialization failed: test");

        let err = LifecycleError::InvalidState("test".to_string());
        assert_eq!(format!("{}", err), "Editor is in invalid state: test");
    }

    #[test]
    fn test_editor_config_default() {
        let config = EditorConfig::default();
        assert_eq!(config.name, "default");
        assert_eq!(config.theme, "light");
        assert_eq!(config.language, "en");
        assert!(config.auto_save);
        assert_eq!(config.auto_save_interval, 30);
    }

    #[test]
    fn test_editor_config_builder() {
        let config = EditorConfig::with_name("test")
            .file_path("/path/to/file")
            .initial_content("# Hello")
            .auto_save(true)
            .auto_save_interval(60)
            .theme("dark")
            .language("fr")
            .live_preview(false);

        assert_eq!(config.name, "test");
        assert_eq!(config.file_path, Some("/path/to/file".to_string()));
        assert_eq!(config.initial_content, Some("# Hello".to_string()));
        assert!(config.auto_save);
        assert_eq!(config.auto_save_interval, 60);
        assert_eq!(config.theme, "dark");
        assert_eq!(config.language, "fr");
        assert!(!config.live_preview);
    }

    #[test]
    fn test_editor_config_validation() {
        // Nom vide
        let config = EditorConfig {
            name: "".to_string(),
            ..Default::default()
        };
        assert!(matches!(
            config.validate(),
            Err(LifecycleError::ConfigurationError(_))
        ));

        // Auto-save activé avec intervalle zéro
        let config = EditorConfig {
            name: "test".to_string(),
            auto_save: true,
            auto_save_interval: 0,
            ..Default::default()
        };
        assert!(matches!(
            config.validate(),
            Err(LifecycleError::ConfigurationError(_))
        ));

        // Configuration valide
        let config = EditorConfig {
            name: "test".to_string(),
            ..Default::default()
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_lifecycle_manager_initialization() {
        let mut manager = LifecycleManager::with_defaults();

        assert_eq!(manager.state(), EditorState::Uninitialized);
        assert!(!manager.is_ready());
        assert!(!manager.is_closed());

        // Initialiser
        assert!(manager.initialize().is_ok());
        assert_eq!(manager.state(), EditorState::Ready);
        assert!(manager.is_ready());
    }

    #[test]
    fn test_lifecycle_manager_with_content() {
        let manager = LifecycleManager::initialize_with_content("# Test").unwrap();

        assert!(manager.is_ready());
        assert_eq!(manager.config().initial_content, Some("# Test".to_string()));
    }

    #[test]
    fn test_lifecycle_manager_shutdown() {
        let mut manager = LifecycleManager::with_defaults();
        manager.initialize().unwrap();

        assert!(manager.is_ready());

        // Fermer
        assert!(manager.shutdown().is_ok());
        assert_eq!(manager.state(), EditorState::Closed);
        assert!(manager.is_closed());
    }

    #[test]
    fn test_lifecycle_manager_shutdown_already_closed() {
        let mut manager = LifecycleManager::with_defaults();
        manager.initialize().unwrap();
        manager.shutdown().unwrap();

        // Essayer de fermer à nouveau
        assert!(matches!(
            manager.shutdown(),
            Err(LifecycleError::InvalidState(_))
        ));
    }

    #[test]
    fn test_lifecycle_manager_shutdown_with_error() {
        let mut manager = LifecycleManager::with_defaults();
        manager.initialize().unwrap();

        assert!(manager.shutdown_with_error("Test error").is_ok());
        assert!(manager.is_error());

        if let EditorState::Error(msg) = manager.state() {
            assert_eq!(msg, "Test error");
        } else {
            panic!("Expected Error state");
        }
    }

    #[test]
    fn test_lifecycle_manager_force_shutdown() {
        let mut manager = LifecycleManager::with_defaults();
        manager.initialize().unwrap();

        assert!(manager.force_shutdown().is_ok());
        assert!(manager.is_closed());
    }

    #[test]
    fn test_lifecycle_manager_reset() {
        let mut manager = LifecycleManager::with_defaults();
        manager.initialize().unwrap();
        assert!(manager.is_ready());

        manager.reset().unwrap();
        assert_eq!(manager.state(), EditorState::Uninitialized);
    }

    #[test]
    fn test_lifecycle_manager_callbacks() {
        use std::sync::{Arc, Mutex};

        let mut manager = LifecycleManager::with_defaults();
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = Arc::clone(&events);

        manager.add_callback(move |event| {
            events_clone.lock().unwrap().push(event.clone());
        });

        manager.initialize().unwrap();

        // Vérifier que les callbacks ont été appelés
        let events_vec = events.lock().unwrap();
        assert!(events_vec.contains(&LifecycleEvent::InitializationStarted));
        assert!(events_vec.contains(&LifecycleEvent::InitializationCompleted));
        assert!(events_vec.contains(&LifecycleEvent::Ready));
    }

    #[test]
    fn test_lifecycle_manager_resources() {
        let mut manager = LifecycleManager::with_defaults();
        let resource = SimpleResource::new("test_resource");

        manager.add_resource(resource);
        manager.initialize().unwrap();

        assert!(manager.shutdown().is_ok());
    }

    #[test]
    fn test_lifecycle_manager_with_ready_state() {
        let mut manager = LifecycleManager::with_defaults();
        manager.initialize().unwrap();

        // Exécuter une fonction avec l'état Ready
        let result = manager.with_ready_state(|| "success").unwrap();
        assert_eq!(result, "success");
    }

    #[test]
    fn test_lifecycle_manager_with_ready_state_error() {
        let manager = LifecycleManager::with_defaults();

        // Essayer d'exécuter avec un état non-Ready
        let result = manager.with_ready_state(|| "success");
        assert!(matches!(result, Err(LifecycleError::InvalidState(_))));
    }

    #[test]
    fn test_lifecycle_manager_set_config() {
        let mut manager = LifecycleManager::with_defaults();

        let new_config = EditorConfig::with_name("new_name");
        assert!(manager.set_config(new_config).is_ok());
        assert_eq!(manager.config().name, "new_name");
    }

    #[test]
    fn test_lifecycle_manager_builder() {
        let manager = LifecycleManagerBuilder::new()
            .name("test")
            .file_path("/path/to/file")
            .with_callback(|_event| {
                // Callback
            })
            .with_resource(SimpleResource::new("resource1"))
            .build();

        assert_eq!(manager.config().name, "test");
        assert_eq!(
            manager.config().file_path,
            Some("/path/to/file".to_string())
        );
    }

    #[test]
    fn test_simple_resource_cleanup() {
        let mut resource = SimpleResource::new("test");
        assert!(!resource.cleaned_up);

        assert!(resource.cleanup().is_ok());
        assert!(resource.cleaned_up);
    }

    #[test]
    fn test_lifecycle_event_partial_eq() {
        assert_eq!(
            LifecycleEvent::InitializationStarted,
            LifecycleEvent::InitializationStarted
        );
        assert_ne!(
            LifecycleEvent::InitializationStarted,
            LifecycleEvent::ShutdownStarted
        );
    }

    #[test]
    fn test_lifecycle_error_error_trait() {
        let err = LifecycleError::InitializationFailed("test".to_string());

        // Vérifier que l'erreur peut être convertie en string
        let err_str = format!("{}", err);
        assert!(err_str.contains("Initialization failed"));
    }

    #[test]
    fn test_editor_state_clone() {
        let state = EditorState::Ready;
        let cloned = state.clone();

        assert_eq!(state, cloned);
    }

    #[test]
    fn test_editor_state_equality() {
        assert_eq!(EditorState::Ready, EditorState::Ready);
        assert_ne!(EditorState::Ready, EditorState::Closed);
        assert_ne!(EditorState::Ready, EditorState::Error("test".to_string()));
    }
}
