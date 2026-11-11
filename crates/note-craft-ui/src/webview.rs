//! WebView Integration Module
//!
//! Ce module fournit:
//! - Abstraction pour l'intégration WebView
//! - Communication entre Rust et JavaScript
//! - Gestion des événements WebView
//! - Intégration avec wry (quand disponible)
//!
//! Note: Ce module fournit une abstraction qui peut être implémentée
//! avec différentes backends WebView (wry, webview2, etc.)

use note_craft_core::Node;
use note_craft_editor::rendering::{RenderingConfig, RenderingPipeline};
use std::str::FromStr;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use thiserror::Error;

/// Types d'erreurs pour WebView
#[derive(Debug, Error)]
pub enum WebViewError {
    /// Erreur lors de la création de la WebView
    #[error("Failed to create WebView: {0}")]
    CreationFailed(String),

    /// Erreur lors du chargement du contenu
    #[error("Failed to load content: {0}")]
    LoadFailed(String),

    /// Erreur de communication JavaScript
    #[error("JavaScript communication error: {0}")]
    JsCommunicationError(String),

    /// Erreur de rendu HTML
    #[error("HTML rendering error: {0}")]
    RenderError(String),

    /// Canal de communication fermé
    #[error("Channel closed")]
    ChannelClosed,

    /// Erreur interne
    #[error("Internal error: {0}")]
    InternalError(String),
}

/// Types de messages entre Rust et JavaScript
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum JsMessage {
    /// Message de log depuis JavaScript
    Log { level: String, message: String },
    /// Sélection de texte
    TextSelected { text: String },
    /// Clic sur un lien
    LinkClicked { url: String },
    /// L'éditeur est prêt
    EditorReady,
    /// Demande de rendu d'un nœud
    RenderNode { node_id: u64 },
    /// Contenu modifié
    ContentChanged { content: String },
    /// Demande d'exécution de commande
    ExecuteCommand {
        command: String,
        args: serde_json::Value,
    },
}

/// Types de messages de Rust vers JavaScript
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum RustMessage {
    /// Initialisation
    Init { config: WebViewConfig },
    /// Mise à jour du contenu HTML
    UpdateContent { html: String, node_id: Option<u64> },
    /// Exécution de script JavaScript
    ExecuteScript { script: String },
    /// Notification
    Notify { title: String, message: String },
    /// Réponse à une commande
    CommandResponse {
        request_id: Option<String>,
        result: serde_json::Value,
    },
}

/// Configuration de la WebView
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WebViewConfig {
    /// Titre de la fenêtre
    pub title: String,
    /// Largeur initiale
    pub width: u32,
    /// Hauteur initiale
    pub height: u32,
    /// Activer les outils de développement
    pub devtools: bool,
    /// Couleur de fond par défaut
    pub background_color: Option<String>,
    /// User Agent
    pub user_agent: Option<String>,
    /// Permettre l'exécution de scripts
    pub allow_scripts: bool,
}

impl Default for WebViewConfig {
    fn default() -> Self {
        Self {
            title: "Note Craft".to_string(),
            width: 1200,
            height: 800,
            devtools: false,
            background_color: Some("#ffffff".to_string()),
            user_agent: None,
            allow_scripts: true,
        }
    }
}

impl WebViewConfig {
    /// Crée une configuration avec un titre
    pub fn with_title(title: &str) -> Self {
        Self {
            title: title.to_string(),
            ..Default::default()
        }
    }

    /// Définit les dimensions
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Active les outils de développement
    pub fn with_devtools(mut self) -> Self {
        self.devtools = true;
        self
    }

    /// Définit la couleur de fond
    pub fn background_color(mut self, color: &str) -> Self {
        self.background_color = Some(color.to_string());
        self
    }

    /// Définit le User Agent
    pub fn user_agent(mut self, user_agent: &str) -> Self {
        self.user_agent = Some(user_agent.to_string());
        self
    }
}

