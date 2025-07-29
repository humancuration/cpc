//! Real-time in-app notifications
//! 
//! This module provides in-app notification delivery capabilities.

use async_trait::async_trait;
use crate::domain::{
    types::{Notification, ChannelType},
    NotificationError,
};
use crate::application::service::{NotificationChannel, DeliveryResult};
use event_bus::{EventBus, DomainEvent, EventSource};

/// In-app notification service
pub struct InAppNotificationChannel {
    /// Event bus for real-time delivery
    event_bus: EventBus,
}

impl InAppNotificationChannel {
    /// Create a new in-app notification channel
    pub fn new(event_bus: EventBus) -> Self {
        Self { event_bus }
    }
}

#[async_trait]
impl NotificationChannel for InAppNotificationChannel {
    fn channel_type(&self) -> ChannelType {
        ChannelType::InApp
    }
    
    async fn send(&self, notification: &Notification) -> Result<DeliveryResult, NotificationError> {
        tracing::debug!(
            "Sending in-app notification to user {}",
            notification.user_id
        );
        
        // Create a domain event for real-time delivery
        let event = DomainEvent::new(
            "notifications".to_string(),
            "in_app_notification".to_string(),
            serde_json::json!({
                "notification_id": notification.id,
                "user_id": notification.user_id,
                "category": notification.category.to_string(),
                "title": notification.title,
                "body": notification.body,
                "payload": notification.payload,
                "priority": notification.priority.to_string(),
            }),
            EventSource::Local,
        );
        
        // Publish the event through the event bus
        match self.event_bus.publish(event).await {
            Ok(_) => {
                Ok(DeliveryResult {
                    channel: ChannelType::InApp,
                    success: true,
                    error: None,
                    timestamp: chrono::Utc::now(),
                })
            }
            Err(e) => {
                Ok(DeliveryResult {
                    channel: ChannelType::InApp,
                    success: false,
                    error: Some(e.to_string()),
                    timestamp: chrono::Utc::now(),
                })
            }
        }
    }
}