//! Database implementations for the Messenger application

use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use std::collections::HashMap;
use std::str::FromStr;

use messenger_domain::{
    models::{Conversation, Message, MediaReference, Participant, ConversationSettings, ParticipantPermissions, MessageContent, MediaType, DeliveryStatus},
    errors::MessengerError,
    services::UserPresence,
};
use messenger_app::repositories::{ConversationRepository, MessageRepository, MediaRepository, PresenceRepository};

/// PostgreSQL implementation of ConversationRepository
pub struct PostgresConversationRepository {
    pool: PgPool,
}

impl PostgresConversationRepository {
    /// Create a new PostgreSQL conversation repository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ConversationRepository for PostgresConversationRepository {
    async fn create(&self, conversation: &Conversation) -> Result<(), MessengerError> {
        // Start a transaction
        let mut tx = self.pool.begin().await
            .map_err(|e| MessengerError::StorageError { message: format!("Failed to start transaction: {}", e) })?;
        
        // Insert the conversation
        sqlx::query(
            "INSERT INTO conversations (id, created_at, is_group, group_name)
             VALUES ($1, $2, $3, $4)"
        )
        .bind(conversation.id)
        .bind(conversation.created_at)
        .bind(conversation.is_group)
        .bind(&conversation.group_name)
        .execute(&mut *tx)
        .await
        .map_err(|e| MessengerError::StorageError { message: format!("Failed to insert conversation: {}", e) })?;
        
        // Insert participants
        for participant in &conversation.participants {
            sqlx::query(
                "INSERT INTO participants (conversation_id, user_id, joined_at, last_read_message_id, can_send_messages, can_manage_participants, can_change_settings, can_delete_messages)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
            )
            .bind(conversation.id)
            .bind(participant.user_id)
            .bind(participant.joined_at)
            .bind(participant.last_read_message_id)
            .bind(participant.permissions.can_send_messages)
            .bind(participant.permissions.can_manage_participants)
            .bind(participant.permissions.can_change_settings)
            .bind(participant.permissions.can_delete_messages)
            .execute(&mut *tx)
            .await
            .map_err(|e| MessengerError::StorageError { message: format!("Failed to insert participant: {}", e) })?;
        }
        
        // Commit the transaction
        tx.commit().await
            .map_err(|e| MessengerError::StorageError { message: format!("Failed to commit transaction: {}", e) })?;
        
        Ok(())
    }
    
    async fn find_by_id(&self, id: Uuid) -> Result<Conversation, MessengerError> {
        // Get the conversation
        let conversation_row = sqlx::query(
            "SELECT id, created_at, is_group, group_name
             FROM conversations
             WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| MessengerError::StorageError { message: format!("Failed to query conversation: {}", e) })?
        .ok_or(MessengerError::ConversationNotFound { id })?;
        
        // Get participants
        let participant_rows = sqlx::query(
            "SELECT user_id, joined_at, last_read_message_id, can_send_messages, can_manage_participants, can_change_settings, can_delete_messages
             FROM participants
             WHERE conversation_id = $1
             ORDER BY joined_at"
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| MessengerError::StorageError { message: format!("Failed to query participants: {}", e) })?;
        
        let participants = participant_rows.into_iter().map(|row| {
            Participant {
                user_id: row.get("user_id"),
                joined_at: row.get("joined_at"),
                last_read_message_id: row.get("last_read_message_id"),
                permissions: ParticipantPermissions {
                    can_send_messages: row.get("can_send_messages"),
                    can_manage_participants: row.get("can_manage_participants"),
                    can_change_settings: row.get("can_change_settings"),
                    can_delete_messages: row.get("can_delete_messages"),
                },
            }
        }).collect();
        
        // For now, we'll use default settings
        // In a real implementation, we would store and retrieve settings from the database
        let settings = ConversationSettings::default();
        
        let conversation = Conversation {
            id: conversation_row.get("id"),
            created_at: conversation_row.get("created_at"),
            is_group: conversation_row.get("is_group"),
            participants,
            settings,
            group_name: conversation_row.get("group_name"),
        };
        
        Ok(conversation)
    }
    
