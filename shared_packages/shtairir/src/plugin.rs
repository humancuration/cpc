//! Plugin system for custom blocks in Shtairir
//! 
//! This module defines the plugin system that allows for dynamic loading
//! of custom blocks and adapters.

use crate::block::{Block, BlockAdapter};
use async_trait::async_trait;
use std::sync::Arc;
use std::path::Path;
use shtairir_core::error::ShtairirError;

/// Unique identifier for a plugin
pub type PluginId = String;

/// Plugin manager for custom blocks
pub struct PluginManager {
    /// Loaded plugins
    plugins: std::collections::HashMap<PluginId, Arc<dyn Plugin>>,
    
    /// Plugin loader
    loader: Box<dyn PluginLoader>,
    
    /// Plugin registry
    // registry: Arc<Registry>, // TODO: Define registry type
    
    /// Configuration
    config: PluginManagerConfig,
}

impl PluginManager {
    /// Create a new plugin manager
    pub fn new(loader: Box<dyn PluginLoader>, config: PluginManagerConfig) -> Self {
        Self {
            plugins: std::collections::HashMap::new(),
            loader,
            // registry,
            config,
        }
    }
    
    /// Load a plugin from a file
    pub async fn load_plugin_from_file(&mut self, path: &Path) -> Result<PluginId, PluginError> {
        let plugin = self.loader.load_from_file(path).await?;
        let plugin_id = plugin.spec().id.clone();
        self.plugins.insert(plugin_id.clone(), plugin);
        Ok(plugin_id)
    }
    
    /// Load a plugin from a library
    pub async fn load_plugin_from_library(&mut self, library: &Library) -> Result<PluginId, PluginError> {
        let plugin = self.loader.load_from_library(library).await?;
        let plugin_id = plugin.spec().id.clone();
        self.plugins.insert(plugin_id.clone(), plugin);
        Ok(plugin_id)
    }
    
    /// Unload a plugin
    pub async fn unload_plugin(&mut self, plugin_id: &PluginId) -> Result<bool, PluginError> {
        if self.plugins.contains_key(plugin_id) {
            self.loader.unload(plugin_id).await?;
            self.plugins.remove(plugin_id);
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    /// Get a plugin by ID
    pub fn get_plugin(&self, plugin_id: &PluginId) -> Option<&Arc<dyn Plugin>> {
        self.plugins.get(plugin_id)
    }
    
    /// Get all loaded plugin IDs
    pub fn get_loaded_plugins(&self) -> Vec<PluginId> {
        self.plugins.keys().cloned().collect()
    }
}

/// Plugin manager configuration
#[derive(Debug, Clone)]
pub struct PluginManagerConfig {
    /// Whether to automatically load plugins from a directory
    pub auto_load: bool,
    
    /// Directory to scan for plugins
    pub plugin_directory: Option<std::path::PathBuf>,
    
    /// Whether to allow loading of unsigned plugins
    pub allow_unsigned: bool,
    
    /// Security policies for plugins
    pub security_policies: std::collections::HashMap<String, String>,
}

impl Default for PluginManagerConfig {
    fn default() -> Self {
        Self {
            auto_load: false,
            plugin_directory: None,
            allow_unsigned: false,
            security_policies: std::collections::HashMap::new(),
        }
    }
}

/// Plugin trait
#[async_trait]
pub trait Plugin: Send + Sync {
    /// Get the plugin specification
    fn spec(&self) -> &PluginSpec;
    
    /// Initialize the plugin
    async fn initialize(&self, context: &PluginContext) -> Result<(), PluginError>;
    
    /// Get blocks provided by this plugin
    fn get_blocks(&self) -> Vec<Arc<dyn Block>>;
    
    /// Get adapters provided by this plugin
    fn get_adapters(&self) -> Vec<Arc<dyn BlockAdapter>>;
    
    /// Shutdown the plugin
    async fn shutdown(&self) -> Result<(), PluginError>;
}

/// Plugin specification
#[derive(Debug, Clone)]
pub struct PluginSpec {
    /// Unique identifier for the plugin
    pub id: PluginId,
    
    /// Plugin name
    pub name: String,
    
    /// Plugin version
    pub version: String,
    
    /// Plugin description
    pub description: String,
    
    /// Plugin authors
    pub authors: Vec<String>,
    
    /// Plugin license
    pub license: String,
    
    /// Plugin dependencies
    pub dependencies: Vec<String>,
    
    /// Plugin capabilities
    pub capabilities: Vec<String>,
}

/// Plugin context
#[derive(Debug, Clone)]
pub struct PluginContext {
    /// Plugin registry
    // TODO: Define registry type
    // pub registry: Arc<Registry>,
    
    /// Event system
    // TODO: Define event system type
    // pub event_system: Arc<dyn EventSystem>,
    
    /// Configuration manager
    // TODO: Define config manager type
    // pub config: Arc<dyn ConfigManager>,
    
    /// Security context
    // TODO: Define security context type
    // pub security_context: SecurityContext,
}

/// Plugin loader trait
#[async_trait]
pub trait PluginLoader: Send + Sync {
    /// Load a plugin from a file
    async fn load_from_file(&self, path: &Path) -> Result<Arc<dyn Plugin>, PluginError>;
    
    /// Load a plugin from a library
    async fn load_from_library(&self, library: &Library) -> Result<Arc<dyn Plugin>, PluginError>;
    
    /// Unload a plugin
    async fn unload(&self, plugin_id: &PluginId) -> Result<(), PluginError>;
    
    /// Get loaded plugins
    fn get_loaded_plugins(&self) -> Vec<PluginId>;
}

/// Library representation
#[derive(Debug, Clone)]
pub struct Library {
    /// Library name
    pub name: String,
    
    /// Library version
    pub version: String,
    
    /// Library path
    pub path: std::path::PathBuf,
}

/// Plugin error
#[derive(Debug, Clone)]
pub struct PluginError {
    /// Error message
    pub message: String,
    
    /// Error details
    pub details: Option<String>,
}

impl PluginError {
    /// Create a new plugin error
    pub fn new(message: String) -> Self {
        Self {
            message,
            details: None,
        }
    }
    
    /// Create a new plugin error with details
    pub fn with_details(mut self, details: String) -> Self {
        self.details = Some(details);
        self
    }
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PluginError: {}", self.message)?;
        if let Some(details) = &self.details {
            write!(f, " ({})", details)?;
        }
        Ok(())
    }
}

impl std::error::Error for PluginError {}

impl From<PluginError> for ShtairirError {
    fn from(error: PluginError) -> Self {
        ShtairirError::Adapter(error.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_plugin_manager_config_default() {
        let config = PluginManagerConfig::default();
        
        assert_eq!(config.auto_load, false);
        assert_eq!(config.plugin_directory, None);
        assert_eq!(config.allow_unsigned, false);
        assert!(config.security_policies.is_empty());
    }
    
    #[test]
    fn test_plugin_error_creation() {
        let error = PluginError::new("Test error".to_string())
            .with_details("Test details".to_string());
        
        assert_eq!(error.message, "Test error");
        assert_eq!(error.details, Some("Test details".to_string()));
        
        let error_string = format!("{}", error);
        assert!(error_string.contains("PluginError: Test error"));
        assert!(error_string.contains("Test details"));
    }
}