//! Core signaling service implementation

use crate::message::{SignalingMessage, PresenceUpdate, CursorPosition, SelectionRange};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};
use uuid::Uuid;
use chrono::Utc;
use tracing::{info, error};

/// Error types for signaling operations
#[derive(Debug, thiserror::Error)]
pub enum SignalingError {
    #[error("Connection not found: {0}")]
    ConnectionNotFound(Uuid),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Broadcast error: {0}")]
    BroadcastError(String),
    
    #[error("Invalid message: {0}")]
    InvalidMessage(String),
}

/// Signaling service for real-time collaboration
pub struct SignalingService {
    /// Active connections by document ID
    connections: Arc<RwLock<HashMap<Uuid, broadcast::Sender<String>>>>,
    
    /// Presence information
    presence: Arc<RwLock<HashMap<Uuid, HashMap<Uuid, PresenceUpdate>>>>,
}

impl SignalingService {
    /// Create a new signaling service
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            presence: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Register a new connection for a document
    pub async fn register_connection(&self, document_id: Uuid) -> Result<broadcast::Receiver<String>, SignalingError> {
        let mut connections = self.connections.write().await;
        
        // Create a new broadcast channel for this document
        let (sender, receiver) = broadcast::channel(100);
        connections.insert(document_id, sender);
        
        info!("Registered connection for document: {}", document_id);
        Ok(receiver)
    }
    
    /// Unregister a connection for a document
    pub async fn unregister_connection(&self, document_id: Uuid) -> Result<(), SignalingError> {
        let mut connections = self.connections.write().await;
        connections.remove(&document_id);
        
        // Clean up presence data for this document
        let mut presence = self.presence.write().await;
        presence.remove(&document_id);
        
        info!("Unregistered connection for document: {}", document_id);
        Ok(())
    }
    
    /// Broadcast a message to all connections for a document
    pub async fn broadcast_message(&self, document_id: Uuid, message: &SignalingMessage) -> Result<(), SignalingError> {
        let connections = self.connections.read().await;
        
        if let Some(sender) = connections.get(&document_id) {
            let serialized = serde_json::to_string(message)
                .map_err(|e| SignalingError::SerializationError(e.to_string()))?;
            
            // Broadcast to all receivers
            if let Err(e) = sender.send(serialized) {
                // This can happen when there are no active receivers, which is fine
                info!("Broadcast send error (possibly no receivers): {}", e);
            }
            
            Ok(())
        } else {
            Err(SignalingError::ConnectionNotFound(document_id))
        }
    }
    
    /// Handle an incoming message
    pub async fn handle_message(&self, document_id: Uuid, message: SignalingMessage) -> Result<(), SignalingError> {
        match message {
            SignalingMessage::JoinDocument { document_id, user_id } => {
                self.handle_join_document(document_id, user_id).await
            },
            SignalingMessage::LeaveDocument { document_id, user_id } => {
                self.handle_leave_document(document_id, user_id).await
            },
            SignalingMessage::PresenceUpdate(update) => {
                self.handle_presence_update(document_id, update).await
            },
            SignalingMessage::PresenceSummary(summary) => {
                // Handle presence summary - typically just broadcast
                let message = SignalingMessage::PresenceSummary(summary);
                self.broadcast_message(document_id, &message).await
            },
            SignalingMessage::CursorUpdate(cursor) => {
                self.handle_cursor_update(document_id, cursor).await
            },
            SignalingMessage::SelectionUpdate { document_id, user_id, selection, timestamp } => {
                self.handle_selection_update(document_id, user_id, selection, timestamp).await
            },
            SignalingMessage::TypingIndicator { document_id, user_id, is_typing, timestamp } => {
                self.handle_typing_indicator(document_id, user_id, is_typing, timestamp).await
            },
            SignalingMessage::Error { code, message } => {
                error!("Received error message: {} - {}", code, message);
                Ok(())
            },
            SignalingMessage::Annotation { document_id, user_id, position, content, timestamp } => {
                self.handle_annotation(document_id, user_id, position, content, timestamp).await
            },
            SignalingMessage::Comment { document_id, user_id, position, content, timestamp } => {
                self.handle_comment(document_id, user_id, position, content, timestamp).await
            },
            SignalingMessage::PresenceStatus { document_id, user_id, status, timestamp } => {
                self.handle_presence_status(document_id, user_id, status, timestamp).await
            },
        }
    }
    
