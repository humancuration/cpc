use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use tokio::sync::broadcast;
use async_trait::async_trait;

use crate::crdt::{CRDTId, DocumentOperation, document::{CRDTDocument, ElementState}};
use crate::domain::models::{Document, DocumentVersion};
use crate::domain::value_objects::DocumentContent;
use crate::domain::errors::DocumentError;
use crate::infrastructure::repository::DocumentRepository;
use crate::collaboration::p2p::{P2PNetwork, P2PSyncService};

pub struct RealtimeCollaborationService {
    repository: Arc<dyn DocumentRepository>,
    p2p_network: Arc<Mutex<P2PNetwork>>,
    p2p_sync_service: Arc<P2PSyncService>,
    documents: Arc<Mutex<HashMap<Uuid, CRDTDocument>>>, // document_id -> CRDTDocument
    operation_broadcasters: Arc<Mutex<HashMap<Uuid, broadcast::Sender<DocumentOperation>>>>, // document_id -> broadcaster
}

impl RealtimeCollaborationService {
    pub fn new(
        repository: Arc<dyn DocumentRepository>,
        p2p_network: Arc<Mutex<P2PNetwork>>,
        p2p_sync_service: Arc<P2PSyncService>,
    ) -> Self {
        Self {
            repository,
            p2p_network,
            p2p_sync_service,
            documents: Arc::new(Mutex::new(HashMap::new())),
            operation_broadcasters: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub fn initialize_document(&self, document_id: Uuid, user_id: Uuid) -> Result<(), DocumentError> {
        // Get the current document from the repository
        let document = tokio::runtime::Handle::current().block_on(
            self.repository.get_document(document_id)
        )?;
        
        // Create a new CRDT document
        let mut crdt_document = CRDTDocument::new(user_id);
        
        // Convert the document content to CRDT operations
        // This is a simplified implementation - in a real system, you would need to
        // properly convert the document structure to CRDT operations
        let content_json = document.content.as_json();
        if let serde_json::Value::Object(content_map) = content_json {
            for (key, value) in content_map {
                let id = crdt_document.generate_id();
                let operation = DocumentOperation::Insert {
                    position: 0, // Simplified - in a real implementation, position would be calculated
                    value: value.clone(),
                    id: id.clone(),
                    parent_id: None,
                };
                crdt_document.apply_operation(&operation, user_id);
            }
        }
        
        // Store the CRDT document
        {
            let mut documents = self.documents.lock().unwrap();
            documents.insert(document_id, crdt_document);
        }
        
        // Initialize broadcaster for this document
        {
            let (sender, _) = broadcast::channel(100);
            let mut broadcasters = self.operation_broadcasters.lock().unwrap();
            broadcasters.insert(document_id, sender);
        }
        
        // Add document to P2P sync service
        if let Some(crdt_document) = {
            let documents = self.documents.lock().unwrap();
            documents.get(&document_id).cloned()
        } {
            self.p2p_sync_service.add_document(document_id, crdt_document);
        }
        
        Ok(())
    }
    
    pub fn apply_operation(&self, document_id: Uuid, operation: DocumentOperation) -> Result<(), DocumentError> {
        // Apply operation to local CRDT document
        self.p2p_sync_service.apply_operation(document_id, &operation)?;
        
        // Broadcast operation to other clients
        if let Some(broadcaster) = {
            let broadcasters = self.operation_broadcasters.lock().unwrap();
            broadcasters.get(&document_id).cloned()
        } {
            let _ = broadcaster.send(operation.clone());
        }
        
        // Broadcast operation to P2P network
        {
            let network = self.p2p_network.lock().unwrap();
            tokio::runtime::Handle::current().block_on(
                network.broadcast_operation(operation)
            )?;
        }
        
        Ok(())
    }
    
    pub fn subscribe_to_operations(&self, document_id: Uuid) -> Result<broadcast::Receiver<DocumentOperation>, DocumentError> {
        let broadcasters = self.operation_broadcasters.lock().unwrap();
        if let Some(sender) = broadcasters.get(&document_id) {
            Ok(sender.subscribe())
        } else {
            Err(DocumentError::DocumentNotFound(document_id.to_string()))
        }
    }
    
    pub fn get_document_content(&self, document_id: Uuid) -> Result<DocumentContent, DocumentError> {
        let documents = self.documents.lock().unwrap();
        if let Some(crdt_document) = documents.get(&document_id) {
            Ok(crdt_document.to_document_content())
        } else {
            Err(DocumentError::DocumentNotFound(document_id.to_string()))
        }
    }
    
    pub fn create_version(
        &self,
        document_id: Uuid,
        user_id: Uuid,
    ) -> Result<DocumentVersion, DocumentError> {
        // Get the current document
        let document = tokio::runtime::Handle::current().block_on(
            self.repository.get_document(document_id)
        )?;
        
        // Check if user has access to the document
        if document.owner_id != user_id {
            let share = tokio::runtime::Handle::current().block_on(
                self.repository.get_document_share(document_id, user_id)
            )?;
            if !share.permission_level.can_edit() {
                return Err(DocumentError::AccessDenied);
            }
        }
        
        // Get the current version number
        let current_version = tokio::runtime::Handle::current().block_on(
            self.repository.get_latest_version_number(document_id)
        ).unwrap_or(0);
        
        // Get the current CRDT document content
        let document_content = self.get_document_content(document_id)?;
        
        let version = DocumentVersion {
            id: Uuid::new_v4(),
            document_id,
            version_number: current_version + 1,
            content: document_content,
            created_at: chrono::Utc::now(),
            created_by: user_id,
        };
        
        tokio::runtime::Handle::current().block_on(
            self.repository.create_document_version(&version)
        )?;
        
        Ok(version)
    }
}