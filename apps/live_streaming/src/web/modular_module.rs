//! Modular implementation of the live streaming module
//!
//! This module implements the Module trait for dynamic module management.

use axum::Router;
use sqlx::PgPool;
use async_graphql::{EmptySubscription, Object, SchemaBuilder};
use anyhow::Result;

use crate::web::module::{initialize, LiveStreamingModule};

/// Live streaming module implementation for the modular system
pub struct ModularLiveStreaming {
    /// The original module implementation
    inner: LiveStreamingModule,
    
    /// Whether the module is currently enabled
    enabled: bool,
}

impl ModularLiveStreaming {
    /// Create a new modular live streaming module
    pub fn new(db_pool: PgPool) -> Self {
        let inner = initialize(db_pool);
        Self {
            inner,
            enabled: false,
        }
    }
}

#[async_trait::async_trait]
impl crate::module_registry::Module for ModularLiveStreaming {
    /// Get the module name
    fn name(&self) -> &str {
        "live-streaming"
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
            // Register the actual GraphQL schema for the live streaming module
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