//! Theme System Module
//!
//! This module provides a comprehensive theme system for Note Craft UI.
//! It supports light, dark, and system themes with automatic switching.

use std::collections::HashMap;
use std::fmt;

use slint::{Color, SharedString};

use crate::slint::{theme as slint_theme, ThemeColors};

/// Available theme variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeVariant {
    /// Light theme
    Light,
    /// Dark theme
    Dark,
    /// Follow system preference
    System,
}

impl fmt::Display for ThemeVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThemeVariant::Light => write!(f, "light"),
            ThemeVariant::Dark => write!(f, "dark"),
            ThemeVariant::System => write!(f, "system"),
        }
    }
}

impl ThemeVariant {
    /// Parses a theme variant from a string
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "light" => Some(Self::Light),
            "dark" => Some(Self::Dark),
            "system" => Some(Self::System),
            _ => None,
        }
    }

    /// Gets all variants
    pub fn variants() -> &'static [Self] {
        &[Self::Light, Self::Dark, Self::System]
    }
}

/// Theme configuration
#[derive(Debug, Clone)]
pub struct ThemeConfig {
    /// Current theme variant
    pub variant: ThemeVariant,
    /// Whether to use system preference
    pub follow_system: bool,
    /// Custom theme colors (overrides defaults)
    pub custom_colors: Option<ThemeColors>,
    /// Whether theme has been explicitly set
    pub explicit: bool,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            variant: ThemeVariant::Dark, // Dark is default
            follow_system: false,
            custom_colors: None,
            explicit: false,
        }
    }
}

impl ThemeConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn dark() -> Self {
        Self {
            variant: ThemeVariant::Dark,
            follow_system: false,
            custom_colors: None,
            explicit: true,
        }
    }

    pub fn light() -> Self {
        Self {
            variant: ThemeVariant::Light,
            follow_system: false,
            custom_colors: None,
            explicit: true,
        }
    }

    pub fn system() -> Self {
        Self {
            variant: ThemeVariant::System,
            follow_system: true,
            custom_colors: None,
            explicit: true,
        }
    }

    pub fn with_variant(mut self, variant: ThemeVariant) -> Self {
        self.variant = variant;
        self.follow_system = matches!(variant, ThemeVariant::System);
        self.explicit = true;
        self
    }

    pub fn with_custom_colors(mut self, colors: ThemeColors) -> Self {
        self.custom_colors = Some(colors);
        self
    }

    pub fn is_dark(&self) -> bool {
        match self.variant {
            ThemeVariant::Dark => true,
            ThemeVariant::Light => false,
            ThemeVariant::System => {
                // Check system preference
                self.detect_system_dark_mode()
            }
        }
    }

    /// Detects if system is in dark mode (stub - platform specific)
    fn detect_system_dark_mode(&self) -> bool {
        // Default to dark for now
        // This would be implemented with platform-specific code
        true
    }
}

/// Theme colors with additional UI-specific colors
#[derive(Debug, Clone, PartialEq)]
pub struct UiThemeColors {
    /// Slint theme colors
    pub slint: ThemeColors,
    /// Background color for editor
    pub editor_background: Color,
    /// Text color for editor
    pub editor_text: Color,
    /// Selection background color
    pub selection_background: Color,
    /// Selection text color
    pub selection_text: Color,
    /// Cursor color
    pub cursor_color: Color,
    /// Line number color
    pub line_number_color: Color,
    /// Code block background
    pub code_block_background: Color,
    /// Inline code background
    pub inline_code_background: Color,
    /// Link color
    pub link_color: Color,
    /// Blockquote border color
    pub blockquote_border: Color,
}

impl UiThemeColors {
    pub fn dark() -> Self {
        Self {
            slint: slint_theme::dark_theme(),
            editor_background: Color::from_rgb_u8(30, 30, 30),
            editor_text: Color::from_rgb_u8(220, 220, 220),
            selection_background: Color::from_rgb_u8(80, 80, 200),
            selection_text: Color::from_rgb_u8(255, 255, 255),
            cursor_color: Color::from_rgb_u8(200, 200, 200),
            line_number_color: Color::from_rgb_u8(120, 120, 120),
            code_block_background: Color::from_rgb_u8(40, 40, 40),
            inline_code_background: Color::from_rgb_u8(50, 50, 50),
            link_color: Color::from_rgb_u8(100, 180, 255),
            blockquote_border: Color::from_rgb_u8(100, 100, 100),
        }
    }

