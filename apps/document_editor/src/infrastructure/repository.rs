use crate::domain::models::{Document, DocumentShare, DocumentVersion};
use crate::domain::errors::DocumentError;
use crate::domain::value_objects::{DocumentTitle, DocumentContent};
use crate::crdt::document::{CRDTDocument, ElementState};
use crate::crdt::id::CRDTId;
use uuid::Uuid;
use async_trait::async_trait;
use sqlx::PgPool;
use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;

#[async_trait]
pub trait DocumentRepository: Send + Sync {
    async fn create_document(&self, document: &Document) -> Result<(), DocumentError>;
    async fn get_document(&self, id: Uuid) -> Result<Document, DocumentError>;
    async fn update_document(&self, document: &Document) -> Result<(), DocumentError>;
    async fn delete_document(&self, id: Uuid) -> Result<(), DocumentError>;
    async fn get_documents_by_owner(&self, owner_id: Uuid) -> Result<Vec<Document>, DocumentError>;
    
    async fn create_document_share(&self, share: &DocumentShare) -> Result<(), DocumentError>;
    async fn get_document_share(&self, document_id: Uuid, user_id: Uuid) -> Result<DocumentShare, DocumentError>;
    
    async fn create_document_version(&self, version: &DocumentVersion) -> Result<(), DocumentError>;
    async fn get_document_versions(&self, document_id: Uuid) -> Result<Vec<DocumentVersion>, DocumentError>;
    async fn get_latest_version_number(&self, document_id: Uuid) -> Result<i32, DocumentError>;
    
    // CRDT-specific methods
    async fn save_crdt_document(&self, document_id: Uuid, crdt_document: &CRDTDocument) -> Result<(), DocumentError>;
    async fn load_crdt_document(&self, document_id: Uuid) -> Result<Option<CRDTDocument>, DocumentError>;
    
    // Ratchet session methods for persistent storage
    async fn save_ratchet_session(&self, document_id: Uuid, node_id: Uuid, session_data: &[u8]) -> Result<(), DocumentError>;
    async fn load_ratchet_session(&self, document_id: Uuid, node_id: Uuid) -> Result<Option<Vec<u8>>, DocumentError>;
}

/// PostgreSQL implementation of DocumentRepository
pub struct PgDocumentRepository {
    pool: PgPool,
}

