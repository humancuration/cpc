//! Chat service implementation that integrates with the shared messenger

use cpc_messenger::models::{Conversation, Message, Participant, MessageContent, StreamMessage, Emote as MessengerEmote, Badge as MessengerBadge};
use cpc_messenger::services::{ConversationService, MessageService, StreamChatService};
use cpc_messenger::errors::MessengerError;
use async_trait::async_trait;
use uuid::Uuid;
use std::collections::HashMap;
use sqlx::PgPool;
use std::error::Error;
use std::sync::Arc;
use serde_json::json;
use cpc_notification_core::application::service::NotificationService;
use cpc_notification_core::domain::types::{Notification, NotificationCategory, NotificationPriority, ChannelType};

/// Chat service for Twitch-like chat functionality
pub struct ChatService {
    db_pool: PgPool,
    /// In-memory cache of active conversations for performance
    active_conversations: HashMap<Uuid, Conversation>,
    /// Notification service for sending notifications
    notification_service: Arc<NotificationService>,
}
/// Twitch-specific chat message with emotes and badges
#[derive(Debug, Clone)]
pub struct TwitchChatMessage {
    /// The base message from the messenger system
    pub base_message: Message,
    
    /// Emotes used in the message
    pub emotes: Vec<Emote>,
    
    /// Badges the user has
    pub badges: Vec<Badge>,
    
    /// Whether this is a moderator message
    pub is_moderator: bool,
    
    /// Whether this is a subscriber message
    pub is_subscriber: bool,
}

/// Represents an emote in chat
#[derive(Debug, Clone)]
pub struct Emote {
    /// Emote ID
    pub id: Uuid,
    
    /// Emote name (e.g., "Kappa")
    pub name: String,
    
    /// Position in the message where the emote appears
    pub positions: (usize, usize),
}

/// Represents a badge in chat
#[derive(Debug, Clone)]
pub struct Badge {
    /// Badge ID
    pub id: Uuid,
    
    /// Badge name (e.g., "moderator", "subscriber")
    pub name: String,
    
    /// Badge version (for tiered badges)
    pub version: Option<String>,
}

impl From<Emote> for MessengerEmote {
    fn from(emote: Emote) -> Self {
        MessengerEmote {
            id: emote.id,
            name: emote.name,
            positions: emote.positions,
        }
    }
}

impl From<Badge> for MessengerBadge {
    fn from(badge: Badge) -> Self {
        MessengerBadge {
            id: badge.id,
            name: badge.name,
            version: badge.version,
        }
    }
}

impl ChatService {
    /// Create a new chat service
    pub fn new(db_pool: PgPool, notification_service: Arc<NotificationService>) -> Self {
        Self {
            db_pool,
            active_conversations: HashMap::new(),
            notification_service,
        }
    }
    
    /// Create a new chat room for a stream
    pub async fn create_stream_chat(&mut self, stream_id: Uuid, channel_owner_id: Uuid) -> Result<Conversation, Box<dyn Error + Send + Sync>> {
        // Create participants list with channel owner as initial participant
        let participants = vec![Participant::new(channel_owner_id)];
        
        // Create conversation through the messenger service
        let conversation = Conversation::new_group(participants, format!("Stream Chat - {}", stream_id));
        
        // Store in active conversations cache
        self.active_conversations.insert(conversation.id, conversation.clone());
        
        Ok(conversation)
    }
    
    /// Convert a TwitchChatMessage to a StreamMessage
    fn to_stream_message(&self, twitch_message: TwitchChatMessage) -> StreamMessage {
        StreamMessage {
            base_message: twitch_message.base_message,
            emotes: twitch_message.emotes.into_iter().map(|e| e.into()).collect(),
            badges: twitch_message.badges.into_iter().map(|b| b.into()).collect(),
            is_moderator: twitch_message.is_moderator,
            is_subscriber: twitch_message.is_subscriber,
        }
    }
    
    
    /// Send a chat message to a stream
    pub async fn send_chat_message(
        &self,
        conversation_id: Uuid,
        sender_id: Uuid,
        sender_name: String,
        content: String,
        emotes: Vec<Emote>,
        badges: Vec<Badge>,
        is_moderator: bool,
        is_subscriber: bool,
    ) -> Result<TwitchChatMessage, Box<dyn Error + Send + Sync>> {
        // Create the base message through the messenger service
        let base_message = Message::new_text(conversation_id, sender_id, content.clone());
        
        let twitch_message = TwitchChatMessage {
            base_message,
            emotes,
            badges,
            is_moderator,
            is_subscriber,
        };
        
        // Send notification for the new chat message
        let notif = Notification::new_immediate(
            conversation_id.to_string(),
            NotificationCategory::Streaming,
            NotificationPriority::Normal,
            "New Chat Message".into(),
            format!("{}: {}", sender_name, Self::content_preview(&content, 25)),
            json!({"type": "chat", "conversation_id": conversation_id}),
            vec![ChannelType::InApp, ChannelType::Push],
        );
        let service_clone = self.notification_service.clone();
        tokio::spawn(async move {
            if let Err(e) = service_clone.send(notif).await {
                tracing::error!("Chat notification failed: {}", e);
            }
        });
        
        // In a real implementation, we would:
        // 1. Validate emotes and badges
        // 2. Check if user has permission to use them
        // 3. Store the message in the database
        // 4. Broadcast to all connected viewers
        
        Ok(twitch_message)
        
        /// Create a preview of content by truncating it to a maximum length
        fn content_preview(content: &str, max_length: usize) -> String {
            if content.len() <= max_length {
                content.to_string()
            } else {
                format!("{}...", &content[..max_length])
            }
        }
    }
    /// Get recent chat messages for a stream
    pub async fn get_recent_messages(&self, conversation_id: Uuid, limit: usize) -> Result<Vec<TwitchChatMessage>, Box<dyn Error + Send + Sync>> {
        // In a real implementation, we would fetch messages from the database
        // and convert them to TwitchChatMessage format
        
        Ok(Vec::new())
    }
    
