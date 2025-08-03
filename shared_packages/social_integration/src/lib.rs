//! Social Integration crate for the CPC platform
//!
//! This crate provides functionality for integrating social features across CPC apps,
//! including unified feeds and cross-posting.

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod graphql;

/// Re-export commonly used types
pub use domain::{
    post::{UnifiedPost, AppSource, PostMetadata},
    social_event::SocialEvent,
};
pub use application::{
    social_integration_service::SocialIntegrationService,
    stream_event_service::StreamEventService,
    social_integration_service::SocialEventConsumer,
    notification_integration::NotificationIntegrationService,
};

pub use graphql::{
    create_schema,
    SocialIntegrationSchema,
};

pub use infrastructure::repositories::{
    PostgresUnifiedPostRepository,
    PostgresUserFollowingRepository,
    UnifiedPostRepository,
    UserFollowingRepository,
};

#[cfg(test)]
mod lib_test;