    async fn update(&self, conversation: &Conversation) -> Result<(), MessengerError> {
        // Start a transaction
        let mut tx = self.pool.begin().await
            .map_err(|e| MessengerError::StorageError { message: format!("Failed to start transaction: {}", e) })?;
        
        // Update the conversation
        sqlx::query(
            "UPDATE conversations
             SET group_name = $2
             WHERE id = $1"
        )
        .bind(conversation.id)
        .bind(&conversation.group_name)
        .execute(&mut *tx)
        .await
        .map_err(|e| MessengerError::StorageError { message: format!("Failed to update conversation: {}", e) })?;
        
        // Delete existing participants
        sqlx::query(
            "DELETE FROM participants
             WHERE conversation_id = $1"
        )
        .bind(conversation.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| MessengerError::StorageError { message: format!("Failed to delete participants: {}", e) })?;
        
        // Insert updated participants
        for participant in &conversation.participants {
            sqlx::query(
                "INSERT INTO participants (conversation_id, user_id, joined_at, last_read_message_id, can_send_messages, can_manage_participants, can_change_settings, can_delete_messages)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
            )
            .bind(conversation.id)
            .bind(participant.user_id)
            .bind(participant.joined_at)
            .bind(participant.last_read_message_id)
            .bind(participant.permissions.can_send_messages)
            .bind(participant.permissions.can_manage_participants)
            .bind(participant.permissions.can_change_settings)
            .bind(participant.permissions.can_delete_messages)
            .execute(&mut *tx)
            .await
            .map_err(|e| MessengerError::StorageError { message: format!("Failed to insert participant: {}", e) })?;
        }
        
        // Commit the transaction
        tx.commit().await
            .map_err(|e| MessengerError::StorageError { message: format!("Failed to commit transaction: {}", e) })?;
        
        Ok(())
    }
    
    async fn delete(&self, id: Uuid) -> Result<(), MessengerError> {
        // In a real implementation, we would need to handle cascading deletes
        // or mark as deleted rather than actually deleting
        sqlx::query(
            "DELETE FROM conversations
             WHERE id = $1"
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| MessengerError::StorageError { message: format!("Failed to delete conversation: {}", e) })?;
        
        Ok(())
    }
    
    async fn find_by_participant(&self, user_id: Uuid) -> Result<Vec<Conversation>, MessengerError> {
        // Get conversations for the user
        let conversation_rows = sqlx::query(
            "SELECT DISTINCT c.id, c.created_at, c.is_group, c.group_name
             FROM conversations c
             JOIN participants p ON c.id = p.conversation_id
             WHERE p.user_id = $1
             ORDER BY c.created_at DESC"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| MessengerError::StorageError { message: format!("Failed to query conversations: {}", e) })?;
        
        let mut conversations = Vec::new();
        
        for conversation_row in conversation_rows {
            let conversation_id: Uuid = conversation_row.get("id");
            
            // Get participants for this conversation
            let participant_rows = sqlx::query(
                "SELECT user_id, joined_at, last_read_message_id, can_send_messages, can_manage_participants, can_change_settings, can_delete_messages
                 FROM participants
                 WHERE conversation_id = $1
                 ORDER BY joined_at"
            )
            .bind(conversation_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| MessengerError::StorageError { message: format!("Failed to query participants: {}", e) })?;
            
            let participants = participant_rows.into_iter().map(|row| {
                Participant {
                    user_id: row.get("user_id"),
                    joined_at: row.get("joined_at"),
                    last_read_message_id: row.get("last_read_message_id"),
                    permissions: ParticipantPermissions {
                        can_send_messages: row.get("can_send_messages"),
                        can_manage_participants: row.get("can_manage_participants"),
                        can_change_settings: row.get("can_change_settings"),
                        can_delete_messages: row.get("can_delete_messages"),
                    },
                }
            }).collect();
            
            // For now, we'll use default settings
            let settings = ConversationSettings::default();
            
            let conversation = Conversation {
                id: conversation_row.get("id"),
                created_at: conversation_row.get("created_at"),
                is_group: conversation_row.get("is_group"),
                participants,
                settings,
                group_name: conversation_row.get("group_name"),
            };
            
            conversations.push(conversation);
        }
        
        Ok(conversations)
    }
    
    async fn find_by_participants(&self, participant_ids: Vec<Uuid>) -> Result<Option<Conversation>, MessengerError> {
        if participant_ids.is_empty() {
            return Ok(None);
        }
        
        // For 1:1 conversations, we can look for conversations with exactly these two participants
        if participant_ids.len() == 2 {
            let conversation_row = sqlx::query(
                "SELECT c.id
                 FROM conversations c
                 JOIN participants p ON c.id = p.conversation_id
                 WHERE c.is_group = false
                 GROUP BY c.id, c.created_at
                 HAVING COUNT(DISTINCT p.user_id) = 2
                    AND COUNT(CASE WHEN p.user_id = $1 THEN 1 END) = 1
                    AND COUNT(CASE WHEN p.user_id = $2 THEN 1 END) = 1"
            )
            .bind(participant_ids[0])
            .bind(participant_ids[1])
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| MessengerError::StorageError { message: format!("Failed to query conversation: {}", e) })?;
            
            if let Some(row) = conversation_row {
                let conversation_id: Uuid = row.get("id");
                let conversation = self.find_by_id(conversation_id).await?;
                return Ok(Some(conversation));
            }
        }
        
        Ok(None)
    }
}

