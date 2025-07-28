use crate::domain::models::{Document, DocumentVersion};
use crate::domain::value_objects::DocumentContent;
use crate::domain::errors::DocumentError;
use crate::infrastructure::repository::DocumentRepository;
use crate::collaboration::service::RealtimeCollaborationService;
use uuid::Uuid;
use std::sync::Arc;

pub struct CollaborationService {
    repository: Arc<dyn DocumentRepository>,
    realtime_service: Arc<RealtimeCollaborationService>,
}

impl CollaborationService {
    pub fn new(
        repository: Arc<dyn DocumentRepository>,
        realtime_service: Arc<RealtimeCollaborationService>,
    ) -> Self {
        CollaborationService { repository, realtime_service }
    }
    
    pub async fn create_version(
        &self,
        document_id: Uuid,
        user_id: Uuid,
        content: serde_json::Value,
    ) -> Result<DocumentVersion, DocumentError> {
        // For the new CRDT-based system, we delegate to the realtime service
        self.realtime_service.create_version(document_id, user_id)
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
    
    pub fn initialize_document(&self, document_id: Uuid, user_id: Uuid) -> Result<(), DocumentError> {
        self.realtime_service.initialize_document(document_id, user_id)
    }
    
    pub fn apply_operation(&self, document_id: Uuid, operation: crate::crdt::operations::DocumentOperation) -> Result<(), DocumentError> {
        self.realtime_service.apply_operation(document_id, operation)
    }
}