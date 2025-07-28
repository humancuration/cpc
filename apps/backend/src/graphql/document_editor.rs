use async_graphql::*;
use std::sync::Arc;
use uuid::Uuid;

use cpc_document_editor::application::document_service::DocumentService;
use cpc_document_editor::application::export_service::ExportService;
use cpc_document_editor::application::collaboration_service::CollaborationService;
use cpc_document_editor::web::graphql::{DocumentEditorQuery, DocumentEditorMutation, DocumentEditorSubscription};

// Re-export the GraphQL types
pub use cpc_document_editor::web::graphql::{DocumentEditorQuery, DocumentEditorMutation, DocumentEditorSubscription};