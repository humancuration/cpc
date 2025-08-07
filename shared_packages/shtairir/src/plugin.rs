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