/// PostgreSQL implementation of MessageRepository
pub struct PostgresMessageRepository {
    pool: PgPool,
}

impl PostgresMessageRepository {
    /// Create a new PostgreSQL message repository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MessageRepository for PostgresMessageRepository {
    async fn create(&self, message: &Message) -> Result<(), MessengerError> {
        let (content_type, content_text, media_id) = match &message.content {
            MessageContent::Text(text) => ("text".to_string(), Some(text.clone()), None),
            MessageContent::Media(media_ref) => ("media".to_string(), None, Some(media_ref.id)),
            MessageContent::System(_) => ("system".to_string(), None, None),
        };
        
        let (status_code, sent_at, delivered_at, read_at) = match &message.delivery_status {
            DeliveryStatus::Pending => (0, None, None, None),
            DeliveryStatus::Sent(time) => (1, Some(*time), None, None),
            DeliveryStatus::Delivered(time) => (2, Some(message.sent_at), Some(*time), None),
            DeliveryStatus::Read(time) => (3, Some(message.sent_at), Some(message.sent_at), Some(*time)),
        };
        
        sqlx::query(
            "INSERT INTO messages (id, conversation_id, sender_id, content_type, content_text, media_id, sent_at, status_code, delivered_at, read_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"
        )
        .bind(message.id)
        .bind(message.conversation_id)
        .bind(message.sender_id)
        .bind(content_type)
        .bind(&content_text)
        .bind(media_id)
        .bind(message.sent_at)
        .bind(status_code)
        .bind(sent_at)
        .bind(delivered_at)
        .bind(read_at)
        .execute(&self.pool)
        .await
        .map_err(|e| MessengerError::StorageError { message: format!("Failed to insert message: {}", e) })?;
        
        Ok(())
    }
    
    async fn find_by_id(&self, id: Uuid) -> Result<Message, MessengerError> {
        let row = sqlx::query(
            "SELECT id, conversation_id, sender_id, content_type, content_text, media_id, sent_at, status_code, delivered_at, read_at
             FROM messages
             WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| MessengerError::StorageError { message: format!("Failed to query message: {}", e) })?
        .ok_or(MessengerError::MessageNotFound { id })?;
        
        let content_type: String = row.get("content_type");
        let content = match content_type.as_str() {
            "text" => MessageContent::Text(row.get("content_text")),
            "media" => {
                // In a real implementation, we would fetch the media reference from the media repository
                // For now, we'll create a placeholder
                MessageContent::Media(MediaReference {
                    id: row.get("media_id"),
                    media_type: MediaType::Image, // Placeholder
                    storage_location: "placeholder".to_string(),
                    thumbnail: None,
                    size_bytes: 0,
                    filename: None,
                })
            },
            "system" => {
                // In a real implementation, we would reconstruct the system message
                // For now, we'll create a placeholder
                MessageContent::System(messenger_domain::models::SystemMessage::UserJoined { user_id: Uuid::nil() })
            },
            _ => return Err(MessengerError::StorageError { message: "Unknown content type".to_string() }),
        };
        
        let status_code: i32 = row.get("status_code");
        let delivery_status = match status_code {
            0 => DeliveryStatus::Pending,
            1 => DeliveryStatus::Sent(row.get("sent_at")),
            2 => DeliveryStatus::Delivered(row.get("delivered_at")),
            3 => DeliveryStatus::Read(row.get("read_at")),
            _ => return Err(MessengerError::StorageError { message: "Unknown status code".to_string() }),
        };
        
        let message = Message {
            id: row.get("id"),
            conversation_id: row.get("conversation_id"),
            sender_id: row.get("sender_id"),
            content,
            sent_at: row.get("sent_at"),
            delivery_status,
        };
        
        Ok(message)
    }
    
