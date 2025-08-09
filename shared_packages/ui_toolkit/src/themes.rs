//! Theme system for the UI toolkit
//!
//! This module provides a comprehensive theme system that supports light/dark mode,
//! color palettes, typography, and spacing scales.

use stylist::Style;
use yew::Classes;
use wasm_bindgen::JsValue;
use web_sys::window;
use serde::{Deserialize, Serialize};

/// Color scheme
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ColorScheme {
    /// Light color scheme
    Light,
    
    /// Dark color scheme
    Dark,
    
    /// System preference (auto)
    System,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self::System
    }
}

/// Design system configuration
#[derive(Debug, Clone)]
pub struct DesignSystem {
    /// Active color scheme (light or dark)
    pub color_scheme: ColorScheme,
    
    /// Color palette for light mode
    pub light_colors: ColorPalette,
    
    /// Color palette for dark mode
    pub dark_colors: ColorPalette,
    
    /// Currently active color palette (based on color_scheme)
    pub colors: ColorPalette,
    
    /// Spacing scale
    pub spacing: SpacingScale,
    
    /// Typography settings
    pub typography: Typography,
    
    /// Border radius values
    pub border_radius: BorderRadius,
    
    /// Shadow values
    pub shadows: Shadows,
    
    /// Breakpoint definitions
    pub breakpoints: Breakpoints,
}

/// Color palette definition
#[derive(Debug, Clone)]
pub struct ColorPalette {
    /// Primary color
    pub primary: String,
    
    /// Secondary color
    pub secondary: String,
    
    /// Success color
    pub success: String,
    
    /// Warning color
    pub warning: String,
    
    /// Danger color
    pub danger: String,
    
    /// Info color
    pub info: String,
    
    /// Light color
    pub light: String,
    
    /// Dark color
    pub dark: String,
    
    /// White color
    pub white: String,
    
    /// Black color
    pub black: String,
    
    /// Gray scale
    pub gray: GrayScale,
    
    /// Text color
    pub text: String,
    
    /// Background color
    pub background: String,
    
    /// Surface color (for cards, etc.)
    pub surface: String,
    
    /// Border color
    pub border: String,
}

/// Gray scale colors
#[derive(Debug, Clone)]
pub struct GrayScale {
    /// Gray 100
    pub _100: String,
    
    /// Gray 200
    pub _200: String,
    
    /// Gray 300
    pub _300: String,
    
    /// Gray 400
    pub _400: String,
    
    /// Gray 500
    pub _500: String,
    
    /// Gray 600
    pub _600: String,
    
    /// Gray 700
    pub _700: String,
    
    /// Gray 800
    pub _800: String,
    
    /// Gray 900
    pub _900: String,
}

/// Spacing scale definition
#[derive(Debug, Clone)]
pub struct SpacingScale {
    /// Extra small spacing (4px)
    pub xs: String,
    
    /// Small spacing (8px)
    pub sm: String,
    
    /// Medium spacing (16px)
    pub md: String,
    
    /// Large spacing (24px)
    pub lg: String,
    
    /// Extra large spacing (32px)
    pub xl: String,
    
    /// Extra extra large spacing (48px)
    pub xxl: String,
}

/// Typography settings
#[derive(Debug, Clone)]
pub struct Typography {
    /// Font family
    pub font_family: String,
    
    /// Font sizes
    pub font_sizes: FontSizes,
    
    /// Font weights
    pub font_weights: FontWeights,
    
    /// Line heights
    pub line_heights: LineHeights,
}

/// Font sizes
#[derive(Debug, Clone)]
pub struct FontSizes {
    /// Extra small (12px)
    pub xs: String,
    
    /// Small (14px)
    pub sm: String,
    
    /// Medium (16px)
    pub md: String,
    
    /// Large (18px)
    pub lg: String,
    
    /// Extra large (20px)
    pub xl: String,
    
    /// Extra extra large (24px)
    pub xxl: String,
    
    /// Extra extra extra large (32px)
    pub xxxl: String,
}

/// Font weights
#[derive(Debug, Clone)]
pub struct FontWeights {
    /// Thin (100)
    pub thin: u16,
    
    /// Extra light (200)
    pub extra_light: u16,
    
    /// Light (300)
    pub light: u16,
    
    /// Regular (400)
    pub regular: u16,
    
    /// Medium (500)
    pub medium: u16,
    
