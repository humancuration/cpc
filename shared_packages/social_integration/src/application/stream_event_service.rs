//! Stream event service for handling live streaming events

use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::social_event::{SocialEvent, SubscriptionTier};
use std::error::Error;
use std::sync::Arc;
use serde_json::json;
use cpc_notification_core::application::service::NotificationService;
use cpc_notification_core::domain::types::{Notification, NotificationCategory, NotificationPriority, ChannelType};

/// Service for handling stream events
#[derive(Debug)]
pub struct StreamEventService {
    /// Notification service for sending notifications
    notification_service: Arc<NotificationService>,
}

impl StreamEventService {
    /// Create a new stream event service
    pub fn new(notification_service: Arc<NotificationService>) -> Self {
        Self {
            notification_service,
        }
    }
    
    /// Handle a stream started event
    pub async fn handle_stream_started(&self, user_id: Uuid, user_name: String, stream_id: Uuid) -> Result<(), Box<dyn Error + Send + Sync>> {
        let event = SocialEvent::StreamStarted {
            user_id,
            stream_id,
            timestamp: chrono::Utc::now(),
        };
        
        // Send notification for the stream started event
        let notif = Notification::new_immediate(
            "".to_string(), // TODO: Get follower IDs
            NotificationCategory::Streaming,
            NotificationPriority::High,
            "Stream Started".into(),
            format!("{} is live!", user_name),
            json!({"stream_id": stream_id}),
            vec![ChannelType::InApp, ChannelType::Push],
        );
        if let Err(e) = self.notification_service.send(notif).await {
            tracing::error!("Stream started notification failed: {}", e);
        }
        
        // In a real implementation, we would store the event and notify followers
        tracing::info!("Handling stream started event: {:?}", event);
        Ok(())
    }
    
    /// Handle a stream ended event
    pub async fn handle_stream_ended(&self, user_id: Uuid, stream_id: Uuid) -> Result<(), Box<dyn Error + Send + Sync>> {
        let event = SocialEvent::StreamEnded {
            user_id,
            stream_id,
            timestamp: chrono::Utc::now(),
        };
        
        // In a real implementation, we would store the event
        tracing::info!("Handling stream ended event: {:?}", event);
        Ok(())
    }
    
    /// Handle a viewer joined event
    pub async fn handle_viewer_joined(&self, user_id: Uuid, user_name: String, broadcaster_id: Uuid, stream_id: Uuid) -> Result<(), Box<dyn Error + Send + Sync>> {
        let event = SocialEvent::ViewerJoined {
            user_id,
            broadcaster_id,
            stream_id,
            timestamp: chrono::Utc::now(),
        };
        
        // Send notification for the viewer joined event
        let notif = Notification::new_immediate(
            broadcaster_id.to_string(),
            NotificationCategory::Streaming,
            NotificationPriority::Low,
            "Viewer Joined".into(),
            format!("{} joined", user_name),
            json!({"stream_id": stream_id}),
            vec![ChannelType::InApp],
        );
        if let Err(e) = self.notification_service.send(notif).await {
            tracing::error!("Viewer joined notification failed: {}", e);
        }
        
        // In a real implementation, we would store the event
        tracing::info!("Handling viewer joined event: {:?}", event);
        Ok(())
    }
    
    /// Handle a chat message sent event
    pub async fn handle_chat_message_sent(&self, user_id: Uuid, stream_id: Uuid, message_id: Uuid) -> Result<(), Box<dyn Error + Send + Sync>> {
        let event = SocialEvent::ChatMessageSent {
            user_id,
            stream_id,
            message_id,
            timestamp: chrono::Utc::now(),
        };
        
        // In a real implementation, we would store the event
        tracing::info!("Handling chat message sent event: {:?}", event);
        Ok(())
    }
    
    /// Handle a subscription created event
    pub async fn handle_subscription_created(&self, user_id: Uuid, channel_id: Uuid, tier: SubscriptionTier) -> Result<(), Box<dyn Error + Send + Sync>> {
        let event = SocialEvent::SubscriptionCreated {
            user_id,
            channel_id,
            tier,
            timestamp: chrono::Utc::now(),
        };
        
        // In a real implementation, we would store the event
        tracing::info!("Handling subscription created event: {:?}", event);
        Ok(())
    }
}

#[async_trait]
impl super::social_integration_service::StreamEventService for StreamEventService {
    async fn handle_stream_started(&self, user_id: Uuid, stream_id: Uuid) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.handle_stream_started(user_id, "".to_string(), stream_id).await
    }
    
    async fn handle_stream_ended(&self, user_id: Uuid, stream_id: Uuid) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.handle_stream_ended(user_id, stream_id).await
    }
    
    async fn handle_viewer_joined(&self, user_id: Uuid, broadcaster_id: Uuid, stream_id: Uuid) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.handle_viewer_joined(user_id, "".to_string(), broadcaster_id, stream_id).await
    }
    
    async fn handle_chat_message_sent(&self, user_id: Uuid, stream_id: Uuid, message_id: Uuid) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.handle_chat_message_sent(user_id, stream_id, message_id).await
    }
    
    async fn handle_subscription_created(&self, user_id: Uuid, channel_id: Uuid, tier: SubscriptionTier) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.handle_subscription_created(user_id, channel_id, tier).await
    }
}