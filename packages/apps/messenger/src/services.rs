//! Application service implementations for the Messenger application

use async_trait::async_trait;
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{trace, debug, error};

use messenger_domain::{
    models::{Conversation, Message, Participant, MessageStatusUpdate, MessageContent, ConversationSettings},
    errors::MessengerError,
    services::{ConversationService, MessageService, MediaService, PresenceService, UserPresence},
};

use crate::repositories::{ConversationRepository, MessageRepository, MediaRepository, PresenceRepository};
use crate::integration::ConsentManager;

/// Implementation of ConversationService
pub struct ConversationServiceImpl {
    conversation_repository: Arc<dyn ConversationRepository>,
    consent_manager: Arc<dyn ConsentManager>,
}

impl ConversationServiceImpl {
    /// Create a new conversation service
    pub fn new(
        conversation_repository: Arc<dyn ConversationRepository>,
        consent_manager: Arc<dyn ConsentManager>,
    ) -> Self {
        Self {
            conversation_repository,
            consent_manager,
        }
    }
}

#[async_trait]
impl ConversationService for ConversationServiceImpl {
    async fn create_conversation(&self, participants: Vec<Participant>, is_group: bool, group_name: Option<String>) -> Result<Conversation, MessengerError> {
        trace!("Creating new conversation with {} participants", participants.len());
        
        // Verify consent for all participants
        for participant in &participants {
            self.consent_manager
                .verify_messaging_consent(participant.user_id)
                .await
                .map_err(|e| MessengerError::PermissionDenied {
                    user_id: participant.user_id,
                    action: format!("create conversation: {}", e),
                })?;
        }
        
        let conversation = if is_group {
            Conversation::new_group(participants, group_name.unwrap_or_else(|| "Group Chat".to_string()))
        } else {
            Conversation::new_1to1(participants)
        };
        
        self.conversation_repository.create(&conversation).await?;
        
        debug!("Created conversation {}", conversation.id);
        Ok(conversation)
    }
    
    async fn get_conversation(&self, conversation_id: Uuid) -> Result<Conversation, MessengerError> {
        trace!("Getting conversation {}", conversation_id);
        
        let conversation = self.conversation_repository.find_by_id(conversation_id).await?;
        Ok(conversation)
    }
    
    async fn add_participant(&self, conversation_id: Uuid, participant: Participant) -> Result<Conversation, MessengerError> {
        trace!("Adding participant {} to conversation {}", participant.user_id, conversation_id);
        
        // Verify consent for the new participant
        self.consent_manager
            .verify_messaging_consent(participant.user_id)
            .await
            .map_err(|e| MessengerError::PermissionDenied {
                user_id: participant.user_id,
                action: format!("add to conversation: {}", e),
            })?;
        
        let mut conversation = self.conversation_repository.find_by_id(conversation_id).await?;
        
        // Check if user is already a participant
        if conversation.participants.iter().any(|p| p.user_id == participant.user_id) {
            return Err(MessengerError::AlreadyParticipant {
                user_id: participant.user_id,
                conversation_id,
            });
        }
        
        // For group conversations, check if we're at the limit
        if conversation.is_group && conversation.participants.len() >= 100 {
            return Err(MessengerError::ConversationFull { max_participants: 100 });
        }
        
        conversation.add_participant(participant);
        self.conversation_repository.update(&conversation).await?;
        
        debug!("Added participant {} to conversation {}", conversation.participants.last().unwrap().user_id, conversation_id);
        Ok(conversation)
    }
    
    async fn remove_participant(&self, conversation_id: Uuid, user_id: Uuid) -> Result<Conversation, MessengerError> {
        trace!("Removing participant {} from conversation {}", user_id, conversation_id);
        
        let mut conversation = self.conversation_repository.find_by_id(conversation_id).await?;
        
        // Check if user is a participant
        if !conversation.participants.iter().any(|p| p.user_id == user_id) {
            return Err(MessengerError::NotParticipant { user_id, conversation_id });
        }
        
        conversation.remove_participant(user_id);
        self.conversation_repository.update(&conversation).await?;
        
        debug!("Removed participant {} from conversation {}", user_id, conversation_id);
        Ok(conversation)
    }
    
    async fn update_settings(&self, conversation_id: Uuid, settings: ConversationSettings) -> Result<Conversation, MessengerError> {
        trace!("Updating settings for conversation {}", conversation_id);
        
        let mut conversation = self.conversation_repository.find_by_id(conversation_id).await?;
        conversation.settings = settings;
        self.conversation_repository.update(&conversation).await?;
        
        debug!("Updated settings for conversation {}", conversation_id);
        Ok(conversation)
    }
    
