//! Sled storage adapter for collaborative documents (local storage)

use crate::core::{DocProvider, DocumentContent, DocumentError, DocumentMetadata, DocumentPermission};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sled::Db;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Sled storage adapter for documents
pub struct SledDocStore {
    db: Arc<Db>,
}

/// Serializable document metadata for Sled storage
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SledDocumentMetadata {
    pub id: String,
    pub title: String,
    pub owner_id: String,
    pub created_at: String, // ISO 8601 string
    pub updated_at: String, // ISO 8601 string
    pub content_type: String,
    pub tags: Vec<String>,
    pub version: u64,
    pub visibility: String, // "public", "friends_only", "private"
}

/// Serializable document content for Sled storage
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SledDocumentContent {
    pub document_id: String,
    pub data: serde_json::Value,
    pub format: String,
}

/// Serializable document permission for Sled storage
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SledDocumentPermission {
    pub id: String,
    pub document_id: String,
    pub user_id: String,
    pub access_level: String, // "read", "write", "admin"
    pub granted_at: String,   // ISO 8601 string
    pub granted_by: String,
}

impl SledDocStore {
    /// Create a new Sled document store
    pub fn new(db: Db) -> Self {
        Self {
            db: Arc::new(db),
        }
    }

    /// Create a new in-memory Sled document store for testing
    pub fn new_in_memory() -> Result<Self, DocumentError> {
        let db = sled::Config::new()
            .temporary(true)
            .open()
            .map_err(|e| DocumentError::StorageError(e.to_string()))?;
        
        let store = Self::new(db);
        store.init_trees()?;
        Ok(store)
    }

    /// Initialize the database trees
    pub fn init_trees(&self) -> Result<(), DocumentError> {
        self.db.open_tree("metadata")
            .map_err(|e| DocumentError::StorageError(e.to_string()))?;
        self.db.open_tree("content")
            .map_err(|e| DocumentError::StorageError(e.to_string()))?;
        self.db.open_tree("permissions")
            .map_err(|e| DocumentError::StorageError(e.to_string()))?;
        Ok(())
    }

    /// Convert DocumentMetadata to SledDocumentMetadata
    fn to_sled_metadata(metadata: &DocumentMetadata) -> SledDocumentMetadata {
        SledDocumentMetadata {
            id: metadata.id.to_string(),
            title: metadata.title.clone(),
            owner_id: metadata.owner_id.to_string(),
            created_at: metadata.created_at.to_rfc3339(),
            updated_at: metadata.updated_at.to_rfc3339(),
            content_type: metadata.content_type.clone(),
            tags: metadata.tags.clone(),
            version: metadata.version,
            visibility: match &metadata.visibility {
                crate::core::Visibility::Public => "public".to_string(),
                crate::core::Visibility::FriendsOnly => "friends_only".to_string(),
                crate::core::Visibility::Private => "private".to_string(),
            },
        }
    }

