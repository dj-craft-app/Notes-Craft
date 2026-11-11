//! Note Craft UI - Composants d'interface
//!
//! Ce module fournit:
//! - Structure de base pour l'UI
//! - Intégration WebView avec wry
//! - Intégration future avec Slint
//!
//! Note: Phase 2 - WebView integration added

use note_craft_core::Node;

pub mod block_ui;
pub mod page_ui;
pub mod sidebar;
pub mod slint;
pub mod webview;

// Ré-export des types principaux
pub use block_ui::{
    BlockRenderInfo, BlockStyleConfig, BlockUiEvent, BlockUiManager, BlockUiManagerBuilder,
    BlockUiState,
};
pub use page_ui::{
    PageConfig, PageLayoutMode, PageRenderInfo, PageSidebarItemBuilder, PageUiEvent, PageUiManager,
    PageUiManagerBuilder, PageUiState, PageViewMode,
};
pub use sidebar::{
    default_sidebar_sections, SidebarConfig, SidebarEvent, SidebarItem, SidebarItemBuilder,
    SidebarSection, SidebarService, SidebarState,
};
pub use slint::{
    theme, utils, SlintAppFactory, SlintEvent, SlintInfo, SlintResourceManager, ThemeColors,
    ThemeColorsBuilder,
};
pub use webview::{
    DefaultWebViewHandler, JsMessage, RustMessage, WebViewConfig, WebViewError,
    WebViewEventHandler, WebViewManager, WebViewManagerBuilder, WebViewPool,
};

/// Initialise l'application UI
pub fn init_ui() -> Result<(), Box<dyn std::error::Error>> {
    // Initialisation de base
    Ok(())
}

/// Structure principale de l'UI
pub struct AppUi {
    /// Gestionnaire de WebView
    webview_manager: Option<webview::WebViewManager>,
}

impl AppUi {
    pub fn new() -> Self {
        Self {
            webview_manager: None,
        }
    }

    /// Affiche un nœud (stub pour compilation)
    pub fn display_node(&mut self, node: &Node) -> Result<(), webview::WebViewError> {
        if let Some(manager) = &mut self.webview_manager {
            manager.display_node(node)
        } else {
            Ok(())
        }
    }

    /// Initialise la WebView
    pub fn initialize_webview(
        &mut self,
        config: webview::WebViewConfig,
    ) -> Result<(), webview::WebViewError> {
        let mut manager = webview::WebViewManager::new(config);
        manager.initialize()?;
        self.webview_manager = Some(manager);
        Ok(())
    }

    /// Retourne le gestionnaire de WebView
    pub fn webview_manager(&mut self) -> Option<&mut webview::WebViewManager> {
        self.webview_manager.as_mut()
    }
}

impl Default for AppUi {
    fn default() -> Self {
        Self::new()
    }
}

/// Tests de compilation
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_init() {
        let result = init_ui();
        assert!(result.is_ok());
    }

    #[test]
    fn test_app_ui_creation() {
        let _app = AppUi::new();
        // Compilation réussie
    }
}
