//! Modular implementation of the invoicing module
//!
//! This module implements the Module trait for dynamic module management.

use axum::Router;
use sqlx::PgPool;
use async_graphql::{EmptySubscription, Object, SchemaBuilder};
use anyhow::Result;
use crate::invoicing::module::{initialize, InvoicingModule};

/// Invoicing module implementation for the modular system
pub struct ModularInvoicing {
    /// The original module implementation
    inner: InvoicingModule,
    
    /// Whether the module is currently enabled
    enabled: bool,
}

impl ModularInvoicing {
    /// Create a new modular invoicing module
    pub fn new(db_pool: PgPool, network: Arc<cpc_net::net::Network>) -> Self {
        let inner = initialize(db_pool, network);
        Self {
            inner,
            enabled: false,
        }
    }
}

#[async_trait::async_trait]
impl crate::module_registry::Module for ModularInvoicing {
    /// Get the module name
    fn name(&self) -> &str {
        "invoicing"
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
            // Register the actual GraphQL schema for the invoicing module
            builder.register_type::<crate::invoicing::graphql::InvoicingQuery>();
            builder.register_type::<crate::invoicing::graphql::InvoicingMutation>();
            builder.register_type::<crate::invoicing::graphql::InvoicingSubscription>();
            
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