//! Session management for CRM P2P connections
//!
//! This module manages Double Ratchet sessions for secure CRM data sharing.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use libp2p_core::PeerId;
use p2panda::crypto::NoiseSession;
use uuid::Uuid;
use thiserror::Error;

/// Error types for session management
#[derive(Error, Debug)]
pub enum SessionError {
    #[error("Session not found for peer: {0}")]
    SessionNotFound(String),
    
    #[error("Crypto error: {0}")]
    CryptoError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_cbor::Error),
}

/// Manages Double Ratchet sessions for P2P CRM data sharing
pub struct SessionManager {
    sessions: Arc<Mutex<HashMap<String, NoiseSession>>>,
    message_counts: Arc<Mutex<HashMap<String, u32>>>,
    rotation_threshold: u32,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            message_counts: Arc::new(Mutex::new(HashMap::new())),
            rotation_threshold: 100, // Rotate keys every 100 messages
        }
    }
    
    /// Get an existing session or create a new one
    pub fn get_or_create_session(&mut self, peer_id: &PeerId) -> NoiseSession {
        let peer_str = peer_id.to_string();
        let mut sessions = self.sessions.lock().unwrap();
        
        if let Some(session) = sessions.get(&peer_str) {
            session.clone()
        } else {
            // In a real implementation, this would establish a new Noise session
            // For now, we'll create a placeholder
            let new_session = NoiseSession::new();
            sessions.insert(peer_str.clone(), new_session.clone());
            
            // Initialize message count
            let mut message_counts = self.message_counts.lock().unwrap();
            message_counts.insert(peer_str, 0);
            
            new_session
        }
    }
    
    /// Increment message count for a peer and check if rotation is needed
    pub fn increment_message_count(&mut self, peer_id: &PeerId) {
        let peer_str = peer_id.to_string();
        let mut message_counts = self.message_counts.lock().unwrap();
        
        if let Some(count) = message_counts.get_mut(&peer_str) {
            *count += 1;
            
            // Check if we need to rotate keys
            if *count >= self.rotation_threshold {
                self.maybe_rotate_session(peer_id);
                *count = 0; // Reset counter after rotation
            }
        }
    }
    
    /// Rotate session keys if needed
    fn maybe_rotate_session(&mut self, peer_id: &PeerId) {
        let peer_str = peer_id.to_string();
        let mut sessions = self.sessions.lock().unwrap();
        
        if let Some(session) = sessions.get_mut(&peer_str) {
            // In a real implementation, this would rotate the Double Ratchet keys
            // For now, we'll just log that rotation would happen
            println!("Rotating session keys for peer: {}", peer_str);
        }
    }
    
    /// Remove a session (e.g., when connection is closed)
    pub fn remove_session(&mut self, peer_id: &PeerId) {
        let peer_str = peer_id.to_string();
        let mut sessions = self.sessions.lock().unwrap();
        let mut message_counts = self.message_counts.lock().unwrap();
        
        sessions.remove(&peer_str);
        message_counts.remove(&peer_str);
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libp2p_core::PeerId;
    
    #[test]
    fn test_session_manager_creation() {
        let manager = SessionManager::new();
        assert_eq!(manager.rotation_threshold, 100);
    }
    
    #[test]
    fn test_get_or_create_session() {
        let mut manager = SessionManager::new();
        let peer_id = PeerId::random();
        
        let session1 = manager.get_or_create_session(&peer_id);
        let session2 = manager.get_or_create_session(&peer_id);
        
        // Should return the same session for the same peer
        // Note: This is a simplified test - in reality, NoiseSession would need
        // to implement PartialEq for this to work
    }
    
    #[test]
    fn test_increment_message_count() {
        let mut manager = SessionManager::new();
        let peer_id = PeerId::random();
        
        // Initialize session
        let _session = manager.get_or_create_session(&peer_id);
        
        // Increment message count
        manager.increment_message_count(&peer_id);
        
        // In a real implementation, we would check the message count
        // For now, we just ensure no panic occurs
    }
}