    /// Semi bold (600)
    pub semi_bold: u16,
    
    /// Bold (700)
    pub bold: u16,
    
    /// Extra bold (800)
    pub extra_bold: u16,
    
    /// Black (900)
    pub black: u16,
}

/// Line heights
#[derive(Debug, Clone)]
pub struct LineHeights {
    /// Tight (1.25)
    pub tight: f32,
    
    /// Snug (1.375)
    pub snug: f32,
    
    /// Normal (1.5)
    pub normal: f32,
    
    /// Relaxed (1.625)
    pub relaxed: f32,
    
    /// Loose (2)
    pub loose: f32,
}

/// Border radius values
#[derive(Debug, Clone)]
pub struct BorderRadius {
    /// Small (4px)
    pub sm: String,
    
    /// Medium (8px)
    pub md: String,
    
    /// Large (12px)
    pub lg: String,
    
    /// Extra large (16px)
    pub xl: String,
    
    /// Full (9999px)
    pub full: String,
}

/// Shadow values
#[derive(Debug, Clone)]
pub struct Shadows {
    /// Small shadow
    pub sm: String,
    
    /// Medium shadow
    pub md: String,
    
    /// Large shadow
    pub lg: String,
    
    /// Extra large shadow
    pub xl: String,
}

/// Breakpoint definitions
#[derive(Debug, Clone)]
pub struct Breakpoints {
    /// Small devices (landscape phones, 576px and up)
    pub sm: String, // 576px
    
    /// Medium devices (tablets, 768px and up)
    pub md: String, // 768px
    
    /// Large devices (desktops, 992px and up)
    pub lg: String, // 992px
    
    /// Extra large devices (large desktops, 1200px and up)
    pub xl: String, // 1200px
    
    /// Extra extra large devices (larger desktops, 1400px and up)
    pub xxl: String, // 1400px
}

impl DesignSystem {
    /// Create a new design system with default values
    pub fn new() -> Self {
        let light_colors = ColorPalette {
            primary: "#4361ee".to_string(),
            secondary: "#3f37c9".to_string(),
            success: "#4caf50".to_string(),
            warning: "#ff9800".to_string(),
            danger: "#f44336".to_string(),
            info: "#2196f3".to_string(),
            light: "#f8f9fa".to_string(),
            dark: "#2b2d42".to_string(),
            white: "#ffffff".to_string(),
            black: "#000000".to_string(),
            gray: GrayScale {
                _100: "#f8f9fa".to_string(),
                _200: "#e9ecef".to_string(),
                _300: "#dee2e6".to_string(),
                _400: "#ced4da".to_string(),
                _500: "#adb5bd".to_string(),
                _600: "#6c757d".to_string(),
                _700: "#495057".to_string(),
                _800: "#343a40".to_string(),
                _900: "#212529".to_string(),
            },
            text: "#2b2d42".to_string(),
            background: "#ffffff".to_string(),
            surface: "#ffffff".to_string(),
            border: "#dee2e6".to_string(),
        };
        
        let dark_colors = ColorPalette {
            primary: "#4cc9f0".to_string(),
            secondary: "#4895ef".to_string(),
            success: "#66bb6a".to_string(),
            warning: "#ffa726".to_string(),
            danger: "#ef5350".to_string(),
            info: "#42a5f5".to_string(),
            light: "#2b2d42".to_string(),
            dark: "#f8f9fa".to_string(),
            white: "#000000".to_string(),
            black: "#ffffff".to_string(),
            gray: GrayScale {
                _100: "#212529".to_string(),
                _200: "#343a40".to_string(),
                _300: "#495057".to_string(),
                _400: "#6c757d".to_string(),
                _500: "#adb5bd".to_string(),
                _600: "#ced4da".to_string(),
                _700: "#dee2e6".to_string(),
                _800: "#e9ecef".to_string(),
                _900: "#f8f9fa".to_string(),
            },
            text: "#f8f9fa".to_string(),
            background: "#121212".to_string(),
            surface: "#1e1e1e".to_string(),
            border: "#495057".to_string(),
        };
        
        Self {
            color_scheme: ColorScheme::System,
            light_colors: light_colors.clone(),
            dark_colors: dark_colors.clone(),
            colors: light_colors,
            spacing: SpacingScale {
                xs: "0.25rem".to_string(), // 4px
                sm: "0.5rem".to_string(),  // 8px
                md: "1rem".to_string(),    // 16px
                lg: "1.5rem".to_string(),  // 24px
                xl: "2rem".to_string(),    // 32px
                xxl: "3rem".to_string(),   // 48px
            },
            typography: Typography {
                font_family: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif".to_string(),
                font_sizes: FontSizes {
                    xs: "0.75rem".to_string(),  // 12px
                    sm: "0.875rem".to_string(), // 14px
                    md: "1rem".to_string(),     // 16px
                    lg: "1.125rem".to_string(), // 18px
                    xl: "1.25rem".to_string(),  // 20px
                    xxl: "1.5rem".to_string(),  // 24px
                    xxxl: "2rem".to_string(),   // 32px
                },
                font_weights: FontWeights {
                    thin: 100,
                    extra_light: 200,
                    light: 300,
                    regular: 400,
                    medium: 500,
                    semi_bold: 600,
                    bold: 700,
                    extra_bold: 800,
                    black: 900,
                },
                line_heights: LineHeights {
                    tight: 1.25,
                    snug: 1.375,
                    normal: 1.5,
                    relaxed: 1.625,
                    loose: 2.0,
                },
            },
            border_radius: BorderRadius {
                sm: "0.25rem".to_string(), // 4px
                md: "0.5rem".to_string(),  // 8px
                lg: "0.75rem".to_string(), // 12px
                xl: "1rem".to_string(),    // 16px
                full: "9999px".to_string(),
            },
            shadows: Shadows {
                sm: "0 0.125rem 0.25rem rgba(0, 0, 0, 0.075)".to_string(),
                md: "0 0.5rem 1rem rgba(0, 0, 0, 0.15)".to_string(),
                lg: "0 1rem 3rem rgba(0, 0, 0, 0.175)".to_string(),
                xl: "0 2rem 4rem rgba(0, 0, 0, 0.2)".to_string(),
            },
            breakpoints: Breakpoints {
                sm: "576px".to_string(),
                md: "768px".to_string(),
                lg: "992px".to_string(),
                xl: "1200px".to_string(),
                xxl: "1400px".to_string(),
            },
        }
    }
    
