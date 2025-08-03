//! Notification integration service for handling social events and sending notifications

use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;
use serde_json::json;
use uuid::Uuid;
use crate::domain::social_event::SocialEvent;
use crate::application::social_integration_service::SocialEventConsumer;
use cpc_notification_core::application::service::NotificationService;
use cpc_notification_core::domain::types::{Notification, NotificationCategory, NotificationPriority, ChannelType};

/// Service for integrating social events with notification system
pub struct NotificationIntegrationService {
    notification_service: Arc<NotificationService>,
}

impl NotificationIntegrationService {
    /// Create a new notification integration service
    pub fn new(notification_service: Arc<NotificationService>) -> Self {
        Self {
            notification_service,
        }
    }
}

#[async_trait]
impl SocialEventConsumer for NotificationIntegrationService {
    async fn handle_event(&self, event: SocialEvent) -> Result<(), Box<dyn Error + Send + Sync>> {
        match event {
            SocialEvent::UserFollowed { follower_id, followed_id, .. } => {
                // Create and send notification for user follow
                let notif = Notification::new_immediate(
                    followed_id.to_string(),
                    NotificationCategory::Social,
                    NotificationPriority::Normal,
                    "New Follower".into(),
                    format!("User {} is now following you", follower_id),
                    json!({"type": "follow", "follower_id": follower_id}),
                    vec![ChannelType::InApp, ChannelType::Push],
                );
                if let Err(e) = self.notification_service.send(notif).await {
                    tracing::error!("Follow notification failed: {}", e);
                }
            }
            SocialEvent::PostCreated { user_id, post_id, .. } => {
                // This would typically be handled by a different service that tracks followers
                // For now, we'll just log it
                tracing::debug!("Post created by user {}: {}", user_id, post_id);
            }
            SocialEvent::CommentCreated { user_id, post_id, .. } => {
                // This would typically be handled by a different service that tracks post owners
                // For now, we'll just log it
                tracing::debug!("Comment created by user {} on post {}", user_id, post_id);
            }
            SocialEvent::PostVoted { user_id, post_id, vote_type, .. } => {
                // This would typically be handled by a different service that tracks post owners
                // For now, we'll just log it
                tracing::debug!("User {} voted {:?} on post {}", user_id, vote_type, post_id);
            }
            SocialEvent::PostShared { user_id, post_id, .. } => {
                // This would typically be handled by a different service that tracks post owners
                // For now, we'll just log it
                tracing::debug!("User {} shared post {}", user_id, post_id);
            }
            SocialEvent::OpportunityShared { user_id, opportunity_id, .. } => {
                // This would typically be handled by a different service that tracks interested users
                // For now, we'll just log it
                tracing::debug!("User {} shared opportunity {}", user_id, opportunity_id);
            }
            SocialEvent::Volunteered { user_id, opportunity_id, hours_contributed, .. } => {
                // This would typically be handled by a different service that tracks opportunity owners
                // For now, we'll just log it
                tracing::debug!("User {} volunteered {} hours for opportunity {}", user_id, hours_contributed, opportunity_id);
            }
            SocialEvent::StreamStarted { user_id, stream_id, .. } => {
                // This is handled by the StreamEventService
                tracing::debug!("Stream started by user {}: {}", user_id, stream_id);
            }
            SocialEvent::StreamEnded { user_id, stream_id, .. } => {
                // This is handled by the StreamEventService
                tracing::debug!("Stream ended by user {}: {}", user_id, stream_id);
            }
            SocialEvent::ViewerJoined { user_id, broadcaster_id, stream_id, .. } => {
                // This is handled by the StreamEventService
                tracing::debug!("Viewer {} joined stream {} of broadcaster {}", user_id, stream_id, broadcaster_id);
            }
            SocialEvent::ChatMessageSent { user_id, stream_id, message_id, .. } => {
                // This is handled by the ChatService
                tracing::debug!("Chat message {} sent by user {} in stream {}", message_id, user_id, stream_id);
            }
            SocialEvent::SubscriptionCreated { user_id, channel_id, tier, .. } => {
                // This is handled by the StreamEventService
                tracing::debug!("User {} subscribed to channel {} with tier {:?}", user_id, channel_id, tier);
            }
        }
        Ok(())
    }
}