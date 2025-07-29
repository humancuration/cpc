//! Basic usage example for the notification service

use notification_core::{
    NotificationService,
    Notification,
    NotificationCategory,
    NotificationPriority,
    ChannelType,
    UserPreferences,
};
use std::collections::HashMap;

/// Mock notification channel for testing
struct MockNotificationChannel {
    channel_type: ChannelType,
}

#[async_trait::async_trait]
impl notification_core::application::service::NotificationChannel for MockNotificationChannel {
    fn channel_type(&self) -> ChannelType {
        self.channel_type.clone()
    }
    
    async fn send(&self, notification: &Notification) -> Result<notification_core::application::service::DeliveryResult, notification_core::NotificationError> {
        println!("Sending {} notification: {} - {}", self.channel_type, notification.title, notification.body);
        Ok(notification_core::application::service::DeliveryResult {
            channel: self.channel_type.clone(),
            success: true,
            error: None,
            timestamp: chrono::Utc::now(),
        })
    }
}

/// Mock preference storage for testing
struct MockPreferenceStorage {
    preferences: HashMap<String, UserPreferences>,
}

#[async_trait::async_trait]
impl notification_core::application::service::PreferenceStorage for MockPreferenceStorage {
    async fn get_preferences(&self, user_id: &str) -> Result<UserPreferences, notification_core::NotificationError> {
        Ok(self.preferences.get(user_id).cloned().unwrap_or_default())
    }
    
    async fn update_preferences(&self, _user_id: &str, _preferences: UserPreferences) -> Result<(), notification_core::NotificationError> {
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create mock channels
    let push_channel = Box::new(MockNotificationChannel {
        channel_type: ChannelType::Push,
    });
    
    let email_channel = Box::new(MockNotificationChannel {
        channel_type: ChannelType::Email,
    });
    
    let in_app_channel = Box::new(MockNotificationChannel {
        channel_type: ChannelType::InApp,
    });
    
    let channels = vec![push_channel, email_channel, in_app_channel];
    
    // Create mock preference storage
    let mut preferences = HashMap::new();
    preferences.insert("user_123".to_string(), UserPreferences::default());
    
    let preference_storage = Box::new(MockPreferenceStorage {
        preferences,
    });
    
    // Create notification service
    let notification_service = NotificationService::new(channels, preference_storage);
    
    // Create a calendar event reminder notification
    let calendar_notification = Notification::new_immediate(
        "user_123".to_string(),
        NotificationCategory::Calendar,
        NotificationPriority::High,
        "Meeting Reminder".to_string(),
        "Your meeting with the team starts in 10 minutes".to_string(),
        serde_json::json!({
            "event_id": "event_456",
            "meeting_link": "https://meet.example.com/abc123"
        }),
        vec![ChannelType::Push, ChannelType::Email, ChannelType::InApp],
    );
    
    // Send the notification
    let results = notification_service.send(calendar_notification).await;
    println!("Sent {} notifications", results.len());
    
    // Create a transaction alert notification
    let transaction_notification = Notification::new_immediate(
        "user_123".to_string(),
        NotificationCategory::Transaction,
        NotificationPriority::Normal,
        "Large Transaction Alert".to_string(),
        "A transaction of $1,250.00 was made on your account".to_string(),
        serde_json::json!({
            "transaction_id": "txn_789",
            "amount": 1250.00,
            "merchant": "Electronics Store"
        }),
        vec![ChannelType::Push, ChannelType::Email],
    );
    
    // Send the notification
    let results = notification_service.send(transaction_notification).await;
    println!("Sent {} notifications", results.len());
    
    println!("Notification service example completed successfully!");
    Ok(())
}