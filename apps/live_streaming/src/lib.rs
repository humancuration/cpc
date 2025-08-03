//! Live Streaming Module
//!
//! This module provides a live streaming platform with real-time chat,
//! channel subscriptions, and social features.

pub mod streaming;
pub mod chat;
pub mod channel;
pub mod social;
pub mod media_processing;
pub mod web;
pub mod module_registry;
pub mod ui;


// Re-export key components for easier access
pub use channel::channel::Channel;
pub use channel::manager::ChannelManager;
pub use chat::chat_service::ChatService;
pub use streaming::broadcaster::Broadcaster;
pub use streaming::viewer::Viewer;
pub use social::follow::FollowService;
pub use social::subscription::SubscriptionService;
pub use web::module::LiveStreamingModule;
pub use web::modular_module::ModularLiveStreaming;
pub use module_registry::create_module;

// Re-export external services
pub use cpc_social_integration::StreamEventService;
pub use cpc_notification_core::StreamNotificationService;