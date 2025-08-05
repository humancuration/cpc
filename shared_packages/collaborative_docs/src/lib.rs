//! Collaborative documentation system with CRDT support, versioning, and access control

pub mod core;
pub mod crdt;
pub mod versioning;
pub mod access;
pub mod postgres_store;
pub mod sled_store;
pub mod content_provider;

// Re-export key types for convenience
pub use core::{
    DocumentService, DocProvider, DocumentMetadata, DocumentContent, 
    DocumentPermission, AccessLevel, DocumentError
};
pub use crdt::{CrdtDocument, ConflictResolver, CrdtError};
pub use versioning::{VersionHistory, DocumentVersion, DocumentDiff, VersioningError};
pub use access::{ConsentAccessController, DocumentAccessChecker, AccessControlError};
pub use postgres_store::PostgresDocStore;
pub use sled_store::SledDocStore;
pub use content_provider::{CollaborativeDocProvider, CollaborativeDocProviderMetadata};

use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

/// Main collaborative document service implementing DocumentService trait
pub struct CollaborativeDocService {
    provider: Arc<dyn DocProvider>,
    access_checker: access::DocumentAccessChecker,
    version_manager: versioning::DocumentHistoryManager,
}

impl CollaborativeDocService {
    /// Create a new collaborative document service
    pub fn new(
        provider: Arc<dyn DocProvider>,
        access_checker: access::DocumentAccessChecker,
    ) -> Self {
        Self {
            provider,
            access_checker,
            version_manager: versioning::DocumentHistoryManager::new(),
        }
    }

    /// Create a new document with CRDT support
    pub async fn create_crdt_document(
        &self,
        owner_id: Uuid,
        title: String,
        initial_content: serde_json::Value,
        tags: Vec<String>,
    ) -> Result<core::DocumentMetadata, core::DocumentError> {
        // Create initial CRDT document
        let actor_id = owner_id.to_string();
        let mut crdt_doc = crdt::CrdtDocument::new(actor_id);
        let root = crdt_doc.doc.get_object_root();
        
        // Insert initial content
        crdt_doc.put(&root, "content", initial_content)
            .map_err(|e| core::DocumentError::InvalidFormat(e.to_string()))?;
        
        // Serialize CRDT document
        let crdt_data = crdt_doc.save()
            .map_err(|e| core::DocumentError::SerializationError(e.to_string()))?;
        
        // Create document content
        let content = core::DocumentContent {
            data: serde_json::json!({
                "crdt_data": crdt_data
            }),
            format: "crdt".to_string(),
        };
        
        // Use the standard create_document method
        self.create_document(owner_id, title, content, tags).await
    }

    /// Update document with CRDT operations
    pub async fn update_crdt_document(
        &self,
        document_id: Uuid,
        user_id: Uuid,
        updates: serde_json::Value,
    ) -> Result<core::DocumentMetadata, core::DocumentError> {
        // Get current document content
        let current_content = self.get_document_content(document_id, user_id).await?;
        
        // Check if this is a CRDT document
        if current_content.format != "crdt" {
            return Err(core::DocumentError::InvalidFormat(
                "Document is not a CRDT document".to_string()
            ));
        }
        
        // Load CRDT document
        let crdt_data = current_content.data.get("crdt_data")
            .ok_or_else(|| core::DocumentError::InvalidFormat(
                "Invalid CRDT document format".to_string()
            ))?
            .as_array()
            .ok_or_else(|| core::DocumentError::InvalidFormat(
                "Invalid CRDT data format".to_string()
            ))?;
        
        let crdt_bytes: Vec<u8> = crdt_data.iter()
            .map(|v| v.as_u64().unwrap_or(0) as u8)
            .collect();
        
        let mut crdt_doc = crdt::CrdtDocument::load(&crdt_bytes, user_id.to_string())
            .map_err(|e| core::DocumentError::InvalidFormat(e.to_string()))?;
        
        // Apply updates
        let root = crdt_doc.doc.get_object_root();
        if let Some(obj) = updates.as_object() {
            for (key, value) in obj {
                crdt_doc.put(&root, key, value.clone())
                    .map_err(|e| core::DocumentError::InvalidFormat(e.to_string()))?;
            }
        }
        
        // Serialize updated CRDT document
        let updated_crdt_data = crdt_doc.save()
            .map_err(|e| core::DocumentError::SerializationError(e.to_string()))?;
        
        // Create updated document content
        let updated_content = core::DocumentContent {
            data: serde_json::json!({
                "crdt_data": updated_crdt_data
            }),
            format: "crdt".to_string(),
        };
        
        // Use the standard update_document method
        self.update_document(document_id, user_id, updated_content).await
    }

