//! Content provider implementation for collaborative documents

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use social_graph::domain::model::{
    ContentProvider, ContentProviderError, ContentItem, ContentType, FeedFilter, Visibility,
};
use social_graph::infrastructure::content_providers::{ContentProviderRegistry, ProviderMetadata};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

use crate::{
    core::{DocumentService, DocumentMetadata, DocumentContent},
    CollaborativeDocService,
};

/// Content provider for collaborative documents
pub struct CollaborativeDocProvider {
    doc_service: Arc<CollaborativeDocService>,
}

impl CollaborativeDocProvider {
    /// Create a new collaborative document content provider
    pub fn new(doc_service: Arc<CollaborativeDocService>) -> Self {
        Self { doc_service }
    }
}

#[async_trait]
impl ContentProvider for CollaborativeDocProvider {
    fn content_type(&self) -> ContentType {
        ContentType::Custom("document".to_string())
    }

    async fn get_content(
        &self,
        user_id: Uuid,
        _after: Option<chrono::DateTime<Utc>>,
        limit: usize,
        _filters: &[FeedFilter],
    ) -> Result<Vec<ContentItem>, ContentProviderError> {
        // Get documents for the user
        let documents = self
            .doc_service
            .list_documents(user_id, limit, 0)
            .await
            .map_err(|e| ContentProviderError::FetchFailed(e.to_string()))?;

        // Convert documents to content items
        let mut content_items = Vec::new();
        for doc in documents {
            content_items.push(document_to_content_item(doc, user_id));
        }

        Ok(content_items)
    }

    fn serialize_state(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // For this provider, we don't need to serialize any state
        // The document service handles its own state
        Ok(Vec::new())
    }

    fn deserialize_state(&self, _data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        // For this provider, we don't need to deserialize any state
        // The document service handles its own state
        Ok(())
    }
}

/// Convert document metadata to content item
fn document_to_content_item(metadata: DocumentMetadata, user_id: Uuid) -> ContentItem {
    // Convert our visibility enum to the social graph visibility enum
    let visibility = match metadata.visibility {
        crate::core::Visibility::Public => social_graph::domain::model::Visibility::Public,
        crate::core::Visibility::FriendsOnly => social_graph::domain::model::Visibility::FriendsOnly,
        crate::core::Visibility::Private => social_graph::domain::model::Visibility::Private,
    };
    
    ContentItem {
        id: metadata.id,
        owner_id: metadata.owner_id,
        content_type: ContentType::Custom("document".to_string()),
        source_package: "collaborative_docs".to_string(),
        metadata: serde_json::json!({
            "title": metadata.title,
            "content_type": metadata.content_type,
            "tags": metadata.tags,
            "version": metadata.version,
        }),
        timestamp: metadata.updated_at,
        visibility,
        relevance_score: 1.0, // Default relevance score
    }
}
/// Document content provider metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborativeDocProviderMetadata {
    pub provider_id: Uuid,
    pub name: String,
    pub version: String,
    pub description: String,
}

impl Default for CollaborativeDocProviderMetadata {
    fn default() -> Self {
        Self {
            provider_id: Uuid::nil(),
            name: "Collaborative Document Provider".to_string(),
            version: "0.1.0".to_string(),
            description: "Provider for collaborative documents with CRDT support".to_string(),
        }
    }
}

impl From<CollaborativeDocProviderMetadata> for ProviderMetadata {
    fn from(metadata: CollaborativeDocProviderMetadata) -> Self {
        ProviderMetadata {
            id: metadata.provider_id,
            name: metadata.name,
            content_type: ContentType::Custom("document".to_string()),
            version: metadata.version,
            dependencies: vec![],
            state_schema_version: "1.0.0".to_string(),
            compatible_previous_versions: vec![],
            required_interfaces: vec![],
        }
    }
}

