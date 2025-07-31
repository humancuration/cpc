# Migration Guide for Notification Service Core

This document provides guidance on migrating existing modules to use the new Notification Service Core.

## Overview

The Notification Service Core provides a unified notification system supporting multiple delivery channels with user-controlled preferences, replacing ad-hoc notification implementations across applications.

## Migration Steps

### 1. Update Cargo.toml

Add the notification core dependency to your module's Cargo.toml:

```toml
[dependencies]
notification_core = { path = "../notification_core" }
```

### 2. Replace Direct Notification Implementations

#### Before (Calendar Module Example)
```rust
// Direct notification implementation
use tokio::sync::mpsc;

struct CalendarNotificationService {
    // Direct push notification sending
    fcm_api_key: String,
}

impl CalendarNotificationService {
    async fn send_event_reminder(&self, user_id: &str, event_title: &str, minutes_until: i64) -> Result<(), Box<dyn std::error::Error>> {
        // Direct push notification sending logic
        // This would typically involve:
        // 1. Looking up the user's device token
        // 2. Formatting the notification payload
        // 3. Sending to FCM/APNs
        // 4. Handling the response
        
        println!("Sending push notification to user {}: {} in {} minutes", user_id, event_title, minutes_until);
        Ok(())
    }
}
```

#### After (Using Notification Service Core)
```rust
// Using Notification Service Core
use notification_core::{NotificationService, Notification, NotificationCategory, NotificationPriority, ChannelType};

struct CalendarService {
    notification_service: NotificationService,
}

impl CalendarService {
    async fn send_event_reminder(&self, user_id: &str, event_title: &str, minutes_until: i64) -> Result<(), Box<dyn std::error::Error>> {
        // Create notification using Notification Service Core
        let notification = Notification::new_immediate(
            user_id.to_string(),
            NotificationCategory::Calendar,
            NotificationPriority::High,
            event_title.to_string(),
            format!("Starting in {} minutes", minutes_until),
            serde_json::json!({ "event_id": "event_123" }),
            vec![ChannelType::Push, ChannelType::InApp],
        );
        
        // Send through the notification service
        let results = self.notification_service.send(notification).await;
        
        // Handle results if needed
        for result in results {
            if !result.success {
                eprintln!("Failed to send notification through {:?}: {:?}", result.channel, result.error);
            }
        }
        
        Ok(())
    }
}
```

### 3. Update Service Initialization

#### Before (Calendar Module Example)
```rust
// Direct notification initialization
let notification_service = CalendarNotificationService {
    fcm_api_key: std::env::var("FCM_API_KEY").unwrap_or_default(),
};
```

#### After (Using Notification Service Core)
```rust
// Using Notification Service Core
use notification_core::{NotificationService, infrastructure::push::PushNotificationChannel};

// Create notification channels
let push_channel = Box::new(PushNotificationChannel::new(
    std::env::var("FCM_API_KEY").ok(),
    std::env::var("APNS_CERT").ok(),
));

let email_channel = Box::new(EmailNotificationChannel::new(
    Some(SmtpConfig {
        host: std::env::var("SMTP_HOST").unwrap_or("localhost".to_string()),
        port: std::env::var("SMTP_PORT").unwrap_or("587".to_string()).parse().unwrap_or(587),
        username: std::env::var("SMTP_USERNAME").unwrap_or_default(),
        password: std::env::var("SMTP_PASSWORD").unwrap_or_default(),
        use_tls: true,
    }),
));

let channels = vec![push_channel, email_channel];

// Create preference storage (this would be a real implementation)
let preference_storage = Box::new(YourPreferenceStorageImplementation::new());

// Create notification service
let notification_service = NotificationService::new(channels, preference_storage);
```

## Scheduling Integration

The Notification Service Core includes a scheduler for recurring notifications:

