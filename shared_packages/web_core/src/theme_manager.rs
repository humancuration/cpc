//! Theme manager for CPC web applications
//!
//! This module provides a theme manager that handles multiple themes,
//! theme switching, and theme customization.

use crate::theme::{DesignSystem, ColorScheme};
use std::collections::HashMap;

/// Theme manager error types
#[derive(Debug)]
pub enum ThemeError {
    /// Theme not found
    ThemeNotFound(String),
    
    /// Invalid theme configuration
    InvalidTheme(String),
}

impl std::fmt::Display for ThemeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeError::ThemeNotFound(name) => write!(f, "Theme not found: {}", name),
            ThemeError::InvalidTheme(msg) => write!(f, "Invalid theme: {}", msg),
        }
    }
}

impl std::error::Error for ThemeError {}

/// Theme manager for handling multiple themes
#[derive(Debug)]
pub struct ThemeManager {
    /// Registered themes
    themes: HashMap<String, DesignSystem>,
    
    /// Active theme name
    active_theme: String,
    
    /// User overrides
    overrides: Option<DesignSystem>,
}

impl ThemeManager {
    /// Create a new theme manager
    pub fn new() -> Self {
        let mut themes = HashMap::new();
        let default_theme = DesignSystem::default();
        themes.insert("default".to_string(), default_theme);
        
        Self {
            themes,
            active_theme: "default".to_string(),
            overrides: None,
        }
    }
    
    /// Register a new theme
    pub fn register_theme(&mut self, name: String, theme: DesignSystem) -> Result<(), ThemeError> {
        // Validate the theme
        if name.is_empty() {
            return Err(ThemeError::InvalidTheme("Theme name cannot be empty".to_string()));
        }
        
        self.themes.insert(name, theme);
        Ok(())
    }
    
    /// Set the active theme
    pub fn set_theme(&mut self, name: &str) -> Result<(), ThemeError> {
        if self.themes.contains_key(name) {
            self.active_theme = name.to_string();
            Ok(())
        } else {
            Err(ThemeError::ThemeNotFound(name.to_string()))
        }
    }
    
    /// Apply partial overrides to the current theme
    pub fn apply_overrides(&mut self, overrides: DesignSystem) {
        self.overrides = Some(overrides);
    }
    
    /// Reset overrides
    pub fn reset_overrides(&mut self) {
        self.overrides = None;
    }
    
    /// Get the current effective theme
    pub fn get_theme(&self) -> DesignSystem {
        let base_theme = self.themes.get(&self.active_theme)
            .cloned()
            .unwrap_or_else(|| DesignSystem::default());
            
        if let Some(overrides) = &self.overrides {
            self.merge_themes(base_theme, overrides.clone())
        } else {
            base_theme
        }
    }
    
    /// Get the active theme name
    pub fn get_active_theme_name(&self) -> &str {
        &self.active_theme
    }
    
    /// Get all registered theme names
    pub fn get_theme_names(&self) -> Vec<String> {
        self.themes.keys().cloned().collect()
    }
    
    /// Set the color scheme for the current theme
    pub fn set_color_scheme(&mut self, scheme: ColorScheme) {
        if let Some(theme) = self.themes.get_mut(&self.active_theme) {
            theme.set_color_scheme(scheme);
        }
        
        // Also update overrides if they exist
        if let Some(overrides) = &mut self.overrides {
            overrides.set_color_scheme(scheme);
        }
    }
    
    /// Merge two themes, with the second taking precedence
    fn merge_themes(&self, base: DesignSystem, overrides: DesignSystem) -> DesignSystem {
        // In a real implementation, we would do a deep merge of the themes
        // For now, we'll just return the overrides
        overrides
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_theme_manager_creation() {
        let manager = ThemeManager::new();
        assert_eq!(manager.get_active_theme_name(), "default");
        assert_eq!(manager.get_theme_names(), vec!["default"]);
    }
    
    #[test]
    fn test_theme_registration() {
        let mut manager = ThemeManager::new();
        let theme = DesignSystem::default();
        
        assert!(manager.register_theme("custom".to_string(), theme).is_ok());
        assert_eq!(manager.get_theme_names().len(), 2);
        assert!(manager.get_theme_names().contains(&"custom".to_string()));
    }
    
    #[test]
    fn test_theme_switching() {
        let mut manager = ThemeManager::new();
        let theme = DesignSystem::default();
        
        manager.register_theme("custom".to_string(), theme).unwrap();
        assert!(manager.set_theme("custom").is_ok());
        assert_eq!(manager.get_active_theme_name(), "custom");
        
        assert!(manager.set_theme("nonexistent").is_err());
    }
    
    #[test]
    fn test_theme_overrides() {
        let mut manager = ThemeManager::new();
        let theme = DesignSystem::default();
        
        manager.apply_overrides(theme);
        assert!(manager.overrides.is_some());
        
        manager.reset_overrides();
        assert!(manager.overrides.is_none());
    }
}