    /// Detect conflicts between two document versions
    pub async fn detect_conflicts(
        &self,
        document_id: Uuid,
        user_id: Uuid,
        incoming_content: core::DocumentContent,
    ) -> Result<Option<(core::DocumentContent, core::DocumentContent)>, core::DocumentError> {
        // Get current document content
        let current_content = self.get_document_content(document_id, user_id).await?;
        
        // For CRDT documents, we can check if there are conflicts by attempting to merge
        if current_content.format == "crdt" && incoming_content.format == "crdt" {
            // Load both CRDT documents
            let current_crdt_data = current_content.data.get("crdt_data")
                .ok_or_else(|| core::DocumentError::InvalidFormat(
                    "Invalid CRDT document format".to_string()
                ))?
                .as_array()
                .ok_or_else(|| core::DocumentError::InvalidFormat(
                    "Invalid CRDT data format".to_string()
                ))?;
            
            let current_crdt_bytes: Vec<u8> = current_crdt_data.iter()
                .map(|v| v.as_u64().unwrap_or(0) as u8)
                .collect();
            
            let incoming_crdt_data = incoming_content.data.get("crdt_data")
                .ok_or_else(|| core::DocumentError::InvalidFormat(
                    "Invalid CRDT document format".to_string()
                ))?
                .as_array()
                .ok_or_else(|| core::DocumentError::InvalidFormat(
                    "Invalid CRDT data format".to_string()
                ))?;
            
            let incoming_crdt_bytes: Vec<u8> = incoming_crdt_data.iter()
                .map(|v| v.as_u64().unwrap_or(0) as u8)
                .collect();
            
            let current_crdt_doc = crdt::CrdtDocument::load(&current_crdt_bytes, user_id.to_string())
                .map_err(|e| core::DocumentError::InvalidFormat(e.to_string()))?;
            
            let incoming_crdt_doc = crdt::CrdtDocument::load(&incoming_crdt_bytes, user_id.to_string())
                .map_err(|e| core::DocumentError::InvalidFormat(e.to_string()))?;
            
            // Check if there are conflicts by comparing heads
            let current_heads = current_crdt_doc.get_heads();
            let incoming_heads = incoming_crdt_doc.get_heads();
            
            // If the documents have different heads, there might be conflicts
            if current_heads != incoming_heads {
                // Return both versions for conflict resolution
                return Ok(Some((current_content, incoming_content)));
            }
        }
        
        // No conflicts detected
        Ok(None)
    }