    /// Convert SledDocumentMetadata to DocumentMetadata
    fn from_sled_metadata(sled_metadata: SledDocumentMetadata) -> Result<DocumentMetadata, DocumentError> {
        Ok(DocumentMetadata {
            id: Uuid::parse_str(&sled_metadata.id)
                .map_err(|e| DocumentError::InvalidFormat(e.to_string()))?,
            title: sled_metadata.title,
            owner_id: Uuid::parse_str(&sled_metadata.owner_id)
                .map_err(|e| DocumentError::InvalidFormat(e.to_string()))?,
            created_at: DateTime::parse_from_rfc3339(&sled_metadata.created_at)
                .map_err(|e| DocumentError::InvalidFormat(e.to_string()))?
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&sled_metadata.updated_at)
                .map_err(|e| DocumentError::InvalidFormat(e.to_string()))?
                .with_timezone(&Utc),
            content_type: sled_metadata.content_type,
            tags: sled_metadata.tags,
            version: sled_metadata.version,
            visibility: match sled_metadata.visibility.as_str() {
                "public" => crate::core::Visibility::Public,
                "friends_only" => crate::core::Visibility::FriendsOnly,
                "private" => crate::core::Visibility::Private,
                _ => crate::core::Visibility::Private, // Default to private for unknown values
            },
        })
    }

    /// Convert DocumentContent to SledDocumentContent
    fn to_sled_content(document_id: Uuid, content: &DocumentContent) -> SledDocumentContent {
        SledDocumentContent {
            document_id: document_id.to_string(),
            data: content.data.clone(),
            format: content.format.clone(),
        }
    }

    /// Convert SledDocumentContent to DocumentContent
    fn from_sled_content(sled_content: SledDocumentContent) -> Result<DocumentContent, DocumentError> {
        Ok(DocumentContent {
            data: sled_content.data,
            format: sled_content.format,
        })
    }

    /// Convert DocumentPermission to SledDocumentPermission
    fn to_sled_permission(document_id: Uuid, permission: &DocumentPermission) -> SledDocumentPermission {
        SledDocumentPermission {
            id: Uuid::new_v4().to_string(),
            document_id: document_id.to_string(),
            user_id: permission.user_id.to_string(),
            access_level: match &permission.access_level {
                crate::core::AccessLevel::Read => "read".to_string(),
                crate::core::AccessLevel::Write => "write".to_string(),
                crate::core::AccessLevel::Admin => "admin".to_string(),
            },
            granted_at: permission.granted_at.to_rfc3339(),
            granted_by: permission.granted_by.to_string(),
        }
    }

    /// Convert SledDocumentPermission to DocumentPermission
    fn from_sled_permission(sled_permission: SledDocumentPermission) -> Result<DocumentPermission, DocumentError> {
        let access_level = match sled_permission.access_level.as_str() {
            "read" => crate::core::AccessLevel::Read,
            "write" => crate::core::AccessLevel::Write,
            "admin" => crate::core::AccessLevel::Admin,
            _ => return Err(DocumentError::InvalidFormat(format!("Invalid access level: {}", sled_permission.access_level))),
        };

        Ok(DocumentPermission {
            user_id: Uuid::parse_str(&sled_permission.user_id)
                .map_err(|e| DocumentError::InvalidFormat(e.to_string()))?,
            access_level,
            granted_at: DateTime::parse_from_rfc3339(&sled_permission.granted_at)
                .map_err(|e| DocumentError::InvalidFormat(e.to_string()))?
                .with_timezone(&Utc),
            granted_by: Uuid::parse_str(&sled_permission.granted_by)
                .map_err(|e| DocumentError::InvalidFormat(e.to_string()))?,
        })
    }
}

#[async_trait]
impl DocProvider for SledDocStore {
    /// Store document metadata
    async fn store_metadata(
        &self,
        metadata: &DocumentMetadata,
    ) -> Result<(), DocumentError> {
        let sled_metadata = Self::to_sled_metadata(metadata);
        let serialized = serde_json::to_vec(&sled_metadata)
            .map_err(|e| DocumentError::SerializationError(e.to_string()))?;

        let tree = self.db.open_tree("metadata")
            .map_err(|e| DocumentError::StorageError(e.to_string()))?;
        
        tree.insert(metadata.id.to_string(), serialized)
            .map_err(|e| DocumentError::StorageError(e.to_string()))?;

        Ok(())
    }

    /// Retrieve document metadata
    async fn retrieve_metadata(
        &self,
        document_id: Uuid,
    ) -> Result<DocumentMetadata, DocumentError> {
        let tree = self.db.open_tree("metadata")
            .map_err(|e| DocumentError::StorageError(e.to_string()))?;

        let key = document_id.to_string();
        let value = tree.get(&key)
            .map_err(|e| DocumentError::StorageError(e.to_string()))?
            .ok_or(DocumentError::DocumentNotFound(document_id))?;

        let sled_metadata: SledDocumentMetadata = serde_json::from_slice(&value)
            .map_err(|e| DocumentError::SerializationError(e.to_string()))?;

        Self::from_sled_metadata(sled_metadata)
    }

