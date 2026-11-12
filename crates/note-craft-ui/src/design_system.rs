//! Design System Module
//!
//! This module provides the Note Craft Design System based on Obsidian.
//! It defines colors, typography, spacing, and component styles.
//!
//! Note: Phase 3 - Step 3.15 (Apply Design System)
//!
//! Source: docs/audits/note_craft_design_system.md

use slint::Color;

// ============================================================================
// Color Palette
// ============================================================================

/// Base color palette for dark theme (primary theme)
pub mod colors {
    use super::Color;

    // Helper function to create color from RGB
    #[allow(dead_code)]
    fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color::from_argb_u8(255, r, g, b)
    }

    // Background colors (mono scale)
    pub const BACKGROUND_PRIMARY: Color = Color::from_argb_u8(255, 0x1C, 0x1C, 0x1C); // #1C1C1C
    pub const BACKGROUND_PRIMARY_ALT: Color = Color::from_argb_u8(255, 0x21, 0x21, 0x21); // #212121
    pub const BACKGROUND_SECONDARY: Color = Color::from_argb_u8(255, 0x28, 0x28, 0x28); // #282828
    pub const BACKGROUND_TERTIARY: Color = Color::from_argb_u8(255, 0x2E, 0x2E, 0x2E); // #2E2E2E
    pub const BACKGROUND_QUATERNARY: Color = Color::from_argb_u8(255, 0x33, 0x33, 0x33); // #333333
    pub const BACKGROUND_QUINARY: Color = Color::from_argb_u8(255, 0x3F, 0x3F, 0x3F); // #3F3F3F
    pub const BACKGROUND_LIGHT: Color = Color::from_argb_u8(255, 0x55, 0x55, 0x55); // #555555

    // Text colors (mono scale)
    pub const TEXT_PRIMARY: Color = Color::from_argb_u8(255, 0xDA, 0xDA, 0xDA); // #dadada
    pub const TEXT_SECONDARY: Color = Color::from_argb_u8(255, 0x99, 0x99, 0x99); // #999999
    pub const TEXT_MUTED: Color = Color::from_argb_u8(255, 0x66, 0x66, 0x66); // #666666
    pub const TEXT_FAINT: Color = Color::from_argb_u8(255, 0x5C, 0x5C, 0x5C); // #5c5c5c

    // Accent colors
    pub const ACCENT_PRIMARY: Color = Color::from_argb_u8(255, 0x02, 0x7A, 0xFF); // #027aff
    pub const ACCENT_HOVER: Color = Color::from_argb_u8(255, 0x80, 0xEB, 0xFF); // #80ebff
    pub const ACCENT_ACTIVE: Color = Color::from_argb_u8(255, 0xAA, 0xF2, 0xFF); // #aaf2ff
    pub const ACCENT_FOCUS_RING: Color = Color::from_argb_u8(255, 0x00, 0xDD, 0xFF); // #0df

    // Syntax colors
    pub const SYNTAX_TEXT_NORMAL: Color = Color::from_argb_u8(255, 0xDA, 0xDA, 0xDA); // #dadada
    pub const SYNTAX_COMMENT: Color = Color::from_argb_u8(255, 0x66, 0x66, 0x66); // #666666
    pub const SYNTAX_FUNCTION: Color = Color::from_argb_u8(255, 0xE0, 0xDE, 0x71); // #e0de71
    pub const SYNTAX_KEYWORD: Color = Color::from_argb_u8(255, 0xFA, 0x99, 0xCD); // #fa99cd
    pub const SYNTAX_STRING: Color = Color::from_argb_u8(255, 0x44, 0xCF, 0x6E); // #44cf6e
    pub const SYNTAX_TAG: Color = Color::from_argb_u8(255, 0xFB, 0x46, 0x4C); // #fb464c
    pub const SYNTAX_OPERATOR: Color = Color::from_argb_u8(255, 0xFB, 0x46, 0x4C); // #fb464c
    pub const SYNTAX_PROPERTY: Color = Color::from_argb_u8(255, 0x53, 0xDF, 0xDD); // #53dfdd
    pub const SYNTAX_PUNCTUATION: Color = Color::from_argb_u8(255, 0x99, 0x99, 0x99); // #999999
    pub const SYNTAX_VARIABLE: Color = Color::from_argb_u8(255, 0xE9, 0x97, 0x3F); // #e9973f
    pub const SYNTAX_VALUE: Color = Color::from_argb_u8(255, 0xA8, 0x82, 0xFF); // #a882ff

    // Callout colors
    pub const CALLOUT_BUG: Color = Color::from_argb_u8(255, 0xFB, 0x46, 0x4C); // #fb464c
    pub const CALLOUT_DEFAULT: Color = Color::from_argb_u8(255, 0x02, 0x7A, 0xFF); // #027aff
    pub const CALLOUT_ERROR: Color = Color::from_argb_u8(255, 0xFB, 0x46, 0x4C); // #fb464c
    pub const CALLOUT_EXAMPLE: Color = Color::from_argb_u8(255, 0xA8, 0x82, 0xFF); // #a882ff
    pub const CALLOUT_FAIL: Color = Color::from_argb_u8(255, 0xFB, 0x46, 0x4C); // #fb464c
    pub const CALLOUT_IMPORTANT: Color = Color::from_argb_u8(255, 0x53, 0xDF, 0xDD); // #53dfdd
    pub const CALLOUT_INFO: Color = Color::from_argb_u8(255, 0x02, 0x7A, 0xFF); // #027aff
    pub const CALLOUT_QUESTION: Color = Color::from_argb_u8(255, 0xE9, 0x97, 0x3F); // #e9973f
    pub const CALLOUT_SUCCESS: Color = Color::from_argb_u8(255, 0x44, 0xCF, 0x6E); // #44cf6e
    pub const CALLOUT_SUMMARY: Color = Color::from_argb_u8(255, 0x53, 0xDF, 0xDD); // #53dfdd
    pub const CALLOUT_TIP: Color = Color::from_argb_u8(255, 0x53, 0xDF, 0xDD); // #53dfdd
    pub const CALLOUT_TODO: Color = Color::from_argb_u8(255, 0x02, 0x7A, 0xFF); // #027aff
    pub const CALLOUT_WARNING: Color = Color::from_argb_u8(255, 0xE9, 0x97, 0x3F); // #e9973f
    pub const CALLOUT_QUOTE: Color = Color::from_argb_u8(255, 0x9E, 0x9E, 0x9E); // #9e9e9e

    // Status colors
    pub const SUCCESS: Color = Color::from_argb_u8(255, 0x44, 0xCF, 0x6E); // #44cf6e
    pub const WARNING: Color = Color::from_argb_u8(255, 0xE9, 0x97, 0x3F); // #e9973f
    pub const ERROR: Color = Color::from_argb_u8(255, 0xFB, 0x46, 0x4C); // #fb464c
    pub const INFO: Color = Color::from_argb_u8(255, 0x02, 0x7A, 0xFF); // #027aff

    // Border colors
    pub const BORDER_PRIMARY: Color = Color::from_argb_u8(255, 0x3F, 0x3F, 0x3F); // #3f3f3f
    pub const BORDER_SECONDARY: Color = Color::from_argb_u8(255, 0x55, 0x55, 0x55); // #555555

    // Hover colors
    pub const HOVER_BACKGROUND: Color = Color::from_argb_u8(255, 0x2E, 0x2E, 0x2E); // #2e2e2e
    pub const HOVER_TEXT: Color = ACCENT_HOVER;
}

