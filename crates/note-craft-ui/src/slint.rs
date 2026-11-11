//! Slint UI Integration Module
//!
//! Ce module fournit:
//! - Intégration avec le framework Slint pour l'interface utilisateur
//! - Composants de base pour Note Craft
//! - Theme system
//! - Communication entre Slint et Rust
//!
//! Note: Ce module fournit l'infrastructure pour Slint.
//! Les composants visuels sont définis dans des fichiers .slint séparés.

use slint::Color;
use slint::SharedString;

/// Couleurs du thème par défaut
pub mod theme {
    use super::*;

    /// Couleurs du thème sombre
    pub const DARK_BACKGROUND: Color = Color::from_rgb_u8(40, 42, 54);
    pub const DARK_SURFACE: Color = Color::from_rgb_u8(68, 71, 90);
    pub const DARK_TEXT: Color = Color::from_rgb_u8(248, 248, 242);
    pub const DARK_TEXT_SECONDARY: Color = Color::from_rgb_u8(170, 170, 170);
    pub const DARK_PRIMARY: Color = Color::from_rgb_u8(88, 101, 242);
    pub const DARK_ACCENT: Color = Color::from_rgb_u8(139, 233, 253);
    pub const DARK_ERROR: Color = Color::from_rgb_u8(255, 85, 85);
    pub const DARK_SUCCESS: Color = Color::from_rgb_u8(80, 250, 123);
    pub const DARK_BORDER: Color = Color::from_rgb_u8(88, 88, 88);

    /// Couleurs du thème clair
    pub const LIGHT_BACKGROUND: Color = Color::from_rgb_u8(248, 248, 248);
    pub const LIGHT_SURFACE: Color = Color::from_rgb_u8(255, 255, 255);
    pub const LIGHT_TEXT: Color = Color::from_rgb_u8(30, 30, 30);
    pub const LIGHT_TEXT_SECONDARY: Color = Color::from_rgb_u8(120, 120, 120);
    pub const LIGHT_PRIMARY: Color = Color::from_rgb_u8(88, 101, 242);
    pub const LIGHT_ACCENT: Color = Color::from_rgb_u8(139, 233, 253);
    pub const LIGHT_ERROR: Color = Color::from_rgb_u8(255, 85, 85);
    pub const LIGHT_SUCCESS: Color = Color::from_rgb_u8(80, 250, 123);
    pub const LIGHT_BORDER: Color = Color::from_rgb_u8(220, 220, 220);

    /// Thème par défaut (sombre)
    pub fn dark_theme() -> ThemeColors {
        ThemeColors {
            background: DARK_BACKGROUND,
            surface: DARK_SURFACE,
            text: DARK_TEXT,
            text_secondary: DARK_TEXT_SECONDARY,
            primary: DARK_PRIMARY,
            accent: DARK_ACCENT,
            error: DARK_ERROR,
            success: DARK_SUCCESS,
            border: DARK_BORDER,
        }
    }

    /// Thème clair
    pub fn light_theme() -> ThemeColors {
        ThemeColors {
            background: LIGHT_BACKGROUND,
            surface: LIGHT_SURFACE,
            text: LIGHT_TEXT,
            text_secondary: LIGHT_TEXT_SECONDARY,
            primary: LIGHT_PRIMARY,
            accent: LIGHT_ACCENT,
            error: LIGHT_ERROR,
            success: LIGHT_SUCCESS,
            border: LIGHT_BORDER,
        }
    }
}

/// Structure pour les couleurs du thème
#[derive(Debug, Clone, PartialEq)]
pub struct ThemeColors {
    pub background: Color,
    pub surface: Color,
    pub text: Color,
    pub text_secondary: Color,
    pub primary: Color,
    pub accent: Color,
    pub error: Color,
    pub success: Color,
    pub border: Color,
}

impl Default for ThemeColors {
    fn default() -> Self {
        theme::dark_theme()
    }
}

impl ThemeColors {
    /// Crée un thème sombre
    pub fn dark() -> Self {
        theme::dark_theme()
    }

    /// Crée un thème clair
    pub fn light() -> Self {
        theme::light_theme()
    }

    /// Convertit en tuple pour Slint
    pub fn to_slint_colors(
        &self,
    ) -> (
        Color,
        Color,
        Color,
        Color,
        Color,
        Color,
        Color,
        Color,
        Color,
    ) {
        (
            self.background,
            self.surface,
            self.text,
            self.text_secondary,
            self.primary,
            self.accent,
            self.error,
            self.success,
            self.border,
        )
    }
}

