//! Health module integration example for the notification service

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
        println!("Health: Sending {} notification: {} - {}", self.channel_type, notification.title, notification.body);
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

/// Health service that uses the notification service
struct HealthService {
    notification_service: NotificationService,
    scheduler: NotificationScheduler,
}

impl HealthService {
    /// Create a new health service
    pub fn new(notification_service: NotificationService) -> Self {
        Self {
            notification_service,
            scheduler: NotificationScheduler::new(),
        }
    }
    
    /// Schedule medication reminders
    pub fn schedule_medication_reminder(
        &mut self,
        user_id: &str,
        medication_name: &str,
        dosage: &str,
        schedule: Vec<i64>, // Hours of day to take medication
    ) -> Result<(), Box<dyn std::error::Error>> {
        for hour in schedule {
            let next_dose_time = Utc::now().date_naive().and_hms_opt(hour as u32, 0, 0)
                .unwrap()
                .and_utc();
            
            // If the time is in the past, schedule for tomorrow
            let next_dose_time = if next_dose_time < Utc::now() {
                next_dose_time + Duration::days(1)
            } else {
                next_dose_time
            };
            
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
    
    /// Send a health alert notification
    pub async fn send_health_alert(
        &self,
        user_id: &str,
        alert_type: &str,
        message: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let notification = Notification::new_immediate(
            user_id.to_string(),
            NotificationCategory::Health,
            NotificationPriority::High,
            "Health Alert".to_string(),
            message.to_string(),
            serde_json::json!({ "alert_type": alert_type }),
            vec![ChannelType::Push, ChannelType::Email],
        );
        
        let results = self.notification_service.send(notification).await;
        println!("Sent {} health alert notifications", results.len());
        
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
    
    // Create health service
    let mut health_service = HealthService::new(notification_service);
    
    // Schedule medication reminders (8 AM and 8 PM)
    health_service.schedule_medication_reminder(
        "user_123",
        "Vitamin D",
        "1 tablet",
        vec![8, 20],
    )?;
    
    // Send a health alert
    health_service.send_health_alert(
        "user_123",
        "heart_rate",
        "Your heart rate is elevated. Please take a moment to rest.",
    ).await?;
    
    // Check for due notifications (in a real app, this would run periodically)
    let due_notifications = health_service.scheduler.get_due_notifications();
    println!("Found {} due notifications", due_notifications.len());
    
    // Send due notifications
    for notification in due_notifications {
        let results = health_service.notification_service.send(notification).await;
        println!("Sent {} notifications", results.len());
    }
    
    println!("Health integration example completed successfully!");
    Ok(())
}