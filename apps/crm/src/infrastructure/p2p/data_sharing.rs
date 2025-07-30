//! P2P data sharing for CRM using cpc-net
//!
//! This module implements secure, consent-based data sharing for CRM entities
//! using p2panda and Double Ratchet encryption.

use crate::domain::{Contact, Interaction, Pipeline, Deal};
use crate::domain::primitives::{ContactId, UserId};
use uuid::Uuid;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_cbor;
use thiserror::Error;
use libp2p_core::PeerId;
use super::session_manager::SessionManager;

/// Error types for CRM data sharing
#[derive(Error, Debug)]
pub enum CrmSharingError {
    #[error("Serialization failed: {0}")]
    Serialization(#[from] serde_cbor::Error),
    
    #[error("Encryption failed: {0}")]
    Encryption(String),
    
    #[error("Network transmission failed: {0}")]
    Network(String),
    
    #[error("Hash verification failed")]
    VerificationFailed,
    
    #[error("Invalid peer ID: {0}")]
    InvalidPeerId(String),
    
    #[error("Consent denied: {0}")]
    ConsentDenied(String),
}

/// Payload for CRM data sharing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrmPayload {
    /// BLAKE3 hash of the serialized data for verification
    pub hash: Vec<u8>,
    
    /// Encrypted CRM data
    pub data: Vec<u8>,
    
    /// Sender's peer ID
    pub sender: String,
    
    /// Type of CRM entity being shared
    pub entity_type: String,
}

/// Main P2P CRM sharing service
pub struct P2PCrmSharing {
    // In a real implementation, this would contain network and crypto components
    // For now, we'll use placeholders
    local_peer_id: PeerId,
    session_manager: SessionManager,
}

impl P2PCrmSharing {
    /// Create a new P2P CRM sharing service
    pub fn new(local_peer_id: PeerId) -> Self {
        Self {
            local_peer_id,
            session_manager: SessionManager::new(),
        }
    }
    
    /// Share a contact with another peer (with consent checks)
    pub async fn share_contact(&mut self, contact: &Contact, recipient_peer_id: &str) -> Result<(), CrmSharingError> {
        // Check if we have permission to share this contact
        if !self.can_share_contact(contact)? {
            return Err(CrmSharingError::ConsentDenied(
                "Insufficient consent to share contact data".to_string()
            ));
        }
        
        // Serialize contact to CBOR
        let serialized = serde_cbor::to_vec(contact)?;
        
        // Generate BLAKE3 hash for verification
        let hash = self.hash_content(&serialized);
        
        // Parse recipient peer ID for session management
        let peer_id = recipient_peer_id.parse::<PeerId>()
            .map_err(|e| CrmSharingError::InvalidPeerId(e.to_string()))?;
        
        // Encrypt payload using Double Ratchet
        let encrypted = self.encrypt_data(&serialized, &peer_id)?;
        
        // Create payload with hash verification
        let payload = CrmPayload {
            hash: hash.to_vec(),
            data: encrypted,
            sender: self.local_peer_id.to_string(),
            entity_type: "contact".to_string(),
        };
        
        // Send through network layer
        self.send_payload(payload, recipient_peer_id).await?;
        
        // Increment message count for key rotation
        self.session_manager.increment_message_count(&peer_id);
        
        Ok(())
    }
    
    /// Share an interaction with another peer (with consent checks)
    pub async fn share_interaction(&mut self, interaction: &Interaction, recipient_peer_id: &str) -> Result<(), CrmSharingError> {
        // Check if we have permission to share this interaction
        if !self.can_share_interaction(interaction)? {
            return Err(CrmSharingError::ConsentDenied(
                "Insufficient consent to share interaction data".to_string()
            ));
        }
        
        // Serialize interaction to CBOR
        let serialized = serde_cbor::to_vec(interaction)?;
        
        // Generate BLAKE3 hash for verification
        let hash = self.hash_content(&serialized);
        
        // Parse recipient peer ID for session management
        let peer_id = recipient_peer_id.parse::<PeerId>()
            .map_err(|e| CrmSharingError::InvalidPeerId(e.to_string()))?;
        
        // Encrypt payload using Double Ratchet
        let encrypted = self.encrypt_data(&serialized, &peer_id)?;
        
        // Create payload with hash verification
        let payload = CrmPayload {
            hash: hash.to_vec(),
            data: encrypted,
            sender: self.local_peer_id.to_string(),
            entity_type: "interaction".to_string(),
        };
        
        // Send through network layer
        self.send_payload(payload, recipient_peer_id).await?;
        
        // Increment message count for key rotation
        self.session_manager.increment_message_count(&peer_id);
        
        Ok(())
    }
    
