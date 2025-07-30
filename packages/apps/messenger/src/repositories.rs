//! Repository traits for data access in the Messenger application

use async_trait::async_trait;
use uuid::Uuid;
use std::collections::HashMap;

use messenger_domain::{
    models::{Conversation, Message, MediaReference, Participant},
    errors::MessengerError,
    services::UserPresence,
};

/// Repository for conversation operations
#[async_trait]
pub trait ConversationRepository: Send + Sync {
    /// Create a new conversation
    async fn create(&self, conversation: &Conversation) -> Result<(), MessengerError>;
    
    /// Find a conversation by ID
    async fn find_by_id(&self, id: Uuid) -> Result<Conversation, MessengerError>;
    
    /// Update an existing conversation
    async fn update(&self, conversation: &Conversation) -> Result<(), MessengerError>;
    
    /// Delete a conversation
    async fn delete(&self, id: Uuid) -> Result<(), MessengerError>;
    
    /// Find conversations by participant
    async fn find_by_participant(&self, user_id: Uuid) -> Result<Vec<Conversation>, MessengerError>;
    
    /// Find conversations by multiple participants (for finding existing 1:1 conversations)
    async fn find_by_participants(&self, participant_ids: Vec<Uuid>) -> Result<Option<Conversation>, MessengerError>;
}

/// Repository for message operations
#[async_trait]
pub trait MessageRepository: Send + Sync {
    /// Create a new message
    async fn create(&self, message: &Message) -> Result<(), MessengerError>;
    
    /// Find a message by ID
    async fn find_by_id(&self, id: Uuid) -> Result<Message, MessengerError>;
    
    /// Update an existing message
    async fn update(&self, message: &Message) -> Result<(), MessengerError>;
    
    /// Delete a message
    async fn delete(&self, id: Uuid) -> Result<(), MessengerError>;
    
    /// Find messages by conversation
    async fn find_by_conversation(&self, conversation_id: Uuid, limit: usize, before_message_id: Option<Uuid>) -> Result<Vec<Message>, MessengerError>;
    
    /// Mark messages as read up to a specific message
    async fn mark_messages_read(&self, conversation_id: Uuid, user_id: Uuid, up_to_message_id: Uuid) -> Result<usize, MessengerError>;
    
    /// Get unread message count for a user in a conversation
    async fn get_unread_count(&self, conversation_id: Uuid, user_id: Uuid) -> Result<usize, MessengerError>;
}

/// Repository for media operations
#[async_trait]
pub trait MediaRepository: Send + Sync {
    /// Store media data
    async fn store_media(&self, media_reference: &MediaReference, data: Vec<u8>) -> Result<(), MessengerError>;
    
    /// Find a media reference by ID
    async fn find_by_id(&self, id: Uuid) -> Result<MediaReference, MessengerError>;
    
    /// Delete media
    async fn delete(&self, id: Uuid) -> Result<(), MessengerError>;
    
    /// Get media data
    async fn get_media_data(&self, id: Uuid) -> Result<Vec<u8>, MessengerError>;
}

/// Repository for presence operations
#[async_trait]
pub trait PresenceRepository: Send + Sync {
    /// Update user presence
    async fn update_presence(&self, user_id: Uuid, presence: UserPresence) -> Result<(), MessengerError>;
    
    /// Get user presence
    async fn get_presence(&self, user_id: Uuid) -> Result<UserPresence, MessengerError>;
    
    /// Get presence for multiple users
    async fn get_multiple_presence(&self, user_ids: Vec<Uuid>) -> Result<HashMap<Uuid, UserPresence>, MessengerError>;
    
    /// Get users who are online
    async fn get_online_users(&self) -> Result<Vec<Uuid>, MessengerError>;
}