    async fn update(&self, message: &Message) -> Result<(), MessengerError> {
        let (status_code, sent_at, delivered_at, read_at) = match &message.delivery_status {
            DeliveryStatus::Pending => (0, None, None, None),
            DeliveryStatus::Sent(time) => (1, Some(*time), None, None),
            DeliveryStatus::Delivered(time) => (2, Some(message.sent_at), Some(*time), None),
            DeliveryStatus::Read(time) => (3, Some(message.sent_at), Some(message.sent_at), Some(*time)),
        };
        
        sqlx::query(
            "UPDATE messages
             SET status_code = $2, sent_at = $3, delivered_at = $4, read_at = $5
             WHERE id = $1"
        )
        .bind(message.id)
        .bind(status_code)
        .bind(sent_at)
        .bind(delivered_at)
        .bind(read_at)
        .execute(&self.pool)
        .await
        .map_err(|e| MessengerError::StorageError { message: format!("Failed to update message: {}", e) })?;
        
        Ok(())
    }
    
    async fn delete(&self, id: Uuid) -> Result<(), MessengerError> {
        sqlx::query(
            "DELETE FROM messages
             WHERE id = $1"
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| MessengerError::StorageError { message: format!("Failed to delete message: {}", e) })?;
        
        Ok(())
    }
    
    async fn find_by_conversation(&self, conversation_id: Uuid, limit: usize, before_message_id: Option<Uuid>) -> Result<Vec<Message>, MessengerError> {
        let query = if let Some(before_id) = before_message_id {
            sqlx::query(
                "SELECT id, conversation_id, sender_id, content_type, content_text, media_id, sent_at, status_code, delivered_at, read_at
                 FROM messages
                 WHERE conversation_id = $1 AND id < $2
                 ORDER BY sent_at DESC
                 LIMIT $3"
            )
            .bind(conversation_id)
            .bind(before_id)
            .bind(limit as i64)
        } else {
            sqlx::query(
                "SELECT id, conversation_id, sender_id, content_type, content_text, media_id, sent_at, status_code, delivered_at, read_at
                 FROM messages
                 WHERE conversation_id = $1
                 ORDER BY sent_at DESC
                 LIMIT $2"
            )
            .bind(conversation_id)
            .bind(limit as i64)
        };
        
        let rows = query
            .fetch_all(&self.pool)
            .await
            .map_err(|e| MessengerError::StorageError { message: format!("Failed to query messages: {}", e) })?;
        
        let mut messages = Vec::new();
        
        for row in rows {
            let content_type: String = row.get("content_type");
            let content = match content_type.as_str() {
                "text" => MessageContent::Text(row.get("content_text")),
                "media" => {
                    // In a real implementation, we would fetch the media reference from the media repository
                    // For now, we'll create a placeholder
                    MessageContent::Media(MediaReference {
                        id: row.get("media_id"),
                        media_type: MediaType::Image, // Placeholder
                        storage_location: "placeholder".to_string(),
                        thumbnail: None,
                        size_bytes: 0,
                        filename: None,
                    })
                },
                "system" => {
                    // In a real implementation, we would reconstruct the system message
                    // For now, we'll create a placeholder
                    MessageContent::System(messenger_domain::models::SystemMessage::UserJoined { user_id: Uuid::nil() })
                },
                _ => return Err(MessengerError::StorageError { message: "Unknown content type".to_string() }),
            };
            
            let status_code: i32 = row.get("status_code");
            let delivery_status = match status_code {
                0 => DeliveryStatus::Pending,
                1 => DeliveryStatus::Sent(row.get("sent_at")),
                2 => DeliveryStatus::Delivered(row.get("delivered_at")),
                3 => DeliveryStatus::Read(row.get("read_at")),
                _ => return Err(MessengerError::StorageError { message: "Unknown status code".to_string() }),
            };
            
            let message = Message {
                id: row.get("id"),
                conversation_id: row.get("conversation_id"),
                sender_id: row.get("sender_id"),
                content,
                sent_at: row.get("sent_at"),
                delivery_status,
            };
            
            messages.push(message);
        }
        
        // Reverse to get chronological order
        messages.reverse();
        
        Ok(messages)
    }
    
