//! Note Craft UI - Composants d'interface
//!
//! Ce module fournit:
//! - Structure de base pour l'UI
//! - Intégration future avec Slint
//!
//! Note: Phase 0 - Stub minimal pour compilation

use note_craft_core::Node;

/// Initialise l'application UI
pub fn init_ui() -> Result<(), Box<dyn std::error::Error>> {
    // Initialisation de base
    Ok(())
}

/// Structure principale de l'UI
pub struct AppUi;

impl AppUi {
    pub fn new() -> Self {
        AppUi
    }

    /// Affiche un nœud (stub pour compilation)
    pub fn display_node(&self, _node: &Node) {
        // À implémenter avec Slint
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