/// Types d'événements Slint
#[derive(Debug, Clone, PartialEq)]
pub enum SlintEvent {
    /// Le contenu a été modifié
    ContentChanged(String),
    /// Un nœud a été sélectionné
    NodeSelected(u64),
    /// Une commande a été exécutée
    CommandExecuted(String),
    /// L'éditeur est prêt
    EditorReady,
    /// Le thème a été changé
    ThemeChanged(String),
    /// Notification affichée
    NotificationShown(String),
}

/// Callback pour les événements Slint
#[allow(dead_code)]
type SlintEventCallback = Box<dyn Fn(SlintEvent) + Send + Sync>;

/// Informations sur l'environnement Slint
#[derive(Debug, Clone)]
pub struct SlintInfo {
    /// Version de Slint
    pub version: String,
    /// Plateforme
    pub platform: String,
    /// Backend de rendu
    pub renderer: String,
}

impl Default for SlintInfo {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            platform: Self::detect_platform(),
            renderer: Self::detect_renderer(),
        }
    }
}

impl SlintInfo {
    /// Crée une nouvelle instance avec les informations de l'environnement
    pub fn new() -> Self {
        Self::default()
    }

    /// Détecte la plateforme
    fn detect_platform() -> String {
        if cfg!(target_os = "macos") {
            "macOS".to_string()
        } else if cfg!(target_os = "windows") {
            "Windows".to_string()
        } else if cfg!(target_os = "linux") {
            "Linux".to_string()
        } else {
            "Unknown".to_string()
        }
    }

    /// Détecte le backend de rendu
    fn detect_renderer() -> String {
        // Slint peut utiliser différents backends
        // Pour l'instant, retourner une valeur par défaut
        "default".to_string()
    }
}

/// Utilitaires pour la conversion entre types Rust et Slint
pub mod utils {
    use super::*;

    /// Convertit une chaîne Rust en SharedString Slint
    pub fn to_shared_string(s: &str) -> SharedString {
        SharedString::from(s)
    }

    /// Convertit une SharedString Slint en String Rust
    pub fn from_shared_string(s: SharedString) -> String {
        s.to_string()
    }

    /// Convertit une couleur Rust en Color Slint
    pub fn to_slint_color(r: u8, g: u8, b: u8) -> Color {
        Color::from_rgb_u8(r, g, b)
    }

    /// Convertit une couleur hex en Color Slint
    pub fn hex_to_slint_color(hex: &str) -> Option<Color> {
        let hex = hex.strip_prefix('#')?;
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some(Color::from_rgb_u8(r, g, b))
        } else {
            None
        }
    }

    /// Convertit un tuple RGB en Color
    pub fn rgb_to_color(r: u8, g: u8, b: u8) -> Color {
        Color::from_rgb_u8(r, g, b)
    }
}

/// Gestionnaire des ressources Slint
///
/// Ce gestionnaire permet de charger et de gérer les fichiers .slint
pub struct SlintResourceManager {
    /// Répertoire de base pour les fichiers .slint
    base_dir: String,
    // Note: On ne peut pas cacher des ComponentHandle car ils ne sont pas Clone
    // Dans une implémentation réelle, on utiliserait WeakComponentHandle
}

impl SlintResourceManager {
    /// Crée un nouveau gestionnaire avec un répertoire de base
    pub fn new(base_dir: &str) -> Self {
        Self {
            base_dir: base_dir.to_string(),
        }
    }

    /// Crée un nouveau gestionnaire avec le répertoire par défaut
    pub fn with_defaults() -> Self {
        Self::new("ui")
    }

    /// Définit le répertoire de base
    pub fn set_base_dir(&mut self, base_dir: &str) {
        self.base_dir = base_dir.to_string();
    }

    /// Retourne le répertoire de base
    pub fn base_dir(&self) -> &str {
        &self.base_dir
    }
}

impl Default for SlintResourceManager {
    fn default() -> Self {
        Self::with_defaults()
    }
}

/// Factory pour créer des applications Slint
pub struct SlintAppFactory {
    /// Thème par défaut
    default_theme: ThemeColors,
    /// Gestionnaire de ressources
    resource_manager: SlintResourceManager,
}