    async fn mark_messages_read(&self, conversation_id: Uuid, user_id: Uuid, up_to_message_id: Uuid) -> Result<usize, MessengerError> {
        // First, get the timestamp of the message we're marking as read up to
        let message_row = sqlx::query(
            "SELECT sent_at
             FROM messages
             WHERE id = $1 AND conversation_id = $2"
        )
        .bind(up_to_message_id)
        .bind(conversation_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| MessengerError::StorageError { message: format!("Failed to query message: {}", e) })?
        .ok_or(MessengerError::MessageNotFound { id: up_to_message_id })?;
        
        let up_to_timestamp: chrono::DateTime<chrono::Utc> = message_row.get("sent_at");
        
        // Update all messages in the conversation sent before or at this timestamp
        let result = sqlx::query(
            "UPDATE messages
             SET status_code = 3, read_at = NOW()
             WHERE conversation_id = $1 AND sender_id != $2 AND sent_at <= $3 AND status_code < 3"
        )
        .bind(conversation_id)
        .bind(user_id)
        .bind(up_to_timestamp)
        .execute(&self.pool)
        .await
        .map_err(|e| MessengerError::StorageError { message: format!("Failed to update messages: {}", e) })?;
        
        // Also update the participant's last read message
        sqlx::query(
            "UPDATE participants
             SET last_read_message_id = $3
             WHERE conversation_id = $1 AND user_id = $2"
        )
        .bind(conversation_id)
        .bind(user_id)
        .bind(up_to_message_id)
        .execute(&self.pool)
        .await
        .map_err(|e| MessengerError::StorageError { message: format!("Failed to update participant: {}", e) })?;
        
        Ok(result.rows_affected() as usize)
    }
    
    async fn get_unread_count(&self, conversation_id: Uuid, user_id: Uuid) -> Result<usize, MessengerError> {
        // Get the last read message ID for this user in this conversation
        let participant_row = sqlx::query(
            "SELECT last_read_message_id
             FROM participants
             WHERE conversation_id = $1 AND user_id = $2"
        )
        .bind(conversation_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| MessengerError::StorageError { message: format!("Failed to query participant: {}", e) })?;
        
        let unread_count = if let Some(row) = participant_row {
            if let Some(last_read_message_id) = row.get::<Option<Uuid>, _>("last_read_message_id") {
                // Count messages sent after the last read message
                let count: i64 = sqlx::query_scalar(
                    "SELECT COUNT(*)
                     FROM messages
                     WHERE conversation_id = $1 AND sender_id != $2 AND id > $3"
                )
                .bind(conversation_id)
                .bind(user_id)
                .bind(last_read_message_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| MessengerError::StorageError { message: format!("Failed to count unread messages: {}", e) })?;
                
                count
            } else {
                // No last read message, count all messages from other users
                let count: i64 = sqlx::query_scalar(
                    "SELECT COUNT(*)
                     FROM messages
                     WHERE conversation_id = $1 AND sender_id != $2"
                )
                .bind(conversation_id)
                .bind(user_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| MessengerError::StorageError { message: format!("Failed to count unread messages: {}", e) })?;
                
                count
            }
        } else {
            // User is not a participant, return 0
            0
        };
        
        Ok(unread_count as usize)
    }
}

/// PostgreSQL implementation of MediaRepository
pub struct PostgresMediaRepository {
    pool: PgPool,
}