/// Trait pour les gestionnaires d'événements WebView
pub trait WebViewEventHandler: Send + Sync {
    /// Gère un message de JavaScript
    fn on_js_message(&mut self, message: JsMessage);

    /// Gère une erreur
    fn on_error(&mut self, error: WebViewError);

    /// Gère la fermeture de la fenêtre
    fn on_close(&mut self);
}

/// Gestionnaire d'événements par défaut
pub struct DefaultWebViewHandler;

impl WebViewEventHandler for DefaultWebViewHandler {
    fn on_js_message(&mut self, message: JsMessage) {
        match message {
            JsMessage::Log { level, message } => {
                log::log!(
                    log::Level::from_str(&level).unwrap_or(log::Level::Info),
                    "JS: {}",
                    message
                );
            }
            JsMessage::TextSelected { text } => {
                log::info!("Text selected: {}", text);
            }
            JsMessage::LinkClicked { url } => {
                log::info!("Link clicked: {}", url);
            }
            JsMessage::EditorReady => {
                log::info!("Editor ready");
            }
            JsMessage::RenderNode { node_id } => {
                log::info!("Render node: {}", node_id);
            }
            JsMessage::ContentChanged { content } => {
                log::info!("Content changed: {} chars", content.len());
            }
            JsMessage::ExecuteCommand { command, args } => {
                log::info!("Execute command: {} with args: {:?}", command, args);
            }
        }
    }

    fn on_error(&mut self, error: WebViewError) {
        log::error!("WebView error: {}", error);
    }

    fn on_close(&mut self) {
        log::info!("WebView closed");
    }
}

/// Trait pour les backends WebView
///
/// Ce trait permet d'abstraire l'implémentation spécifique du backend WebView
/// (wry, webview2, etc.)
pub trait WebViewBackend: Send + Sync {
    /// Initialise la WebView
    fn initialize(&mut self, config: &WebViewConfig) -> Result<(), WebViewError>;

    /// Affiche du contenu HTML
    fn display_html(&mut self, html: &str, node_id: Option<u64>) -> Result<(), WebViewError>;

    /// Exécute un script JavaScript
    fn execute_script(&mut self, script: &str) -> Result<(), WebViewError>;

    /// Envoie un message à JavaScript
    fn send_message(&mut self, message: RustMessage) -> Result<(), WebViewError>;

    /// Traite les événements entrants
    fn process_events(&mut self) -> Result<bool, WebViewError>;

    /// Exécute la boucle principale
    fn run(&mut self) -> Result<(), WebViewError>;

    /// Ferme la WebView
    fn close(&mut self) -> Result<(), WebViewError>;

    /// Retourne le canal de réception des messages
    fn receiver(&self) -> Option<Arc<Mutex<Receiver<JsMessage>>>>;
}

/// Backend par défaut (stub pour compilation sans wry)
///
/// Ce backend permet de compiler le code sans avoir wry disponible.
/// Il peut être remplacé par une implémentation concrète avec wry.
pub struct StubWebViewBackend {
    /// Canal de réception des messages (protégé par Mutex pour Sync)
    rx: Option<Arc<Mutex<Receiver<JsMessage>>>>,
    /// Canal d'envoi des messages
    tx: Option<Sender<JsMessage>>,
    /// Configuration
    #[allow(dead_code)]
    config: WebViewConfig,
    /// Pipeline de rendu
    #[allow(dead_code)]
    render_pipeline: RenderingPipeline,
    /// Gestionnaire d'événements
    event_handler: Arc<Mutex<Box<dyn WebViewEventHandler>>>,
}

impl StubWebViewBackend {
    pub fn new(config: WebViewConfig) -> Self {
        let (js_tx, js_rx) = channel::<JsMessage>();

        Self {
            rx: Some(Arc::new(Mutex::new(js_rx))),
            tx: Some(js_tx),
            config,
            render_pipeline: RenderingPipeline::default(),
            event_handler: Arc::new(Mutex::new(Box::new(DefaultWebViewHandler))),
        }
    }
}

