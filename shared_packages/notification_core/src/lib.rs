//! # Notification Service Core
//!
//! Provides a unified notification system supporting multiple delivery channels
//! with user-controlled preferences, replacing ad-hoc notification implementations
//! across applications.

/// Domain layer containing core business logic and entities
pub mod domain;

/// Application layer containing use cases and service orchestration
pub mod application;

/// Infrastructure layer containing adapters for external systems
pub mod infrastructure;

// Re-export commonly used types
pub use domain::{types::Notification, preferences::UserPreferences, NotificationError};
pub use application::service::NotificationService;
pub use application::stream_notification_service::StreamNotificationService;