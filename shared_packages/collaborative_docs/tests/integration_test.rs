//! Integration tests for the collaborative documentation system

use collaborative_docs::{
    CollaborativeDocService,
    core::{DocProvider, DocumentContent, DocumentMetadata, DocumentPermission, AccessLevel, DocumentError},
    access::DocumentAccessChecker,
};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

/// A test document provider that simulates storage
struct TestDocProvider {
    metadata: std::sync::Mutex<std::collections::HashMap<Uuid, DocumentMetadata>>,
    content: std::sync::Mutex<std::collections::HashMap<Uuid, DocumentContent>>,
    permissions: std::sync::Mutex<std::collections::HashMap<Uuid, Vec<DocumentPermission>>>,
}

impl TestDocProvider {
    fn new() -> Self {
        Self {
            metadata: std::sync::Mutex::new(std::collections::HashMap::new()),
            content: std::sync::Mutex::new(std::collections::HashMap::new()),
            permissions: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl DocProvider for TestDocProvider {
    async fn store_metadata(&self, metadata: &DocumentMetadata) -> Result<(), DocumentError> {
        self.metadata.lock().unwrap().insert(metadata.id, metadata.clone());
        Ok(())
    }

    async fn retrieve_metadata(&self, document_id: Uuid) -> Result<DocumentMetadata, DocumentError> {
        self.metadata.lock().unwrap().get(&document_id)
            .cloned()
            .ok_or(DocumentError::DocumentNotFound(document_id))
    }

    async fn store_content(&self, document_id: Uuid, content: &DocumentContent) -> Result<(), DocumentError> {
        self.content.lock().unwrap().insert(document_id, content.clone());
        Ok(())
    }

    async fn retrieve_content(&self, document_id: Uuid) -> Result<DocumentContent, DocumentError> {
        self.content.lock().unwrap().get(&document_id)
            .cloned()
            .ok_or(DocumentError::DocumentNotFound(document_id))
    }

    async fn delete_document(&self, document_id: Uuid) -> Result<(), DocumentError> {
        self.metadata.lock().unwrap().remove(&document_id);
        self.content.lock().unwrap().remove(&document_id);
        self.permissions.lock().unwrap().remove(&document_id);
        Ok(())
    }

    async fn list_documents(&self, user_id: Uuid, limit: usize, offset: usize) -> Result<Vec<DocumentMetadata>, DocumentError> {
        let metadata_map = self.metadata.lock().unwrap();
        let mut docs: Vec<DocumentMetadata> = metadata_map.values()
            .filter(|metadata| metadata.owner_id == user_id)
            .cloned()
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

    async fn store_permission(&self, document_id: Uuid, permission: &DocumentPermission) -> Result<(), DocumentError> {
        let mut permissions_map = self.permissions.lock().unwrap();
        let permissions = permissions_map.entry(document_id).or_insert_with(Vec::new);
        
        // Remove existing permission for this user if it exists
        permissions.retain(|p| p.user_id != permission.user_id);
        
        // Add the new permission
        permissions.push(permission.clone());
        
        Ok(())
    }

    async fn retrieve_permissions(&self, document_id: Uuid) -> Result<Vec<DocumentPermission>, DocumentError> {
        let permissions_map = self.permissions.lock().unwrap();
        Ok(permissions_map.get(&document_id).cloned().unwrap_or_default())
    }

    async fn delete_permission(&self, document_id: Uuid, user_id: Uuid) -> Result<(), DocumentError> {
        let mut permissions_map = self.permissions.lock().unwrap();
        if let Some(permissions) = permissions_map.get_mut(&document_id) {
            permissions.retain(|p| p.user_id != user_id);
        }
        Ok(())
    }
}

#[tokio::test]
async fn test_document_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    // Create test provider
    let provider = Arc::new(TestDocProvider::new());
    
    // Create access checker
    let access_checker = DocumentAccessChecker::new(None);
    
    // Create document service
    let doc_service = Arc::new(CollaborativeDocService::new(
        provider as Arc<dyn DocProvider>,
        access_checker,
    ));
    
    // Create users
    let owner_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    
    // Create a document
    let content = DocumentContent {
        data: json!({"text": "Hello, world!"}),
        format: "json".to_string(),
    };
    
    let metadata = doc_service.create_document(
        owner_id,
        "Test Document".to_string(),
        content,
        vec!["test".to_string()],
    ).await?;
    
    // Verify document was created
    assert_eq!(metadata.title, "Test Document");
    assert_eq!(metadata.owner_id, owner_id);
    assert_eq!(metadata.version, 1);
    
    // Retrieve document metadata
    let retrieved_metadata = doc_service.get_document_metadata(metadata.id, owner_id).await?;
    assert_eq!(retrieved_metadata.id, metadata.id);
    assert_eq!(retrieved_metadata.title, "Test Document");
    
    // Retrieve document content
    let retrieved_content = doc_service.get_document_content(metadata.id, owner_id).await?;
    assert_eq!(retrieved_content.format, "json");
    
    // Update document
    let updated_content = DocumentContent {
        data: json!({"text": "Updated content"}),
        format: "json".to_string(),
    };
    
    let updated_metadata = doc_service.update_document(
        metadata.id,
        owner_id,
        updated_content,
    ).await?;
    
    // Verify document was updated
    assert_eq!(updated_metadata.version, 2);
    
    // List documents
    let documents = doc_service.list_documents(owner_id, 10, 0).await?;
    assert_eq!(documents.len(), 1);
    assert_eq!(documents[0].id, metadata.id);
    
    // Grant access
    doc_service.grant_access(
        metadata.id,
        owner_id,
        user_id,
        AccessLevel::Read,
    ).await?;
    
    // Verify permission was granted
    let permissions = doc_service.get_permissions(metadata.id, owner_id).await?;
    assert_eq!(permissions.len(), 2); // Owner (admin) + user (read)
    
    // Check that user can read the document
    let user_metadata = doc_service.get_document_metadata(metadata.id, user_id).await;
    assert!(user_metadata.is_ok());
    
    // Check that user cannot write to the document
    let update_result = doc_service.update_document(
        metadata.id,
        user_id,
        DocumentContent {
            data: json!({"text": "Unauthorized update"}),
            format: "json".to_string(),
        },
    ).await;
    
    // This should fail because user only has read access
    assert!(update_result.is_err());
    
    // Grant write access
    doc_service.grant_access(
        metadata.id,
        owner_id,
        user_id,
        AccessLevel::Write,
    ).await?;
    
    // Now user should be able to update
    let updated_content = DocumentContent {
        data: json!({"text": "Authorized update by user"}),
        format: "json".to_string(),
    };
    
    let update_result = doc_service.update_document(
        metadata.id,
        user_id,
        updated_content,
    ).await;
    
    assert!(update_result.is_ok());
    
    Ok(())
}

#[tokio::test]
async fn test_access_control() -> Result<(), Box<dyn std::error::Error>> {
    // Create test provider
    let provider = Arc::new(TestDocProvider::new());
    
    // Create access checker
    let access_checker = DocumentAccessChecker::new(None);
    
    // Create document service
    let doc_service = Arc::new(CollaborativeDocService::new(
        provider as Arc<dyn DocProvider>,
        access_checker,
    ));
    
    // Create users
    let owner_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let unauthorized_user_id = Uuid::new_v4();
    
    // Create a document
    let content = DocumentContent {
        data: json!({"text": "Private content"}),
        format: "json".to_string(),
    };
    
    let metadata = doc_service.create_document(
        owner_id,
        "Private Document".to_string(),
        content,
        vec![],
    ).await?;
    
    // Verify unauthorized user cannot access the document
    let metadata_result = doc_service.get_document_metadata(metadata.id, unauthorized_user_id).await;
    assert!(metadata_result.is_err());
    
    let content_result = doc_service.get_document_content(metadata.id, unauthorized_user_id).await;
    assert!(content_result.is_err());
    
    // Grant read access
    doc_service.grant_access(
        metadata.id,
        owner_id,
        user_id,
        AccessLevel::Read,
    ).await?;
    
    // Verify user can now read the document
    let metadata_result = doc_service.get_document_metadata(metadata.id, user_id).await;
    assert!(metadata_result.is_ok());
    
    let content_result = doc_service.get_document_content(metadata.id, user_id).await;
    assert!(content_result.is_ok());
    
    // But user still cannot update
    let update_result = doc_service.update_document(
        metadata.id,
        user_id,
        DocumentContent {
            data: json!({"text": "Unauthorized update"}),
            format: "json".to_string(),
        },
    ).await;
    
    assert!(update_result.is_err());
    
    Ok(())
}

#[tokio::test]
async fn test_document_deletion() -> Result<(), Box<dyn std::error::Error>> {
    // Create test provider
    let provider = Arc::new(TestDocProvider::new());
    
    // Create access checker
    let access_checker = DocumentAccessChecker::new(None);
    
    // Create document service
    let doc_service = Arc::new(CollaborativeDocService::new(
        provider as Arc<dyn DocProvider>,
        access_checker,
    ));
    
    // Create users
    let owner_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    
    // Create a document
    let content = DocumentContent {
        data: json!({"text": "Content to be deleted"}),
        format: "json".to_string(),
    };
    
    let metadata = doc_service.create_document(
        owner_id,
        "Document to Delete".to_string(),
        content,
        vec![],
    ).await?;
    
    // Verify document exists
    let retrieved_metadata = doc_service.get_document_metadata(metadata.id, owner_id).await;
    assert!(retrieved_metadata.is_ok());
    
    // Grant read access to user
    doc_service.grant_access(
        metadata.id,
        owner_id,
        user_id,
        AccessLevel::Read,
    ).await?;
    
    // Verify user can access the document
    let user_access = doc_service.get_document_metadata(metadata.id, user_id).await;
    assert!(user_access.is_ok());
    
    // Try to delete as user (should fail)
    let delete_result = doc_service.delete_document(metadata.id, user_id).await;
    assert!(delete_result.is_err());
    
    // Delete as owner (should succeed)
    let delete_result = doc_service.delete_document(metadata.id, owner_id).await;
    assert!(delete_result.is_ok());
    
    // Verify document is deleted
    let retrieved_metadata = doc_service.get_document_metadata(metadata.id, owner_id).await;
    assert!(retrieved_metadata.is_err());
    
    // Verify user can no longer access the document
    let user_access = doc_service.get_document_metadata(metadata.id, user_id).await;
    assert!(user_access.is_err());
    
    Ok(())
}