impl WebViewBackend for StubWebViewBackend {
    fn initialize(&mut self, _config: &WebViewConfig) -> Result<(), WebViewError> {
        Ok(())
    }

    fn display_html(&mut self, html: &str, node_id: Option<u64>) -> Result<(), WebViewError> {
        // Dans un vrai backend, cela afficherait le HTML dans la WebView
        log::debug!("Display HTML: {} chars, node_id: {:?}", html.len(), node_id);
        Ok(())
    }

    fn execute_script(&mut self, script: &str) -> Result<(), WebViewError> {
        log::debug!("Execute script: {}", script);
        Ok(())
    }

    fn send_message(&mut self, message: RustMessage) -> Result<(), WebViewError> {
        // Sérialiser le message
        let json = serde_json::to_string(&message)
            .map_err(|e| WebViewError::JsCommunicationError(format!("{}", e)))?;
        log::debug!("Send message to JS: {}", json);
        Ok(())
    }

    fn process_events(&mut self) -> Result<bool, WebViewError> {
        if let Some(rx) = &self.rx {
            let rx_lock = rx.lock().unwrap();
            match rx_lock.try_recv() {
                Ok(message) => {
                    let mut handler = self.event_handler.lock().unwrap();
                    handler.on_js_message(message);
                    Ok(true)
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => Ok(false),
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    Err(WebViewError::ChannelClosed)
                }
            }
        } else {
            Ok(false)
        }
    }

