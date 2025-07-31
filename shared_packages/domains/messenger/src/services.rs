//! Domain service interfaces for the Messenger application

use crate::{models::{Conversation, Message, Participant, MessageStatusUpdate}, errors::MessengerError};
use async_trait::async_trait;
use uuid::Uuid;
use std::collections::HashMap;

/// Service for conversation operations
#[async_trait]
pub trait ConversationService: Send + Sync {
    /// Create a new conversation
    async fn create_conversation(&self, participants: Vec<Participant>, is_group: bool, group_name: Option<String>) -> Result<Conversation, MessengerError>;
    
    /// Get a conversation by ID
    async fn get_conversation(&self, conversation_id: Uuid) -> Result<Conversation, MessengerError>;
    
    /// Add a participant to a conversation
    async fn add_participant(&self, conversation_id: Uuid, participant: Participant) -> Result<Conversation, MessengerError>;
    
    /// Remove a participant from a conversation
    async fn remove_participant(&self, conversation_id: Uuid, user_id: Uuid) -> Result<Conversation, MessengerError>;
    
    /// Update conversation settings
    async fn update_settings(&self, conversation_id: Uuid, settings: crate::models::ConversationSettings) -> Result<Conversation, MessengerError>;
    
    /// Get conversations for a user
    async fn get_user_conversations(&self, user_id: Uuid) -> Result<Vec<Conversation>, MessengerError>;
}

/// Service for message operations
#[async_trait]
pub trait MessageService: Send + Sync {
    /// Send a new message
    async fn send_message(&self, conversation_id: Uuid, sender_id: Uuid, content: crate::models::MessageContent) -> Result<Message, MessengerError>;
    
    /// Get messages for a conversation
    async fn get_conversation_messages(&self, conversation_id: Uuid, limit: usize, before_message_id: Option<Uuid>) -> Result<Vec<Message>, MessengerError>;
    
    /// Get a message by ID
    async fn get_message(&self, message_id: Uuid) -> Result<Message, MessengerError>;
    
    /// Update message delivery status
    async fn update_message_status(&self, update: MessageStatusUpdate) -> Result<(), MessengerError>;
    
    /// Mark messages as read
    async fn mark_messages_read(&self, conversation_id: Uuid, user_id: Uuid, up_to_message_id: Uuid) -> Result<usize, MessengerError>;
    
    /// Delete a message
    async fn delete_message(&self, message_id: Uuid, user_id: Uuid) -> Result<(), MessengerError>;
}

/// Service for media operations
#[async_trait]
pub trait MediaService: Send + Sync {
    /// Upload media
    async fn upload_media(&self, media_data: Vec<u8>, media_type: crate::models::MediaType, user_id: Uuid) -> Result<crate::models::MediaReference, MessengerError>;
    
    /// Get media by ID
    async fn get_media(&self, media_id: Uuid) -> Result<crate::models::MediaReference, MessengerError>;
    
    /// Delete media
    async fn delete_media(&self, media_id: Uuid, user_id: Uuid) -> Result<(), MessengerError>;
}

/// Service for presence operations
#[async_trait]
pub trait PresenceService: Send + Sync {
    /// Update user presence
    async fn update_presence(&self, user_id: Uuid, status: UserPresence) -> Result<(), MessengerError>;
    
    /// Get user presence
    async fn get_presence(&self, user_id: Uuid) -> Result<UserPresence, MessengerError>;
    
    /// Get presence for multiple users
    async fn get_multiple_presence(&self, user_ids: Vec<Uuid>) -> Result<HashMap<Uuid, UserPresence>, MessengerError>;
}

/// User presence status
#[derive(Debug, Clone)]
pub enum UserPresence {
    /// User is online
    Online,
    
    /// User is away
    Away,
    
    /// User is offline
    Offline,
    
    /// User is busy
    Busy,
}