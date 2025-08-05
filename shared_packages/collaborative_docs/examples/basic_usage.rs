//! Basic usage example for the collaborative documentation system

use collaborative_docs::{
    CollaborativeDocService,
    core::{DocProvider, DocumentContent, DocumentMetadata, DocumentPermission, AccessLevel},
    sled_store::SledDocStore,
    access::DocumentAccessChecker,
};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;
use tempfile::TempDir;
use chrono::Utc;

/// A simple in-memory document provider for demonstration
struct InMemoryDocProvider {
    metadata: std::sync::Mutex<std::collections::HashMap<Uuid, DocumentMetadata>>,
    content: std::sync::Mutex<std::collections::HashMap<Uuid, DocumentContent>>,
    permissions: std::sync::Mutex<std::collections::HashMap<Uuid, Vec<DocumentPermission>>>,
}

impl InMemoryDocProvider {
    fn new() -> Self {
        Self {
            metadata: std::sync::Mutex::new(std::collections::HashMap::new()),
            content: std::sync::Mutex::new(std::collections::HashMap::new()),
            permissions: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl DocProvider for InMemoryDocProvider {
    async fn store_metadata(&self, metadata: &DocumentMetadata) -> Result<(), collaborative_docs::core::DocumentError> {
        self.metadata.lock().unwrap().insert(metadata.id, metadata.clone());
        Ok(())
    }

    async fn retrieve_metadata(&self, document_id: Uuid) -> Result<DocumentMetadata, collaborative_docs::core::DocumentError> {
        self.metadata.lock().unwrap().get(&document_id)
            .cloned()
            .ok_or(collaborative_docs::core::DocumentError::DocumentNotFound(document_id))
    }

    async fn store_content(&self, document_id: Uuid, content: &DocumentContent) -> Result<(), collaborative_docs::core::DocumentError> {
        self.content.lock().unwrap().insert(document_id, content.clone());
        Ok(())
    }

    async fn retrieve_content(&self, document_id: Uuid) -> Result<DocumentContent, collaborative_docs::core::DocumentError> {
        self.content.lock().unwrap().get(&document_id)
            .cloned()
            .ok_or(collaborative_docs::core::DocumentError::DocumentNotFound(document_id))
    }

    async fn delete_document(&self, document_id: Uuid) -> Result<(), collaborative_docs::core::DocumentError> {
        self.metadata.lock().unwrap().remove(&document_id);
        self.content.lock().unwrap().remove(&document_id);
        self.permissions.lock().unwrap().remove(&document_id);
        Ok(())
    }

    async fn list_documents(&self, user_id: Uuid, limit: usize, offset: usize) -> Result<Vec<DocumentMetadata>, collaborative_docs::core::DocumentError> {
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

    async fn store_permission(&self, document_id: Uuid, permission: &DocumentPermission) -> Result<(), collaborative_docs::core::DocumentError> {
        let mut permissions_map = self.permissions.lock().unwrap();
        let permissions = permissions_map.entry(document_id).or_insert_with(Vec::new);
        
        // Remove existing permission for this user if it exists
        permissions.retain(|p| p.user_id != permission.user_id);
        
        // Add the new permission
        permissions.push(permission.clone());
        
        Ok(())
    }

    async fn retrieve_permissions(&self, document_id: Uuid) -> Result<Vec<DocumentPermission>, collaborative_docs::core::DocumentError> {
        let permissions_map = self.permissions.lock().unwrap();
        Ok(permissions_map.get(&document_id).cloned().unwrap_or_default())
    }

    async fn delete_permission(&self, document_id: Uuid, user_id: Uuid) -> Result<(), collaborative_docs::core::DocumentError> {
        let mut permissions_map = self.permissions.lock().unwrap();
        if let Some(permissions) = permissions_map.get_mut(&document_id) {
            permissions.retain(|p| p.user_id != user_id);
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Collaborative Documentation System - Basic Usage Example");
    println!("=====================================================");
    
    // Create an in-memory document provider for demonstration
    let provider = Arc::new(InMemoryDocProvider::new());
    
    // Create access checker
    let access_checker = DocumentAccessChecker::new(None);
    
    // Create document service
    let doc_service = Arc::new(CollaborativeDocService::new(
        provider as Arc<dyn DocProvider>,
        access_checker,
    ));
    
    // Create users
    let owner_id = Uuid::new_v4();
    let collaborator_id = Uuid::new_v4();
    
    println!("Created users:");
    println!("  Owner ID: {}", owner_id);
    println!("  Collaborator ID: {}", collaborator_id);
    
    // Create a document
    println!("\n1. Creating a new document...");
    let content = DocumentContent {
        data: json!({
            "title": "Welcome to Collaborative Docs",
            "content": "This is a collaborative document system built with Rust!",
            "sections": [
                {
                    "heading": "Introduction",
                    "text": "This system supports real-time collaboration."
                }
            ]
        }),
        format: "json".to_string(),
    };
    
    let metadata = doc_service.create_document(
        owner_id,
        "My First Document".to_string(),
        content,
        vec!["example".to_string(), "collaboration".to_string()],
    ).await?;
    
    println!("  Created document with ID: {}", metadata.id);
    println!("  Title: {}", metadata.title);
    println!("  Version: {}", metadata.version);
    
    // Retrieve document metadata
    println!("\n2. Retrieving document metadata...");
    let retrieved_metadata = doc_service.get_document_metadata(metadata.id, owner_id).await?;
    println!("  Retrieved title: {}", retrieved_metadata.title);
    println!("  Content type: {}", retrieved_metadata.content_type);
    println!("  Tags: {:?}", retrieved_metadata.tags);
    
    // Retrieve document content
    println!("\n3. Retrieving document content...");
    let retrieved_content = doc_service.get_document_content(metadata.id, owner_id).await?;
    println!("  Content format: {}", retrieved_content.format);
    if let Some(title) = retrieved_content.data.get("title") {
        println!("  Content title: {}", title);
    }
    
    // Update document
    println!("\n4. Updating document...");
    let updated_content = DocumentContent {
        data: json!({
            "title": "Welcome to Collaborative Docs",
            "content": "This is a collaborative document system built with Rust!",
            "sections": [
                {
                    "heading": "Introduction",
                    "text": "This system supports real-time collaboration."
                },
                {
                    "heading": "Features",
                    "text": "CRDT support, versioning, access control."
                }
            ]
        }),
        format: "json".to_string(),
    };
    
    let updated_metadata = doc_service.update_document(
        metadata.id,
        owner_id,
        updated_content,
    ).await?;
    
    println!("  Updated document version: {}", updated_metadata.version);
    
    // Grant access to collaborator
    println!("\n5. Granting access to collaborator...");
    doc_service.grant_access(
        metadata.id,
        owner_id,
        collaborator_id,
        AccessLevel::Write,
    ).await?;
    
    println!("  Granted write access to collaborator");
    
    // List documents for owner
    println!("\n6. Listing documents for owner...");
    let documents = doc_service.list_documents(owner_id, 10, 0).await?;
    println!("  Found {} documents", documents.len());
    for doc in documents {
        println!("  - {} (v{})", doc.title, doc.version);
    }
    
    // Get permissions
    println!("\n7. Retrieving document permissions...");
    let permissions = doc_service.get_permissions(metadata.id, owner_id).await?;
    println!("  Document has {} permissions", permissions.len());
    for perm in permissions {
        let level = match perm.access_level {
            AccessLevel::Read => "Read",
            AccessLevel::Write => "Write",
            AccessLevel::Admin => "Admin",
        };
        println!("  - User {} has {} access", perm.user_id, level);
    }
    
    println!("\nExample completed successfully!");
    
    Ok(())
}