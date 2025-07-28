use crate::domain::models::{Document, DocumentShare, DocumentVersion, PermissionLevel};
use crate::domain::value_objects::{DocumentTitle, DocumentContent};
use crate::domain::errors::DocumentError;
use crate::infrastructure::repository::DocumentRepository;
use uuid::Uuid;
use std::sync::Arc;

pub struct DocumentService {
    repository: Arc<dyn DocumentRepository>,
}

impl DocumentService {
    pub fn new(repository: Arc<dyn DocumentRepository>) -> Self {
        DocumentService { repository }
    }
    
    pub async fn create_document(
        &self,
        owner_id: Uuid,
        title: String,
        content: serde_json::Value,
    ) -> Result<Document, DocumentError> {
        let document_title = DocumentTitle::new(title)?;
        let document_content = DocumentContent::new(content);
        let document = Document::new(owner_id, document_title, document_content);
        
        self.repository.create_document(&document).await?;
        Ok(document)
    }
    
    pub async fn get_document(
        &self,
        document_id: Uuid,
        user_id: Uuid,
    ) -> Result<Document, DocumentError> {
        let document = self.repository.get_document(document_id).await?;
        
        // Check if user has access to the document
        if document.owner_id != user_id {
            let share = self.repository.get_document_share(document_id, user_id).await;
            if share.is_err() {
                return Err(DocumentError::AccessDenied);
            }
        }
        
        Ok(document)
    }
    
    pub async fn update_document_content(
        &self,
        document_id: Uuid,
        user_id: Uuid,
        content: serde_json::Value,
    ) -> Result<Document, DocumentError> {
        let mut document = self.get_document(document_id, user_id).await?;
        
        // Check permissions
        if document.owner_id != user_id {
            let share = self.repository.get_document_share(document_id, user_id).await?;
            if !share.permission_level.can_edit() {
                return Err(DocumentError::AccessDenied);
            }
        }
        
        let document_content = DocumentContent::new(content);
        document.update_content(document_content)?;
        
        self.repository.update_document(&document).await?;
        Ok(document)
    }
    
    pub async fn update_document_title(
        &self,
        document_id: Uuid,
        user_id: Uuid,
        title: String,
    ) -> Result<Document, DocumentError> {
        let mut document = self.get_document(document_id, user_id).await?;
        
        // Check permissions
        if document.owner_id != user_id {
            let share = self.repository.get_document_share(document_id, user_id).await?;
            if !share.permission_level.can_edit() {
                return Err(DocumentError::AccessDenied);
            }
        }
        
        let document_title = DocumentTitle::new(title)?;
        document.update_title(document_title)?;
        
        self.repository.update_document(&document).await?;
        Ok(document)
    }
    
    pub async fn delete_document(
        &self,
        document_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), DocumentError> {
        let document = self.get_document(document_id, user_id).await?;
        
        // Only owner can delete
        if document.owner_id != user_id {
            return Err(DocumentError::AccessDenied);
        }
        
        self.repository.delete_document(document_id).await?;
        Ok(())
    }
    
    pub async fn share_document(
        &self,
        document_id: Uuid,
        owner_id: Uuid,
        shared_with: Uuid,
        permission_level: String,
    ) -> Result<DocumentShare, DocumentError> {
        let document = self.get_document(document_id, owner_id).await?;
        
        // Only owner can share
        if document.owner_id != owner_id {
            return Err(DocumentError::AccessDenied);
        }
        
        let permission = PermissionLevel::from_str(&permission_level)?;
        let share = DocumentShare {
            id: Uuid::new_v4(),
            document_id,
            shared_with,
            permission_level: permission,
            created_at: chrono::Utc::now(),
            expires_at: None,
        };
        
        self.repository.create_document_share(&share).await?;
        Ok(share)
    }
    
    pub async fn get_documents_by_owner(
        &self,
        owner_id: Uuid,
    ) -> Result<Vec<Document>, DocumentError> {
        self.repository.get_documents_by_owner(owner_id).await
    }
}