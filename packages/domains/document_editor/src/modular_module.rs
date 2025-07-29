//! Modular implementation of the document editor module
//!
//! This module implements the Module trait for dynamic module management.

use axum::Router;
use sqlx::PgPool;
use async_graphql::{EmptySubscription, Object, SchemaBuilder};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::application::document_service::DocumentService;
use crate::application::export_service::ExportService;
use crate::application::collaboration_service::CollaborationService;
use crate::infrastructure::PgDocumentRepository;
use crate::web::graphql::{DocumentEditorQuery, DocumentEditorMutation, DocumentEditorSubscription};
use crate::module_registry::Module;

/// Document editor module implementation for the modular system
pub struct ModularDocumentEditor {
    /// Document service
    document_service: Arc<DocumentService>,
    
    /// Export service
    export_service: Arc<ExportService>,
    
    /// Collaboration service
    collaboration_service: Arc<CollaborationService>,
    
    /// Whether the module is currently enabled
    enabled: bool,
}

impl ModularDocumentEditor {
    /// Create a new modular document editor module
    pub fn new(db_pool: PgPool) -> Self {
        // Create repository
        let repository = Arc::new(PgDocumentRepository::new(db_pool.clone()));
        
        // Create services
        let document_service = Arc::new(DocumentService::new(repository.clone()));
        let export_service = Arc::new(ExportService::new());
        let collaboration_service = Arc::new(CollaborationService::new(repository.clone()));
        
        Self {
            document_service,
            export_service,
            collaboration_service,
            enabled: false,
        }
    }
    
    /// Get a reference to the document service
    pub fn document_service(&self) -> &Arc<DocumentService> {
        &self.document_service
    }
    
    /// Get a reference to the export service
    pub fn export_service(&self) -> &Arc<ExportService> {
        &self.export_service
    }
    
    /// Get a reference to the collaboration service
    pub fn collaboration_service(&self) -> &Arc<CollaborationService> {
        &self.collaboration_service
    }
}

#[async_trait::async_trait]
impl Module for ModularDocumentEditor {
    /// Get the module name
    fn name(&self) -> &str {
        "document_editor"
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
        // TODO: Implement HTTP endpoints for the document editor
        None
    }
    
    /// Register module types with the schema builder
    fn register_schema(&self, builder: &mut SchemaBuilder<Object, Object, EmptySubscription>) {
        if self.enabled {
            // Register the GraphQL schema for the document editor module
            builder.register_type::<DocumentEditorQuery>();
            builder.register_type::<DocumentEditorMutation>();
            builder.register_type::<DocumentEditorSubscription>();
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