    pub fn light() -> Self {
        Self {
            slint: slint_theme::light_theme(),
            editor_background: Color::from_rgb_u8(255, 255, 255),
            editor_text: Color::from_rgb_u8(30, 30, 30),
            selection_background: Color::from_rgb_u8(200, 200, 255),
            selection_text: Color::from_rgb_u8(0, 0, 0),
            cursor_color: Color::from_rgb_u8(0, 0, 0),
            line_number_color: Color::from_rgb_u8(150, 150, 150),
            code_block_background: Color::from_rgb_u8(240, 240, 240),
            inline_code_background: Color::from_rgb_u8(230, 230, 230),
            link_color: Color::from_rgb_u8(0, 100, 200),
            blockquote_border: Color::from_rgb_u8(200, 200, 200),
        }
    }

    pub fn system() -> Self {
        // Default to dark
        Self::dark()
    }

    pub fn from_config(config: &ThemeConfig) -> Self {
        match config.variant {
            ThemeVariant::Dark => Self::dark(),
            ThemeVariant::Light => Self::light(),
            ThemeVariant::System => Self::system(),
        }
    }
}

impl Default for UiThemeColors {
    fn default() -> Self {
        Self::dark()
    }
}

/// Theme component colors for specific UI elements
#[derive(Debug, Clone)]
pub struct ThemeComponentColors {
    /// Sidebar colors
    pub sidebar: SidebarTheme,
    /// Editor colors
    pub editor: EditorTheme,
    /// Button colors
    pub button: ButtonTheme,
    /// Input colors
    pub input: InputTheme,
    /// Dialog colors
    pub dialog: DialogTheme,
}

impl ThemeComponentColors {
    pub fn dark() -> Self {
        Self {
            sidebar: SidebarTheme::dark(),
            editor: EditorTheme::dark(),
            button: ButtonTheme::dark(),
            input: InputTheme::dark(),
            dialog: DialogTheme::dark(),
        }
    }

    pub fn light() -> Self {
        Self {
            sidebar: SidebarTheme::light(),
            editor: EditorTheme::light(),
            button: ButtonTheme::light(),
            input: InputTheme::light(),
            dialog: DialogTheme::light(),
        }
    }
}

impl Default for ThemeComponentColors {
    fn default() -> Self {
        Self::dark()
    }
}

/// Sidebar theme colors
#[derive(Debug, Clone)]
pub struct SidebarTheme {
    pub background: Color,
    pub text: Color,
    pub text_secondary: Color,
    pub hover_background: Color,
    pub selected_background: Color,
    pub selected_text: Color,
    pub border: Color,
}

impl SidebarTheme {
    pub fn dark() -> Self {
        Self {
            background: Color::from_rgb_u8(40, 42, 54),
            text: Color::from_rgb_u8(200, 200, 200),
            text_secondary: Color::from_rgb_u8(150, 150, 150),
            hover_background: Color::from_rgb_u8(60, 62, 74),
            selected_background: Color::from_rgb_u8(80, 82, 94),
            selected_text: Color::from_rgb_u8(255, 255, 255),
            border: Color::from_rgb_u8(60, 60, 60),
        }
    }

    pub fn light() -> Self {
        Self {
            background: Color::from_rgb_u8(245, 245, 245),
            text: Color::from_rgb_u8(50, 50, 50),
            text_secondary: Color::from_rgb_u8(120, 120, 120),
            hover_background: Color::from_rgb_u8(230, 230, 230),
            selected_background: Color::from_rgb_u8(200, 220, 255),
            selected_text: Color::from_rgb_u8(0, 0, 0),
            border: Color::from_rgb_u8(220, 220, 220),
        }
    }
}

/// Editor theme colors
#[derive(Debug, Clone)]
pub struct EditorTheme {
    pub background: Color,
    pub text: Color,
    pub selection_background: Color,
    pub selection_text: Color,
    pub cursor: Color,
    pub line_numbers: Color,
    pub line_highlight: Color,
    pub code_block_background: Color,
    pub inline_code_background: Color,
}