    async fn get_user_conversations(&self, user_id: Uuid) -> Result<Vec<Conversation>, MessengerError> {
        trace!("Getting conversations for user {}", user_id);
        
        // Verify consent for messaging
        self.consent_manager
            .verify_messaging_consent(user_id)
            .await
            .map_err(|e| MessengerError::PermissionDenied {
                user_id,
                action: format!("view conversations: {}", e),
            })?;
        
        let conversations = self.conversation_repository.find_by_participant(user_id).await?;
        Ok(conversations)
    }
}

/// Implementation of MessageService
pub struct MessageServiceImpl {
    message_repository: Arc<dyn MessageRepository>,
    conversation_repository: Arc<dyn ConversationRepository>,
    consent_manager: Arc<dyn ConsentManager>,
}

impl MessageServiceImpl {
    /// Create a new message service
    pub fn new(
        message_repository: Arc<dyn MessageRepository>,
        conversation_repository: Arc<dyn ConversationRepository>,
        consent_manager: Arc<dyn ConsentManager>,
    ) -> Self {
        Self {
            message_repository,
            conversation_repository,
            consent_manager,
        }
    }
}

#[async_trait]
impl MessageService for MessageServiceImpl {
    async fn send_message(&self, conversation_id: Uuid, sender_id: Uuid, content: MessageContent) -> Result<Message, MessengerError> {
        trace!("Sending message to conversation {}", conversation_id);
        
        // Verify consent for messaging
        self.consent_manager
            .verify_messaging_consent(sender_id)
            .await
            .map_err(|e| MessengerError::PermissionDenied {
                user_id: sender_id,
                action: format!("send message: {}", e),
            })?;
        
        // Verify user is a participant in the conversation
        let conversation = self.conversation_repository.find_by_id(conversation_id).await?;
        if !conversation.participants.iter().any(|p| p.user_id == sender_id) {
            return Err(MessengerError::NotParticipant { user_id: sender_id, conversation_id });
        }
        
        // Check permissions
        let sender_participant = conversation.participants.iter().find(|p| p.user_id == sender_id).unwrap();
        if !sender_participant.permissions.can_send_messages {
            return Err(MessengerError::PermissionDenied {
                user_id: sender_id,
                action: "send message".to_string(),
            });
        }
        
        // If this is a media message, verify media sharing consent
        if matches!(content, MessageContent::Media(_)) {
            self.consent_manager
                .verify_media_sharing_consent(sender_id)
                .await
                .map_err(|e| MessengerError::PermissionDenied {
                    user_id: sender_id,
                    action: format!("send media: {}", e),
                })?;
        }
        
        let mut message = match content {
            MessageContent::Text(text) => Message::new_text(conversation_id, sender_id, text),
            MessageContent::Media(media) => Message::new_media(conversation_id, sender_id, media),
            MessageContent::System(_) => {
                return Err(MessengerError::InvalidInput {
                    message: "Cannot send system messages directly".to_string(),
                });
            }
        };
        
        // Mark as sent
        message.mark_sent();
        
        self.message_repository.create(&message).await?;
        
        debug!("Sent message {} to conversation {}", message.id, conversation_id);
        Ok(message)
    }
    
    async fn get_conversation_messages(&self, conversation_id: Uuid, limit: usize, before_message_id: Option<Uuid>) -> Result<Vec<Message>, MessengerError> {
        trace!("Getting messages for conversation {}", conversation_id);
        
        let messages = self.message_repository.find_by_conversation(conversation_id, limit, before_message_id).await?;
        Ok(messages)
    }
    
    async fn get_message(&self, message_id: Uuid) -> Result<Message, MessengerError> {
        trace!("Getting message {}", message_id);
        
        let message = self.message_repository.find_by_id(message_id).await?;
        Ok(message)
    }
    
    async fn update_message_status(&self, update: MessageStatusUpdate) -> Result<(), MessengerError> {
        trace!("Updating status for message {}", update.message_id);
        
        let mut message = self.message_repository.find_by_id(update.message_id).await?;
        message.delivery_status = update.new_status;
        self.message_repository.update(&message).await?;
        
        debug!("Updated status for message {} to {:?}", message.id, message.delivery_status);
        Ok(())
    }
    
    async fn mark_messages_read(&self, conversation_id: Uuid, user_id: Uuid, up_to_message_id: Uuid) -> Result<usize, MessengerError> {
        trace!("Marking messages as read in conversation {} up to message {}", conversation_id, up_to_message_id);
        
        // Verify user is a participant in the conversation
        let conversation = self.conversation_repository.find_by_id(conversation_id).await?;
        if !conversation.participants.iter().any(|p| p.user_id == user_id) {
            return Err(MessengerError::NotParticipant { user_id, conversation_id });
        }
        
        let count = self.message_repository.mark_messages_read(conversation_id, user_id, up_to_message_id).await?;
        
        // Update participant's last read message
        // This would typically be done in a transaction with the above operation
        debug!("Marked {} messages as read in conversation {} for user {}", count, conversation_id, user_id);
        Ok(count)
    }
    
