//! Calendar module integration example for the notification service

use notification_core::{
    NotificationService,
    Notification,
    NotificationCategory,
    NotificationPriority,
    ChannelType,
    UserPreferences,
    application::scheduler::{NotificationScheduler, Recurrence},
};
use std::collections::HashMap;
use chrono::{Utc, Duration};

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
        println!("Calendar: Sending {} notification: {} - {}", self.channel_type, notification.title, notification.body);
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

/// Calendar service that uses the notification service
struct CalendarService {
    notification_service: NotificationService,
    scheduler: NotificationScheduler,
}

impl CalendarService {
    /// Create a new calendar service
    pub fn new(notification_service: NotificationService) -> Self {
        Self {
            notification_service,
            scheduler: NotificationScheduler::new(),
        }
    }
    
    /// Create a notification for an upcoming event
    pub fn create_event_notification(
        &self,
        user_id: &str,
        event_title: &str,
        minutes_until: i64,
    ) -> Notification {
        Notification::new_immediate(
            user_id.to_string(),
            NotificationCategory::Calendar,
            NotificationPriority::High,
            event_title.to_string(),
            format!("Starting in {} minutes", minutes_until),
            serde_json::json!({}),
            vec![ChannelType::Push, ChannelType::InApp],
        )
    }
    
    /// Schedule event reminders
    pub fn schedule_event_reminders(
        &mut self,
        user_id: &str,
        event_title: &str,
        event_id: &str,
        reminder_times: Vec<i64>, // Minutes before event
    ) -> Result<(), Box<dyn std::error::Error>> {
        for minutes in reminder_times {
            let reminder_time = Utc::now() + Duration::minutes(minutes);
            let notification = Notification::new_scheduled(
                user_id.to_string(),
                NotificationCategory::Calendar,
                NotificationPriority::High,
                event_title.to_string(),
                format!("Starting in {} minutes", minutes),
                serde_json::json!({ "event_id": event_id }),
                vec![ChannelType::Push, ChannelType::InApp],
                reminder_time,
            );
            
            self.scheduler.schedule(notification)?;
        }
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create mock channels
    let push_channel = Box::new(MockNotificationChannel {
        channel_type: ChannelType::Push,
    });
    
    let in_app_channel = Box::new(MockNotificationChannel {
        channel_type: ChannelType::InApp,
    });
    
    let channels = vec![push_channel, in_app_channel];
    
    // Create mock preference storage
    let mut preferences = HashMap::new();
    preferences.insert("user_123".to_string(), UserPreferences::default());
    
    let preference_storage = Box::new(MockPreferenceStorage {
        preferences,
    });
    
    // Create notification service
    let notification_service = NotificationService::new(channels, preference_storage);
    
    // Create calendar service
    let mut calendar_service = CalendarService::new(notification_service);
    
    // Schedule event reminders (10 minutes and 1 minute before event)
    calendar_service.schedule_event_reminders(
        "user_123",
        "Team Meeting",
        "event_456",
        vec![10, 1],
    )?;
    
    // Check for due notifications (in a real app, this would run periodically)
    let due_notifications = calendar_service.scheduler.get_due_notifications();
    println!("Found {} due notifications", due_notifications.len());
    
    // Send due notifications
    for notification in due_notifications {
        let results = calendar_service.notification_service.send(notification).await;
        println!("Sent {} notifications", results.len());
    }
    
    println!("Calendar integration example completed successfully!");
    Ok(())
}