impl SlintAppFactory {
    /// Crée une nouvelle factory
    pub fn new() -> Self {
        Self {
            default_theme: ThemeColors::default(),
            resource_manager: SlintResourceManager::with_defaults(),
        }
    }

    /// Crée une nouvelle factory avec un thème spécifique
    pub fn with_theme(theme: ThemeColors) -> Self {
        Self {
            default_theme: theme,
            resource_manager: SlintResourceManager::with_defaults(),
        }
    }

    /// Définit le thème par défaut
    pub fn set_default_theme(&mut self, theme: ThemeColors) {
        self.default_theme = theme;
    }

    /// Définit le gestionnaire de ressources
    pub fn set_resource_manager(&mut self, manager: SlintResourceManager) {
        self.resource_manager = manager;
    }

    /// Retourne le thème par défaut
    pub fn default_theme(&self) -> &ThemeColors {
        &self.default_theme
    }

    /// Retourne le gestionnaire de ressources
    pub fn resource_manager(&self) -> &SlintResourceManager {
        &self.resource_manager
    }
}

impl Default for SlintAppFactory {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait pour les composants Slint personnalisés
pub trait SlintComponent: Send + Sync {
    /// Nom du composant
    fn name(&self) -> &str;

    /// Chemin du fichier .slint
    fn path(&self) -> &str;

    /// Initialise le composant
    fn init(&mut self);

    /// Nettoie le composant
    fn cleanup(&mut self);
}

/// Builder pour ThemeColors
pub struct ThemeColorsBuilder {
    background: Option<Color>,
    surface: Option<Color>,
    text: Option<Color>,
    text_secondary: Option<Color>,
    primary: Option<Color>,
    accent: Option<Color>,
    error: Option<Color>,
    success: Option<Color>,
    border: Option<Color>,
}

impl ThemeColorsBuilder {
    pub fn new() -> Self {
        Self {
            background: None,
            surface: None,
            text: None,
            text_secondary: None,
            primary: None,
            accent: None,
            error: None,
            success: None,
            border: None,
        }
    }

    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    pub fn surface(mut self, color: Color) -> Self {
        self.surface = Some(color);
        self
    }

    pub fn text(mut self, color: Color) -> Self {
        self.text = Some(color);
        self
    }

    pub fn text_secondary(mut self, color: Color) -> Self {
        self.text_secondary = Some(color);
        self
    }

    pub fn primary(mut self, color: Color) -> Self {
        self.primary = Some(color);
        self
    }

    pub fn accent(mut self, color: Color) -> Self {
        self.accent = Some(color);
        self
    }

    pub fn error(mut self, color: Color) -> Self {
        self.error = Some(color);
        self
    }

    pub fn success(mut self, color: Color) -> Self {
        self.success = Some(color);
        self
    }

    pub fn border(mut self, color: Color) -> Self {
        self.border = Some(color);
        self
    }

    pub fn build(self) -> ThemeColors {
        ThemeColors {
            background: self.background.unwrap_or(theme::DARK_BACKGROUND),
            surface: self.surface.unwrap_or(theme::DARK_SURFACE),
            text: self.text.unwrap_or(theme::DARK_TEXT),
            text_secondary: self.text_secondary.unwrap_or(theme::DARK_TEXT_SECONDARY),
            primary: self.primary.unwrap_or(theme::DARK_PRIMARY),
            accent: self.accent.unwrap_or(theme::DARK_ACCENT),
            error: self.error.unwrap_or(theme::DARK_ERROR),
            success: self.success.unwrap_or(theme::DARK_SUCCESS),
            border: self.border.unwrap_or(theme::DARK_BORDER),
        }
    }
}

impl Default for ThemeColorsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_dark() {
        let theme = theme::dark_theme();
        assert_eq!(theme.background, theme::DARK_BACKGROUND);
        assert_eq!(theme.text, theme::DARK_TEXT);
    }

    #[test]
    fn test_theme_light() {
        let theme = theme::light_theme();
        assert_eq!(theme.background, theme::LIGHT_BACKGROUND);
        assert_eq!(theme.text, theme::LIGHT_TEXT);
    }

    #[test]
    fn test_theme_colors_default() {
        let theme = ThemeColors::default();
        assert_eq!(theme.background, theme::DARK_BACKGROUND);
    }

    #[test]
    fn test_theme_colors_dark() {
        let theme = ThemeColors::dark();
        assert_eq!(theme.background, theme::DARK_BACKGROUND);
    }

