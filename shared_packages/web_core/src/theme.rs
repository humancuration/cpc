//! Design system for CPC web applications
//!
//! This module provides a design system implementation that defines
//! the visual language, spacing, colors, and typography for CPC web applications.

use stylist::Style;
use yew::Classes;

/// Color scheme
#[derive(Debug, Clone, PartialEq)]
pub enum ColorScheme {
    /// Light color scheme
    Light,
    
    /// Dark color scheme
    Dark,
    
    /// Follow system preference
    System,
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
}

/// Gray scale colors
#[derive(Debug, Clone)]
pub struct GrayScale {
    /// Gray 100
    pub 100: String,
    
    /// Gray 200
    pub 200: String,
    
    /// Gray 300
    pub 300: String,
    
    /// Gray 400
    pub 400: String,
    
    /// Gray 500
    pub 500: String,
    
    /// Gray 600
    pub 600: String,
    
    /// Gray 700
    pub 700: String,
    
    /// Gray 800
    pub 800: String,
    
    /// Gray 900
    pub 900: String,
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
            primary: "#007bff".to_string(),
            secondary: "#6c757d".to_string(),
            success: "#28a745".to_string(),
            warning: "#ffc107".to_string(),
            danger: "#dc3545".to_string(),
            info: "#17a2b8".to_string(),
            light: "#f8f9fa".to_string(),
            dark: "#343a40".to_string(),
            white: "#ffffff".to_string(),
            black: "#000000".to_string(),
            gray: GrayScale {
                100: "#f8f9fa".to_string(),
                200: "#e9ecef".to_string(),
                300: "#dee2e6".to_string(),
                400: "#ced4da".to_string(),
                500: "#adb5bd".to_string(),
                600: "#6c757d".to_string(),
                700: "#495057".to_string(),
                800: "#343a40".to_string(),
                900: "#212529".to_string(),
            },
        };
        
        let dark_colors = ColorPalette {
            primary: "#0d6efd".to_string(),
            secondary: "#6c757d".to_string(),
            success: "#198754".to_string(),
            warning: "#ffc107".to_string(),
            danger: "#dc3545".to_string(),
            info: "#0dcaf0".to_string(),
            light: "#212529".to_string(),
            dark: "#f8f9fa".to_string(),
            white: "#000000".to_string(),
            black: "#ffffff".to_string(),
            gray: GrayScale {
                100: "#212529".to_string(),
                200: "#343a40".to_string(),
                300: "#495057".to_string(),
                400: "#6c757d".to_string(),
                500: "#adb5bd".to_string(),
                600: "#ced4da".to_string(),
                700: "#dee2e6".to_string(),
                800: "#e9ecef".to_string(),
                900: "#f8f9fa".to_string(),
            },
        };
        
        Self {
            color_scheme: ColorScheme::Light,
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
                font_family: "system-ui, -apple-system, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif".to_string(),
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
                // In a real implementation, we would detect the system preference
                // For now, we'll default to light mode
                self.light_colors.clone()
            }
        };
    }
    
    /// Get CSS variables for the current theme
    pub fn get_css_variables(&self) -> String {
        format!(
            ":root {{\n\
             \t--cpc-primary: {};\n\
             \t--cpc-secondary: {};\n\
             \t--cpc-success: {};\n\
             \t--cpc-warning: {};\n\
             \t--cpc-danger: {};\n\
             \t--cpc-info: {};\n\
             \t--cpc-light: {};\n\
             \t--cpc-dark: {};\n\
             \t--cpc-white: {};\n\
             \t--cpc-black: {};\n\
             \t--cpc-gray-100: {};\n\
             \t--cpc-gray-200: {};\n\
             \t--cpc-gray-300: {};\n\
             \t--cpc-gray-400: {};\n\
             \t--cpc-gray-500: {};\n\
             \t--cpc-gray-600: {};\n\
             \t--cpc-gray-700: {};\n\
             \t--cpc-gray-800: {};\n\
             \t--cpc-gray-900: {};\n\
             \t--cpc-breakpoint-sm: {};\n\
             \t--cpc-breakpoint-md: {};\n\
             \t--cpc-breakpoint-lg: {};\n\
             \t--cpc-breakpoint-xl: {};\n\
             \t--cpc-breakpoint-xxl: {};\n\
             }}\n\
             \n\
             [data-theme=\"dark\"] {{\n\
             \t--cpc-primary: {};\n\
             \t--cpc-secondary: {};\n\
             \t--cpc-success: {};\n\
             \t--cpc-warning: {};\n\
             \t--cpc-danger: {};\n\
             \t--cpc-info: {};\n\
             \t--cpc-light: {};\n\
             \t--cpc-dark: {};\n\
             \t--cpc-white: {};\n\
             \t--cpc-black: {};\n\
             \t--cpc-gray-100: {};\n\
             \t--cpc-gray-200: {};\n\
             \t--cpc-gray-300: {};\n\
             \t--cpc-gray-400: {};\n\
             \t--cpc-gray-500: {};\n\
             \t--cpc-gray-600: {};\n\
             \t--cpc-gray-700: {};\n\
             \t--cpc-gray-800: {};\n\
             \t--cpc-gray-900: {};\n\
             }}",
            self.light_colors.primary,
            self.light_colors.secondary,
            self.light_colors.success,
            self.light_colors.warning,
            self.light_colors.danger,
            self.light_colors.info,
            self.light_colors.light,
            self.light_colors.dark,
            self.light_colors.white,
            self.light_colors.black,
            self.light_colors.gray.100,
            self.light_colors.gray.200,
            self.light_colors.gray.300,
            self.light_colors.gray.400,
            self.light_colors.gray.500,
            self.light_colors.gray.600,
            self.light_colors.gray.700,
            self.light_colors.gray.800,
            self.light_colors.gray.900,
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
            self.dark_colors.gray.100,
            self.dark_colors.gray.200,
            self.dark_colors.gray.300,
            self.dark_colors.gray.400,
            self.dark_colors.gray.500,
            self.dark_colors.gray.600,
            self.dark_colors.gray.700,
            self.dark_colors.gray.800,
            self.dark_colors.gray.900,
        )
    }
    
    /// Get CSS media queries for breakpoints
    pub fn get_media_queries(&self) -> String {
        format!(
            "@media (min-width: {}) {{\n\
             \t/* Small devices */\n\
             }}\n\
             \n\
             @media (min-width: {}) {{\n\
             \t/* Medium devices */\n\
             }}\n\
             \n\
             @media (min-width: {}) {{\n\
             \t/* Large devices */\n\
             }}\n\
             \n\
             @media (min-width: {}) {{\n\
             \t/* Extra large devices */\n\
             }}\n\
             \n\
             @media (min-width: {}) {{\n\
             \t/* Extra extra large devices */\n\
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