    /// Get the default design system
    pub fn default() -> Self {
        Self::new()
    }
    
    /// Set the color scheme
    pub fn set_color_scheme(&mut self, scheme: ColorScheme) {
        self.color_scheme = scheme.clone();
        self.colors = match scheme {
            ColorScheme::Light => self.light_colors.clone(),
            ColorScheme::Dark => self.dark_colors.clone(),
            ColorScheme::System => {
                // Check system preference
                if let Some(window) = window() {
                    if let Ok(media_query) = window.match_media("(prefers-color-scheme: dark)") {
                        if let Some(mq) = media_query {
                            if mq.matches() {
                                return self.dark_colors.clone();
                            }
                        }
                    }
                }
                self.light_colors.clone()
            }
        };
    }
    
    /// Get the effective color scheme (resolves System to actual scheme)
    pub fn get_effective_color_scheme(&self) -> ColorScheme {
        match self.color_scheme {
            ColorScheme::Light => ColorScheme::Light,
            ColorScheme::Dark => ColorScheme::Dark,
            ColorScheme::System => {
                // Check system preference
                if let Some(window) = window() {
                    if let Ok(media_query) = window.match_media("(prefers-color-scheme: dark)") {
                        if let Some(mq) = media_query {
                            if mq.matches() {
                                return ColorScheme::Dark;
                            }
                        }
                    }
                }
                ColorScheme::Light
            }
        }
    }
    