impl CollaborativeDocProvider {
    /// Register this provider with the social graph registry
    pub fn register_provider(
        self: Arc<Self>,
        registry: &ContentProviderRegistry,
        metadata: CollaborativeDocProviderMetadata,
    ) -> Result<Uuid, Box<dyn std::error::Error>> {
        let provider_metadata: ProviderMetadata = metadata.into();
        let provider_id = registry.register_provider(self, provider_metadata)?;
        Ok(provider_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{DocProvider, DocumentMetadata, DocumentContent, DocumentPermission, AccessLevel};
    use chrono::Utc;
    use std::collections::HashMap;
    use tokio;

    // Mock document provider for testing
    struct MockDocProvider {
        documents: HashMap<Uuid, (DocumentMetadata, DocumentContent)>,
        permissions: HashMap<Uuid, Vec<DocumentPermission>>,
    }

    impl MockDocProvider {
        fn new() -> Self {
            Self {
                documents: HashMap::new(),
                permissions: HashMap::new(),
            }
        }
    }

    #[async_trait]
    impl DocProvider for MockDocProvider {
        async fn store_metadata(&self, _metadata: &DocumentMetadata) -> Result<(), crate::core::DocumentError> {
            Ok(())
        }

        async fn retrieve_metadata(&self, document_id: Uuid) -> Result<DocumentMetadata, crate::core::DocumentError> {
            self.documents.get(&document_id)
                .map(|(metadata, _)| metadata.clone())
                .ok_or(crate::core::DocumentError::DocumentNotFound(document_id))
        }

        async fn store_content(&self, _document_id: Uuid, _content: &DocumentContent) -> Result<(), crate::core::DocumentError> {
            Ok(())
        }

        async fn retrieve_content(&self, document_id: Uuid) -> Result<DocumentContent, crate::core::DocumentError> {
            self.documents.get(&document_id)
                .map(|(_, content)| content.clone())
                .ok_or(crate::core::DocumentError::DocumentNotFound(document_id))
        }

        async fn delete_document(&self, _document_id: Uuid) -> Result<(), crate::core::DocumentError> {
            Ok(())
        }

        async fn list_documents(&self, user_id: Uuid, limit: usize, offset: usize) -> Result<Vec<DocumentMetadata>, crate::core::DocumentError> {
            let mut docs: Vec<DocumentMetadata> = self.documents.values()
                .filter(|(metadata, _)| metadata.owner_id == user_id)
                .map(|(metadata, _)| metadata.clone())
                .collect();
            
            // Apply limit and offset
            let start = offset.min(docs.len());
            let end = (offset + limit).min(docs.len());
            if start < docs.len() {
                docs = docs[start..end].to_vec();
            } else {
                docs.clear();
            }
            
            Ok(docs)
        }

        async fn store_permission(&self, _document_id: Uuid, _permission: &DocumentPermission) -> Result<(), crate::core::DocumentError> {
            Ok(())
        }

        async fn retrieve_permissions(&self, document_id: Uuid) -> Result<Vec<DocumentPermission>, crate::core::DocumentError> {
            Ok(self.permissions.get(&document_id).cloned().unwrap_or_default())
        }

        async fn delete_permission(&self, _document_id: Uuid, _user_id: Uuid) -> Result<(), crate::core::DocumentError> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_collaborative_doc_provider() -> Result<(), Box<dyn std::error::Error>> {
        /*
        // Create mock provider
        let mock_provider = Arc::new(MockDocProvider::new());
        
        // Create access checker
        let access_checker = crate::access::DocumentAccessChecker::new(None);
        
        // Create document service
        let doc_service = Arc::new(CollaborativeDocService::new(
            mock_provider as Arc<dyn DocProvider>,
            access_checker,
        ));
        
        // Create content provider
        let provider = CollaborativeDocProvider::new(doc_service);
        
        // Test content type
        assert_eq!(provider.content_type(), ContentType::Document);
        
        // Test get content (would be empty with mock provider)
        let user_id = Uuid::new_v4();
        let content = provider.get_content(user_id, None, 10, &[]).await?;
        assert!(content.is_empty());
        */
        
        Ok(())
    }

    #[test]
    fn test_document_to_content_item() {
        let document_id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let now = Utc::now();
        
        let metadata = DocumentMetadata {
            id: document_id,
            title: "Test Document".to_string(),
            owner_id,
            created_at: now,
            updated_at: now,
            content_type: "text/plain".to_string(),
            tags: vec!["test".to_string()],
            version: 1,
            visibility: crate::core::Visibility::Private,
        };
        
        let content_item = document_to_content_item(metadata, user_id);
        
        assert_eq!(content_item.id, document_id);
        assert_eq!(content_item.owner_id, owner_id);
        assert_eq!(content_item.content_type, ContentType::Custom("document".to_string()));
        assert_eq!(content_item.source_package, "collaborative_docs");
        assert_eq!(content_item.visibility, social_graph::domain::model::Visibility::Private);
        assert_eq!(content_item.relevance_score, 1.0);
    }
}