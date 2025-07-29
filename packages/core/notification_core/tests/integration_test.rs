//! Integration tests for the notification service

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
    
    async fn send(&self, _notification: &Notification) -> Result<notification_core::application::service::DeliveryResult, notification_core::NotificationError> {
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

#[tokio::test]
async fn test_notification_sending() {
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
    
    // Create a notification
    let notification = Notification::new_immediate(
        "user_123".to_string(),
        NotificationCategory::Calendar,
        NotificationPriority::High,
        "Meeting Reminder".to_string(),
        "Your meeting starts in 10 minutes".to_string(),
        serde_json::json!({}),
        vec![ChannelType::Push, ChannelType::Email, ChannelType::InApp],
    );
    
    // Send the notification
    let results = notification_service.send(notification).await;
    
    // Verify results
    assert_eq!(results.len(), 3);
    assert!(results.iter().all(|r| r.success));
}

#[tokio::test]
async fn test_notification_with_preferences() {
    // Create mock channels
    let push_channel = Box::new(MockNotificationChannel {
        channel_type: ChannelType::Push,
    });
    
    let email_channel = Box::new(MockNotificationChannel {
        channel_type: ChannelType::Email,
    });
    
    let channels = vec![push_channel, email_channel];
    
    // Create mock preference storage with custom preferences
    let mut category_preferences = HashMap::new();
    category_preferences.insert(
        NotificationCategory::Calendar,
        notification_core::domain::preferences::CategoryPreference::new(
            true,
            NotificationPriority::High,
        ),
    );
    
    let mut channel_preferences = HashMap::new();
    channel_preferences.insert(
        ChannelType::Email,
        notification_core::domain::preferences::ChannelPreference::new(false, false),
    );
    
    let mut preferences = HashMap::new();
    preferences.insert(
        "user_123".to_string(),
        UserPreferences {
            enabled: true,
            default_priority: NotificationPriority::Normal,
            category_preferences,
            channel_preferences,
            quiet_hours: None,
            timezone: "UTC".to_string(),
        },
    );
    
    let preference_storage = Box::new(MockPreferenceStorage {
        preferences,
    });
    
    // Create notification service
    let notification_service = NotificationService::new(channels, preference_storage);
    
    // Create a notification with normal priority (should be filtered out by category preference)
    let notification = Notification::new_immediate(
        "user_123".to_string(),
        NotificationCategory::Calendar,
        NotificationPriority::Normal,
        "Meeting Reminder".to_string(),
        "Your meeting starts in 10 minutes".to_string(),
        serde_json::json!({}),
        vec![ChannelType::Push, ChannelType::Email],
    );
    
    // Send the notification
    let results = notification_service.send(notification).await;
    
    // Only push should be sent (email is disabled, normal priority is filtered out)
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].channel, ChannelType::Push);
}