    /// Store document content
    async fn store_content(
        &self,
        document_id: Uuid,
        content: &DocumentContent,
    ) -> Result<(), DocumentError> {
        let sled_content = Self::to_sled_content(document_id, content);
        let serialized = serde_json::to_vec(&sled_content)
            .map_err(|e| DocumentError::SerializationError(e.to_string()))?;

        let tree = self.db.open_tree("content")
            .map_err(|e| DocumentError::StorageError(e.to_string()))?;

        tree.insert(document_id.to_string(), serialized)
            .map_err(|e| DocumentError::StorageError(e.to_string()))?;

        Ok(())
    }

    /// Retrieve document content
    async fn retrieve_content(
        &self,
        document_id: Uuid,
    ) -> Result<DocumentContent, DocumentError> {
        let tree = self.db.open_tree("content")
            .map_err(|e| DocumentError::StorageError(e.to_string()))?;

        let key = document_id.to_string();
        let value = tree.get(&key)
            .map_err(|e| DocumentError::StorageError(e.to_string()))?
            .ok_or(DocumentError::DocumentNotFound(document_id))?;

        let sled_content: SledDocumentContent = serde_json::from_slice(&value)
            .map_err(|e| DocumentError::SerializationError(e.to_string()))?;

        Self::from_sled_content(sled_content)
    }

    /// Delete document
    async fn delete_document(
        &self,
        document_id: Uuid,
    ) -> Result<(), DocumentError> {
        let doc_id_str = document_id.to_string();

        // Delete content
        let content_tree = self.db.open_tree("content")
            .map_err(|e| DocumentError::StorageError(e.to_string()))?;
        content_tree.remove(&doc_id_str)
            .map_err(|e| DocumentError::StorageError(e.to_string()))?;

        // Delete permissions
        let permissions_tree = self.db.open_tree("permissions")
            .map_err(|e| DocumentError::StorageError(e.to_string()))?;
        
        // We need to find all permissions for this document
        let mut permissions_to_remove = Vec::new();
        for result in permissions_tree.iter() {
            let (key, value) = result
                .map_err(|e| DocumentError::StorageError(e.to_string()))?;
            
            let key_str = String::from_utf8(key.to_vec())
                .map_err(|e| DocumentError::SerializationError(e.to_string()))?;
            
            let sled_permission: SledDocumentPermission = serde_json::from_slice(&value)
                .map_err(|e| DocumentError::SerializationError(e.to_string()))?;
            
            if sled_permission.document_id == doc_id_str {
                permissions_to_remove.push(key_str);
            }
        }

        for permission_key in permissions_to_remove {
            permissions_tree.remove(&permission_key)
                .map_err(|e| DocumentError::StorageError(e.to_string()))?;
        }

        // Delete metadata
        let metadata_tree = self.db.open_tree("metadata")
            .map_err(|e| DocumentError::StorageError(e.to_string()))?;
        metadata_tree.remove(&doc_id_str)
            .map_err(|e| DocumentError::StorageError(e.to_string()))?;

        Ok(())
    }

