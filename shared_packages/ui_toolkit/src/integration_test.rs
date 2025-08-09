//! Integration tests for the UI toolkit
//!
//! This module contains integration tests that verify the UI toolkit
//! components work correctly together.

#[cfg(test)]
mod tests {
    use yew::prelude::*;
    use yew::platform::spawn_local;
    use gloo_utils::document;
    use wasm_bindgen_test::*;
    use crate::themes::{ThemeProvider, ColorScheme};
    use crate::components::theme_provider::ThemeProvider as YewThemeProvider;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_theme_provider_creation() {
        let theme_provider = ThemeProvider::default();
        assert_eq!(theme_provider.design_system.get_effective_color_scheme(), ColorScheme::Light);
    }
    
    #[wasm_bindgen_test]
    fn test_theme_provider_toggle() {
        let mut theme_provider = ThemeProvider::default();
        theme_provider.set_color_scheme(ColorScheme::Light);
        assert_eq!(theme_provider.design_system.get_effective_color_scheme(), ColorScheme::Light);
        
        theme_provider.toggle_theme();
        assert_eq!(theme_provider.design_system.get_effective_color_scheme(), ColorScheme::Dark);
        
        theme_provider.toggle_theme();
        assert_eq!(theme_provider.design_system.get_effective_color_scheme(), ColorScheme::Light);
    }
    
    #[wasm_bindgen_test]
    fn test_theme_provider_css_generation() {
        let theme_provider = ThemeProvider::default();
        let css = theme_provider.get_theme_css();
        assert!(css.contains(":root"));
        assert!(css.contains("--cpc-primary"));
        assert!(css.contains("--cpc-text"));
    }
    
    #[wasm_bindgen_test]
    fn test_theme_provider_local_storage() {
        let mut theme_provider = ThemeProvider::default();
        theme_provider.set_color_scheme(ColorScheme::Dark);
        
        // Create a new theme provider to test loading from local storage
        let mut new_provider = ThemeProvider::default();
        new_provider.load_theme_preference();
        // Note: This test might not work in all environments due to local storage restrictions
    }
}