    /// Get CSS variables for the current theme
    pub fn get_css_variables(&self) -> String {
        let effective_scheme = self.get_effective_color_scheme();
        let colors = match effective_scheme {
            ColorScheme::Light => &self.light_colors,
            ColorScheme::Dark => &self.dark_colors,
            ColorScheme::System => &self.colors, // This should never happen due to get_effective_color_scheme
        };
        
        format!(
            ":root {{
\t--cpc-primary: {};
\t--cpc-secondary: {};
\t--cpc-success: {};
\t--cpc-warning: {};
\t--cpc-danger: {};
\t--cpc-info: {};
\t--cpc-light: {};
\t--cpc-dark: {};
\t--cpc-white: {};
\t--cpc-black: {};
\t--cpc-gray-100: {};
\t--cpc-gray-200: {};
\t--cpc-gray-300: {};
\t--cpc-gray-400: {};
\t--cpc-gray-500: {};
\t--cpc-gray-600: {};
\t--cpc-gray-700: {};
\t--cpc-gray-800: {};
\t--cpc-gray-900: {};
\t--cpc-text: {};
\t--cpc-background: {};
\t--cpc-surface: {};
\t--cpc-border: {};
\t--cpc-border-radius-sm: {};
\t--cpc-border-radius-md: {};
\t--cpc-border-radius-lg: {};
\t--cpc-border-radius-xl: {};
\t--cpc-border-radius-full: {};
\t--cpc-spacing-xs: {};
\t--cpc-spacing-sm: {};
\t--cpc-spacing-md: {};
\t--cpc-spacing-lg: {};
\t--cpc-spacing-xl: {};
\t--cpc-spacing-xxl: {};
\t--cpc-font-family: {};
\t--cpc-font-size-xs: {};
\t--cpc-font-size-sm: {};
\t--cpc-font-size-md: {};
\t--cpc-font-size-lg: {};
\t--cpc-font-size-xl: {};
\t--cpc-font-size-xxl: {};
\t--cpc-font-size-xxxl: {};
\t--cpc-shadow-sm: {};
\t--cpc-shadow-md: {};
\t--cpc-shadow-lg: {};
\t--cpc-shadow-xl: {};
\t--cpc-breakpoint-sm: {};
\t--cpc-breakpoint-md: {};
\t--cpc-breakpoint-lg: {};
\t--cpc-breakpoint-xl: {};
\t--cpc-breakpoint-xxl: {};
}}

[data-theme=\"dark\"] {{
\t--cpc-primary: {};
\t--cpc-secondary: {};
\t--cpc-success: {};
\t--cpc-warning: {};
\t--cpc-danger: {};
\t--cpc-info: {};
\t--cpc-light: {};
\t--cpc-dark: {};
\t--cpc-white: {};
\t--cpc-black: {};
\t--cpc-gray-100: {};
\t--cpc-gray-200: {};
\t--cpc-gray-300: {};
\t--cpc-gray-400: {};
\t--cpc-gray-500: {};
\t--cpc-gray-600: {};
\t--cpc-gray-700: {};
\t--cpc-gray-800: {};
\t--cpc-gray-900: {};
\t--cpc-text: {};
\t--cpc-background: {};
\t--cpc-surface: {};
\t--cpc-border: {};
}}",
            colors.primary,
            colors.secondary,
            colors.success,
            colors.warning,
            colors.danger,
            colors.info,
            colors.light,
            colors.dark,
            colors.white,
            colors.black,
            colors.gray._100,
            colors.gray._200,
            colors.gray._300,
            colors.gray._400,
            colors.gray._500,
            colors.gray._600,
            colors.gray._700,
            colors.gray._800,
            colors.gray._900,
            colors.text,
            colors.background,
            colors.surface,
            colors.border,
            self.border_radius.sm,
            self.border_radius.md,
            self.border_radius.lg,
            self.border_radius.xl,
            self.border_radius.full,
            self.spacing.xs,
            self.spacing.sm,
            self.spacing.md,
            self.spacing.lg,
            self.spacing.xl,
            self.spacing.xxl,
            self.typography.font_family,
            self.typography.font_sizes.xs,
            self.typography.font_sizes.sm,
            self.typography.font_sizes.md,
            self.typography.font_sizes.lg,
            self.typography.font_sizes.xl,
            self.typography.font_sizes.xxl,
            self.typography.font_sizes.xxxl,
            self.shadows.sm,
            self.shadows.md,
            self.shadows.lg,
            self.shadows.xl,
            self.breakpoints.sm,
            self.breakpoints.md,
            self.breakpoints.lg,
            self.breakpoints.xl,
            self.breakpoints.xxl,
            self.dark_colors.primary,
            self.dark_colors.secondary,
            self.dark_colors.success,
            self.dark_colors.warning,
            self.dark_colors.danger,
            self.dark_colors.info,
            self.dark_colors.light,
            self.dark_colors.dark,
            self.dark_colors.white,
            self.dark_colors.black,
            self.dark_colors.gray._100,
            self.dark_colors.gray._200,
            self.dark_colors.gray._300,
            self.dark_colors.gray._400,
            self.dark_colors.gray._500,
            self.dark_colors.gray._600,
            self.dark_colors.gray._700,
            self.dark_colors.gray._800,
            self.dark_colors.gray._900,
            self.dark_colors.text,
            self.dark_colors.background,
            self.dark_colors.surface,
            self.dark_colors.border,
        )
    }
    
    /// Get CSS media queries for breakpoints
    pub fn get_media_queries(&self) -> String {
        format!(
            "@media (min-width: {}) {{
\t/* Small devices */
}}

