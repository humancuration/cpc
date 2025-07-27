//! P2P data sharing for invoices and quotes using cpc-net

use crate::domain::{Invoice, Quote};
use uuid::Uuid;
use std::sync::Arc;
use cpc_net::{crypto, net, secure_storage, circuit_breaker};
use serde::{Deserialize, Serialize};
use serde_cbor;
use thiserror::Error;
use libp2p_core::PeerId;
use super::session_manager::SessionManager;

/// Error types for invoice sharing
#[derive(Debug, Error)]
pub enum InvoiceSharingError {
    #[error("Serialization failed: {0}")]
    Serialization(#[from] serde_cbor::Error),
    
    #[error("Encryption failed: {0}")]
    Encryption(String),
    
    #[error("Circuit breaker is open")]
    CircuitBreakerOpen,
    
    #[error("Network transmission failed: {0}")]
    Network(String),
    
    #[error("Hash verification failed")]
    VerificationFailed,
    
    #[error("Invalid peer ID: {0}")]
    InvalidPeerId(String),
}

/// Payload for invoice/quote sharing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoicePayload {
    /// BLAKE3 hash of the serialized data for verification
    pub hash: Vec<u8>,
    /// Encrypted invoice/quote data
    pub data: Vec<u8>,
    /// Sender's peer ID
    pub sender: String,
}
pub struct P2PInvoiceSharing {
    network: Arc<net::Network>,
    local_peer_id: PeerId,
    session_manager: SessionManager,
    secure_storage: Arc<secure_storage::SecureStorage>,
    circuit_breaker: std::sync::Mutex<circuit_breaker::NetworkCircuitBreaker>,
}
}

impl P2PInvoiceSharing {
    pub fn new(network: Arc<net::Network>, local_peer_id: PeerId) -> Self {
        Self {
            network,
            local_peer_id,
            session_manager: SessionManager::new(),
            secure_storage: Arc::new(secure_storage::SecureStorage::new()),
            circuit_breaker: std::sync::Mutex::new(circuit_breaker::NetworkCircuitBreaker::new()),
        }
    }
    
    pub async fn share_invoice(&mut self, invoice: &Invoice, recipient_peer_id: &str) -> Result<(), InvoiceSharingError> {
        // 1. Serialize invoice to CBOR
        let serialized = serde_cbor::to_vec(invoice)?;
        
        // 2. Generate BLAKE3 hash for verification
        let hash = crypto::hash_content(&serialized);
        
        // 3. Parse recipient peer ID for session management
        let peer_id = recipient_peer_id.parse::<PeerId>()
            .map_err(|e| InvoiceSharingError::InvalidPeerId(e.to_string()))?;
        
        // 4. Encrypt payload using Double Ratchet
        let encrypted = self.encrypt_data(&serialized, &peer_id)?;
        
        // 5. Create payload with hash verification
        let payload = InvoicePayload {
            hash: hash.to_vec(),
            data: encrypted,
            sender: self.local_peer_id.to_string(),
        };
        
        // 6. Send through network layer with circuit protection
        self.send_payload_with_circuit(payload, recipient_peer_id).await?;
        
        // 7. Increment message count for key rotation
        self.session_manager.increment_message_count(&peer_id);
        
        Ok(())
    }
    
    pub async fn share_quote(&mut self, quote: &Quote, recipient_peer_id: &str) -> Result<(), InvoiceSharingError> {
        // 1. Serialize quote to CBOR
        let serialized = serde_cbor::to_vec(quote)?;
        
        // 2. Generate BLAKE3 hash for verification
        let hash = crypto::hash_content(&serialized);
        
        // 3. Parse recipient peer ID for session management
        let peer_id = recipient_peer_id.parse::<PeerId>()
            .map_err(|e| InvoiceSharingError::InvalidPeerId(e.to_string()))?;
        
        // 4. Encrypt payload using Double Ratchet
        let encrypted = self.encrypt_data(&serialized, &peer_id)?;
        
        // 5. Create payload with hash verification
        let payload = InvoicePayload {
            hash: hash.to_vec(),
            data: encrypted,
            sender: self.local_peer_id.to_string(),
        };
        
        // 6. Send through network layer with circuit protection
        self.send_payload_with_circuit(payload, recipient_peer_id).await?;
        
        // 7. Increment message count for key rotation
        self.session_manager.increment_message_count(&peer_id);
        
        Ok(())
    }
    
    pub async fn notify_client(&mut self, invoice_id: Uuid, recipient_peer_id: &str) -> Result<(), InvoiceSharingError> {
        // Implementation would notify the client via p2p
        // For now, we'll just return Ok(())
        Ok(())
    }
    
    
    /// Encrypt data using proper Double Ratchet implementation
    fn encrypt_data(&mut self, data: &[u8], peer_id: &PeerId) -> Result<Vec<u8>, InvoiceSharingError> {
        let mut session = self.session_manager.get_or_create_session(peer_id);
        let encrypted = session.encrypt(data).map_err(|e| {
            InvoiceSharingError::Encryption(format!("Encryption failed: {}", e))
        })?;
        Ok(encrypted)
    }
    
    /// Decrypt data using proper Double Ratchet implementation
    fn decrypt_data(&mut self, data: &[u8], peer_id: &PeerId) -> Result<Vec<u8>, InvoiceSharingError> {
        let mut session = self.session_manager.get_or_create_session(peer_id);
        let decrypted = session.decrypt(data).map_err(|e| {
            InvoiceSharingError::Encryption(format!("Decryption failed: {}", e))
        })?;
        Ok(decrypted)
    
    /// Send payload through the network with circuit breaker protection
    async fn send_payload_with_circuit(&mut self, payload: InvoicePayload, recipient_peer_id: &str) -> Result<(), InvoiceSharingError> {
        // Check if circuit breaker allows execution
        let can_execute = {
            let mut circuit_breaker = self.circuit_breaker.lock().unwrap();
            circuit_breaker.can_execute()
        };
        
        if !can_execute {
            return Err(InvoiceSharingError::CircuitBreakerOpen);
        }
        
        // Parse recipient peer ID
        let _recipient = recipient_peer_id.parse::<PeerId>()
            .map_err(|e| InvoiceSharingError::InvalidPeerId(e.to_string()))?;
        
        // In a real implementation, this would use the actual network to send the payload
        // For now, we'll just print a message
        println!("Sending payload to peer: {}", recipient_peer_id);
        println!("Payload sender: {}", payload.sender);
        println!("Payload hash: {:?}", payload.hash);
        println!("Payload data length: {}", payload.data.len());
        
        // Simulate network operation result for circuit breaker
        // In a real implementation, this would be based on actual network results
        let network_result = Ok(()); // Simulate success
        
        // Update circuit breaker based on result
        {
            let mut circuit_breaker = self.circuit_breaker.lock().unwrap();
            match network_result {
                Ok(_) => {
                    circuit_breaker.on_success();
                }
                Err(_) => {
                    circuit_breaker.on_failure();
                }
            }
        }
        
        match network_result {
            Ok(_) => {
                Ok(())
            }
            Err(e) => {
                Err(InvoiceSharingError::Network(e.to_string()))
            }
        }
    }
}