//! Tests for the theme system
//!
//! This module contains tests for the theme system functionality.

use wasm_bindgen_test::*;
use web_core::theme::DesignSystem;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_design_system_creation() {
    let theme = DesignSystem::new();
    
    // Test default values
    assert_eq!(theme.colors.primary, "#007bff");
    assert_eq!(theme.colors.secondary, "#6c757d");
    assert_eq!(theme.spacing.md, "1rem");
    assert_eq!(theme.typography.font_family, "system-ui, -apple-system, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif");
}

#[wasm_bindgen_test]
fn test_design_system_default() {
    let theme = DesignSystem::default();
    
    // Test that default and new create the same result
    let theme2 = DesignSystem::new();
    assert_eq!(theme.colors.primary, theme2.colors.primary);
    assert_eq!(theme.spacing.md, theme2.spacing.md);
}

#[wasm_bindgen_test]
fn test_color_palette() {
    let theme = DesignSystem::default();
    
    // Test semantic colors
    assert_eq!(theme.colors.success, "#28a745");
    assert_eq!(theme.colors.warning, "#ffc107");
    assert_eq!(theme.colors.danger, "#dc3545");
    assert_eq!(theme.colors.info, "#17a2b8");
    
    // Test grayscale
    assert_eq!(theme.colors.gray.100, "#f8f9fa");
    assert_eq!(theme.colors.gray.500, "#adb5bd");
    assert_eq!(theme.colors.gray.900, "#212529");
}

#[wasm_bindgen_test]
fn test_spacing_scale() {
    let theme = DesignSystem::default();
    
    // Test spacing values
    assert_eq!(theme.spacing.xs, "0.25rem"); // 4px
    assert_eq!(theme.spacing.sm, "0.5rem");  // 8px
    assert_eq!(theme.spacing.md, "1rem");    // 16px
    assert_eq!(theme.spacing.lg, "1.5rem");  // 24px
    assert_eq!(theme.spacing.xl, "2rem");    // 32px
    assert_eq!(theme.spacing.xxl, "3rem");   // 48px
}

#[wasm_bindgen_test]
fn test_typography() {
    let theme = DesignSystem::default();
    
    // Test font sizes
    assert_eq!(theme.typography.font_sizes.xs, "0.75rem");  // 12px
    assert_eq!(theme.typography.font_sizes.md, "1rem");     // 16px
    assert_eq!(theme.typography.font_sizes.xxl, "1.5rem");  // 24px
    
    // Test font weights
    assert_eq!(theme.typography.font_weights.regular, 400);
    assert_eq!(theme.typography.font_weights.bold, 700);
    
    // Test line heights
    assert_eq!(theme.typography.line_heights.normal, 1.5);
}

#[wasm_bindgen_test]
fn test_border_radius() {
    let theme = DesignSystem::default();
    
    // Test border radius values
    assert_eq!(theme.border_radius.sm, "0.25rem"); // 4px
    assert_eq!(theme.border_radius.md, "0.5rem");  // 8px
    assert_eq!(theme.border_radius.full, "9999px");
}

#[wasm_bindgen_test]
fn test_shadows() {
    let theme = DesignSystem::default();
    
    // Test shadow values
    assert_eq!(theme.shadows.sm, "0 0.125rem 0.25rem rgba(0, 0, 0, 0.075)");
    assert_eq!(theme.shadows.md, "0 0.5rem 1rem rgba(0, 0, 0, 0.15)");
}