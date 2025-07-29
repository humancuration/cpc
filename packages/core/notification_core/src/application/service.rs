//! Notification orchestration
//! 
//! This module contains the NotificationService which is the primary entry point
//! for notification operations.

use std::sync::Arc;
use crate::domain::{
    types::{Notification, NotificationCategory, ChannelType},
    preferences::{UserPreferences, CategoryPreference, ChannelPreference},
    NotificationError,
};
use tracing::{trace, debug};

/// Trait for notification channel implementations
#[async_trait::async_trait]
pub trait NotificationChannel: Send + Sync {
    /// Get the channel type
    fn channel_type(&self) -> ChannelType;
    
    /// Send a notification through this channel
    async fn send(&self, notification: &Notification) -> Result<DeliveryResult, NotificationError>;
}

/// Result of a delivery attempt
#[derive(Debug, Clone)]
pub struct DeliveryResult {
    /// Channel the notification was sent through
    pub channel: ChannelType,
    
    /// Whether the delivery was successful
    pub success: bool,
    
    /// Error message if delivery failed
    pub error: Option<String>,
    
    /// Timestamp when the delivery was attempted
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Storage trait for user preferences
#[async_trait::async_trait]
pub trait PreferenceStorage: Send + Sync {
    /// Get user preferences
    async fn get_preferences(&self, user_id: &str) -> Result<UserPreferences, NotificationError>;
    
    /// Update user preferences
    async fn update_preferences(&self, user_id: &str, preferences: UserPreferences) -> Result<(), NotificationError>;
}

/// Primary entry point for notification operations
pub struct NotificationService {
    /// Notification channels
    channels: Vec<Box<dyn NotificationChannel>>,
    
    /// User preference storage
    preferences: Box<dyn PreferenceStorage>,
}

impl NotificationService {
    /// Create a new notification service
    pub fn new(
        channels: Vec<Box<dyn NotificationChannel>>,
        preferences: Box<dyn PreferenceStorage>,
    ) -> Self {
        Self {
            channels,
            preferences,
        }
    }
    
    /// Send a notification
    pub async fn send(&self, notification: Notification) -> Vec<DeliveryResult> {
        trace!("Sending notification: {}", notification);
        
        // Get user preferences
        let preferences = match self.preferences.get_preferences(&notification.user_id).await {
            Ok(prefs) => prefs,
            Err(e) => {
                tracing::error!("Failed to get user preferences: {}", e);
                // Use default preferences if we can't get user preferences
                UserPreferences::default()
            }
        };
        
        // Filter delivery channels based on user preferences
        let filtered_channels: Vec<&Box<dyn NotificationChannel>> = self.channels
            .iter()
            .filter(|channel| {
                preferences.should_send_notification(
                    &notification.category,
                    &channel.channel_type(),
                    &notification.priority,
                )
            })
            .collect();
        
        // Send notification through each filtered channel
        let mut results = Vec::new();
        
        for channel in filtered_channels {
            match channel.send(&notification).await {
                Ok(result) => {
                    results.push(result);
                    debug!("Notification sent through channel: {:?}", channel.channel_type());
                }
                Err(e) => {
                    tracing::error!("Failed to send notification through channel {:?}: {}", channel.channel_type(), e);
                    results.push(DeliveryResult {
                        channel: channel.channel_type(),
                        success: false,
                        error: Some(e.to_string()),
                        timestamp: chrono::Utc::now(),
                    });
                }
            }
        }
        
        results
    }
    
    /// Get notification history for a user
    pub async fn get_history(
        &self,
        user_id: &str,
        query: HistoryQuery,
    ) -> Result<Vec<Notification>, NotificationError> {
        // This is a simplified implementation
        // In a real implementation, we would have a more sophisticated query mechanism
        todo!("Implement notification history querying")
    }
    
    /// Update user preferences
    pub async fn update_preferences(
        &self,
        user_id: &str,
        preferences: UserPreferences,
    ) -> Result<(), NotificationError> {
        self.preferences.update_preferences(user_id, preferences).await
    }
}

/// Query for retrieving notification history
#[derive(Debug, Clone)]
pub struct HistoryQuery {
    /// Filter by category (None means all categories)
    pub category: Option<NotificationCategory>,
    
    /// Filter by read status (None means all statuses)
    pub read: Option<bool>,
    
    /// Limit the number of results
    pub limit: usize,
    
    /// Offset for pagination
    pub offset: usize,
    
    /// Start timestamp
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    
    /// End timestamp
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
}