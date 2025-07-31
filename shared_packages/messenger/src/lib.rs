//! # Messenger Application Layer
//!
//! This module contains the application services and use cases for the Messenger application.
//! It implements the domain service interfaces and orchestrates the business logic.

/// Application services
pub mod services;
/// Repositories for data access
pub mod repositories;
/// Integration with external services
pub mod integration;

// Re-export commonly used types
pub use services::{ConversationServiceImpl, MessageServiceImpl, MediaServiceImpl, PresenceServiceImpl};