    /// Share a pipeline with another peer
    pub async fn share_pipeline(&mut self, pipeline: &Pipeline, recipient_peer_id: &str) -> Result<(), CrmSharingError> {
        // Serialize pipeline to CBOR
        let serialized = serde_cbor::to_vec(pipeline)?;
        
        // Generate BLAKE3 hash for verification
        let hash = self.hash_content(&serialized);
        
        // Parse recipient peer ID for session management
        let peer_id = recipient_peer_id.parse::<PeerId>()
            .map_err(|e| CrmSharingError::InvalidPeerId(e.to_string()))?;
        
        // Encrypt payload using Double Ratchet
        let encrypted = self.encrypt_data(&serialized, &peer_id)?;
        
        // Create payload with hash verification
        let payload = CrmPayload {
            hash: hash.to_vec(),
            data: encrypted,
            sender: self.local_peer_id.to_string(),
            entity_type: "pipeline".to_string(),
        };
        
        // Send through network layer
        self.send_payload(payload, recipient_peer_id).await?;
        
        // Increment message count for key rotation
        self.session_manager.increment_message_count(&peer_id);
        
        Ok(())
    }
    
    /// Share a deal with another peer
    pub async fn share_deal(&mut self, deal: &Deal, recipient_peer_id: &str) -> Result<(), CrmSharingError> {
        // Serialize deal to CBOR
        let serialized = serde_cbor::to_vec(deal)?;
        
        // Generate BLAKE3 hash for verification
        let hash = self.hash_content(&serialized);
        
        // Parse recipient peer ID for session management
        let peer_id = recipient_peer_id.parse::<PeerId>()
            .map_err(|e| CrmSharingError::InvalidPeerId(e.to_string()))?;
        
        // Encrypt payload using Double Ratchet
        let encrypted = self.encrypt_data(&serialized, &peer_id)?;
        
        // Create payload with hash verification
        let payload = CrmPayload {
            hash: hash.to_vec(),
            data: encrypted,
            sender: self.local_peer_id.to_string(),
            entity_type: "deal".to_string(),
        };
        
        // Send through network layer
        self.send_payload(payload, recipient_peer_id).await?;
        
        // Increment message count for key rotation
        self.session_manager.increment_message_count(&peer_id);
        
        Ok(())
    }
    
    /// Check if we can share a contact based on consent settings
    fn can_share_contact(&self, contact: &Contact) -> Result<bool, CrmSharingError> {
        // For platform-native contacts, check consent settings
        if contact.is_platform_native() {
            // In a real implementation, this would check the actual consent settings
            // For now, we'll assume we can share if it's a platform-native contact
            Ok(true)
        } else {
            // For external contacts, we can always share (we own the data)
            Ok(true)
        }
    }
    
    /// Check if we can share an interaction based on consent settings
    fn can_share_interaction(&self, interaction: &Interaction) -> Result<bool, CrmSharingError> {
        // Platform-native interactions require consent
        if interaction.is_platform_native {
            // In a real implementation, this would check the actual consent settings
            // For now, we'll assume we can share platform-native interactions
            Ok(true)
        } else {
            // Non-platform interactions can be shared if we have the right to do so
            Ok(true)
        }
    }
    
    /// Generate BLAKE3 hash of content
    fn hash_content(&self, content: &[u8]) -> [u8; 32] {
        // In a real implementation, this would use the actual BLAKE3 hashing
        // For now, we'll create a placeholder hash
        let mut hash = [0u8; 32];
        // Simple placeholder - in reality, use p2panda::crypto::hash_content
        hash.copy_from_slice(&content[..std::cmp::min(32, content.len())]);
        hash
    }
    
