//! # Messenger Infrastructure Layer
//!
//! This module contains the infrastructure implementations for the Messenger application.
//! It provides concrete implementations of the repository interfaces and exposes APIs.

/// Database implementations
pub mod database;
/// GraphQL API
pub mod graphql;
/// WebSocket service for real-time communication
pub mod websocket;
/// Authentication and OAuth2 integration
pub mod auth;
/// Media storage
pub mod media;

// Re-export commonly used types
pub use database::{PostgresConversationRepository, PostgresMessageRepository, PostgresMediaRepository, SledPresenceRepository};
pub use graphql::MessengerSchema;
pub use websocket::WebSocketServer;
pub use auth::OAuth2IdentityProvider;