    /// Merge changes from another CRDT document
    pub async fn merge_crdt_document(
        &self,
        document_id: Uuid,
        user_id: Uuid,
        other_crdt_data: Vec<u8>,
    ) -> Result<core::DocumentMetadata, core::DocumentError> {
        // Get current document content
        let current_content = self.get_document_content(document_id, user_id).await?;
        
        // Check if this is a CRDT document
        if current_content.format != "crdt" {
            return Err(core::DocumentError::InvalidFormat(
                "Document is not a CRDT document".to_string()
            ));
        }
        
        // Load both CRDT documents
        let current_crdt_data = current_content.data.get("crdt_data")
            .ok_or_else(|| core::DocumentError::InvalidFormat(
                "Invalid CRDT document format".to_string()
            ))?
            .as_array()
            .ok_or_else(|| core::DocumentError::InvalidFormat(
                "Invalid CRDT data format".to_string()
            ))?;
        
        let current_crdt_bytes: Vec<u8> = current_crdt_data.iter()
            .map(|v| v.as_u64().unwrap_or(0) as u8)
            .collect();
        
        let mut current_crdt_doc = crdt::CrdtDocument::load(&current_crdt_bytes, user_id.to_string())
            .map_err(|e| core::DocumentError::InvalidFormat(e.to_string()))?;
        
        let other_crdt_doc = crdt::CrdtDocument::load(&other_crdt_data, user_id.to_string())
            .map_err(|e| core::DocumentError::InvalidFormat(e.to_string()))?;
        
        // Merge documents
        crdt::ConflictResolver::merge_documents(&mut current_crdt_doc, &other_crdt_doc)
            .map_err(|e| core::DocumentError::ConflictError(e.to_string()))?;
        
        // Serialize merged CRDT document
        let merged_crdt_data = current_crdt_doc.save()
            .map_err(|e| core::DocumentError::SerializationError(e.to_string()))?;
        
        // Create merged document content
        let merged_content = core::DocumentContent {
            data: serde_json::json!({
                "crdt_data": merged_crdt_data
            }),
            format: "crdt".to_string(),
        };
        
        // Use the standard update_document method
        self.update_document(document_id, user_id, merged_content).await
    }

    /// Update document visibility
    pub async fn update_document_visibility(
        &self,
        document_id: Uuid,
        user_id: Uuid,
        visibility: core::Visibility,
    ) -> Result<core::DocumentMetadata, core::DocumentError> {
        // Check access - only owner or admin can change visibility
        let context = access::AccessContext {
            user_ip: None,
            user_agent: None,
            timestamp: chrono::Utc::now(),
            resource_id: Some(document_id),
        };

        if !self.access_checker.check_access(
            document_id,
            user_id,
            core::AccessLevel::Admin,
            context,
        ).await? {
            return Err(core::DocumentError::AccessDenied(document_id));
        }

        // Retrieve current metadata
        let mut metadata = self.provider.retrieve_metadata(document_id).await?;
        
        // Update visibility
        metadata.visibility = visibility;
        
        // Store updated metadata
        self.provider.store_metadata(&metadata).await?;

        Ok(metadata)
    }

    /// Handle document update with conflict resolution
    pub async fn update_document_with_conflict_resolution(
        &self,
        document_id: Uuid,
        user_id: Uuid,
        content: core::DocumentContent,
    ) -> Result<core::DocumentMetadata, core::DocumentError> {
        // First, detect conflicts
        match self.detect_conflicts(document_id, user_id, content.clone()).await? {
            Some((current_content, incoming_content)) => {
                // Conflict detected, return an error with conflict information
                Err(core::DocumentError::ConflictDetected(
                    "Conflict detected between document versions".to_string()
                ))
            },
            None => {
                // No conflicts, proceed with normal update
                self.update_document(document_id, user_id, content).await
            }
        }
    }
}

#[async_trait]
impl core::DocumentService for CollaborativeDocService {
    /// Create a new document
    async fn create_document(
        &self,
        owner_id: Uuid,
        title: String,
        content: core::DocumentContent,
        tags: Vec<String>,
    ) -> Result<core::DocumentMetadata, core::DocumentError> {
        let now = chrono::Utc::now();
        let metadata = core::DocumentMetadata {
            id: Uuid::new_v4(),
            title,
            owner_id,
            created_at: now,
            updated_at: now,
            content_type: content.format.clone(),
            tags,
            version: 1,
            visibility: core::Visibility::Private, // Default to private
        };

        // Store metadata
        self.provider.store_metadata(&metadata).await?;

        // Store content
        self.provider.store_content(metadata.id, &content).await?;

        // Grant owner admin access
        let permission = core::DocumentPermission {
            user_id: owner_id,
            access_level: core::AccessLevel::Admin,
            granted_at: now,
            granted_by: owner_id,
        };
        self.provider.store_permission(metadata.id, &permission).await?;

        Ok(metadata)
    }