/// Light theme color palette
pub mod light_colors {
    use super::Color;

    pub const BACKGROUND_PRIMARY: Color = Color::from_argb_u8(255, 0xFF, 0xFF, 0xFF);
    pub const BACKGROUND_SECONDARY: Color = Color::from_argb_u8(255, 0xF8, 0xF8, 0xF8);
    pub const BACKGROUND_TERTIARY: Color = Color::from_argb_u8(255, 0xF0, 0xF0, 0xF0);
    pub const TEXT_PRIMARY: Color = Color::from_argb_u8(255, 0x33, 0x33, 0x33);
    pub const TEXT_SECONDARY: Color = Color::from_argb_u8(255, 0x66, 0x66, 0x66);
    pub const TEXT_MUTED: Color = Color::from_argb_u8(255, 0x99, 0x99, 0x99);
    pub const BORDER_PRIMARY: Color = Color::from_argb_u8(255, 0xE0, 0xE0, 0xE0);
}

// ============================================================================
// Typography
// ============================================================================

/// Typography constants
pub mod typography {
    /// Font sizes (in pixels)
    pub const FONT_SIZE_XS: u16 = 10;
    pub const FONT_SIZE_SM: u16 = 12;
    pub const FONT_SIZE_BASE: u16 = 14;
    pub const FONT_SIZE_LG: u16 = 16;
    pub const FONT_SIZE_XL: u16 = 18;
    pub const FONT_SIZE_2XL: u16 = 20;
    pub const FONT_SIZE_3XL: u16 = 24;
    pub const FONT_SIZE_4XL: u16 = 28;
    pub const FONT_SIZE_5XL: u16 = 32;
    pub const FONT_SIZE_6XL: u16 = 36;

