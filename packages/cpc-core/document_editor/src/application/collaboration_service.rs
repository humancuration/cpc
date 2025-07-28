use crate::domain::models::{Document, DocumentVersion};
use crate::domain::value_objects::DocumentContent;
use crate::domain::errors::DocumentError;
use crate::infrastructure::repository::DocumentRepository;
use uuid::Uuid;
use std::sync::Arc;

pub struct CollaborationService {
    repository: Arc<dyn DocumentRepository>,
}

impl CollaborationService {
    pub fn new(repository: Arc<dyn DocumentRepository>) -> Self {
        CollaborationService { repository }
    }
    
    pub async fn create_version(
        &self,
        document_id: Uuid,
        user_id: Uuid,
        content: serde_json::Value,
    ) -> Result<DocumentVersion, DocumentError> {
        // Get the current document
        let document = self.repository.get_document(document_id).await?;
        
        // Check if user has access to the document
        if document.owner_id != user_id {
            let share = self.repository.get_document_share(document_id, user_id).await?;
            if !share.permission_level.can_edit() {
                return Err(DocumentError::AccessDenied);
            }
        }
        
        // Get the current version number
        let current_version = self.repository.get_latest_version_number(document_id).await
            .unwrap_or(0);
        
        let document_content = DocumentContent::new(content);
        let version = DocumentVersion {
            id: Uuid::new_v4(),
            document_id,
            version_number: current_version + 1,
            content: document_content,
            created_at: chrono::Utc::now(),
            created_by: user_id,
        };
        
        self.repository.create_document_version(&version).await?;
        Ok(version)
    }
    
    pub async fn get_document_versions(
        &self,
        document_id: Uuid,
        user_id: Uuid,
    ) -> Result<Vec<DocumentVersion>, DocumentError> {
        // Get the document to verify access
        let document = self.repository.get_document(document_id).await?;
        
        // Check if user has access to the document
        if document.owner_id != user_id {
            let share = self.repository.get_document_share(document_id, user_id).await?;
            if !share.permission_level.can_comment() {
                return Err(DocumentError::AccessDenied);
            }
        }
        
        self.repository.get_document_versions(document_id).await
    }
}