impl EditorTheme {
    pub fn dark() -> Self {
        Self {
            background: Color::from_rgb_u8(30, 30, 30),
            text: Color::from_rgb_u8(220, 220, 220),
            selection_background: Color::from_rgb_u8(80, 80, 200),
            selection_text: Color::from_rgb_u8(255, 255, 255),
            cursor: Color::from_rgb_u8(200, 200, 200),
            line_numbers: Color::from_rgb_u8(120, 120, 120),
            line_highlight: Color::from_rgb_u8(40, 40, 40),
            code_block_background: Color::from_rgb_u8(40, 40, 40),
            inline_code_background: Color::from_rgb_u8(50, 50, 50),
        }
    }

    pub fn light() -> Self {
        Self {
            background: Color::from_rgb_u8(255, 255, 255),
            text: Color::from_rgb_u8(30, 30, 30),
            selection_background: Color::from_rgb_u8(200, 200, 255),
            selection_text: Color::from_rgb_u8(0, 0, 0),
            cursor: Color::from_rgb_u8(0, 0, 0),
            line_numbers: Color::from_rgb_u8(150, 150, 150),
            line_highlight: Color::from_rgb_u8(245, 245, 245),
            code_block_background: Color::from_rgb_u8(240, 240, 240),
            inline_code_background: Color::from_rgb_u8(230, 230, 230),
        }
    }
}

/// Button theme colors
#[derive(Debug, Clone)]
pub struct ButtonTheme {
    pub background: Color,
    pub text: Color,
    pub hover_background: Color,
    pub hover_text: Color,
    pub active_background: Color,
    pub active_text: Color,
    pub border: Color,
    pub disabled_background: Color,
    pub disabled_text: Color,
}

impl ButtonTheme {
    pub fn dark() -> Self {
        Self {
            background: Color::from_rgb_u8(60, 60, 60),
            text: Color::from_rgb_u8(220, 220, 220),
            hover_background: Color::from_rgb_u8(80, 80, 80),
            hover_text: Color::from_rgb_u8(255, 255, 255),
            active_background: Color::from_rgb_u8(100, 100, 100),
            active_text: Color::from_rgb_u8(255, 255, 255),
            border: Color::from_rgb_u8(80, 80, 80),
            disabled_background: Color::from_rgb_u8(50, 50, 50),
            disabled_text: Color::from_rgb_u8(150, 150, 150),
        }
    }

    pub fn light() -> Self {
        Self {
            background: Color::from_rgb_u8(230, 230, 230),
            text: Color::from_rgb_u8(30, 30, 30),
            hover_background: Color::from_rgb_u8(200, 200, 200),
            hover_text: Color::from_rgb_u8(0, 0, 0),
            active_background: Color::from_rgb_u8(180, 180, 180),
            active_text: Color::from_rgb_u8(0, 0, 0),
            border: Color::from_rgb_u8(180, 180, 180),
            disabled_background: Color::from_rgb_u8(240, 240, 240),
            disabled_text: Color::from_rgb_u8(150, 150, 150),
        }
    }
}

/// Input theme colors
#[derive(Debug, Clone)]
pub struct InputTheme {
    pub background: Color,
    pub text: Color,
    pub border: Color,
    pub focus_border: Color,
    pub placeholder: Color,
    pub disabled_background: Color,
    pub disabled_text: Color,
}

impl InputTheme {
    pub fn dark() -> Self {
        Self {
            background: Color::from_rgb_u8(50, 50, 50),
            text: Color::from_rgb_u8(220, 220, 220),
            border: Color::from_rgb_u8(80, 80, 80),
            focus_border: Color::from_rgb_u8(100, 150, 255),
            placeholder: Color::from_rgb_u8(150, 150, 150),
            disabled_background: Color::from_rgb_u8(40, 40, 40),
            disabled_text: Color::from_rgb_u8(120, 120, 120),
        }
    }

    pub fn light() -> Self {
        Self {
            background: Color::from_rgb_u8(250, 250, 250),
            text: Color::from_rgb_u8(30, 30, 30),
            border: Color::from_rgb_u8(200, 200, 200),
            focus_border: Color::from_rgb_u8(0, 100, 200),
            placeholder: Color::from_rgb_u8(150, 150, 150),
            disabled_background: Color::from_rgb_u8(240, 240, 240),
            disabled_text: Color::from_rgb_u8(120, 120, 120),
        }
    }
}

