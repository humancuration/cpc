//! # Event Bus System
//!
//! Generalizes the Bevy ECS integration pattern from `consent_manager` into a standardized
//! event distribution system for real-time updates across applications.

/// Domain layer containing core business logic and entities
pub mod domain;

/// Application layer containing use cases and service orchestration
pub mod application;

/// Infrastructure layer containing adapters for external systems
pub mod infrastructure;

// Re-export commonly used types
pub use domain::{event::DomainEvent, subscription::Subscription, EventError};
pub use application::service::EventBus;
pub use infrastructure::bevy::EventBusPlugin;