    /// Font weights
    pub const FONT_WEIGHT_LIGHT: u16 = 300;
    pub const FONT_WEIGHT_NORMAL: u16 = 400;
    pub const FONT_WEIGHT_MEDIUM: u16 = 500;
    pub const FONT_WEIGHT_SEMIBOLD: u16 = 600;
    pub const FONT_WEIGHT_BOLD: u16 = 700;

    /// Line heights
    pub const LINE_HEIGHT_TIGHT: u16 = 16;
    pub const LINE_HEIGHT_NORMAL: u16 = 20;
    pub const LINE_HEIGHT_RELAXED: u16 = 24;
}

// ============================================================================
// Spacing
// ============================================================================

/// Spacing system based on 4px grid
pub mod spacing {
    pub const SPACING_0: u16 = 0;
    pub const SPACING_1: u16 = 4;
    pub const SPACING_2: u16 = 8;
    pub const SPACING_3: u16 = 12;
    pub const SPACING_4: u16 = 16;
    pub const SPACING_5: u16 = 20;
    pub const SPACING_6: u16 = 24;
    pub const SPACING_8: u16 = 32;
    pub const SPACING_10: u16 = 40;
    pub const SPACING_12: u16 = 48;
    pub const SPACING_16: u16 = 64;
}

// ============================================================================
// Border Radius
// ============================================================================

/// Border radius constants
pub mod border_radius {
    pub const RADIUS_NONE: u16 = 0;
    pub const RADIUS_SM: u16 = 2;
    pub const RADIUS_BASE: u16 = 4;
    pub const RADIUS_LG: u16 = 6;
    pub const RADIUS_XL: u16 = 8;
    pub const RADIUS_FULL: u16 = 9999;
}

// ============================================================================
// Z-Index Layers
// ============================================================================

/// Z-index layers for UI elements
pub mod z_index {
    pub const Z_BASE: u16 = 0;
    pub const Z_DROPDOWN: u16 = 100;
    pub const Z_STICKY: u16 = 200;
    pub const Z_FIXED: u16 = 300;
    pub const Z_MODAL_BACKDROP: u16 = 400;
    pub const Z_MODAL: u16 = 500;
    pub const Z_TOOLTIP: u16 = 600;
    pub const Z_NOTIFICATION: u16 = 700;
    pub const Z_MAX: u16 = 9999;
}

// ============================================================================
// Theme Colors
// ============================================================================

/// Theme colors that can be converted to Slint colors
#[derive(Debug, Clone)]
pub struct DesignSystemThemeColors {
    /// Background colors
    pub background_primary: Color,
    pub background_secondary: Color,
    pub background_tertiary: Color,

    /// Text colors
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_muted: Color,

    /// Accent colors
    pub accent_primary: Color,
    pub accent_hover: Color,

    /// Border colors
    pub border_primary: Color,
    pub border_secondary: Color,

    /// Status colors
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
}