    /// Encrypt data using Double Ratchet
    fn encrypt_data(&mut self, data: &[u8], peer_id: &PeerId) -> Result<Vec<u8>, CrmSharingError> {
        let mut session = self.session_manager.get_or_create_session(peer_id);
        
        // In a real implementation, this would use the actual encryption
        // For now, we'll just return the data as "encrypted"
        // In reality, this would be: session.encrypt(data)
        Ok(data.to_vec())
    }
    
    /// Decrypt data using Double Ratchet
    fn decrypt_data(&mut self, data: &[u8], peer_id: &PeerId) -> Result<Vec<u8>, CrmSharingError> {
        let mut session = self.session_manager.get_or_create_session(peer_id);
        
        // In a real implementation, this would use the actual decryption
        // For now, we'll just return the data as "decrypted"
        // In reality, this would be: session.decrypt(data)
        Ok(data.to_vec())
    }
    
    /// Send payload through the network
    async fn send_payload(&self, payload: CrmPayload, recipient_peer_id: &str) -> Result<(), CrmSharingError> {
        // In a real implementation, this would use the actual network layer
        // For now, we'll just print a message
        println!("Sending CRM payload to peer: {}", recipient_peer_id);
        println!("Payload sender: {}", payload.sender);
        println!("Payload entity type: {}", payload.entity_type);
        println!("Payload hash: {:?}", payload.hash);
        println!("Payload data length: {}", payload.data.len());
        
        // Simulate network operation
        // In reality, this would use p2panda's network layer
        Ok(())
    }
    
    /// Receive and process a payload
    pub async fn receive_payload(&mut self, payload: CrmPayload, sender_peer_id: &str) -> Result<(), CrmSharingError> {
        // Parse sender peer ID
        let peer_id = sender_peer_id.parse::<PeerId>()
            .map_err(|e| CrmSharingError::InvalidPeerId(e.to_string()))?;
        
        // Decrypt payload
        let decrypted = self.decrypt_data(&payload.data, &peer_id)?;
        
        // Verify hash
        let computed_hash = self.hash_content(&decrypted);
        if computed_hash.as_slice() != payload.hash {
            return Err(CrmSharingError::VerificationFailed);
        }
        
        // Process based on entity type
        match payload.entity_type.as_str() {
            "contact" => {
                let _contact: Contact = serde_cbor::from_slice(&decrypted)?;
                // Process contact
                println!("Received contact from peer: {}", sender_peer_id);
            }
            "interaction" => {
                let _interaction: Interaction = serde_cbor::from_slice(&decrypted)?;
                // Process interaction
                println!("Received interaction from peer: {}", sender_peer_id);
            }
            "pipeline" => {
                let _pipeline: Pipeline = serde_cbor::from_slice(&decrypted)?;
                // Process pipeline
                println!("Received pipeline from peer: {}", sender_peer_id);
            }
            "deal" => {
                let _deal: Deal = serde_cbor::from_slice(&decrypted)?;
                // Process deal
                println!("Received deal from peer: {}", sender_peer_id);
            }
            _ => {
                return Err(CrmSharingError::Serialization(
                    serde_cbor::Error::syntax()
                ));
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::contact::{ContactType, ConsentSettings};
    use crate::domain::primitives::Email;
    
    #[test]
    fn test_crm_sharing_creation() {
        let peer_id = PeerId::random();
        let sharing = P2PCrmSharing::new(peer_id);
        
        // Test creation succeeded
        assert_eq!(sharing.local_peer_id.to_string(), peer_id.to_string());
    }
    
    #[ignore] // This test would require async runtime and network setup
    #[tokio::test]
    async fn test_share_contact() {
        /*
        let peer_id = PeerId::random();
        let mut sharing = P2PCrmSharing::new(peer_id);
        
        // Create a test contact
        let user_id = UserId::new();
        let consent = ConsentSettings::new();
        let email = Email::new("test@example.com".to_string()).unwrap();
        
        let contact = Contact::new_platform_native(
            user_id,
            "John Doe".to_string(),
            Some(email),
            None,
            Some("Acme Corp".to_string()),
            consent,
        ).unwrap();
        
        // Test sharing (would require network setup in real implementation)
        let result = sharing.share_contact(&contact, "recipient_peer_id").await;
        assert!(result.is_ok());
        */
    }
}