    #[test]
    fn test_theme_colors_light() {
        let theme = ThemeColors::light();
        assert_eq!(theme.background, theme::LIGHT_BACKGROUND);
    }

    #[test]
    fn test_theme_colors_to_slint_colors() {
        let theme = theme::dark_theme();
        let colors = theme.to_slint_colors();
        assert_eq!(colors.0, theme.background);
        assert_eq!(colors.1, theme.surface);
        assert_eq!(colors.2, theme.text);
    }

    #[test]
    fn test_slint_info() {
        let info = SlintInfo::new();
        assert!(!info.version.is_empty());
        assert!(!info.platform.is_empty());
        assert!(!info.renderer.is_empty());
    }

    #[test]
    fn test_utils_to_shared_string() {
        let s = "test";
        let shared = utils::to_shared_string(s);
        assert_eq!(shared.to_string(), s);
    }

    #[test]
    fn test_utils_from_shared_string() {
        let s = "test";
        let shared = utils::to_shared_string(s);
        let result = utils::from_shared_string(shared);
        assert_eq!(result, s);
    }

    #[test]
    fn test_utils_hex_to_slint_color() {
        let color = utils::hex_to_slint_color("#FF0000").unwrap();
        let expected = Color::from_rgb_u8(255, 0, 0);
        assert_eq!(color, expected);

        let color = utils::hex_to_slint_color("#00FF00").unwrap();
        let expected = Color::from_rgb_u8(0, 255, 0);
        assert_eq!(color, expected);

        assert!(utils::hex_to_slint_color("invalid").is_none());
    }

    #[test]
    fn test_utils_rgb_to_color() {
        let color = utils::rgb_to_color(255, 0, 0);
        let expected = Color::from_rgb_u8(255, 0, 0);
        assert_eq!(color, expected);
    }

    #[test]
    fn test_slint_event_clone() {
        let event = SlintEvent::ContentChanged("test".to_string());
        let cloned = event.clone();
        match cloned {
            SlintEvent::ContentChanged(content) => {
                assert_eq!(content, "test");
            }
            _ => panic!("Wrong event type"),
        }
    }

    #[test]
    fn test_theme_colors_clone() {
        let theme = theme::dark_theme();
        let cloned = theme.clone();
        assert_eq!(theme.background, cloned.background);
        assert_eq!(theme.text, cloned.text);
    }

    #[test]
    fn test_theme_colors_partial_eq() {
        let theme1 = theme::dark_theme();
        let theme2 = theme::dark_theme();
        assert_eq!(theme1, theme2);

        let theme3 = theme::light_theme();
        assert_ne!(theme1, theme3);
    }

    #[test]
    fn test_slint_resource_manager() {
        let manager = SlintResourceManager::new("ui");
        assert_eq!(manager.base_dir(), "ui");

        let manager = SlintResourceManager::with_defaults();
        assert_eq!(manager.base_dir(), "ui");
    }

    #[test]
    fn test_slint_app_factory() {
        let factory = SlintAppFactory::new();
        assert!(factory.default_theme().background == theme::DARK_BACKGROUND);

        let factory = SlintAppFactory::with_theme(theme::light_theme());
        assert!(factory.default_theme().background == theme::LIGHT_BACKGROUND);
    }

    #[test]
    fn test_theme_colors_builder() {
        let theme = ThemeColorsBuilder::new()
            .background(Color::from_rgb_u8(0, 0, 0))
            .text(Color::from_rgb_u8(255, 255, 255))
            .build();

        assert_eq!(theme.background, Color::from_rgb_u8(0, 0, 0));
        assert_eq!(theme.text, Color::from_rgb_u8(255, 255, 255));
        // Les autres couleurs utilisent les valeurs par défaut
        assert_eq!(theme.surface, theme::DARK_SURFACE);
    }

    #[test]
    fn test_theme_colors_builder_default() {
        let theme = ThemeColorsBuilder::default().build();
        assert_eq!(theme, ThemeColors::default());
    }

    #[test]
    fn test_slint_event_partial_eq() {
        let event1 = SlintEvent::ContentChanged("test".to_string());
        let event2 = SlintEvent::ContentChanged("test".to_string());
        assert_eq!(event1, event2);

        let event3 = SlintEvent::EditorReady;
        assert_ne!(event1, event3);
    }
}