    /// Handle user joining a document
    async fn handle_join_document(&self, document_id: Uuid, user_id: Uuid) -> Result<(), SignalingError> {
        info!("User {} joined document {}", user_id, document_id);
        
        // Send current presence information to the new user
        let presence = self.presence.read().await;
        if let Some(document_presence) = presence.get(&document_id) {
            for (_, update) in document_presence {
                let message = SignalingMessage::PresenceUpdate(update.clone());
                self.broadcast_message(document_id, &message).await?;
            }
        }
        
        Ok(())
    }
    
    /// Handle user leaving a document
    async fn handle_leave_document(&self, document_id: Uuid, user_id: Uuid) -> Result<(), SignalingError> {
        info!("User {} left document {}", user_id, document_id);
        
        // Remove user's presence
        {
            let mut presence = self.presence.write().await;
            if let Some(document_presence) = presence.get_mut(&document_id) {
                document_presence.remove(&user_id);
            }
        }
        
        // Broadcast leave message
        let message = SignalingMessage::LeaveDocument { document_id, user_id };
        self.broadcast_message(document_id, &message).await
    }
    
    /// Handle presence update
    async fn handle_presence_update(&self, document_id: Uuid, update: PresenceUpdate) -> Result<(), SignalingError> {
        // Store the presence update
        {
            let mut presence = self.presence.write().await;
            let document_presence = presence.entry(document_id).or_insert_with(HashMap::new);
            document_presence.insert(update.user_id, update.clone());
        }
        
        // Broadcast to all other users
        let message = SignalingMessage::PresenceUpdate(update);
        self.broadcast_message(document_id, &message).await
    }
    
    /// Handle cursor update
    async fn handle_cursor_update(&self, document_id: Uuid, cursor: CursorPosition) -> Result<(), SignalingError> {
        // Broadcast cursor update
        let message = SignalingMessage::CursorUpdate(cursor);
        self.broadcast_message(document_id, &message).await
    }
    
    /// Handle selection update
    async fn handle_selection_update(
        &self, 
        document_id: Uuid, 
        user_id: Uuid, 
        selection: Option<SelectionRange>, 
        timestamp: chrono::DateTime<Utc>
    ) -> Result<(), SignalingError> {
        // Broadcast selection update
        let message = SignalingMessage::SelectionUpdate {
            document_id,
            user_id,
            selection,
            timestamp,
        };
        self.broadcast_message(document_id, &message).await
    }
    
    /// Handle typing indicator
    async fn handle_typing_indicator(
        &self, 
        document_id: Uuid, 
        user_id: Uuid, 
        is_typing: bool, 
        timestamp: chrono::DateTime<Utc>
    ) -> Result<(), SignalingError> {
        // Broadcast typing indicator
        let message = SignalingMessage::TypingIndicator {
            document_id,
            user_id,
            is_typing,
            timestamp,
        };
        self.broadcast_message(document_id, &message).await
    }
    
    /// Handle annotation message
    async fn handle_annotation(
        &self,
        document_id: Uuid,
        user_id: Uuid,
        position: crate::message::Position,
        content: String,
        timestamp: chrono::DateTime<Utc>
    ) -> Result<(), SignalingError> {
        // Broadcast annotation
        let message = SignalingMessage::Annotation {
            document_id,
            user_id,
            position,
            content,
            timestamp,
        };
        self.broadcast_message(document_id, &message).await
    }
    
    /// Handle comment message
    async fn handle_comment(
        &self,
        document_id: Uuid,
        user_id: Uuid,
        position: crate::message::Position,
        content: String,
        timestamp: chrono::DateTime<Utc>
    ) -> Result<(), SignalingError> {
        // Broadcast comment
        let message = SignalingMessage::Comment {
            document_id,
            user_id,
            position,
            content,
            timestamp,
        };
        self.broadcast_message(document_id, &message).await
    }
    
    /// Handle presence status update
    async fn handle_presence_status(
        &self,
        document_id: Uuid,
        user_id: Uuid,
        status: String,
        timestamp: chrono::DateTime<Utc>
    ) -> Result<(), SignalingError> {
        // Broadcast presence status
        let message = SignalingMessage::PresenceStatus {
            document_id,
            user_id,
            status,
            timestamp,
        };
        self.broadcast_message(document_id, &message).await
    }
    
    /// Get current presence information for a document
    pub async fn get_document_presence(&self, document_id: Uuid) -> HashMap<Uuid, PresenceUpdate> {
        let presence = self.presence.read().await;
        presence.get(&document_id).cloned().unwrap_or_default()
    }
}

impl Default for SignalingService {
    fn default() -> Self {
        Self::new()
    }
}