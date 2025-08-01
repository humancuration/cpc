// DEPRECATED - Refactored to use collaboration_engine
// This file has been deprecated as part of the refactor to use the collaboration_engine package.
// The new implementation can be found in the application/collaboration_service.rs file.
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;
use uuid::Uuid;
use async_trait::async_trait;
use tokio::sync::broadcast;
use p2panda_core::{Node, Keypair, Signature, Hash};
use p2panda_net::Network;
use p2panda_store::Store;
use double_ratchet::ratchet::{Ratchet, RatchetKey, RatchetSecret};

use crate::crdt::{CRDTId, DocumentOperation, document::CRDTDocument};
use crate::domain::errors::DocumentError;
use crate::collaboration::operation_queue::{OperationQueue, QueuedOperation};

#[cfg(feature = "p2p")]
use crate::collaboration::transport::{
    quic::{QuicEndpoint, QuicTransportConfig},
    turn::TurnServerConfig,
    error::NetworkError,
};

// PandaOperation with security metadata
#[derive(Debug, Clone)]
pub struct PandaOperation {
    pub operation: DocumentOperation,
    pub hash: [u8; 32],
    pub signature: Vec<u8>,
    pub timestamp: i64,
    pub sequence_number: u64,
    pub sender_id: Uuid,
}

// Network interface trait to maintain compatibility with existing code
#[async_trait]
pub trait NetworkInterface: Send + Sync {
    async fn broadcast_operation(&self, document_id: Uuid, operation: DocumentOperation) -> Result<(), DocumentError>;
    fn subscribe_to_operations(&self) -> broadcast::Receiver<DocumentOperation>;
    fn get_node_id(&self) -> Uuid;
    fn set_connected(&self, connected: bool);
    async fn process_queued_operations(&self) -> Result<usize, DocumentError>;
}

// PandaNetwork implementation using p2panda
pub struct PandaNetwork {
    node: Arc<Node>,
    document_subscriptions: Arc<Mutex<HashMap<Uuid, broadcast::Sender<DocumentOperation>>>>,
    keypair: Keypair,
    operation_sender: broadcast::Sender<DocumentOperation>,
    operation_queue: Arc<OperationQueue>,
    is_connected: Arc<Mutex<bool>>,
    ratchet_manager: Arc<Mutex<HashMap<Uuid, Ratchet>>>, // document_id -> ratchet session
    
    #[cfg(feature = "p2p")]
    transport: Option<Arc<Mutex<QuicEndpoint>>>,
}

impl PandaNetwork {
    pub fn new(node_id: Uuid) -> Self {
        // Create a keypair for this node
        let keypair = Keypair::new();
        
        // Create p2panda node
        let node = Arc::new(Node::new(keypair.clone()));
        
        // Create broadcast channel for operations
        let (sender, _) = broadcast::channel(100);
        
        // Create operation queue for offline support
        let operation_queue = Arc::new(OperationQueue::new(1000));
        
        // Initially not connected
        let is_connected = Arc::new(Mutex::new(false));
        
        // Create ratchet manager for end-to-end encryption
        let ratchet_manager = Arc::new(Mutex::new(HashMap::new()));
        
        // Initialize QUIC transport with STUN fallback
        #[cfg(feature = "p2p")]
        let transport = None; // Will be initialized later with proper configuration
        
        Self {
            node,
            document_subscriptions: Arc::new(Mutex::new(HashMap::new())),
            keypair,
            operation_sender: sender,
            operation_queue,
            is_connected,
            ratchet_manager,
            
            #[cfg(feature = "p2p")]
            transport,
        }
    }
    
    #[cfg(feature = "p2p")]
    pub fn initialize_transport(
        &mut self,
        local_addr: SocketAddr,
        stun_servers: Vec<SocketAddr>,
        turn_servers: Vec<TurnServerConfig>,
    ) -> Result<(), NetworkError> {
        let config = QuicTransportConfig::default();
        let transport = QuicEndpoint::new(local_addr, stun_servers, turn_servers, config)?;
        self.transport = Some(Arc::new(Mutex::new(transport)));
        Ok(())
    }
    
