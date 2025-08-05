//! Redis-based signaling service implementation for scalability

use crate::message::{SignalingMessage, PresenceUpdate};
use crate::signaling::SignalingError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};
use tokio_stream::StreamExt;
use uuid::Uuid;
use chrono::{Utc, Duration};
use tracing::{info, error, debug};
use redis::AsyncCommands;

/// Redis-based signaling service for real-time collaboration
pub struct RedisSignalingService {
    /// Redis client
    redis_client: redis::Client,
    
    /// Redis connection manager
    redis_manager: redis::aio::ConnectionManager,
    
    /// Active connections by document ID
    connections: Arc<RwLock<HashMap<Uuid, broadcast::Sender<String>>>>,
    
    /// Presence information
    presence: Arc<RwLock<HashMap<Uuid, HashMap<Uuid, PresenceUpdate>>>>,
    
    /// Last presence summary timestamp
    last_summary: Arc<RwLock<HashMap<Uuid, chrono::DateTime<Utc>>>>,
    
    /// Redis channel prefix
    channel_prefix: String,
}

impl RedisSignalingService {
    /// Create a new Redis signaling service
    pub async fn new(redis_url: &str, channel_prefix: Option<String>) -> Result<Self, SignalingError> {
        let client = redis::Client::open(redis_url)
            .map_err(|e| SignalingError::SerializationError(format!("Failed to create Redis client: {}", e)))?;
        
        let manager = redis::aio::ConnectionManager::new(client.clone())
            .await
            .map_err(|e| SignalingError::SerializationError(format!("Failed to create Redis connection manager: {}", e)))?;
        
        Ok(Self {
            redis_client: client,
            redis_manager: manager,
            connections: Arc::new(RwLock::new(HashMap::new())),
            presence: Arc::new(RwLock::new(HashMap::new())),
            last_summary: Arc::new(RwLock::new(HashMap::new())),
            channel_prefix: channel_prefix.unwrap_or_else(|| "cpc_signaling".to_string()),
        })
    }
    
    /// Get Redis channel name for a document
    fn get_channel_name(&self, document_id: Uuid) -> String {
        format!("{}:{}", self.channel_prefix, document_id)
    }
    
    /// Register a new connection for a document
    pub async fn register_connection(&self, document_id: Uuid) -> Result<broadcast::Receiver<String>, SignalingError> {
        let mut connections = self.connections.write().await;
        
        // Create a new broadcast channel for this document
        let (sender, receiver) = broadcast::channel(1000); // Larger buffer for scalability
        connections.insert(document_id, sender);
        
        // Subscribe to Redis channel for this document
        let channel_name = self.get_channel_name(document_id);
        let mut pubsub = self.redis_client.get_async_connection().await
            .map_err(|e| SignalingError::SerializationError(format!("Failed to get Redis connection: {}", e)))?
            .into_pubsub();
        
        pubsub.subscribe(&channel_name).await
            .map_err(|e| SignalingError::SerializationError(format!("Failed to subscribe to Redis channel: {}", e)))?;
        
        // Start listening to Redis messages
        let connections_clone = self.connections.clone();
        let document_id_clone = document_id;
        tokio::spawn(async move {
            let mut stream = pubsub.on_message();
            while let Some(msg) = stream.next().await {
                if let Ok(payload) = msg.get_payload::<String>() {
                    if let Some(sender) = connections_clone.read().await.get(&document_id_clone) {
                        if let Err(e) = sender.send(payload.clone()) {
                            debug!("Failed to broadcast Redis message: {}", e);
                        }
                    }
                }
            }
        });
        
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
        
        // Clean up last summary data for this document
        let mut last_summary = self.last_summary.write().await;
        last_summary.remove(&document_id);
        
        // Unsubscribe from Redis channel
        let channel_name = self.get_channel_name(document_id);
        let mut conn = self.redis_manager.clone();
        conn.unsubscribe(&channel_name).await
            .map_err(|e| SignalingError::SerializationError(format!("Failed to unsubscribe from Redis channel: {}", e)))?;
        
        info!("Unregistered connection for document: {}", document_id);
        Ok(())
    }
    