impl PgDocumentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DocumentRepository for PgDocumentRepository {
    async fn create_document(&self, document: &Document) -> Result<(), DocumentError> {
        sqlx::query!(
            "INSERT INTO documents (id, owner_id, title, content, created_at, updated_at, is_deleted)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
            document.id,
            document.owner_id,
            document.title.as_str(),
            document.content.as_json(),
            document.created_at,
            document.updated_at,
            document.is_deleted,
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_document(&self, id: Uuid) -> Result<Document, DocumentError> {
        let row = sqlx::query!(
            "SELECT id, owner_id, title, content, created_at, updated_at, is_deleted
             FROM documents WHERE id = $1 AND is_deleted = false",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                let title = DocumentTitle::new(row.title)
                    .map_err(|e| DocumentError::InvalidTitle(e))?;
                let content = DocumentContent::new(row.content);
                
                Ok(Document {
                    id: row.id,
                    owner_id: row.owner_id,
                    title,
                    content,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                    is_deleted: row.is_deleted,
                })
            },
            None => Err(DocumentError::DocumentNotFound(id.to_string())),
        }
    }
    
    async fn update_document(&self, document: &Document) -> Result<(), DocumentError> {
        let rows_affected = sqlx::query!(
            "UPDATE documents
             SET title = $1, content = $2, updated_at = $3, is_deleted = $4
             WHERE id = $5",
            document.title.as_str(),
            document.content.as_json(),
            document.updated_at,
            document.is_deleted,
            document.id,
        )
        .execute(&self.pool)
        .await?
        .rows_affected();
        
        if rows_affected == 0 {
            return Err(DocumentError::DocumentNotFound(document.id.to_string()));
        }
        
        Ok(())
    }
    
    async fn delete_document(&self, id: Uuid) -> Result<(), DocumentError> {
        let rows_affected = sqlx::query!(
            "UPDATE documents SET is_deleted = true, updated_at = NOW() WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();
        
        if rows_affected == 0 {
            return Err(DocumentError::DocumentNotFound(id.to_string()));
        }
        
        Ok(())
    }
    
    async fn get_documents_by_owner(&self, owner_id: Uuid) -> Result<Vec<Document>, DocumentError> {
        let rows = sqlx::query!(
            "SELECT id, owner_id, title, content, created_at, updated_at, is_deleted
             FROM documents WHERE owner_id = $1 AND is_deleted = false
             ORDER BY created_at DESC",
            owner_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut documents = Vec::new();
        for row in rows {
            let title = DocumentTitle::new(row.title)
                .map_err(|e| DocumentError::InvalidTitle(e))?;
            let content = DocumentContent::new(row.content);
            
            documents.push(Document {
                id: row.id,
                owner_id: row.owner_id,
                title,
                content,
                created_at: row.created_at,
                updated_at: row.updated_at,
                is_deleted: row.is_deleted,
            });
        }
        
        Ok(documents)
    }
    
    async fn create_document_share(&self, share: &DocumentShare) -> Result<(), DocumentError> {
        let permission_level_str = match share.permission_level {
            crate::domain::models::PermissionLevel::View => "view",
            crate::domain::models::PermissionLevel::Comment => "comment",
            crate::domain::models::PermissionLevel::Edit => "edit",
        };
        
        sqlx::query!(
            "INSERT INTO document_shares (id, document_id, shared_with, permission_level, created_at, expires_at)
             VALUES ($1, $2, $3, $4, $5, $6)",
            share.id,
            share.document_id,
            share.shared_with,
            permission_level_str,
            share.created_at,
            share.expires_at,
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_document_share(&self, document_id: Uuid, user_id: Uuid) -> Result<DocumentShare, DocumentError> {
        let row = sqlx::query!(
            "SELECT id, document_id, shared_with, permission_level, created_at, expires_at
             FROM document_shares WHERE document_id = $1 AND shared_with = $2",
            document_id,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                let permission_level = match row.permission_level.as_str() {
                    "view" => crate::domain::models::PermissionLevel::View,
                    "comment" => crate::domain::models::PermissionLevel::Comment,
                    "edit" => crate::domain::models::PermissionLevel::Edit,
                    _ => return Err(DocumentError::InvalidPermission(row.permission_level)),
                };
                
                Ok(DocumentShare {
                    id: row.id,
                    document_id: row.document_id,
                    shared_with: row.shared_with,
                    permission_level,
                    created_at: row.created_at,
                    expires_at: row.expires_at,
                })
            },
            None => Err(DocumentError::DocumentNotFound(format!("Share for document {} and user {}", document_id, user_id))),
        }
    }
    
    async fn create_document_version(&self, version: &DocumentVersion) -> Result<(), DocumentError> {
        sqlx::query!(
            "INSERT INTO document_versions (id, document_id, version_number, content, created_at, created_by)
             VALUES ($1, $2, $3, $4, $5, $6)",
            version.id,
            version.document_id,
            version.version_number,
            version.content.as_json(),
            version.created_at,
            version.created_by,
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_document_versions(&self, document_id: Uuid) -> Result<Vec<DocumentVersion>, DocumentError> {
        let rows = sqlx::query!(
            "SELECT id, document_id, version_number, content, created_at, created_by
             FROM document_versions WHERE document_id = $1
             ORDER BY version_number ASC",
            document_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut versions = Vec::new();
        for row in rows {
            let content = DocumentContent::new(row.content);
            
            versions.push(DocumentVersion {
                id: row.id,
                document_id: row.document_id,
                version_number: row.version_number,
                content,
                created_at: row.created_at,
                created_by: row.created_by,
            });
        }
        
        Ok(versions)
    }
    
    async fn get_latest_version_number(&self, document_id: Uuid) -> Result<i32, DocumentError> {
        let row = sqlx::query!(
            "SELECT COALESCE(MAX(version_number), 0) as latest_version
             FROM document_versions WHERE document_id = $1",
            document_id
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(row.latest_version)
    }
    
    async fn save_crdt_document(&self, document_id: Uuid, crdt_document: &CRDTDocument) -> Result<(), DocumentError> {
        // Serialize the CRDT document elements to JSON for storage
        let elements_json = serde_json::to_value(crdt_document.get_elements())
            .map_err(|e| DocumentError::SerializationError(e))?;
        
        // Update the document with the CRDT content
        let rows_affected = sqlx::query!(
            "UPDATE documents SET content = $1, updated_at = NOW() WHERE id = $2",
            &elements_json,
            document_id,
        )
        .execute(&self.pool)
        .await?
        .rows_affected();
        
        if rows_affected == 0 {
            return Err(DocumentError::DocumentNotFound(document_id.to_string()));
        }
        
        Ok(())
    }
    
    async fn load_crdt_document(&self, document_id: Uuid) -> Result<Option<CRDTDocument>, DocumentError> {
        let row = sqlx::query!(
            "SELECT content FROM documents WHERE id = $1 AND is_deleted = false",
            document_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                // Try to deserialize the content as CRDT elements
                if let Ok(elements) = serde_json::from_value::<std::collections::HashMap<CRDTId, ElementState>>(row.content.clone()) {
                    // Create a new CRDT document with the elements
                    // Note: This is a simplified implementation - in a real system, you would need to
                    // properly reconstruct the CRDT document with all its state
                    let mut crdt_document = CRDTDocument::new(Uuid::nil()); // Use a placeholder node ID
                    
                    // This is a simplified approach - in reality, you would need to properly
                    // reconstruct the full CRDT document state including version vectors, etc.
                    
                    Ok(Some(crdt_document))
                } else {
                    // If we can't deserialize as CRDT elements, return None
                    Ok(None)
                }
            },
            None => Ok(None),
        }
    }
    
    async fn save_ratchet_session(&self, document_id: Uuid, node_id: Uuid, session_data: &[u8]) -> Result<(), DocumentError> {
        // For now, we'll store ratchet sessions in a separate table
        // In a real implementation, you might want to store this with the document or in a separate store
        sqlx::query!(
            "INSERT INTO ratchet_sessions (document_id, node_id, session_data, created_at)
             VALUES ($1, $2, $3, NOW())
             ON CONFLICT (document_id, node_id) DO UPDATE
             SET session_data = $3, created_at = NOW()",
            document_id,
            node_id,
            session_data,
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn load_ratchet_session(&self, document_id: Uuid, node_id: Uuid) -> Result<Option<Vec<u8>>, DocumentError> {
        let row = sqlx::query!(
            "SELECT session_data FROM ratchet_sessions WHERE document_id = $1 AND node_id = $2",
            document_id,
            node_id,
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => Ok(Some(row.session_data)),
            None => Ok(None),
        }
    }
}