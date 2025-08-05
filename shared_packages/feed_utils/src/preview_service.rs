//! Service for generating content previews for feeds

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use social_graph::domain::model::{ContentItem, ContentType};
use collaborative_docs::core::{DocumentPreview, DocumentService};

/// Error types for preview operations
#[derive(Debug, thiserror::Error)]
pub enum PreviewError {
    #[error("Document not found: {0}")]
    DocumentNotFound(Uuid),
    
    #[error("Access denied for document: {0}")]
    AccessDenied(Uuid),
    
    #[error("Preview generation failed: {0}")]
    GenerationFailed(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

/// Preview service for generating content previews for feeds
pub struct PreviewService {
    /// Document service for retrieving document information
    document_service: Arc<dyn DocumentService>,
}

impl PreviewService {
    /// Create a new preview service
    pub fn new(document_service: Arc<dyn DocumentService>) -> Self {
        Self {
            document_service,
        }
    }
    
    /// Generate a preview for a document
    pub async fn get_document_preview(
        &self,
        document_id: Uuid,
        user_id: Uuid,
    ) -> Result<DocumentPreview, PreviewError> {
        // Use the document service to get the preview
        self.document_service
            .get_document_preview(document_id, user_id)
            .await
            .map_err(|e| match e {
                collaborative_docs::core::DocumentError::DocumentNotFound(id) => PreviewError::DocumentNotFound(id),
                collaborative_docs::core::DocumentError::AccessDenied(id) => PreviewError::AccessDenied(id),
                _ => PreviewError::GenerationFailed(e.to_string()),
            })
    }
    
    /// Convert a document preview to a social graph content item for feed integration
    pub fn document_preview_to_content_item(
        &self,
        preview: DocumentPreview,
        owner_id: Uuid,
    ) -> ContentItem {
        ContentItem {
            id: preview.id,
            owner_id,
            content_type: ContentType::Custom("document".to_string()),
            source_package: "collaborative_docs".to_string(),
            metadata: serde_json::json!({
                "title": preview.title,
                "content_type": preview.content_type,
                "tags": preview.tags,
                "excerpt": preview.excerpt,
                "word_count": preview.word_count,
            }),
            timestamp: preview.updated_at,
            visibility: social_graph::domain::model::Visibility::Private, // Default to private, would be set by content provider
            relevance_score: 1.0, // Default relevance score
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    
    #[test]
    fn test_preview_service_creation() {
        // Note: This is a simplified test that would require a mock document service
        // In a real implementation, you would use a mock
    }
}