/// Dialog theme colors
#[derive(Debug, Clone)]
pub struct DialogTheme {
    pub background: Color,
    pub text: Color,
    pub border: Color,
    pub title_bar: Color,
    pub title_text: Color,
}

impl DialogTheme {
    pub fn dark() -> Self {
        Self {
            background: Color::from_rgb_u8(50, 50, 50),
            text: Color::from_rgb_u8(220, 220, 220),
            border: Color::from_rgb_u8(80, 80, 80),
            title_bar: Color::from_rgb_u8(40, 40, 40),
            title_text: Color::from_rgb_u8(255, 255, 255),
        }
    }

    pub fn light() -> Self {
        Self {
            background: Color::from_rgb_u8(250, 250, 250),
            text: Color::from_rgb_u8(30, 30, 30),
            border: Color::from_rgb_u8(200, 200, 200),
            title_bar: Color::from_rgb_u8(230, 230, 230),
            title_text: Color::from_rgb_u8(0, 0, 0),
        }
    }
}

/// Complete theme including all components
#[derive(Debug, Clone)]
pub struct Theme {
    /// Theme variant
    pub variant: ThemeVariant,
    /// UI theme colors
    pub colors: UiThemeColors,
    /// Component-specific colors
    pub components: ThemeComponentColors,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            variant: ThemeVariant::Dark,
            colors: UiThemeColors::dark(),
            components: ThemeComponentColors::dark(),
        }
    }

    pub fn light() -> Self {
        Self {
            variant: ThemeVariant::Light,
            colors: UiThemeColors::light(),
            components: ThemeComponentColors::light(),
        }
    }

    pub fn system() -> Self {
        Self {
            variant: ThemeVariant::System,
            colors: UiThemeColors::system(),
            components: ThemeComponentColors::dark(),
        }
    }

    pub fn from_config(config: &ThemeConfig) -> Self {
        match config.variant {
            ThemeVariant::Dark => Self::dark(),
            ThemeVariant::Light => Self::light(),
            ThemeVariant::System => Self::system(),
        }
    }

    pub fn get_slint_colors(&self) -> &ThemeColors {
        &self.colors.slint
    }

    pub fn is_dark(&self) -> bool {
        matches!(self.variant, ThemeVariant::Dark)
            || (matches!(self.variant, ThemeVariant::System) && self.detect_system_dark_mode())
    }

    fn detect_system_dark_mode(&self) -> bool {
        // Default to dark
        true
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

/// Theme event
#[derive(Debug, Clone, PartialEq)]
pub enum ThemeEvent {
    /// Theme was changed
    Changed(ThemeVariant),
    /// System theme preference changed
    SystemChanged(bool), // true = dark mode
}

/// Theme manager - central theme management
pub struct ThemeManager {
    /// Current theme
    current_theme: Theme,
    /// Configuration
    config: ThemeConfig,
    /// Custom themes by name
    custom_themes: HashMap<String, Theme>,
    /// Event handler
    event_handler: Option<Box<dyn Fn(ThemeEvent) + Send + Sync>>,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self {
            current_theme: Theme::dark(),
            config: ThemeConfig::default(),
            custom_themes: HashMap::new(),
            event_handler: None,
        }
    }

    pub fn with_config(config: ThemeConfig) -> Self {
        let theme = Theme::from_config(&config);
        Self {
            current_theme: theme,
            config,
            custom_themes: HashMap::new(),
            event_handler: None,
        }
    }

    pub fn dark() -> Self {
        Self {
            current_theme: Theme::dark(),
            config: ThemeConfig::dark(),
            custom_themes: HashMap::new(),
            event_handler: None,
        }
    }

    pub fn light() -> Self {
        Self {
            current_theme: Theme::light(),
            config: ThemeConfig::light(),
            custom_themes: HashMap::new(),
            event_handler: None,
        }
    }

    pub fn set_theme(&mut self, variant: ThemeVariant) -> &mut Self {
        let old_variant = self.current_theme.variant;
        let new_config = self.config.clone().with_variant(variant);
        self.current_theme = Theme::from_config(&new_config);
        self.config = new_config;

        if old_variant != variant {
            if let Some(handler) = &self.event_handler {
                handler(ThemeEvent::Changed(variant));
            }
        }

        self
    }

    pub fn toggle(&mut self) -> &mut Self {
        let new_variant = match self.current_theme.variant {
            ThemeVariant::Dark => ThemeVariant::Light,
            ThemeVariant::Light => ThemeVariant::Dark,
            ThemeVariant::System => ThemeVariant::Dark,
        };
        self.set_theme(new_variant)
    }

    pub fn set_dark(&mut self) -> &mut Self {
        self.set_theme(ThemeVariant::Dark)
    }

    pub fn set_light(&mut self) -> &mut Self {
        self.set_theme(ThemeVariant::Light)
    }

    pub fn set_system(&mut self) -> &mut Self {
        self.set_theme(ThemeVariant::System)
    }

    pub fn get_theme(&self) -> &Theme {
        &self.current_theme
    }

    pub fn get_slint_colors(&self) -> &ThemeColors {
        self.current_theme.get_slint_colors()
    }

    pub fn get_ui_colors(&self) -> &UiThemeColors {
        &self.current_theme.colors
    }

    pub fn get_component_colors(&self) -> &ThemeComponentColors {
        &self.current_theme.components
    }

    pub fn get_config(&self) -> &ThemeConfig {
        &self.config
    }

    pub fn is_dark(&self) -> bool {
        self.current_theme.is_dark()
    }

    pub fn set_event_handler<F>(&mut self, handler: F) -> &mut Self
    where
        F: Fn(ThemeEvent) + Send + Sync + 'static,
    {
        self.event_handler = Some(Box::new(handler));
        self
    }

    pub fn handle_event(&mut self, event: ThemeEvent) {
        match event {
            ThemeEvent::Changed(variant) => {
                self.set_theme(variant);
            }
            ThemeEvent::SystemChanged(is_dark) => {
                // Handle system theme change
                if matches!(self.current_theme.variant, ThemeVariant::System) {
                    // System theme is active, update if needed
                    // For now, we'll just notify
                    if let Some(handler) = &self.event_handler {
                        handler(ThemeEvent::SystemChanged(is_dark));
                    }
                }
            }
        }
    }

    pub fn register_custom_theme(&mut self, name: impl Into<String>, theme: Theme) -> &mut Self {
        self.custom_themes.insert(name.into(), theme);
        self
    }

    pub fn get_custom_theme(&self, name: &str) -> Option<&Theme> {
        self.custom_themes.get(name)
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating theme managers
#[derive(Default)]
pub struct ThemeManagerBuilder {
    config: ThemeConfig,
    custom_themes: HashMap<String, Theme>,
}

impl ThemeManagerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_variant(mut self, variant: ThemeVariant) -> Self {
        self.config.variant = variant;
        self
    }

    pub fn dark(mut self) -> Self {
        self.config.variant = ThemeVariant::Dark;
        self
    }

    pub fn light(mut self) -> Self {
        self.config.variant = ThemeVariant::Light;
        self
    }

    pub fn system(mut self) -> Self {
        self.config.variant = ThemeVariant::System;
        self
    }

    pub fn add_custom_theme(mut self, name: impl Into<String>, theme: Theme) -> Self {
        self.custom_themes.insert(name.into(), theme);
        self
    }

    pub fn build(self) -> ThemeManager {
        let mut manager = ThemeManager::with_config(self.config);
        for (name, theme) in self.custom_themes {
            manager.custom_themes.insert(name, theme);
        }
        manager
    }
}

// ============================================================================
// Slint Integration
// ============================================================================

/// Converts ThemeColors to a Slint-compatible map
pub fn theme_colors_to_slint_map(colors: &ThemeColors) -> HashMap<String, SharedString> {
    let mut map = HashMap::new();
    map.insert(
        "background".to_string(),
        color_to_hex_string(colors.background),
    );
    map.insert("surface".to_string(), color_to_hex_string(colors.surface));
    map.insert("text".to_string(), color_to_hex_string(colors.text));
    map.insert(
        "text_secondary".to_string(),
        color_to_hex_string(colors.text_secondary),
    );
    map.insert("primary".to_string(), color_to_hex_string(colors.primary));
    map.insert("accent".to_string(), color_to_hex_string(colors.accent));
    map.insert("error".to_string(), color_to_hex_string(colors.error));
    map.insert("success".to_string(), color_to_hex_string(colors.success));
    map.insert("border".to_string(), color_to_hex_string(colors.border));
    map
}

/// Converts Color to hex string
pub fn color_to_hex_string(color: Color) -> SharedString {
    let r = color.red();
    let g = color.green();
    let b = color.blue();
    SharedString::from(format!("#{:02x}{:02x}{:02x}", r, g, b))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_variant_display() {
        assert_eq!(format!("{}", ThemeVariant::Light), "light");
        assert_eq!(format!("{}", ThemeVariant::Dark), "dark");
        assert_eq!(format!("{}", ThemeVariant::System), "system");
    }

    #[test]
    fn test_theme_variant_from_str() {
        assert_eq!(ThemeVariant::from_str("light"), Some(ThemeVariant::Light));
        assert_eq!(ThemeVariant::from_str("dark"), Some(ThemeVariant::Dark));
        assert_eq!(ThemeVariant::from_str("system"), Some(ThemeVariant::System));
        assert_eq!(ThemeVariant::from_str("unknown"), None);
    }

    #[test]
    fn test_theme_config_default() {
        let config = ThemeConfig::default();
        assert_eq!(config.variant, ThemeVariant::Dark);
        assert!(!config.follow_system);
        assert!(!config.explicit);
    }

    #[test]
    fn test_theme_config_dark() {
        let config = ThemeConfig::dark();
        assert_eq!(config.variant, ThemeVariant::Dark);
        assert!(config.explicit);
    }

    #[test]
    fn test_theme_config_light() {
        let config = ThemeConfig::light();
        assert_eq!(config.variant, ThemeVariant::Light);
        assert!(config.explicit);
    }

    #[test]
    fn test_theme_config_system() {
        let config = ThemeConfig::system();
        assert_eq!(config.variant, ThemeVariant::System);
        assert!(config.follow_system);
        assert!(config.explicit);
    }

    #[test]
    fn test_theme_default() {
        let theme = Theme::default();
        assert_eq!(theme.variant, ThemeVariant::Dark);
    }

    #[test]
    fn test_theme_dark() {
        let theme = Theme::dark();
        assert_eq!(theme.variant, ThemeVariant::Dark);
        assert!(theme.is_dark());
    }

    #[test]
    fn test_theme_light() {
        let theme = Theme::light();
        assert_eq!(theme.variant, ThemeVariant::Light);
        assert!(!theme.is_dark());
    }

    #[test]
    fn test_theme_from_config() {
        let config = ThemeConfig::light();
        let theme = Theme::from_config(&config);
        assert_eq!(theme.variant, ThemeVariant::Light);
    }

    #[test]
    fn test_theme_manager_new() {
        let manager = ThemeManager::new();
        assert!(manager.is_dark());
    }

    #[test]
    fn test_theme_manager_dark() {
        let manager = ThemeManager::dark();
        assert!(manager.is_dark());
        assert_eq!(manager.get_theme().variant, ThemeVariant::Dark);
    }

    #[test]
    fn test_theme_manager_light() {
        let manager = ThemeManager::light();
        assert!(!manager.is_dark());
        assert_eq!(manager.get_theme().variant, ThemeVariant::Light);
    }

    #[test]
    fn test_theme_manager_set_theme() {
        let mut manager = ThemeManager::new();
        manager.set_theme(ThemeVariant::Light);
        assert!(!manager.is_dark());
        assert_eq!(manager.get_theme().variant, ThemeVariant::Light);
    }

    #[test]
    fn test_theme_manager_toggle() {
        let mut manager = ThemeManager::dark();
        assert!(manager.is_dark());

        manager.toggle();
        assert!(!manager.is_dark());

        manager.toggle();
        assert!(manager.is_dark());
    }

    #[test]
    fn test_theme_manager_set_dark() {
        let mut manager = ThemeManager::light();
        manager.set_dark();
        assert!(manager.is_dark());
    }

    #[test]
    fn test_theme_manager_set_light() {
        let mut manager = ThemeManager::dark();
        manager.set_light();
        assert!(!manager.is_dark());
    }

    #[test]
    fn test_theme_manager_get_slint_colors() {
        let manager = ThemeManager::dark();
        let colors = manager.get_slint_colors();
        assert_eq!(colors, &ThemeColors::dark());
    }

    #[test]
    fn test_theme_manager_get_ui_colors() {
        let manager = ThemeManager::dark();
        let colors = manager.get_ui_colors();
        assert_eq!(colors, &UiThemeColors::dark());
    }

    #[test]
    fn test_theme_component_colors_dark() {
        let colors = ThemeComponentColors::dark();
        assert_eq!(colors.sidebar.background, Color::from_rgb_u8(40, 42, 54));
        assert_eq!(colors.editor.background, Color::from_rgb_u8(30, 30, 30));
    }

    #[test]
    fn test_theme_component_colors_light() {
        let colors = ThemeComponentColors::light();
        assert_eq!(colors.sidebar.background, Color::from_rgb_u8(245, 245, 245));
        assert_eq!(colors.editor.background, Color::from_rgb_u8(255, 255, 255));
    }

    #[test]
    fn test_theme_event_handling() {
        use std::sync::{Arc, Mutex};

        let mut manager = ThemeManager::new();
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events.clone();

        manager.set_event_handler(move |event| {
            events_clone.lock().unwrap().push(event);
        });

        manager.set_theme(ThemeVariant::Light);
        let events = events.lock().unwrap();
        assert!(!events.is_empty());
        assert!(matches!(
            events[0],
            ThemeEvent::Changed(ThemeVariant::Light)
        ));
    }

    #[test]
    fn test_theme_builder() {
        let manager = ThemeManagerBuilder::new().dark().build();
        assert!(manager.is_dark());
    }

    #[test]
    fn test_color_to_hex_string() {
        let color = Color::from_rgb_u8(255, 0, 0);
        let hex = color_to_hex_string(color);
        assert_eq!(hex.as_str(), "#ff0000");
    }

    #[test]
    fn test_theme_colors_to_slint_map() {
        let colors = ThemeColors::dark();
        let map = theme_colors_to_slint_map(&colors);
        assert!(map.contains_key("background"));
        assert!(map.contains_key("text"));
        assert!(map.contains_key("primary"));
    }

    #[test]
    fn test_ui_theme_colors_dark() {
        let colors = UiThemeColors::dark();
        assert_eq!(colors.editor_background, Color::from_rgb_u8(30, 30, 30));
        assert_eq!(colors.editor_text, Color::from_rgb_u8(220, 220, 220));
    }

    #[test]
    fn test_ui_theme_colors_light() {
        let colors = UiThemeColors::light();
        assert_eq!(colors.editor_background, Color::from_rgb_u8(255, 255, 255));
        assert_eq!(colors.editor_text, Color::from_rgb_u8(30, 30, 30));
    }

    #[test]
    fn test_sidebar_theme_dark() {
        let theme = SidebarTheme::dark();
        assert_eq!(theme.background, Color::from_rgb_u8(40, 42, 54));
        assert_eq!(theme.text, Color::from_rgb_u8(200, 200, 200));
    }

    #[test]
    fn test_editor_theme_dark() {
        let theme = EditorTheme::dark();
        assert_eq!(theme.background, Color::from_rgb_u8(30, 30, 30));
        assert_eq!(theme.text, Color::from_rgb_u8(220, 220, 220));
    }

    #[test]
    fn test_button_theme_dark() {
        let theme = ButtonTheme::dark();
        assert_eq!(theme.background, Color::from_rgb_u8(60, 60, 60));
        assert_eq!(theme.text, Color::from_rgb_u8(220, 220, 220));
    }

    #[test]
    fn test_input_theme_dark() {
        let theme = InputTheme::dark();
        assert_eq!(theme.background, Color::from_rgb_u8(50, 50, 50));
        assert_eq!(theme.text, Color::from_rgb_u8(220, 220, 220));
    }

    #[test]
    fn test_dialog_theme_dark() {
        let theme = DialogTheme::dark();
        assert_eq!(theme.background, Color::from_rgb_u8(50, 50, 50));
        assert_eq!(theme.text, Color::from_rgb_u8(220, 220, 220));
    }

    #[test]
    fn test_theme_custom_themes() {
        let mut manager = ThemeManager::new();
        let custom_theme = Theme::light();
        manager.register_custom_theme("custom", custom_theme);

        assert!(manager.get_custom_theme("custom").is_some());
    }

    #[test]
    fn test_theme_builder_with_custom_theme() {
        let custom_theme = Theme::light();
        let manager = ThemeManagerBuilder::new()
            .add_custom_theme("custom", custom_theme)
            .build();

        assert!(manager.get_custom_theme("custom").is_some());
    }
}