    /// Add an emote to the system
    pub async fn add_emote(&self, emote: Emote) -> Result<(), Box<dyn Error + Send + Sync>> {
        // In a real implementation, we would store the emote in the database
        
        Ok(())
    }
    
    /// Get emote by name
    pub async fn get_emote_by_name(&self, name: &str) -> Result<Option<Emote>, Box<dyn Error + Send + Sync>> {
        // In a real implementation, we would fetch the emote from the database
        
        Ok(None)
    }
}

// Implement the StreamChatService trait from the messenger
#[async_trait]
impl StreamChatService for ChatService {
    async fn create_stream_chat(&self, stream_id: Uuid, channel_owner_id: Uuid) -> Result<Conversation, MessengerError> {
        // In a real implementation, we would create the chat room
        // For now, we'll return a mock conversation
        let participants = vec![Participant::new(channel_owner_id)];
        let conversation = Conversation::new_group(participants, format!("Stream Chat - {}", stream_id));
        Ok(conversation)
    }
    
    async fn send_stream_message(
        &self,
        conversation_id: Uuid,
        sender_id: Uuid,
        content: String,
        emotes: Vec<MessengerEmote>,
        badges: Vec<MessengerBadge>,
        is_moderator: bool,
        is_subscriber: bool,
    ) -> Result<StreamMessage, MessengerError> {
        // Create the base message
        let base_message = Message::new_text(conversation_id, sender_id, content);
        
        let stream_message = StreamMessage {
            base_message,
            emotes,
            badges,
            is_moderator,
            is_subscriber,
        };
        
        Ok(stream_message)
    }
    
    async fn get_recent_stream_messages(&self, conversation_id: Uuid, limit: usize) -> Result<Vec<StreamMessage>, MessengerError> {
        // In a real implementation, we would fetch messages from the database
        Ok(Vec::new())
    }
    
    async fn add_emote(&self, emote: MessengerEmote) -> Result<(), MessengerError> {
        // In a real implementation, we would store the emote in the database
        Ok(())
    }
    
    async fn get_emote_by_name(&self, name: &str) -> Result<Option<MessengerEmote>, MessengerError> {
        // In a real implementation, we would fetch the emote from the database
        Ok(None)
    }
}

// Implement the ConversationService trait from the messenger
#[async_trait]
impl ConversationService for ChatService {
    async fn create_conversation(&self, participants: Vec<Participant>, is_group: bool, group_name: Option<String>) -> Result<Conversation, cpc_messenger::errors::MessengerError> {
        // Implementation would go here
        todo!("Implement create_conversation")
    }
    
    async fn get_conversation(&self, conversation_id: Uuid) -> Result<Conversation, cpc_messenger::errors::MessengerError> {
        // Implementation would go here
        todo!("Implement get_conversation")
    }
    
    async fn add_participant(&self, conversation_id: Uuid, participant: Participant) -> Result<Conversation, cpc_messenger::errors::MessengerError> {
        // Implementation would go here
        todo!("Implement add_participant")
    }
    
    async fn remove_participant(&self, conversation_id: Uuid, user_id: Uuid) -> Result<Conversation, cpc_messenger::errors::MessengerError> {
        // Implementation would go here
        todo!("Implement remove_participant")
    }
    
    async fn update_settings(&self, conversation_id: Uuid, settings: cpc_messenger::models::ConversationSettings) -> Result<Conversation, cpc_messenger::errors::MessengerError> {
        // Implementation would go here
        todo!("Implement update_settings")
    }
    
    async fn get_user_conversations(&self, user_id: Uuid) -> Result<Vec<Conversation>, cpc_messenger::errors::MessengerError> {
        // Implementation would go here
        todo!("Implement get_user_conversations")
    }
}

// Implement the MessageService trait from the messenger
#[async_trait]
impl MessageService for ChatService {
    async fn send_message(&self, conversation_id: Uuid, sender_id: Uuid, content: cpc_messenger::models::MessageContent) -> Result<Message, cpc_messenger::errors::MessengerError> {
        // Implementation would go here
        todo!("Implement send_message")
    }
    
    async fn get_conversation_messages(&self, conversation_id: Uuid, limit: usize, before_message_id: Option<Uuid>) -> Result<Vec<Message>, cpc_messenger::errors::MessengerError> {
        // Implementation would go here
        todo!("Implement get_conversation_messages")
    }
    
    async fn get_message(&self, message_id: Uuid) -> Result<Message, cpc_messenger::errors::MessengerError> {
        // Implementation would go here
        todo!("Implement get_message")
    }
    
    async fn update_message_status(&self, update: cpc_messenger::models::MessageStatusUpdate) -> Result<(), cpc_messenger::errors::MessengerError> {
        // Implementation would go here
        todo!("Implement update_message_status")
    }
    
    async fn mark_messages_read(&self, conversation_id: Uuid, user_id: Uuid, up_to_message_id: Uuid) -> Result<usize, cpc_messenger::errors::MessengerError> {
        // Implementation would go here
        todo!("Implement mark_messages_read")
    }
    
    async fn delete_message(&self, message_id: Uuid, user_id: Uuid) -> Result<(), cpc_messenger::errors::MessengerError> {
        // Implementation would go here
        todo!("Implement delete_message")
    }
}