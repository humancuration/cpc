//! Error types for the Messenger domain

use thiserror::Error;
use uuid::Uuid;

/// Errors that can occur in the Messenger domain
#[derive(Error, Debug)]
pub enum MessengerError {
    /// Conversation not found
    #[error("Conversation not found: {id}")]
    ConversationNotFound { id: Uuid },
    
    /// Message not found
    #[error("Message not found: {id}")]
    MessageNotFound { id: Uuid },
    
    /// User not found
    #[error("User not found: {id}")]
    UserNotFound { id: Uuid },
    
    /// User is not a participant in the conversation
    #[error("User {user_id} is not a participant in conversation {conversation_id}")]
    NotParticipant { user_id: Uuid, conversation_id: Uuid },
    
    /// User does not have permission to perform the action
    #[error("User {user_id} does not have permission to {action}")]
    PermissionDenied { user_id: Uuid, action: String },
    
    /// Invalid input provided
    #[error("Invalid input: {message}")]
    InvalidInput { message: String },
    
    /// Conversation is full (for group conversations)
    #[error("Conversation is full (maximum {max_participants} participants)")]
    ConversationFull { max_participants: usize },
    
    /// User is already a participant in the conversation
    #[error("User {user_id} is already a participant in conversation {conversation_id}")]
    AlreadyParticipant { user_id: Uuid, conversation_id: Uuid },
    
    /// Media upload failed
    #[error("Media upload failed: {message}")]
    MediaUploadFailed { message: String },
    
    /// Media not found
    #[error("Media not found: {id}")]
    MediaNotFound { id: Uuid },
    
    /// Storage error
    #[error("Storage error: {message}")]
    StorageError { message: String },
    
    /// Validation error
    #[error("Validation error: {message}")]
    ValidationError { message: String },
}