@media (min-width: {}) {{
\t/* Medium devices */
}}

@media (min-width: {}) {{
\t/* Large devices */
}}

@media (min-width: {}) {{
\t/* Extra large devices */
}}

@media (min-width: {}) {{
\t/* Extra extra large devices */
}}",
            self.breakpoints.sm,
            self.breakpoints.md,
            self.breakpoints.lg,
            self.breakpoints.xl,
            self.breakpoints.xxl,
        )
    }
    
    /// Create CSS classes from the design system
    pub fn create_classes(&self, styles: &str) -> Result<Classes, stylist::StyleError> {
        Style::new(styles).map(|s| Classes::from(s.get_class_name()))
    }
}

impl Default for DesignSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Theme provider component
#[derive(Debug, Clone)]
pub struct ThemeProvider {
    /// The current design system
    pub design_system: DesignSystem,
}

impl ThemeProvider {
    /// Create a new theme provider
    pub fn new() -> Self {
        Self {
            design_system: DesignSystem::new(),
        }
    }
    
    /// Create a new theme provider with a specific color scheme
    pub fn with_scheme(scheme: ColorScheme) -> Self {
        let mut provider = Self::new();
        provider.set_color_scheme(scheme);
        provider
    }
    
    /// Set the color scheme
    pub fn set_color_scheme(&mut self, scheme: ColorScheme) {
        self.design_system.set_color_scheme(scheme);
        self.save_theme_preference(&scheme);
    }
    
    /// Toggle between light and dark mode
    pub fn toggle_theme(&mut self) {
        let current_scheme = self.design_system.get_effective_color_scheme();
        let new_scheme = match current_scheme {
            ColorScheme::Light => ColorScheme::Dark,
            ColorScheme::Dark => ColorScheme::Light,
            ColorScheme::System => {
                // If system preference is dark, toggle to light and vice versa
                if let Some(window) = window() {
                    if let Ok(media_query) = window.match_media("(prefers-color-scheme: dark)") {
                        if let Some(mq) = media_query {
                            if mq.matches() {
                                ColorScheme::Light
                            } else {
                                ColorScheme::Dark
                            }
                        } else {
                            ColorScheme::Dark
                        }
                    } else {
                        ColorScheme::Dark
                    }
                } else {
                    ColorScheme::Dark
                }
            }
        };
        self.set_color_scheme(new_scheme);
    }
    
    /// Load theme preference from local storage
    pub fn load_theme_preference(&mut self) {
        if let Some(window) = window() {
            if let Ok(storage) = window.local_storage() {
                if let Some(storage) = storage {
                    if let Ok(Some(theme)) = storage.get_item("cpc-theme-preference") {
                        match theme.as_str() {
                            "light" => self.set_color_scheme(ColorScheme::Light),
                            "dark" => self.set_color_scheme(ColorScheme::Dark),
                            "system" => self.set_color_scheme(ColorScheme::System),
                            _ => self.set_color_scheme(ColorScheme::System),
                        }
                    }
                }
            }
        }
    }
    
    /// Save theme preference to local storage
    fn save_theme_preference(&self, scheme: &ColorScheme) {
        if let Some(window) = window() {
            if let Ok(storage) = window.local_storage() {
                if let Some(storage) = storage {
                    let theme_str = match scheme {
                        ColorScheme::Light => "light",
                        ColorScheme::Dark => "dark",
                        ColorScheme::System => "system",
                    };
                    let _ = storage.set_item("cpc-theme-preference", theme_str);
                }
            }
        }
    }
    
    /// Get the current theme CSS
    pub fn get_theme_css(&self) -> String {
        format!("{}\n\n{}",
            self.design_system.get_css_variables(),
            self.design_system.get_media_queries())
    }
}

impl Default for ThemeProvider {
    fn default() -> Self {
        let mut provider = Self::new();
        provider.load_theme_preference();
        provider
    }
}