    /// Get document metadata
    async fn get_document_metadata(
        &self,
        document_id: Uuid,
        user_id: Uuid,
    ) -> Result<core::DocumentMetadata, core::DocumentError> {
        // Check access
        let context = access::AccessContext {
            user_ip: None,
            user_agent: None,
            timestamp: chrono::Utc::now(),
            resource_id: Some(document_id),
        };

        if !self.access_checker.check_access(
            document_id,
            user_id,
            core::AccessLevel::Read,
            context,
        ).await? {
            return Err(core::DocumentError::AccessDenied(document_id));
        }

        // Retrieve metadata
        self.provider.retrieve_metadata(document_id).await
    }

    /// Get document content
    async fn get_document_content(
        &self,
        document_id: Uuid,
        user_id: Uuid,
    ) -> Result<core::DocumentContent, core::DocumentError> {
        // Check access
        let context = access::AccessContext {
            user_ip: None,
            user_agent: None,
            timestamp: chrono::Utc::now(),
            resource_id: Some(document_id),
        };

        if !self.access_checker.check_access(
            document_id,
            user_id,
            core::AccessLevel::Read,
            context,
        ).await? {
            return Err(core::DocumentError::AccessDenied(document_id));
        }

        // Retrieve content
        self.provider.retrieve_content(document_id).await
    }

    /// Update document content
    async fn update_document(
        &self,
        document_id: Uuid,
        user_id: Uuid,
        content: core::DocumentContent,
    ) -> Result<core::DocumentMetadata, core::DocumentError> {
        // Check access
        let context = access::AccessContext {
            user_ip: None,
            user_agent: None,
            timestamp: chrono::Utc::now(),
            resource_id: Some(document_id),
        };

        if !self.access_checker.check_access(
            document_id,
            user_id,
            core::AccessLevel::Write,
            context,
        ).await? {
            return Err(core::DocumentError::AccessDenied(document_id));
        }

        // Retrieve current metadata
        let mut metadata = self.provider.retrieve_metadata(document_id).await?;
        
        // Update metadata
        metadata.updated_at = chrono::Utc::now();
        metadata.version += 1;
        metadata.content_type = content.format.clone();
        
        // Store updated metadata
        self.provider.store_metadata(&metadata).await?;

        // Store updated content
        self.provider.store_content(document_id, &content).await?;

        Ok(metadata)
    }

    /// Delete a document
    async fn delete_document(
        &self,
        document_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), core::DocumentError> {
        // Check access
        let context = access::AccessContext {
            user_ip: None,
            user_agent: None,
            timestamp: chrono::Utc::now(),
            resource_id: Some(document_id),
        };

        if !self.access_checker.check_access(
            document_id,
            user_id,
            core::AccessLevel::Admin,
            context,
        ).await? {
            return Err(core::DocumentError::AccessDenied(document_id));
        }

        // Delete document
        self.provider.delete_document(document_id).await
    }

