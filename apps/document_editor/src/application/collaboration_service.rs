use crate::domain::models::{Document, DocumentVersion};
use crate::domain::value_objects::DocumentContent;
use crate::domain::errors::DocumentError;
use crate::infrastructure::repository::DocumentRepository;
use crate::crdt::operations::DocumentOperation;
use crate::collaboration::sync::SyncManager;
use uuid::Uuid;
use std::sync::Arc;
use std::collections::HashMap;

pub struct CollaborationService {
    repository: Arc<dyn DocumentRepository>,
    sync_managers: HashMap<Uuid, Arc<SyncManager>>, // document_id -> SyncManager
}

impl CollaborationService {
    pub fn new(
        repository: Arc<dyn DocumentRepository>,
    ) -> Self {
        CollaborationService {
            repository,
            sync_managers: HashMap::new(),
        }
    }
    
    pub async fn create_version(
        &self,
        document_id: Uuid,
        user_id: Uuid,
        content: serde_json::Value,
    ) -> Result<DocumentVersion, DocumentError> {
        // Get the document to verify access
        let document = self.repository.get_document(document_id).await?;
        
        let version = DocumentVersion {
            id: Uuid::new_v4(),
            document_id,
            version_number: 1, // This should be properly incremented in a real implementation
            content: DocumentContent::new(content),
            created_at: chrono::Utc::now(),
            created_by: user_id,
        };
        
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
    
    pub fn initialize_document(&mut self, document_id: Uuid, user_id: Uuid) -> Result<(), DocumentError> {
        // Initialize sync manager for this document if it doesn't exist
        if !self.sync_managers.contains_key(&document_id) {
            let sync_manager = Arc::new(SyncManager::new(document_id));
            self.sync_managers.insert(document_id, sync_manager);
        }
        Ok(())
    }
    
    pub fn apply_operation(&mut self, document_id: Uuid, op: DocumentOperation) -> Result<(), DocumentError> {
        // Get or create sync manager for this document
        if !self.sync_managers.contains_key(&document_id) {
            self.initialize_document(document_id, Uuid::nil())?;
        }
        
        let sync_manager = self.sync_managers.get(&document_id)
            .ok_or(DocumentError::DocumentNotFound(document_id.to_string()))?;
        
        // Apply operation through sync manager
        sync_manager.apply_operation(op)
    }
    
    pub fn update_presence(&mut self, document_id: Uuid, user_id: Uuid, cursor: collaboration_engine::core::Position) -> Result<(), DocumentError> {
        // Get or create sync manager for this document
        if !self.sync_managers.contains_key(&document_id) {
            self.initialize_document(document_id, Uuid::nil())?;
        }
        
        let sync_manager = self.sync_managers.get(&document_id)
            .ok_or(DocumentError::DocumentNotFound(document_id.to_string()))?;
        
        // Update presence through sync manager
        sync_manager.update_presence(user_id, Some(cursor), None, false)
    }
    
    pub fn get_presences(&self, document_id: Uuid) -> Result<Vec<collaboration_engine::presence::UserPresence>, DocumentError> {
        let sync_manager = self.sync_managers.get(&document_id)
            .ok_or(DocumentError::DocumentNotFound(document_id.to_string()))?;
        
        Ok(sync_manager.get_presences())
    }
    
    pub fn set_user_priority(&mut self, document_id: Uuid, user_id: Uuid, priority: i32) -> Result<(), DocumentError> {
        // Get or create sync manager for this document
        if !self.sync_managers.contains_key(&document_id) {
            self.initialize_document(document_id, Uuid::nil())?;
        }
        
        let sync_manager = self.sync_managers.get(&document_id)
            .ok_or(DocumentError::DocumentNotFound(document_id.to_string()))?;
        
        // Set user priority through sync manager
        sync_manager.set_user_priority(user_id, priority);
        Ok(())
    }
}