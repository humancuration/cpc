//! GraphQL API for the Messenger application

use async_graphql::{
    Schema, EmptySubscription, Object, Result, Context, SimpleObject, InputObject, Enum,
    connection::{Connection, EmptyFields, QueryResult},
    types::ID,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::sync::Arc;

use messenger_domain::{
    models::{Conversation, Message, Participant, MessageContent, MediaType, DeliveryStatus},
    errors::MessengerError,
};
use messenger_app::services::{ConversationService, MessageService, PresenceService};

/// GraphQL schema for the Messenger application
pub type MessengerSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

/// Create a new GraphQL schema
pub fn create_schema(
    conversation_service: Arc<dyn ConversationService>,
    message_service: Arc<dyn MessageService>,
    presence_service: Arc<dyn PresenceService>,
) -> MessengerSchema {
    Schema::build(
        QueryRoot {
            conversation_service: conversation_service.clone(),
            message_service: message_service.clone(),
            presence_service: presence_service.clone(),
        },
        MutationRoot {
            conversation_service,
            message_service,
        },
        SubscriptionRoot,
    )
    .finish()
}

/// Root query object
pub struct QueryRoot {
    conversation_service: Arc<dyn ConversationService>,
    message_service: Arc<dyn MessageService>,
    presence_service: Arc<dyn PresenceService>,
}

#[Object]
impl QueryRoot {
    /// Get a conversation by ID
    async fn conversation(&self, ctx: &Context<'_>, id: ID) -> Result<ConversationObject> {
        let conversation_id = parse_id(&id)?;
        let conversation = self.conversation_service.get_conversation(conversation_id).await
            .map_err(|e| format_graphql_error(e))?;
        Ok(ConversationObject::from(conversation))
    }
    
    /// Get all conversations for the current user
    async fn conversations(&self, ctx: &Context<'_>) -> Result<Vec<ConversationObject>> {
        // In a real implementation, we would get the user ID from the context
        // For now, we'll use a placeholder
        let user_id = Uuid::nil(); // Placeholder
        let conversations = self.conversation_service.get_user_conversations(user_id).await
            .map_err(|e| format_graphql_error(e))?;
        Ok(conversations.into_iter().map(ConversationObject::from).collect())
    }
}

/// Root mutation object
pub struct MutationRoot {
    conversation_service: Arc<dyn ConversationService>,
    message_service: Arc<dyn MessageService>,
}

#[Object]
impl MutationRoot {
    /// Send a message to a conversation
    async fn send_message(&self, ctx: &Context<'_>, input: SendMessageInput) -> Result<MessageObject> {
        let conversation_id = parse_id(&input.conversation_id)?;
        let sender_id = parse_id(&input.sender_id)?;
        
        let content = MessageContent::Text(input.content);
        let message = self.message_service.send_message(conversation_id, sender_id, content).await
            .map_err(|e| format_graphql_error(e))?;
        Ok(MessageObject::from(message))
    }
    
    /// Create a new group conversation
    async fn create_group(&self, ctx: &Context<'_>, input: CreateGroupInput) -> Result<ConversationObject> {
        let participant_ids: Result<Vec<Uuid>, _> = input.participant_ids
            .iter()
            .map(|id| parse_id(id))
            .collect();
        let participant_ids = participant_ids?;
        
        let participants: Vec<Participant> = participant_ids
            .into_iter()
            .map(Participant::new)
            .collect();
        
        let conversation = self.conversation_service
            .create_conversation(participants, true, Some(input.name.clone()))
            .await
            .map_err(|e| format_graphql_error(e))?;
        Ok(ConversationObject::from(conversation))
    }
}

/// Root subscription object
pub struct SubscriptionRoot;

#[Object]
impl SubscriptionRoot {
    // Subscriptions would be implemented here
}

/// GraphQL representation of a Conversation
#[derive(SimpleObject)]
pub struct ConversationObject {
    /// Unique identifier for the conversation
    pub id: ID,
    
    /// Participants in the conversation
    pub participants: Vec<UserObject>,
    
    /// Whether this is a group conversation
    pub is_group: bool,
    
    /// For group conversations, the name of the group
    pub group_name: Option<String>,
    
    /// The last message in the conversation
    pub last_message: Option<MessageObject>,
}

impl From<Conversation> for ConversationObject {
    fn from(conversation: Conversation) -> Self {
        Self {
            id: ID::from(conversation.id.to_string()),
            participants: conversation.participants
                .into_iter()
                .map(|p| UserObject {
                    id: ID::from(p.user_id.to_string()),
                })
                .collect(),
            is_group: conversation.is_group,
            group_name: conversation.group_name,
            last_message: None, // In a real implementation, we would fetch the last message
        }
    }
}

/// GraphQL representation of a Message
#[derive(SimpleObject)]
pub struct MessageObject {
    /// Unique identifier for the message
    pub id: ID,
    
    /// Content of the message
    pub content: String,
    
    /// User who sent the message
    pub sender: UserObject,
    
    /// When the message was sent
    pub sent_at: DateTime<Utc>,
    
    /// Delivery status of the message
    pub status: MessageStatus,
    
    /// Conversation this message belongs to
    pub conversation_id: ID,
}

impl From<Message> for MessageObject {
    fn from(message: Message) -> Self {
        let content = match &message.content {
            MessageContent::Text(text) => text.clone(),
            MessageContent::Media(_) => "[Media]".to_string(),
            MessageContent::System(_) => "[System]".to_string(),
        };
        
        Self {
            id: ID::from(message.id.to_string()),
            content,
            sender: UserObject {
                id: ID::from(message.sender_id.to_string()),
            },
            sent_at: message.sent_at,
            status: MessageStatus::from(message.delivery_status),
            conversation_id: ID::from(message.conversation_id.to_string()),
        }
    }
}

/// GraphQL representation of a User
#[derive(SimpleObject)]
pub struct UserObject {
    /// Unique identifier for the user
    pub id: ID,
}

/// GraphQL representation of message status
#[derive(Enum, Clone, Copy, PartialEq, Eq)]
pub enum MessageStatus {
    /// Message is pending sending
    Pending,
    
    /// Message has been sent to the server
    Sent,
    
    /// Message has been delivered to all recipients
    Delivered,
    
    /// Message has been read by at least one recipient
    Read,
}

impl From<DeliveryStatus> for MessageStatus {
    fn from(status: DeliveryStatus) -> Self {
        match status {
            DeliveryStatus::Pending => MessageStatus::Pending,
            DeliveryStatus::Sent(_) => MessageStatus::Sent,
            DeliveryStatus::Delivered(_) => MessageStatus::Delivered,
            DeliveryStatus::Read(_) => MessageStatus::Read,
        }
    }
}

/// Input for sending a message
#[derive(InputObject)]
pub struct SendMessageInput {
    /// Conversation to send the message to
    pub conversation_id: ID,
    
    /// User sending the message
    pub sender_id: ID,
    
    /// Content of the message
    pub content: String,
}

/// Input for creating a group
#[derive(InputObject)]
pub struct CreateGroupInput {
    /// Name of the group
    pub name: String,
    
    /// IDs of participants to add to the group
    pub participant_ids: Vec<ID>,
}

/// Parse an ID string into a Uuid
fn parse_id(id: &ID) -> Result<Uuid> {
    Uuid::parse_str(id.as_str())
        .map_err(|_| "Invalid ID format".into())
}

/// Format a MessengerError as a GraphQL error
fn format_graphql_error(error: MessengerError) -> async_graphql::Error {
    match error {
        MessengerError::ConversationNotFound { id } => {
            format!("Conversation not found: {}", id).into()
        }
        MessengerError::MessageNotFound { id } => {
            format!("Message not found: {}", id).into()
        }
        MessengerError::UserNotFound { id } => {
            format!("User not found: {}", id).into()
        }
        MessengerError::NotParticipant { user_id, conversation_id } => {
            format!("User {} is not a participant in conversation {}", user_id, conversation_id).into()
        }
        MessengerError::PermissionDenied { user_id, action } => {
            format!("User {} does not have permission to {}", user_id, action).into()
        }
        MessengerError::InvalidInput { message } => {
            format!("Invalid input: {}", message).into()
        }
        MessengerError::ConversationFull { max_participants } => {
            format!("Conversation is full (maximum {} participants)", max_participants).into()
        }
        MessengerError::AlreadyParticipant { user_id, conversation_id } => {
            format!("User {} is already a participant in conversation {}", user_id, conversation_id).into()
        }
        MessengerError::MediaUploadFailed { message } => {
            format!("Media upload failed: {}", message).into()
        }
        MessengerError::MediaNotFound { id } => {
            format!("Media not found: {}", id).into()
        }
        MessengerError::StorageError { message } => {
            format!("Storage error: {}", message).into()
        }
        MessengerError::ValidationError { message } => {
            format!("Validation error: {}", message).into()
        }
    }
}