//! Error types for the notification service

/// Error type for notification operations
#[derive(thiserror::Error, Debug)]
pub enum NotificationError {
    #[error("Notification serialization failed: {0}")]
    SerializationError(String),
    
    #[error("Notification deserialization failed: {0}")]
    DeserializationError(String),
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Delivery error: {0}")]
    DeliveryError(String),
    
    #[error("Invalid notification: {0}")]
    InvalidNotification(String),
    
    #[error("User preferences not found")]
    PreferencesNotFound,
    
    #[error("Notification not found")]
    NotificationNotFound,
    
    #[error("Channel not supported: {0}")]
    ChannelNotSupported(String),
    
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
}