impl PostgresMediaRepository {
    /// Create a new PostgreSQL media repository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MediaRepository for PostgresMediaRepository {
    async fn store_media(&self, media_reference: &MediaReference, _data: Vec<u8>) -> Result<(), MessengerError> {
        // In a real implementation, we would store the media data in a file system or object storage
        // and only store metadata in the database
        
        sqlx::query(
            "INSERT INTO media (id, media_type, storage_location, size_bytes, filename, created_at)
             VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(media_reference.id)
        .bind(format!("{:?}", media_reference.media_type))
        .bind(&media_reference.storage_location)
        .bind(media_reference.size_bytes as i64)
        .bind(&media_reference.filename)
        .bind(chrono::Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| MessengerError::StorageError { message: format!("Failed to insert media: {}", e) })?;
        
        Ok(())
    }
    
    async fn find_by_id(&self, id: Uuid) -> Result<MediaReference, MessengerError> {
        let row = sqlx::query(
            "SELECT id, media_type, storage_location, size_bytes, filename
             FROM media
             WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| MessengerError::StorageError { message: format!("Failed to query media: {}", e) })?
        .ok_or(MessengerError::MediaNotFound { id })?;
        
        let media_type_str: String = row.get("media_type");
        let media_type = match media_type_str.as_str() {
            "Image" => MediaType::Image,
            "Document" => MediaType::Document,
            "Audio" => MediaType::Audio,
            "Video" => MediaType::Video,
            _ => return Err(MessengerError::StorageError { message: "Unknown media type".to_string() }),
        };
        
        let media_reference = MediaReference {
            id: row.get("id"),
            media_type,
            storage_location: row.get("storage_location"),
            thumbnail: None, // In a real implementation, we would fetch thumbnail info
            size_bytes: row.get::<i64, _>("size_bytes") as u64,
            filename: row.get("filename"),
        };
        
        Ok(media_reference)
    }
    
    async fn delete(&self, id: Uuid) -> Result<(), MessengerError> {
        sqlx::query(
            "DELETE FROM media
             WHERE id = $1"
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| MessengerError::StorageError { message: format!("Failed to delete media: {}", e) })?;
        
        Ok(())
    }
    
    async fn get_media_data(&self, _id: Uuid) -> Result<Vec<u8>, MessengerError> {
        // In a real implementation, we would fetch the media data from storage
        // For now, we'll return empty data
        Ok(Vec::new())
    }
}

/// Sled implementation of PresenceRepository
pub struct SledPresenceRepository {
    db: sled::Db,
}

impl SledPresenceRepository {
    /// Create a new Sled presence repository
    pub fn new(path: &str) -> Result<Self, MessengerError> {
        let db = sled::open(path)
            .map_err(|e| MessengerError::StorageError { message: format!("Failed to open Sled database: {}", e) })?;
        
        Ok(Self { db })
    }
}

#[async_trait]
impl PresenceRepository for SledPresenceRepository {
    async fn update_presence(&self, user_id: Uuid, presence: UserPresence) -> Result<(), MessengerError> {
        let key = format!("presence:{}", user_id);
        let value = serde_json::to_vec(&presence)
            .map_err(|e| MessengerError::StorageError { message: format!("Failed to serialize presence: {}", e) })?;
        
        self.db.insert(key, value)
            .map_err(|e| MessengerError::StorageError { message: format!("Failed to store presence: {}", e) })?;
        
        Ok(())
    }
    
    async fn get_presence(&self, user_id: Uuid) -> Result<UserPresence, MessengerError> {
        let key = format!("presence:{}", user_id);
        let value = self.db.get(&key)
            .map_err(|e| MessengerError::StorageError { message: format!("Failed to get presence: {}", e) })?
            .ok_or(MessengerError::UserNotFound { id: user_id })?;
        
        let presence = serde_json::from_slice(&value)
            .map_err(|e| MessengerError::StorageError { message: format!("Failed to deserialize presence: {}", e) })?;
        
        Ok(presence)
    }
    
    async fn get_multiple_presence(&self, user_ids: Vec<Uuid>) -> Result<HashMap<Uuid, UserPresence>, MessengerError> {
        let mut presence_map = HashMap::new();
        
        for user_id in user_ids {
            let key = format!("presence:{}", user_id);
            if let Ok(Some(value)) = self.db.get(&key) {
                if let Ok(presence) = serde_json::from_slice::<UserPresence>(&value) {
                    presence_map.insert(user_id, presence);
                }
            }
        }
        
        Ok(presence_map)
    }
    
    async fn get_online_users(&self) -> Result<Vec<Uuid>, MessengerError> {
        let mut online_users = Vec::new();
        
        for result in self.db.iter() {
            let (key, value) = result
                .map_err(|e| MessengerError::StorageError { message: format!("Failed to iterate database: {}", e) })?;
            
            if let Ok(key_str) = std::str::from_utf8(&key) {
                if key_str.starts_with("presence:") {
                    if let Ok(presence) = serde_json::from_slice::<UserPresence>(&value) {
                        if matches!(presence, UserPresence::Online) {
                            if let Some(user_id_str) = key_str.strip_prefix("presence:") {
                                if let Ok(user_id) = Uuid::from_str(user_id_str) {
                                    online_users.push(user_id);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(online_users)
    }
}