impl Default for DesignSystemThemeColors {
    fn default() -> Self {
        Self {
            background_primary: colors::BACKGROUND_PRIMARY,
            background_secondary: colors::BACKGROUND_SECONDARY,
            background_tertiary: colors::BACKGROUND_TERTIARY,
            text_primary: colors::TEXT_PRIMARY,
            text_secondary: colors::TEXT_SECONDARY,
            text_muted: colors::TEXT_MUTED,
            accent_primary: colors::ACCENT_PRIMARY,
            accent_hover: colors::ACCENT_HOVER,
            border_primary: colors::BORDER_PRIMARY,
            border_secondary: colors::BORDER_SECONDARY,
            success: colors::SUCCESS,
            warning: colors::WARNING,
            error: colors::ERROR,
            info: colors::INFO,
        }
    }
}

impl DesignSystemThemeColors {
    /// Creates a new theme with dark colors
    pub fn dark() -> Self {
        Self::default()
    }

    /// Creates a new theme with light colors
    pub fn light() -> Self {
        Self {
            background_primary: light_colors::BACKGROUND_PRIMARY,
            background_secondary: light_colors::BACKGROUND_SECONDARY,
            background_tertiary: light_colors::BACKGROUND_TERTIARY,
            text_primary: light_colors::TEXT_PRIMARY,
            text_secondary: light_colors::TEXT_SECONDARY,
            text_muted: light_colors::TEXT_MUTED,
            accent_primary: colors::ACCENT_PRIMARY,
            accent_hover: colors::ACCENT_HOVER,
            border_primary: light_colors::BORDER_PRIMARY,
            border_secondary: light_colors::BORDER_PRIMARY,
            success: colors::SUCCESS,
            warning: colors::WARNING,
            error: colors::ERROR,
            info: colors::INFO,
        }
    }

    /// Gets colors for a specific theme variant
    pub fn for_variant(variant: crate::theme::ThemeVariant) -> Self {
        match variant {
            crate::theme::ThemeVariant::Dark => Self::dark(),
            crate::theme::ThemeVariant::Light => Self::light(),
            crate::theme::ThemeVariant::System => {
                // Default to dark for system (will be overridden by actual system detection)
                Self::dark()
            }
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colors_defined() {
        assert_eq!(
            colors::BACKGROUND_PRIMARY,
            Color::from_argb_u8(255, 0x1C, 0x1C, 0x1C)
        );
        assert_eq!(
            colors::TEXT_PRIMARY,
            Color::from_argb_u8(255, 0xDA, 0xDA, 0xDA)
        );
        assert_eq!(
            colors::ACCENT_PRIMARY,
            Color::from_argb_u8(255, 0x02, 0x7A, 0xFF)
        );
    }

    #[test]
    fn test_light_colors_defined() {
        assert_eq!(
            light_colors::BACKGROUND_PRIMARY,
            Color::from_argb_u8(255, 0xFF, 0xFF, 0xFF)
        );
        assert_eq!(
            light_colors::TEXT_PRIMARY,
            Color::from_argb_u8(255, 0x33, 0x33, 0x33)
        );
    }

    #[test]
    fn test_typography_constants() {
        assert_eq!(typography::FONT_SIZE_BASE, 14);
        assert_eq!(typography::FONT_SIZE_LG, 16);
        assert_eq!(typography::FONT_WEIGHT_NORMAL, 400);
        assert_eq!(typography::FONT_WEIGHT_BOLD, 700);
    }

    #[test]
    fn test_spacing_constants() {
        assert_eq!(spacing::SPACING_4, 16);
        assert_eq!(spacing::SPACING_8, 32);
    }

    #[test]
    fn test_border_radius_constants() {
        assert_eq!(border_radius::RADIUS_BASE, 4);
        assert_eq!(border_radius::RADIUS_LG, 6);
    }

    #[test]
    fn test_design_system_theme_colors_dark() {
        let colors = DesignSystemThemeColors::dark();
        assert_eq!(colors.background_primary, colors::BACKGROUND_PRIMARY);
        assert_eq!(colors.text_primary, colors::TEXT_PRIMARY);
    }

    #[test]
    fn test_design_system_theme_colors_light() {
        let colors = DesignSystemThemeColors::light();
        assert_eq!(colors.background_primary, light_colors::BACKGROUND_PRIMARY);
        assert_eq!(colors.text_primary, light_colors::TEXT_PRIMARY);
    }
}
