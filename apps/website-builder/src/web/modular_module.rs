//! Modular implementation of the website builder module
//!
//! This module implements the Module trait for dynamic module management.

use axum::Router;
use sqlx::PgPool;
use async_graphql::{EmptySubscription, Object, SchemaBuilder};
use anyhow::Result;
use crate::web::module::{initialize, WebsiteBuilderModule};

/// Website builder module implementation for the modular system
pub struct ModularWebsiteBuilder {
    /// The original module implementation
    inner: WebsiteBuilderModule,
    
    /// Whether the module is currently enabled
    enabled: bool,
}

impl ModularWebsiteBuilder {
    /// Create a new modular website builder module
    pub fn new(db_pool: PgPool) -> Self {
        let inner = initialize(db_pool);
        Self {
            inner,
            enabled: false,
        }
    }
}

#[async_trait::async_trait]
impl super::super::module_registry::Module for ModularWebsiteBuilder {
    /// Get the module name
    fn name(&self) -> &str {
        "website-builder"
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
            // In a real implementation, this would register the actual GraphQL schema
            // For now, we'll just add placeholder types
            let mut query = Object::new("WebsiteBuilderQuery");
            query = query.field(async_graphql::Field::new("websiteBuilderTest", async_graphql::Type::NonNull(async_graphql::Type::Named("String".to_string())), |_, _| async { Ok::<_, async_graphql::Error>("test".to_string()) }));
            
            let mut mutation = Object::new("WebsiteBuilderMutation");
            mutation = mutation.field(async_graphql::Field::new("websiteBuilderMutationTest", async_graphql::Type::NonNull(async_graphql::Type::Named("String".to_string())), |_, _| async { Ok::<_, async_graphql::Error>("test".to_string()) }));
            
            // Note: In a real implementation, we would properly merge these with the builder
            // For now, this is a placeholder implementation
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