    /// List documents for a user
    async fn list_documents(
        &self,
        user_id: Uuid,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<DocumentMetadata>, DocumentError> {
        let tree = self.db.open_tree("metadata")
            .map_err(|e| DocumentError::StorageError(e.to_string()))?;

        let user_id_str = user_id.to_string();
        let mut documents = Vec::new();
        let mut count = 0;
        let mut skip = 0;

        for result in tree.iter() {
            let (_, value) = result
                .map_err(|e| DocumentError::StorageError(e.to_string()))?;

            let sled_metadata: SledDocumentMetadata = serde_json::from_slice(&value)
                .map_err(|e| DocumentError::SerializationError(e.to_string()))?;

            // Check if this document belongs to the user
            if sled_metadata.owner_id == user_id_str {
                if skip < offset {
                    skip += 1;
                    continue;
                }

                if count >= limit {
                    break;
                }

                documents.push(Self::from_sled_metadata(sled_metadata)?);
                count += 1;
            }
        }

        Ok(documents)
    }

    /// Store document permission
    async fn store_permission(
        &self,
        document_id: Uuid,
        permission: &DocumentPermission,
    ) -> Result<(), DocumentError> {
        let sled_permission = Self::to_sled_permission(document_id, permission);
        let serialized = serde_json::to_vec(&sled_permission)
            .map_err(|e| DocumentError::SerializationError(e.to_string()))?;

        let tree = self.db.open_tree("permissions")
            .map_err(|e| DocumentError::StorageError(e.to_string()))?;

        // Use a composite key: document_id + user_id
        let key = format!("{}_{}", document_id, permission.user_id);
        tree.insert(key, serialized)
            .map_err(|e| DocumentError::StorageError(e.to_string()))?;

        Ok(())
    }

    /// Retrieve document permissions
    async fn retrieve_permissions(
        &self,
        document_id: Uuid,
    ) -> Result<Vec<DocumentPermission>, DocumentError> {
        let tree = self.db.open_tree("permissions")
            .map_err(|e| DocumentError::StorageError(e.to_string()))?;

        let doc_id_str = document_id.to_string();
        let mut permissions = Vec::new();

        for result in tree.iter() {
            let (_, value) = result
                .map_err(|e| DocumentError::StorageError(e.to_string()))?;

            let sled_permission: SledDocumentPermission = serde_json::from_slice(&value)
                .map_err(|e| DocumentError::SerializationError(e.to_string()))?;

            // Check if this permission is for the requested document
            if sled_permission.document_id == doc_id_str {
                permissions.push(Self::from_sled_permission(sled_permission)?);
            }
        }

        Ok(permissions)
    }

    /// Delete document permission
    async fn delete_permission(
        &self,
        document_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), DocumentError> {
        let tree = self.db.open_tree("permissions")
            .map_err(|e| DocumentError::StorageError(e.to_string()))?;

        let key = format!("{}_{}", document_id, user_id);
        tree.remove(&key)
            .map_err(|e| DocumentError::StorageError(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::TempDir;

    #[test]
    fn test_sled_store() -> Result<(), Box<dyn std::error::Error>> {
        // Create a temporary directory for the database
        let temp_dir = TempDir::new()?;
        let db = sled::open(temp_dir.path().join("test_db"))?;

        let store = SledDocStore::new(db);
        store.init_trees()?;

        let document_id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();

        let metadata = DocumentMetadata {
            id: document_id,
            title: "Test Document".to_string(),
            owner_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            content_type: "text/plain".to_string(),
            tags: vec!["test".to_string()],
            version: 1,
            visibility: crate::core::Visibility::Private,
        };

        // Test storing metadata
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(store.store_metadata(&metadata))?;

        // Test retrieving metadata
        let retrieved_metadata = rt.block_on(store.retrieve_metadata(document_id))?;
        assert_eq!(retrieved_metadata.title, "Test Document");

        let content = DocumentContent {
            data: json!({"text": "Hello, world!"}),
            format: "json".to_string(),
        };

        // Test storing content
        rt.block_on(store.store_content(document_id, &content))?;

        // Test retrieving content
        let retrieved_content = rt.block_on(store.retrieve_content(document_id))?;
        assert_eq!(retrieved_content.format, "json");

        // Test listing documents
        let documents = rt.block_on(store.list_documents(owner_id, 10, 0))?;
        assert!(!documents.is_empty());

        // Test permissions
        let permission = DocumentPermission {
            user_id: Uuid::new_v4(),
            access_level: crate::core::AccessLevel::Read,
            granted_at: Utc::now(),
            granted_by: owner_id,
        };

        // Test storing permission
        rt.block_on(store.store_permission(document_id, &permission))?;

        // Test retrieving permissions
        let permissions = rt.block_on(store.retrieve_permissions(document_id))?;
        assert_eq!(permissions.len(), 1);

        // Clean up
        rt.block_on(store.delete_document(document_id))?;

        Ok(())
    }
}