    pub fn add_document_subscription(&mut self, document_id: Uuid) -> Result<(), DocumentError> {
        let (sender, _) = broadcast::channel(100);
        let mut subscriptions = self.document_subscriptions.lock().unwrap();
        subscriptions.insert(document_id, sender);
        Ok(())
    }
    
    /// Initialize a new ratchet session for a document
    pub fn initialize_ratchet_session(&self, document_id: Uuid) -> Result<(), DocumentError> {
        let mut ratchet_manager = self.ratchet_manager.lock().unwrap();
        
        // Create a new ratchet session with a random key
        let key = RatchetKey::from_bytes(rand::random());
        let ratchet = Ratchet::new_alice(key);
        
        ratchet_manager.insert(document_id, ratchet);
        Ok(())
    }
    
    /// Save ratchet session to persistent storage
    pub async fn save_ratchet_session(&self, document_id: Uuid, node_id: Uuid) -> Result<(), DocumentError> {
        if let Some(ratchet) = self.get_ratchet_session(document_id) {
            // Serialize the ratchet session
            let session_data = ratchet.serialize();
            // In a real implementation, you would save this to the collaboration service
            // For now, we'll just print a message
            println!("Saving ratchet session for document {} and node {}", document_id, node_id);
        }
        Ok(())
    }
    
    /// Load ratchet session from persistent storage
    pub async fn load_ratchet_session(&self, document_id: Uuid, node_id: Uuid) -> Result<(), DocumentError> {
        // In a real implementation, you would load this from the collaboration service
        // For now, we'll just print a message
        println!("Loading ratchet session for document {} and node {}", document_id, node_id);
        Ok(())
    }
    
    /// Get an existing ratchet session for a document
    fn get_ratchet_session(&self, document_id: Uuid) -> Option<Ratchet> {
        let ratchet_manager = self.ratchet_manager.lock().unwrap();
        ratchet_manager.get(&document_id).cloned()
    }
    
    fn sign_operation(&self, operation: &DocumentOperation, document_id: Uuid) -> Result<PandaOperation, DocumentError> {
        // Serialize the operation
        let operation_bytes = serde_json::to_vec(operation)
            .map_err(|e| DocumentError::SerializationError(e))?;
        
        // Encrypt the operation with Double Ratchet if we have a session
        let encrypted_bytes = if let Some(mut ratchet) = self.get_ratchet_session(document_id) {
            // Encrypt the operation bytes
            let encrypted = ratchet.ratchet_encrypt(&operation_bytes, &[]);
            // Update the ratchet session
            let mut ratchet_manager = self.ratchet_manager.lock().unwrap();
            ratchet_manager.insert(document_id, ratchet);
            encrypted
        } else {
            // If no ratchet session, just use the plain bytes
            operation_bytes
        };
        
        // Hash the encrypted operation with BLAKE3
        let hash = blake3::hash(&encrypted_bytes).as_bytes().clone();
        
        // Sign the hash with Ed25519
        let signature = self.keypair.sign(&hash);
        
        // Create CRDT ID with current node info
        let crdt_id = CRDTId {
            node_id: self.get_node_id(),
            counter: 0, // This would be incremented in a real implementation
            timestamp: chrono::Utc::now().timestamp(),
        };
        
        // For now, we'll just use a simple timestamp as sequence number
        let sequence_number = chrono::Utc::now().timestamp() as u64;
        
        Ok(PandaOperation {
            operation: operation.clone(),
            hash,
            signature: signature.to_bytes().to_vec(),
            timestamp: crdt_id.timestamp,
            sequence_number,
            sender_id: self.get_node_id(),
        })
    }
    