```rust
use notification_core::{
    NotificationService,
    Notification,
    NotificationCategory,
    NotificationPriority,
    ChannelType,
    application::scheduler::{NotificationScheduler, Recurrence},
};

struct HealthService {
    notification_service: NotificationService,
    scheduler: NotificationScheduler,
}

impl HealthService {
    fn schedule_medication_reminder(
        &mut self,
        user_id: &str,
        medication_name: &str,
        dosage: &str,
        schedule: Vec<i64>, // Hours of day to take medication
    ) -> Result<(), Box<dyn std::error::Error>> {
        for hour in schedule {
            let next_dose_time = chrono::Utc::now().date_naive().and_hms_opt(hour as u32, 0, 0)
                .unwrap()
                .and_utc();
            
            let notification = Notification::new_scheduled(
                user_id.to_string(),
                NotificationCategory::Health,
                NotificationPriority::Normal,
                format!("Time to take {}", medication_name),
                format!("Please take {} of {}", dosage, medication_name),
                serde_json::json!({ "medication": medication_name }),
                vec![ChannelType::Push, ChannelType::InApp],
                next_dose_time,
            );
            
            // Register as a recurring daily notification
            self.scheduler.register(notification, Recurrence::Daily)?;
        }
        
        Ok(())
    }
}
```

## User Preferences Management

The Notification Service Core provides comprehensive user preference management:

```rust
use notification_core::{NotificationService, UserPreferences, NotificationCategory, ChannelType, NotificationPriority};

struct PreferenceManager {
    notification_service: NotificationService,
}

impl PreferenceManager {
    async fn update_user_preferences(&self, user_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut preferences = UserPreferences::new();
        
        // Disable marketing notifications
        preferences.category_preferences.insert(
            NotificationCategory::Marketing,
            notification_core::domain::preferences::CategoryPreference::new(false, NotificationPriority::Normal),
        );
        
        // Disable email notifications
        preferences.channel_preferences.insert(
            ChannelType::Email,
            notification_core::domain::preferences::ChannelPreference::new(false, false),
        );
        
        // Set quiet hours (10 PM to 7 AM)
        preferences.quiet_hours = Some((22, 7));
        
        // Update preferences
        self.notification_service.update_preferences(user_id, preferences).await?;
        
        Ok(())
    }
}
```

## Testing During Migration

Use mock implementations for testing:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use notification_core::{NotificationService, Notification, NotificationCategory, NotificationPriority, ChannelType};
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
            println!("Mock sending {} notification: {} - {}", self.channel_type, notification.title, notification.body);
            Ok(notification_core::application::service::DeliveryResult {
                channel: self.channel_type.clone(),
                success: true,
                error: None,
                timestamp: chrono::Utc::now(),
            })
        }
    }

    #[tokio::test]
    async fn test_with_mock_channels() {
        // Create mock channels
        let push_channel = Box::new(MockNotificationChannel {
            channel_type: ChannelType::Push,
        });
        
        let channels = vec![push_channel];
        
        // Create notification service with mock channels
        let notification_service = NotificationService::new(channels, Box::new(YourMockPreferenceStorage::new()));
        
        // Test your service logic
        let notification = Notification::new_immediate(
            "user_123".to_string(),
            NotificationCategory::Calendar,
            NotificationPriority::High,
            "Test Event".to_string(),
            "This is a test notification".to_string(),
            serde_json::json!({}),
            vec![ChannelType::Push],
        );
        
        let results = notification_service.send(notification).await;
        assert_eq!(results.len(), 1);
        assert!(results[0].success);
    }
}
```

## Performance Considerations

1. The Notification Service Core uses efficient async operations for sending notifications.

2. Notification filtering based on user preferences is done before sending to minimize unnecessary operations.

3. The scheduler runs efficiently and only processes notifications that are due.

## Troubleshooting

### Common Issues

1. **Channel Not Supported**: Ensure all required notification channels are properly configured and added to the NotificationService.

2. **Preference Storage Errors**: Verify that the preference storage implementation is correctly set up and accessible.

3. **Scheduling Issues**: Check that scheduled times are correctly calculated and that the scheduler is running periodically.

### Logging and Monitoring

The Notification Service Core uses tracing for logging. Enable tracing in your application to monitor notification operations:

```rust
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // ... rest of your application
}
```

This will provide detailed logs of notification operations, including sending, scheduling, and any errors that occur.