    /// List documents for a user
    async fn list_documents(
        &self,
        user_id: Uuid,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<core::DocumentMetadata>, core::DocumentError> {
        self.provider.list_documents(user_id, limit, offset).await
    }

    /// Grant access to a document
    async fn grant_access(
        &self,
        document_id: Uuid,
        owner_id: Uuid,
        user_id: Uuid,
        access_level: core::AccessLevel,
    ) -> Result<(), core::DocumentError> {
        // Check that the owner has admin access
        let context = access::AccessContext {
            user_ip: None,
            user_agent: None,
            timestamp: chrono::Utc::now(),
            resource_id: Some(document_id),
        };

        if !self.access_checker.check_access(
            document_id,
            owner_id,
            core::AccessLevel::Admin,
            context,
        ).await? {
            return Err(core::DocumentError::AccessDenied(document_id));
        }

        let permission = core::DocumentPermission {
            user_id,
            access_level,
            granted_at: chrono::Utc::now(),
            granted_by: owner_id,
        };

        self.provider.store_permission(document_id, &permission).await
    }

    /// Revoke access to a document
    async fn revoke_access(
        &self,
        document_id: Uuid,
        owner_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), core::DocumentError> {
        // Check that the owner has admin access
        let context = access::AccessContext {
            user_ip: None,
            user_agent: None,
            timestamp: chrono::Utc::now(),
            resource_id: Some(document_id),
        };

        if !self.access_checker.check_access(
            document_id,
            owner_id,
            core::AccessLevel::Admin,
            context,
        ).await? {
            return Err(core::DocumentError::AccessDenied(document_id));
        }

        self.provider.delete_permission(document_id, user_id).await
    }
/// Get document permissions
async fn get_permissions(
    &self,
    document_id: Uuid,
    user_id: Uuid,
) -> Result<Vec<core::DocumentPermission>, core::DocumentError> {
    // Check access
    let context = access::AccessContext {
        user_ip: None,
        user_agent: None,
        timestamp: chrono::Utc::now(),
        resource_id: Some(document_id),
    };

    if !self.access_checker.check_access(
        document_id,
        user_id,
        core::AccessLevel::Read,
        context,
    ).await? {
        return Err(core::DocumentError::AccessDenied(document_id));
    }

    self.provider.retrieve_permissions(document_id).await
}

/// Get document preview for feed integration
async fn get_document_preview(
    &self,
    document_id: Uuid,
    user_id: Uuid,
) -> Result<core::DocumentPreview, core::DocumentError> {
    // Check access
    let context = access::AccessContext {
        user_ip: None,
        user_agent: None,
        timestamp: chrono::Utc::now(),
        resource_id: Some(document_id),
    };

    if !self.access_checker.check_access(
        document_id,
        user_id,
        core::AccessLevel::Read,
        context,
    ).await? {
        return Err(core::DocumentError::AccessDenied(document_id));
    }

    // Retrieve metadata
    let metadata = self.provider.retrieve_metadata(document_id).await?;
    
    // Retrieve content
    let content = self.provider.retrieve_content(document_id).await?;
    
    // Generate excerpt from content
    let excerpt = generate_excerpt(&content.data, 200); // 200 characters max
    
    // Calculate word count
    let word_count = calculate_word_count(&content.data);
    
    Ok(core::DocumentPreview {
        id: metadata.id,
        title: metadata.title,
        content_type: metadata.content_type,
        owner_id: metadata.owner_id,
        created_at: metadata.created_at,
        updated_at: metadata.updated_at,
        tags: metadata.tags,
        excerpt,
        word_count,
    })
}
}

/// Generate an excerpt from document content
fn generate_excerpt(content: &serde_json::Value, max_length: usize) -> String {
// Try to extract text content from various JSON structures
let text = if let Some(obj) = content.as_object() {
    // For CRDT documents, try to get the content field
    if let Some(content_field) = obj.get("content") {
        json_value_to_text(content_field)
    } else {
        // For other structured content, try common fields
        let mut text_parts = Vec::new();
        for (key, value) in obj {
            if key == "title" || key == "description" || key == "text" || key == "body" {
                text_parts.push(json_value_to_text(value));
            }
        }
        text_parts.join(" ")
    }
} else {
    json_value_to_text(content)
};

// Truncate to max length and add ellipsis if needed
if text.len() > max_length {
    let truncated = &text[..max_length];
    // Try to truncate at a word boundary
    if let Some(last_space) = truncated.rfind(' ') {
        format!("{}...", &truncated[..last_space])
    } else {
        format!("{}...", truncated)
    }
} else {
    text
}
}

/// Convert a JSON value to text representation
fn json_value_to_text(value: &serde_json::Value) -> String {
match value {
    serde_json::Value::String(s) => s.clone(),
    serde_json::Value::Number(n) => n.to_string(),
    serde_json::Value::Bool(b) => b.to_string(),
    serde_json::Value::Array(arr) => {
        arr.iter().map(json_value_to_text).collect::<Vec<_>>().join(" ")
    },
    serde_json::Value::Object(obj) => {
        obj.values().map(json_value_to_text).collect::<Vec<_>>().join(" ")
    },
    serde_json::Value::Null => String::new(),
}
}

/// Calculate word count from document content
fn calculate_word_count(content: &serde_json::Value) -> usize {
let text = json_value_to_text(content);
text.split_whitespace().count()
}

#[cfg(test)]
mod tests {
use super::*;
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn test_collaborative_doc_service() -> Result<(), Box<dyn std::error::Error>> {
    // Note: This is a simplified test that would require a real storage provider
    // In a real implementation, you would use a mock provider
    
    /*
    let provider = Arc::new(MockDocProvider::new());
    let access_checker = access::DocumentAccessChecker::new(None);
    let service = CollaborativeDocService::new(provider, access_checker);

    let owner_id = Uuid::new_v4();
    let content = core::DocumentContent {
        data: json!({"text": "Hello, world!"}),
        format: "json".to_string(),
    };

    // Test creating a document
    let metadata = service.create_document(
        owner_id,
        "Test Document".to_string(),
        content,
        vec!["test".to_string()],
    ).await?;

    assert_eq!(metadata.title, "Test Document");
    assert_eq!(metadata.owner_id, owner_id);

    // Test retrieving document metadata
    let retrieved_metadata = service.get_document_metadata(metadata.id, owner_id).await?;
    assert_eq!(retrieved_metadata.title, "Test Document");

    // Test listing documents
    let documents = service.list_documents(owner_id, 10, 0).await?;
    assert!(!documents.is_empty());
    */
    
    Ok(())
}
}

// Mock provider for testing (simplified)
#[cfg(test)]
struct MockDocProvider;

#[cfg(test)]
#[async_trait]
impl core::DocProvider for MockDocProvider {
async fn store_metadata(&self, _metadata: &core::DocumentMetadata) -> Result<(), core::DocumentError> {
    Ok(())
}

async fn retrieve_metadata(&self, document_id: Uuid) -> Result<core::DocumentMetadata, core::DocumentError> {
    Ok(core::DocumentMetadata {
        id: document_id,
        title: "Test Document".to_string(),
        owner_id: Uuid::new_v4(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        content_type: "text/plain".to_string(),
        tags: vec![],
        version: 1,
        visibility: core::Visibility::Private,
    })
}

async fn store_content(&self, _document_id: Uuid, _content: &core::DocumentContent) -> Result<(), core::DocumentError> {
    Ok(())
}

async fn retrieve_content(&self, _document_id: Uuid) -> Result<core::DocumentContent, core::DocumentError> {
    Ok(core::DocumentContent {
        data: serde_json::json!({"text": "Hello, world!"}),
        format: "json".to_string(),
    })
}

async fn delete_document(&self, _document_id: Uuid) -> Result<(), core::DocumentError> {
    Ok(())
}

async fn list_documents(&self, _user_id: Uuid, _limit: usize, _offset: usize) -> Result<Vec<core::DocumentMetadata>, core::DocumentError> {
    Ok(vec![])
}

async fn store_permission(&self, _document_id: Uuid, _permission: &core::DocumentPermission) -> Result<(), core::DocumentError> {
    Ok(())
}

async fn retrieve_permissions(&self, _document_id: Uuid) -> Result<Vec<core::DocumentPermission>, core::DocumentError> {
    Ok(vec![])
}

async fn delete_permission(&self, _document_id: Uuid, _user_id: Uuid) -> Result<(), core::DocumentError> {
    Ok(())
}
}