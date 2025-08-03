//! Module registry for the live streaming application
//!
//! This module provides traits and structures for dynamically managing modules.

use axum::Router;
use sqlx::PgPool;
use async_graphql::{EmptySubscription, Object, SchemaBuilder};
use anyhow::Result;

/// Trait that all modules must implement
#[async_trait::async_trait]
pub trait Module: Send + Sync {
    /// Get the module name
    fn name(&self) -> &str;
    
    /// Get the module version
    fn version(&self) -> &str;
    
    /// Check if the module is currently enabled
    fn is_enabled(&self) -> bool;
    
    /// Get the module's router for HTTP endpoints
    fn router(&self) -> Option<Router>;
    
    /// Register module types with the schema builder
    fn register_schema(&self, builder: &mut SchemaBuilder<Object, Object, EmptySubscription>);
    
    /// Enable the module
    async fn enable(&mut self, pool: &PgPool) -> Result<()>;
    
    /// Disable the module
    async fn disable(&mut self, pool: &PgPool) -> Result<()>;
}

/// Create a new module instance
pub fn create_module(db_pool: PgPool) -> impl Module {
    crate::web::modular_module::ModularLiveStreaming::new(db_pool)
}