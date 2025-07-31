//! # Messenger Domain Layer
//!
//! This module contains the core domain entities and logic for the Messenger application.
//! It follows hexagonal architecture principles with a strict separation between domain logic
//! and infrastructure concerns.

/// Core domain entities
pub mod models;
/// Domain errors
pub mod errors;
/// Domain services
pub mod services;

// Re-export commonly used types
pub use models::{Conversation, Message, Participant, MessageContent, MediaReference, DeliveryStatus};
pub use errors::MessengerError;