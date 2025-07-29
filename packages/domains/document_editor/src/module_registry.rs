//! Module registry trait for the document editor module
//!
//! This module defines the Module trait that all modules must implement
//! to be registered in the system.

use std::sync::Arc;
use tokio::sync::RwLock;
use sqlx::PgPool;
use axum::Router;
use async_graphql::{EmptySubscription, Object, SchemaBuilder as GraphQLSchemaBuilder};
use anyhow::Result;
use semver::{Version, VersionReq};
use uuid::Uuid;

use crate::application::document_service::DocumentService;
use crate::application::export_service::ExportService;
use crate::application::collaboration_service::CollaborationService;
use crate::infrastructure::repository::PgDocumentRepository;
use crate::collaboration::panda_network::{PandaNetwork, PandaSyncService};
use crate::collaboration::service::RealtimeCollaborationService;

/// Trait that all modules must implement to be registered in the system
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
    fn register_schema(&self, builder: &mut GraphQLSchemaBuilder<Object, Object, EmptySubscription>);
    
    /// Enable the module
    async fn enable(&mut self, pool: &PgPool) -> Result<()>;
    
    /// Disable the module
    async fn disable(&mut self, pool: &PgPool) -> Result<()>;
    
    /// Get the document service
    fn get_document_service(&self) -> Option<Arc<DocumentService>>;
    
    /// Get the export service
    fn get_export_service(&self) -> Option<Arc<ExportService>>;
    
    /// Get the collaboration service
    fn get_collaboration_service(&self) -> Option<Arc<CollaborationService>>;
}

/// Module dependency requirement
#[derive(Debug, Clone)]
pub enum DependencyRequirement {
    /// Required dependency with version constraint
    Required { name: String, constraint: VersionReq },
    /// Optional dependency with version constraint
    Optional { name: String, constraint: VersionReq },
}

/// Registered module with metadata
pub struct RegisteredModule {
    /// The module instance
    pub module: Arc<RwLock<dyn Module>>,
    /// Module dependencies
    pub dependencies: Vec<DependencyRequirement>,
    /// Module version
    pub version: Version,
}