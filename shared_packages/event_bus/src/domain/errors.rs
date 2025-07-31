//! Error types for the event bus system

/// Error type for event bus operations
#[derive(thiserror::Error, Debug)]
pub enum EventError {
    #[error("Event serialization failed: {0}")]
    SerializationError(String),
    
    #[error("Event deserialization failed: {0}")]
    DeserializationError(String),
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Subscription error: {0}")]
    SubscriptionError(String),
    
    #[error("Event not found")]
    EventNotFound,
    
    #[error("Invalid event filter: {0}")]
    InvalidFilter(String),
}