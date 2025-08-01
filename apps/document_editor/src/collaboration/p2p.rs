// DEPRECATED - Refactored to use collaboration_engine
// This file has been deprecated as part of the refactor to use the collaboration_engine package.
// The new implementation can be found in the application/collaboration_service.rs file.
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use async_trait::async_trait;
use tokio::sync::broadcast;

use crate::crdt::{CRDTId, DocumentOperation, document::CRDTDocument};
use crate::domain::errors::DocumentError;

// Simplified P2P network interface
// In a real implementation, this would integrate with p2panda
pub struct P2PNetwork {
    node_id: Uuid,
    peers: HashMap<Uuid, String>, // peer_id -> address
    operation_sender: broadcast::Sender<DocumentOperation>,
    operation_receiver: broadcast::Receiver<DocumentOperation>,
}

impl P2PNetwork {
    pub fn new(node_id: Uuid) -> Self {
        let (sender, receiver) = broadcast::channel(100);
        Self {
            node_id,
            peers: HashMap::new(),
            operation_sender: sender,
            operation_receiver: receiver,
        }
    }
    
    pub fn add_peer(&mut self, peer_id: Uuid, address: String) {
        self.peers.insert(peer_id, address);
    }
    
    pub async fn broadcast_operation(&self, operation: DocumentOperation) -> Result<(), DocumentError> {
        self.operation_sender.send(operation)
            .map_err(|_| DocumentError::SerializationError(serde_json::Error::custom("Failed to broadcast operation")))?;
        Ok(())
    }
    
    pub fn subscribe_to_operations(&self) -> broadcast::Receiver<DocumentOperation> {
        self.operation_sender.subscribe()
    }
    
    pub fn get_node_id(&self) -> Uuid {
        self.node_id
    }
}

// P2P Document Sync Service
pub struct P2PSyncService {
    network: Arc<Mutex<P2PNetwork>>,
    documents: Arc<Mutex<HashMap<Uuid, CRDTDocument>>>, // document_id -> CRDTDocument
}

impl P2PSyncService {
    pub fn new(network: Arc<Mutex<P2PNetwork>>) -> Self {
        Self {
            network,
            documents: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub fn add_document(&self, document_id: Uuid, crdt_document: CRDTDocument) {
        let mut documents = self.documents.lock().unwrap();
        documents.insert(document_id, crdt_document);
    }
    
    pub fn apply_operation(&self, document_id: Uuid, operation: &DocumentOperation) -> Result<(), DocumentError> {
        let mut documents = self.documents.lock().unwrap();
        if let Some(document) = documents.get_mut(&document_id) {
            let network = self.network.lock().unwrap();
            let source_node = operation.id().map(|id| id.node_id).unwrap_or(network.get_node_id());
            document.apply_operation(operation, source_node);
            Ok(())
        } else {
            Err(DocumentError::DocumentNotFound(document_id.to_string()))
        }
    }
    
    pub fn get_document(&self, document_id: Uuid) -> Option<CRDTDocument> {
        let documents = self.documents.lock().unwrap();
        documents.get(&document_id).cloned()
    }
}