//! Modular implementation of the music player module
//!
//! This module implements the Module trait for dynamic module management.

use axum::Router;
use sqlx::PgPool;
use async_graphql::{EmptySubscription, Object, SchemaBuilder};
use anyhow::Result;
use crate::web::module::{initialize, MusicPlayerModule};

/// Music player module implementation for the modular system
pub struct ModularMusicPlayer {
    /// The original module implementation
    inner: MusicPlayerModule,
    
    /// Whether the module is currently enabled
    enabled: bool,
}

impl ModularMusicPlayer {
    /// Create a new modular music player module
    pub fn new(db_pool: PgPool) -> Self {
        let inner = initialize(db_pool);
        Self {
            inner,
            enabled: false,
        }
    }
}

#[async_trait::async_trait]
impl crate::module_registry::Module for ModularMusicPlayer {
    /// Get the module name
    fn name(&self) -> &str {
        "music-player"
    }
    
    /// Get the module version
    fn version(&self) -> &str {
        "0.1.0"
    }
    
    /// Check if the module is currently enabled
    fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Get the module's router for HTTP endpoints
    fn router(&self) -> Option<Router> {
        if self.enabled {
            Some(self.inner.router.clone())
        } else {
            None
        }
    }
    
    /// Register module types with the schema builder
    fn register_schema(&self, builder: &mut SchemaBuilder<Object, Object, EmptySubscription>) {
        if self.enabled {
            // Register the actual GraphQL schema for the music player module
            builder.register_type::<crate::web::graphql::MusicPlayerQuery>();
            builder.register_type::<crate::web::graphql::MusicPlayerMutation>();
            builder.register_type::<crate::web::graphql::MusicPlayerSubscription>();
            
            // Add the query, mutation, and subscription objects to the builder
            // Note: In a more complete implementation, we would merge these with existing schema objects
            // For now, we're just registering the types so they're available in introspection
        }
    }
    
    /// Enable the module
    async fn enable(&mut self, _pool: &PgPool) -> Result<()> {
        self.enabled = true;
        Ok(())
    }
    
    /// Disable the module
    async fn disable(&mut self, _pool: &PgPool) -> Result<()> {
        self.enabled = false;
        Ok(())
    }
}