    fn run(&mut self) -> Result<(), WebViewError> {
        log::info!("WebView stub running");
        // Boucle simple pour traiter les événements
        loop {
            if !self.process_events()? {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }

    fn close(&mut self) -> Result<(), WebViewError> {
        log::info!("WebView stub closed");
        self.rx = None;
        self.tx = None;
        Ok(())
    }

    fn receiver(&self) -> Option<Arc<Mutex<Receiver<JsMessage>>>> {
        self.rx.clone()
    }
}

/// Gestionnaire de la WebView (utilise un backend)
pub struct WebViewManager {
    /// Le backend WebView
    backend: Box<dyn WebViewBackend>,
    /// Configuration
    config: WebViewConfig,
    /// Pipeline de rendu
    render_pipeline: RenderingPipeline,
}

impl WebViewManager {
    /// Crée un nouveau gestionnaire de WebView avec un backend
    pub fn with_backend(backend: Box<dyn WebViewBackend>, config: WebViewConfig) -> Self {
        Self {
            backend,
            config,
            render_pipeline: RenderingPipeline::default(),
        }
    }

    /// Crée un nouveau gestionnaire avec le backend stub
    pub fn new(config: WebViewConfig) -> Self {
        Self::with_backend(Box::new(StubWebViewBackend::new(config.clone())), config)
    }

    /// Crée un nouveau gestionnaire avec configuration par défaut
    pub fn with_defaults() -> Self {
        Self::new(WebViewConfig::default())
    }

    /// Définit la configuration
    pub fn set_config(&mut self, config: WebViewConfig) {
        self.config = config;
    }

    /// Initialise la WebView
    pub fn initialize(&mut self) -> Result<(), WebViewError> {
        self.backend.initialize(&self.config)
    }

    /// Affiche un nœud dans la WebView
    pub fn display_node(&mut self, node: &Node) -> Result<(), WebViewError> {
        // Rendre le markdown du nœud en HTML
        let html = self
            .render_pipeline
            .render_markdown(&node.content)
            .map_err(|e| WebViewError::RenderError(format!("{}", e)))?;

        self.backend.display_html(&html, Some(node.id.0))
    }

    /// Affiche du contenu Markdown directement
    pub fn display_markdown(&mut self, markdown: &str) -> Result<(), WebViewError> {
        let html = self
            .render_pipeline
            .render_markdown(markdown)
            .map_err(|e| WebViewError::RenderError(format!("{}", e)))?;

        self.backend.display_html(&html, None)
    }

    /// Exécute un script JavaScript
    pub fn execute_script(&mut self, script: &str) -> Result<(), WebViewError> {
        self.backend.execute_script(script)
    }

    /// Envoie un message à JavaScript
    pub fn send_message(&mut self, message: RustMessage) -> Result<(), WebViewError> {
        self.backend.send_message(message)
    }

    /// Traite les événements entrants
    pub fn process_events(&mut self) -> Result<bool, WebViewError> {
        self.backend.process_events()
    }

    /// Exécute la boucle principale de la WebView
    pub fn run(&mut self) -> Result<(), WebViewError> {
        self.backend.run()
    }

    /// Ferme la WebView
    pub fn close(&mut self) -> Result<(), WebViewError> {
        self.backend.close()
    }

    /// Retourne la configuration actuelle
    pub fn config(&self) -> &WebViewConfig {
        &self.config
    }

    /// Définit le pipeline de rendu
    pub fn set_render_pipeline(&mut self, pipeline: RenderingPipeline) {
        self.render_pipeline = pipeline;
    }

    /// Retourne le backend (pour un accès avancé)
    pub fn backend(&mut self) -> &mut dyn WebViewBackend {
        self.backend.as_mut()
    }

    /// Change le backend
    pub fn set_backend(&mut self, backend: Box<dyn WebViewBackend>) {
        self.backend = backend;
    }
}

impl Default for WebViewManager {
    fn default() -> Self {
        Self::with_defaults()
    }
}

/// Builder pour WebViewManager
pub struct WebViewManagerBuilder {
    config: WebViewConfig,
    render_config: Option<RenderingConfig>,
}

impl WebViewManagerBuilder {
    /// Crée un nouveau builder
    pub fn new() -> Self {
        Self {
            config: WebViewConfig::default(),
            render_config: None,
        }
    }

    /// Définit la configuration
    pub fn config(mut self, config: WebViewConfig) -> Self {
        self.config = config;
        self
    }

    /// Définit la configuration de rendu
    pub fn render_config(mut self, config: RenderingConfig) -> Self {
        self.render_config = Some(config);
        self
    }

    /// Construit le WebViewManager
    pub fn build(self) -> WebViewManager {
        let mut manager = WebViewManager::new(self.config);

        if let Some(config) = self.render_config {
            manager.render_pipeline = RenderingPipeline::with_config(config);
        }

        manager
    }
}

impl Default for WebViewManagerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Structure pour gérer plusieurs WebViews
pub struct WebViewPool {
    managers: Vec<WebViewManager>,
    current_index: usize,
}

impl WebViewPool {
    /// Crée un nouveau pool
    pub fn new() -> Self {
        Self {
            managers: Vec::new(),
            current_index: 0,
        }
    }

    /// Ajoute un gestionnaire de WebView
    pub fn add_manager(&mut self, manager: WebViewManager) {
        self.managers.push(manager);
    }

    /// Retourne le gestionnaire courant
    pub fn current(&mut self) -> Option<&mut WebViewManager> {
        if self.current_index < self.managers.len() {
            Some(&mut self.managers[self.current_index])
        } else {
            None
        }
    }

    /// Change de WebView courante
    pub fn switch_to(&mut self, index: usize) -> Result<(), WebViewError> {
        if index < self.managers.len() {
            self.current_index = index;
            Ok(())
        } else {
            Err(WebViewError::CreationFailed(format!(
                "Index out of bounds: {}",
                index
            )))
        }
    }

    /// Ferme toutes les WebViews
    pub fn close_all(&mut self) -> Result<(), WebViewError> {
        for manager in &mut self.managers {
            manager.close()?;
        }
        Ok(())
    }

    /// Nombre de WebViews
    pub fn len(&self) -> usize {
        self.managers.len()
    }

    /// Vérifie si le pool est vide
    pub fn is_empty(&self) -> bool {
        self.managers.is_empty()
    }
}

impl Default for WebViewPool {
    fn default() -> Self {
        Self::new()
    }
}

/// HTML de base pour la WebView
pub fn get_initial_html() -> String {
    r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Note Craft</title>
    <style>
        body {
            margin: 0;
            padding: 0;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background-color: var(--bg-color, #ffffff);
            color: var(--text-color, #333333);
        }
        #app {
            width: 100%;
            height: 100vh;
            overflow: auto;
        }
    </style>
</head>
<body>
    <div id="app"></div>
    <script>
        // Gestion des messages de Rust (simulée pour le stub)
        function handleRustMessage(message) {
            switch (message.type) {
                case 'Init':
                    console.log('Initialized with config:', message.config);
                    if (message.config.background_color) {
                        document.body.style.backgroundColor = message.config.background_color;
                    }
                    break;
                case 'UpdateContent':
                    console.log('Updating content for node:', message.node_id);
                    document.getElementById('app').innerHTML = message.html;
                    break;
                case 'ExecuteScript':
                    try {
                        eval(message.script);
                    } catch (e) {
                        console.error('Script error:', e);
                    }
                    break;
                case 'Notify':
                    alert(message.title + ': ' + message.message);
                    break;
            }
        }

        // L'éditeur est prêt
        window.addEventListener('load', () => {
            console.log('Note Craft WebView ready');
        });
    </script>
</body>
</html>"#
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webview_config_default() {
        let config = WebViewConfig::default();
        assert_eq!(config.title, "Note Craft");
        assert_eq!(config.width, 1200);
        assert_eq!(config.height, 800);
        assert!(!config.devtools);
    }

    #[test]
    fn test_webview_config_builder() {
        let config = WebViewConfig::with_title("Test Editor")
            .size(1024, 768)
            .with_devtools()
            .background_color("#f0f0f0")
            .user_agent("NoteCraft/1.0");

        assert_eq!(config.title, "Test Editor");
        assert_eq!(config.width, 1024);
        assert_eq!(config.height, 768);
        assert!(config.devtools);
        assert_eq!(config.background_color, Some("#f0f0f0".to_string()));
        assert_eq!(config.user_agent, Some("NoteCraft/1.0".to_string()));
    }

    #[test]
    fn test_js_message_serialization() {
        let msg = JsMessage::Log {
            level: "info".to_string(),
            message: "Test message".to_string(),
        };

        let json = serde_json::to_string(&msg).unwrap();
        let deserialized: JsMessage = serde_json::from_str(&json).unwrap();

        match deserialized {
            JsMessage::Log { level, message } => {
                assert_eq!(level, "info");
                assert_eq!(message, "Test message");
            }
            _ => panic!("Wrong message type"),
        }
    }

    #[test]
    fn test_rust_message_serialization() {
        let msg = RustMessage::UpdateContent {
            html: "<p>Test</p>".to_string(),
            node_id: Some(1),
        };

        let json = serde_json::to_string(&msg).unwrap();
        let deserialized: RustMessage = serde_json::from_str(&json).unwrap();

        match deserialized {
            RustMessage::UpdateContent { html, node_id } => {
                assert_eq!(html, "<p>Test</p>");
                assert_eq!(node_id, Some(1));
            }
            _ => panic!("Wrong message type"),
        }
    }

    #[test]
    fn test_webview_error_display() {
        let err = WebViewError::CreationFailed("test".to_string());
        assert_eq!(format!("{}", err), "Failed to create WebView: test");

        let err = WebViewError::JsCommunicationError("test".to_string());
        assert_eq!(format!("{}", err), "JavaScript communication error: test");
    }

    #[test]
    fn test_webview_manager_new() {
        let manager = WebViewManager::with_defaults();
        assert!(manager.config().title == "Note Craft");
    }

    #[test]
    fn test_webview_manager_config() {
        let config = WebViewConfig::with_title("Test");
        let manager = WebViewManager::new(config.clone());
        assert_eq!(manager.config().title, "Test");
    }

    #[test]
    fn test_webview_builder() {
        let config = WebViewConfig::with_title("Builder Test");
        let manager = WebViewManagerBuilder::new().config(config).build();

        assert_eq!(manager.config().title, "Builder Test");
    }

    #[test]
    fn test_webview_pool() {
        let mut pool = WebViewPool::new();
        assert!(pool.is_empty());

        pool.add_manager(WebViewManager::with_defaults());
        pool.add_manager(WebViewManager::with_defaults());

        assert_eq!(pool.len(), 2);
        assert!(!pool.is_empty());
    }

    #[test]
    fn test_default_webview_handler() {
        let mut handler = DefaultWebViewHandler;

        // Tester tous les types de messages
        handler.on_js_message(JsMessage::Log {
            level: "info".to_string(),
            message: "test".to_string(),
        });

        handler.on_js_message(JsMessage::TextSelected {
            text: "selected".to_string(),
        });

        handler.on_js_message(JsMessage::LinkClicked {
            url: "http://example.com".to_string(),
        });

        handler.on_js_message(JsMessage::EditorReady);
        handler.on_js_message(JsMessage::RenderNode { node_id: 1 });
        handler.on_js_message(JsMessage::ContentChanged {
            content: "new content".to_string(),
        });

        handler.on_js_message(JsMessage::ExecuteCommand {
            command: "test".to_string(),
            args: serde_json::json!({}),
        });

        handler.on_error(WebViewError::CreationFailed("test".to_string()));
        handler.on_close();
    }

    #[test]
    fn test_webview_config_clone() {
        let config = WebViewConfig::default();
        let cloned = config.clone();
        assert_eq!(config.title, cloned.title);
        assert_eq!(config.width, cloned.width);
    }

    #[test]
    fn test_js_message_clone() {
        let msg = JsMessage::EditorReady;
        let cloned = msg.clone();
        assert!(matches!(cloned, JsMessage::EditorReady));
    }

    #[test]
    fn test_rust_message_clone() {
        let msg = RustMessage::Init {
            config: WebViewConfig::default(),
        };
        let cloned = msg.clone();
        match cloned {
            RustMessage::Init { config } => {
                assert_eq!(config.title, "Note Craft");
            }
            _ => panic!("Wrong message type"),
        }
    }

    #[test]
    fn test_get_initial_html() {
        let html = get_initial_html();
        assert!(html.contains("Note Craft"));
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("<div id=\"app\"></div>"));
    }

    #[test]
    fn test_stub_backend_creation() {
        let backend = StubWebViewBackend::new(WebViewConfig::default());
        assert!(backend.receiver().is_some());
    }

    #[test]
    fn test_webview_manager_with_backend() {
        let backend = Box::new(StubWebViewBackend::new(WebViewConfig::default()));
        let manager = WebViewManager::with_backend(backend, WebViewConfig::default());
        assert_eq!(manager.config().title, "Note Craft");
    }

    #[test]
    fn test_webview_manager_display_markdown() {
        let mut manager = WebViewManager::with_defaults();
        // Initialiser avec le backend stub
        let backend = Box::new(StubWebViewBackend::new(WebViewConfig::default()));
        manager.set_backend(backend);

        // Tester l'affichage (le stub ne fera que logger)
        assert!(manager.display_markdown("# Test").is_ok());
    }

    #[test]
    fn test_webview_pool_switch() {
        let mut pool = WebViewPool::new();
        pool.add_manager(WebViewManager::with_defaults());
        pool.add_manager(WebViewManager::with_defaults());

        assert!(pool.switch_to(0).is_ok());
        assert!(pool.switch_to(1).is_ok());
        assert!(pool.switch_to(2).is_err());
    }
}