    async fn delete_message(&self, message_id: Uuid, user_id: Uuid) -> Result<(), MessengerError> {
        trace!("Deleting message {}", message_id);
        
        let message = self.message_repository.find_by_id(message_id).await?;
        
        // Verify user is a participant in the conversation
        let conversation = self.conversation_repository.find_by_id(message.conversation_id).await?;
        if !conversation.participants.iter().any(|p| p.user_id == user_id) {
            return Err(MessengerError::NotParticipant {
                user_id,
                conversation_id: message.conversation_id,
            });
        }
        
        // Check permissions
        let user_participant = conversation.participants.iter().find(|p| p.user_id == user_id).unwrap();
        if !user_participant.permissions.can_delete_messages && message.sender_id != user_id {
            return Err(MessengerError::PermissionDenied {
                user_id,
                action: "delete message".to_string(),
            });
        }
        
        self.message_repository.delete(message_id).await?;
        
        debug!("Deleted message {}", message_id);
        Ok(())
    }
}

/// Implementation of MediaService
pub struct MediaServiceImpl {
    media_repository: Arc<dyn MediaRepository>,
    consent_manager: Arc<dyn ConsentManager>,
}

impl MediaServiceImpl {
    /// Create a new media service
    pub fn new(
        media_repository: Arc<dyn MediaRepository>,
        consent_manager: Arc<dyn ConsentManager>,
    ) -> Self {
        Self {
            media_repository,
            consent_manager,
        }
    }
}

#[async_trait]
impl MediaService for MediaServiceImpl {
    async fn upload_media(&self, media_data: Vec<u8>, media_type: messenger_domain::models::MediaType, user_id: Uuid) -> Result<messenger_domain::models::MediaReference, MessengerError> {
        trace!("Uploading media for user {}", user_id);
        
        // Verify consent for media sharing
        self.consent_manager
            .verify_media_sharing_consent(user_id)
            .await
            .map_err(|e| MessengerError::PermissionDenied {
                user_id,
                action: format!("upload media: {}", e),
            })?;
        
        // In a real implementation, we would:
        // 1. Validate the media data
        // 2. Store it in a secure location
        // 3. Generate a media reference
        // 4. Create thumbnails if needed
        
        // For now, we'll create a mock implementation
        let media_reference = messenger_domain::models::MediaReference {
            id: Uuid::new_v4(),
            media_type,
            storage_location: format!("media/{}", Uuid::new_v4()),
            thumbnail: None,
            size_bytes: media_data.len() as u64,
            filename: None,
        };
        
        // In a real implementation, we would store the media data
        // self.media_repository.store_media(&media_reference, media_data).await?;
        
        debug!("Uploaded media {} for user {}", media_reference.id, user_id);
        Ok(media_reference)
    }
    
    async fn get_media(&self, media_id: Uuid) -> Result<messenger_domain::models::MediaReference, MessengerError> {
        trace!("Getting media {}", media_id);
        
        let media_reference = self.media_repository.find_by_id(media_id).await?;
        Ok(media_reference)
    }
    
    async fn delete_media(&self, media_id: Uuid, user_id: Uuid) -> Result<(), MessengerError> {
        trace!("Deleting media {} for user {}", media_id, user_id);
        
        let media_reference = self.media_repository.find_by_id(media_id).await?;
        
        // Verify user owns the media or has permission to delete it
        // This would typically be checked against metadata stored with the media
        
        self.media_repository.delete(media_id).await?;
        
        debug!("Deleted media {} for user {}", media_id, user_id);
        Ok(())
    }
}

/// Implementation of PresenceService
pub struct PresenceServiceImpl {
    presence_repository: Arc<dyn PresenceRepository>,
}

impl PresenceServiceImpl {
    /// Create a new presence service
    pub fn new(presence_repository: Arc<dyn PresenceRepository>) -> Self {
        Self {
            presence_repository,
        }
    }
}

#[async_trait]
impl PresenceService for PresenceServiceImpl {
    async fn update_presence(&self, user_id: Uuid, status: UserPresence) -> Result<(), MessengerError> {
        trace!("Updating presence for user {} to {:?}", user_id, status);
        
        self.presence_repository.update_presence(user_id, status).await?;
        
        debug!("Updated presence for user {} to {:?}", user_id, status);
        Ok(())
    }
    
    async fn get_presence(&self, user_id: Uuid) -> Result<UserPresence, MessengerError> {
        trace!("Getting presence for user {}", user_id);
        
        let presence = self.presence_repository.get_presence(user_id).await?;
        Ok(presence)
    }
    
    async fn get_multiple_presence(&self, user_ids: Vec<Uuid>) -> Result<HashMap<Uuid, UserPresence>, MessengerError> {
        trace!("Getting presence for {} users", user_ids.len());
        
        let presence_map = self.presence_repository.get_multiple_presence(user_ids).await?;
        Ok(presence_map)
    }
}