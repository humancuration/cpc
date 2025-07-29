# Notification Service Core

Provides a unified notification system supporting multiple delivery channels with user-controlled preferences, replacing ad-hoc notification implementations across applications.

## Overview

The Notification Service Core offers a comprehensive solution for managing notifications across all CPC applications. It supports multiple delivery channels, user-controlled preferences, scheduling, and recurrence patterns.

## Features

- **Multi-Channel Support**: Push notifications, email, in-app notifications, SMS, and social media
- **User Preferences**: Fine-grained control over notification categories and channels
- **Scheduling**: One-time and recurring notification scheduling
- **Priority Management**: Notification prioritization with quiet hours support
- **Real-time Delivery**: Integration with the Event Bus System for immediate delivery
- **Comprehensive Logging**: Detailed tracking of notification delivery attempts
- **Extensible Architecture**: Easy to add new notification channels

## Architecture

The module follows hexagonal architecture principles with clear separation of concerns:

```
Domain Layer
├── types.rs        # Notification types, channels
├── preferences.rs  # User preferences
└── errors.rs       # Error types

Application Layer
├── service.rs      # Notification orchestration
└── scheduler.rs    # Timing logic

Infrastructure Layer
├── push.rs         # Mobile push (FCM, APNs)
├── email.rs        # Email templates/delivery
├── in_app.rs       # Real-time in-app notifications
└── social.rs       # Social platform delivery
```

## Usage

### Basic Usage

```rust
use notification_core::{
    NotificationService,
    Notification,
    NotificationCategory,
    NotificationPriority,
    ChannelType,
    UserPreferences,
};

// Create notification channels
let push_channel = Box::new(PushNotificationChannel::new(
    std::env::var("FCM_API_KEY").ok(),
    std::env::var("APNS_CERT").ok(),
));

let email_channel = Box::new(EmailNotificationChannel::new(
    Some(SmtpConfig {
        host: "smtp.example.com".to_string(),
        port: 587,
        username: "user@example.com".to_string(),
        password: "password".to_string(),
        use_tls: true,
    }),
));

let channels = vec![push_channel, email_channel];

// Create preference storage (implementation not shown)
let preference_storage = Box::new(YourPreferenceStorageImplementation::new());

// Create notification service
let notification_service = NotificationService::new(channels, preference_storage);

// Create and send a notification
let notification = Notification::new_immediate(
    "user_123".to_string(),
    NotificationCategory::Calendar,
    NotificationPriority::High,
    "Meeting Reminder".to_string(),
    "Your meeting starts in 10 minutes".to_string(),
    serde_json::json!({ "event_id": "event_456" }),
    vec![ChannelType::Push, ChannelType::Email],
);

let results = notification_service.send(notification).await;
println!("Sent {} notifications", results.len());
```

### Calendar Module Integration

```rust
// Calendar service using Notification Service Core
use notification_core::{NotificationService, Notification, NotificationCategory, NotificationPriority, ChannelType};

struct CalendarService {
    notification_service: NotificationService,
}

impl CalendarService {
    async fn send_event_reminder(&self, user_id: &str, event_title: &str, minutes_until: i64) -> Result<(), Box<dyn std::error::Error>> {
        let notification = Notification::new_immediate(
            user_id.to_string(),
            NotificationCategory::Calendar,
            NotificationPriority::High,
            event_title.to_string(),
            format!("Starting in {} minutes", minutes_until),
            serde_json::json!({ "event_id": "event_123" }),
            vec![ChannelType::Push, ChannelType::InApp],
        );
        
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

### Health Module Integration

```rust
// Health service using Notification Service Core
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

## Integration Examples

See the examples directory for integration examples with:
- Basic usage
- Calendar module integration
- Health module integration

Run examples with:
```bash
cargo run --example basic_usage
cargo run --example calendar_integration
cargo run --example health_integration
```

## Testing

Run tests with:
```bash
cargo test
```

## Migration

See [MIGRATION.md](MIGRATION.md) for detailed migration guidance for existing modules.

## Dependencies

- **tokio**: Async runtime
- **serde**: Serialization framework
- **event_bus**: Event bus system for real-time delivery
- **oauth2**: OAuth2 library for social notifications
- **lettre**: Email library (optional)
- **tracing**: Logging and monitoring

## License

This module is part of the CPC software ecosystem and is licensed under the CPC license.