    fn verify_operation(&self, panda_operation: &PandaOperation, document_id: Uuid) -> Result<bool, DocumentError> {
        // Serialize the operation
        let operation_bytes = serde_json::to_vec(&panda_operation.operation)
            .map_err(|e| DocumentError::SerializationError(e))?;
        
        // Decrypt the operation with Double Ratchet if we have a session
        let decrypted_bytes = if let Some(mut ratchet) = self.get_ratchet_session(document_id) {
            // Decrypt the operation bytes
            let decrypted = ratchet.ratchet_decrypt(&operation_bytes, &[]);
            // Update the ratchet session
            let mut ratchet_manager = self.ratchet_manager.lock().unwrap();
            ratchet_manager.insert(document_id, ratchet);
            decrypted
        } else {
            // If no ratchet session, just use the plain bytes
            operation_bytes
        };
        
        // Hash the decrypted operation with BLAKE3
        let hash = blake3::hash(&decrypted_bytes).as_bytes().clone();
        
        // Verify the hash matches
        if hash != panda_operation.hash {
            return Ok(false);
        }
        
        // Create a signature from the bytes
        let signature = Signature::from_bytes(&panda_operation.signature)
            .map_err(|_| DocumentError::InvalidSignature)?;
        
        // Verify the signature
        Ok(self.keypair.verify(&hash, &signature))
    }
}

#[async_trait]
impl NetworkInterface for PandaNetwork {
    async fn broadcast_operation(&self, document_id: Uuid, operation: DocumentOperation) -> Result<(), DocumentError> {
        // Check if we're connected to the network
        let is_connected = {
            let connected = self.is_connected.lock().unwrap();
            *connected
        };
        
        if is_connected {
            // Sign and secure the operation
            let panda_operation = self.sign_operation(&operation, document_id)?;
            
            // Serialize the panda operation for p2panda
            let operation_bytes = serde_json::to_vec(&panda_operation)
                .map_err(|e| DocumentError::SerializationError(e))?;
            
            // Route through QUIC transport if available
            #[cfg(feature = "p2p")]
            if let Some(transport) = &self.transport {
                let transport = transport.lock().unwrap();
                tokio::runtime::Handle::current().block_on(
                    transport.send_to_all(document_id, &operation_bytes)
                )?;
            } else {
                // Fallback to local broadcast for testing
                let _ = self.operation_sender.send(operation);
            }
            
            #[cfg(not(feature = "p2p"))]
            {
                // Fallback to local broadcast when p2p feature is disabled
                let _ = self.operation_sender.send(operation);
            }
            
            Ok(())
        } else {
            // If not connected, queue the operation for later
            self.operation_queue.enqueue(document_id, operation)?;
            Ok(())
        }
    }
    
    fn subscribe_to_operations(&self) -> broadcast::Receiver<DocumentOperation> {
        self.operation_sender.subscribe()
    }
    
    fn get_node_id(&self) -> Uuid {
        // Convert p2panda public key to UUID
        // This is a simplified implementation
        let public_key_bytes = self.keypair.public().to_bytes();
        let mut uuid_bytes = [0u8; 16];
        uuid_bytes.copy_from_slice(&public_key_bytes[..16]);
        Uuid::from_bytes(uuid_bytes)
    }
    
    fn set_connected(&self, connected: bool) {
        let mut is_connected = self.is_connected.lock().unwrap();
        *is_connected = connected;
        
        // Update transport connection state
        #[cfg(feature = "p2p")]
        if let Some(transport) = &self.transport {
            transport.lock().unwrap().set_connected(connected);
        }
    }
    
    async fn process_queued_operations(&self) -> Result<usize, DocumentError> {
        let mut processed_count = 0;
        
        // Process all queued operations
        while let Some(queued_op) = self.operation_queue.dequeue() {
            // Try to broadcast the operation
            if let Err(e) = self.broadcast_operation(queued_op.document_id, queued_op.operation.clone()).await {
                // If broadcasting fails, put the operation back in the queue
                self.operation_queue.enqueue(queued_op.document_id, queued_op.operation)?;
                // Increment the attempt counter
                self.operation_queue.increment_attempts(queued_op.document_id, &queued_op.operation);
                return Err(e);
            }
            processed_count += 1;
        }
        
        Ok(processed_count)
    }
    
}

// P2P Document Sync Service using p2panda
pub struct PandaSyncService {
    network: Arc<Mutex<PandaNetwork>>,
    documents: Arc<Mutex<HashMap<Uuid, CRDTDocument>>>, // document_id -> CRDTDocument
}

impl PandaSyncService {
    pub fn new(network: Arc<Mutex<PandaNetwork>>) -> Self {
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