    /// Broadcast a message to all connections for a document
    pub async fn broadcast_message(&self, document_id: Uuid, message: &SignalingMessage) -> Result<(), SignalingError> {
        let serialized = serde_json::to_string(message)
            .map_err(|e| SignalingError::SerializationError(e.to_string()))?;
        
        // Publish to Redis channel
        let channel_name = self.get_channel_name(document_id);
        let mut conn = self.redis_manager.clone();
        conn.publish(&channel_name, &serialized).await
            .map_err(|e| SignalingError::SerializationError(format!("Failed to publish to Redis: {}", e)))?;
        
        // Also broadcast locally to any direct connections
        let connections = self.connections.read().await;
        if let Some(sender) = connections.get(&document_id) {
            if let Err(e) = sender.send(serialized) {
                debug!("Local broadcast send error: {}", e);
            }
        }
        
        Ok(())
    }
    
    /// Handle an incoming message
    pub async fn handle_message(&self, document_id: Uuid, message: SignalingMessage) -> Result<(), SignalingError> {
        match message {
            SignalingMessage::JoinDocument { document_id, user_id } => {
                self.handle_join_document(document_id, user_id).await
            },
            SignalingMessage::LeaveDocument { document_id, user_id } => {
                self.handle_leave_document(document_id, user_id).await
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
            /// Handle user joining a document
            async fn handle_join_document(&self, document_id: Uuid, user_id: Uuid) -> Result<(), SignalingError> {
                info!("User {} joined document {}", user_id, document_id);
                
                // Send presence summary to the new user
                let summary = self.create_presence_summary(document_id).await;
                let message = SignalingMessage::PresenceSummary(summary);
                self.broadcast_message(document_id, &message).await?;
                
                Ok(())
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
        
        // Check if we should send a presence summary (every 5 seconds)
        let should_send_summary = {
            let last_summary = self.last_summary.read().await;
            if let Some(last_time) = last_summary.get(&document_id) {
                Utc::now().signed_duration_since(*last_time) > Duration::seconds(5)
            } else {
                true
            }
        };
        
        if should_send_summary {
            // Update last summary timestamp
            {
                let mut last_summary = self.last_summary.write().await;
                last_summary.insert(document_id, Utc::now());
            }
            
            // Create and send presence summary
            let summary = self.create_presence_summary(document_id).await;
            let message = SignalingMessage::PresenceSummary(summary);
            self.broadcast_message(document_id, &message).await?;
        } else {
            // Broadcast individual presence update
            let message = SignalingMessage::PresenceUpdate(update);
            self.broadcast_message(document_id, &message).await?;
        }
        
        Ok(())
    }
    
    /// Create a presence summary for a document
    async fn create_presence_summary(&self, document_id: Uuid) -> crate::message::PresenceSummary {
        let presence = self.presence.read().await;
        let mut users = std::collections::HashMap::new();
        
        if let Some(document_presence) = presence.get(&document_id) {
            // Filter out stale presences (older than 30 seconds)
            let now = Utc::now();
            for (user_id, presence_update) in document_presence {
                if now.signed_duration_since(presence_update.last_active) <= Duration::seconds(30) {
                    let status = if now.signed_duration_since(presence_update.timestamp) <= Duration::seconds(5) {
                        if presence_update.is_typing {
                            crate::message::PresenceStatus::Online
                        } else {
                            crate::message::PresenceStatus::Online
                        }
                    } else {
                        crate::message::PresenceStatus::Away
                    };
                    
                    let presence_user = crate::message::PresenceUser {
                        avatar_url: presence_update.avatar_url.clone(),
                        color: presence_update.color.clone(),
                        status,
                    };
                    users.insert(*user_id, presence_user);
                }
            }
        }
        
        crate::message::PresenceSummary {
            users,
            expires_at: Utc::now() + Duration::seconds(30),
        }
    }
    
    /// Handle cursor update
    async fn handle_cursor_update(&self, document_id: Uuid, cursor: crate::message::CursorPosition) -> Result<(), SignalingError> {
        // Broadcast cursor update
        let message = SignalingMessage::CursorUpdate(cursor);
        self.broadcast_message(document_id, &message).await
    }
    
    /// Handle selection update
    async fn handle_selection_update(
        &self, 
        document_id: Uuid, 
        user_id: Uuid, 
        selection: Option<crate::message::SelectionRange>, 
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