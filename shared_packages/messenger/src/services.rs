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
    
    /// Update message content
    async fn update_message(&self, message_id: Uuid, user_id: Uuid, new_content: crate::models::MessageContent) -> Result<Message, MessengerError>;
    
    /// Update message delivery status
    async fn update_message_status(&self, update: MessageStatusUpdate) -> Result<(), MessengerError>;
    
    /// Mark messages as read
    async fn mark_messages_read(&self, conversation_id: Uuid, user_id: Uuid, up_to_message_id: Uuid) -> Result<usize, MessengerError>;
    
    /// Delete a message (soft delete)
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

/// Service for reaction operations
#[async_trait]
pub trait ReactionService: Send + Sync {
    /// Add a reaction to a message
    async fn add_reaction(&self, message_id: Uuid, user_id: Uuid, reaction_type: String) -> Result<crate::models::Reaction, MessengerError>;
    
    /// Remove a reaction from a message
    async fn remove_reaction(&self, message_id: Uuid, user_id: Uuid, reaction_type: String) -> Result<(), MessengerError>;
    
    /// Get all reactions for a message
    async fn get_message_reactions(&self, message_id: Uuid) -> Result<Vec<crate::models::Reaction>, MessengerError>;
}

/// Service for thread operations
#[async_trait]
pub trait ThreadService: Send + Sync {
    /// Create a new thread from a message
    async fn create_thread(&self, parent_message_id: Uuid, conversation_id: Uuid) -> Result<crate::models::MessageThread, MessengerError>;
    
    /// Get a thread by ID
    async fn get_thread(&self, thread_id: crate::models::ThreadId) -> Result<crate::models::MessageThread, MessengerError>;
    
    /// Get messages in a thread
    async fn get_thread_messages(&self, thread_id: crate::models::ThreadId, limit: usize, before_message_id: Option<Uuid>) -> Result<Vec<crate::models::Message>, MessengerError>;
    
    /// Add a message to a thread
    async fn add_message_to_thread(&self, thread_id: crate::models::ThreadId, message: crate::models::Message) -> Result<(), MessengerError>;
}

/// Service for group management operations
#[async_trait]
pub trait GroupService: Send + Sync {
    /// Update group settings
    async fn update_group_settings(&self, conversation_id: Uuid, user_id: Uuid, settings: crate::models::ConversationSettings) -> Result<crate::models::Conversation, MessengerError>;
    
    /// Transfer admin rights to another user
    async fn transfer_admin(&self, conversation_id: Uuid, current_admin_id: Uuid, new_admin_id: Uuid) -> Result<crate::models::Conversation, MessengerError>;
    
    /// Ban a participant from a group
    async fn ban_participant(&self, conversation_id: Uuid, admin_id: Uuid, user_id: Uuid) -> Result<crate::models::Conversation, MessengerError>;
    
    /// Update participant permissions
    async fn update_participant_permissions(&self, conversation_id: Uuid, admin_id: Uuid, user_id: Uuid, permissions: crate::models::ParticipantPermissions) -> Result<crate::models::Conversation, MessengerError>;
}

/// Service for moderation operations
#[async_trait]
pub trait ModerationService: Send + Sync {
    /// Delete a message
    async fn delete_message(&self, message_id: Uuid, moderator_id: Uuid) -> Result<(), MessengerError>;
    
    /// Timeout a user
    async fn timeout_user(&self, conversation_id: Uuid, moderator_id: Uuid, user_id: Uuid, duration: std::time::Duration) -> Result<(), MessengerError>;
    
    /// Remove timeout from a user
    async fn remove_timeout(&self, conversation_id: Uuid, moderator_id: Uuid, user_id: Uuid) -> Result<(), MessengerError>;
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

/// Service for stream chat operations
#[async_trait]
pub trait StreamChatService: Send + Sync {
    /// Create a stream chat room
    async fn create_stream_chat(&self, stream_id: Uuid, channel_owner_id: Uuid) -> Result<Conversation, MessengerError>;
    
    /// Send a stream chat message
    async fn send_stream_message(
        &self,
        conversation_id: Uuid,
        sender_id: Uuid,
        content: String,
        emotes: Vec<crate::models::Emote>,
        badges: Vec<crate::models::Badge>,
        is_moderator: bool,
        is_subscriber: bool,
    ) -> Result<crate::models::StreamMessage, MessengerError>;
    
    /// Get recent stream messages
    async fn get_recent_stream_messages(&self, conversation_id: Uuid, limit: usize) -> Result<Vec<crate::models::StreamMessage>, MessengerError>;
    
    /// Add an emote to the system
    async fn add_emote(&self, emote: crate::models::Emote) -> Result<(), MessengerError>;
    
    /// Get emote by name
    async fn get_emote_by_name(&self, name: &str) -> Result<Option<crate::models::Emote>, MessengerError>;
}