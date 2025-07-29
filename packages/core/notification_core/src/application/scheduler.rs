//! Timing logic
//! 
//! This module handles scheduling and recurrence for notifications.

use crate::domain::types::{Notification, NotificationPriority};
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;
use uuid::Uuid;

/// Recurrence pattern for scheduled notifications
#[derive(Debug, Clone)]
pub enum Recurrence {
    /// No recurrence
    None,
    /// Daily recurrence
    Daily,
    /// Weekly recurrence
    Weekly,
    /// Monthly recurrence
    Monthly,
    /// Yearly recurrence
    Yearly,
    /// Custom interval in seconds
    Custom(i64),
}

/// Notification scheduler
pub struct NotificationScheduler {
    /// Scheduled notifications
    scheduled_notifications: HashMap<Uuid, ScheduledNotification>,
    
    /// Recurring notifications
    recurring_notifications: HashMap<Uuid, RecurringNotification>,
}

/// A scheduled notification
#[derive(Debug, Clone)]
pub struct ScheduledNotification {
    /// The notification to send
    pub notification: Notification,
    
    /// When to send it
    pub scheduled_time: DateTime<Utc>,
}

/// A recurring notification
#[derive(Debug, Clone)]
pub struct RecurringNotification {
    /// The notification template
    pub notification: Notification,
    
    /// Recurrence pattern
    pub recurrence: Recurrence,
    
    /// Last time it was sent
    pub last_sent: Option<DateTime<Utc>>,
    
    /// Next time it should be sent
    pub next_send: DateTime<Utc>,
}

impl NotificationScheduler {
    /// Create a new notification scheduler
    pub fn new() -> Self {
        Self {
            scheduled_notifications: HashMap::new(),
            recurring_notifications: HashMap::new(),
        }
    }
    
    /// Schedule a one-time notification
    pub fn schedule(&mut self, notification: Notification) -> Result<Uuid, Box<dyn std::error::Error>> {
        if let Some(scheduled_time) = notification.scheduled_time {
            let id = notification.id;
            self.scheduled_notifications.insert(
                id,
                ScheduledNotification {
                    notification,
                    scheduled_time,
                },
            );
            Ok(id)
        } else {
            Err("Notification must have a scheduled time".into())
        }
    }
    
    /// Register a recurring notification
    pub fn register(
        &mut self,
        notification: Notification,
        recurrence: Recurrence,
    ) -> Result<Uuid, Box<dyn std::error::Error>> {
        let next_send = if let Some(scheduled_time) = notification.scheduled_time {
            scheduled_time
        } else {
            Utc::now()
        };
        
        let id = notification.id;
        self.recurring_notifications.insert(
            id,
            RecurringNotification {
                notification,
                recurrence,
                last_sent: None,
                next_send,
            },
        );
        Ok(id)
    }
    
    /// Cancel a scheduled notification
    pub fn cancel(&mut self, id: &Uuid) -> bool {
        self.scheduled_notifications.remove(id).is_some() ||
        self.recurring_notifications.remove(id).is_some()
    }
    
    /// Get notifications that should be sent now
    pub fn get_due_notifications(&mut self) -> Vec<Notification> {
        let now = Utc::now();
        let mut due_notifications = Vec::new();
        
        // Check scheduled notifications
        self.scheduled_notifications.retain(|id, scheduled| {
            if scheduled.scheduled_time <= now {
                due_notifications.push(scheduled.notification.clone());
                false // Remove from scheduled notifications
            } else {
                true // Keep in scheduled notifications
            }
        });
        
        // Check recurring notifications
        for (_, recurring) in self.recurring_notifications.iter_mut() {
            if recurring.next_send <= now {
                // Create a new notification based on the template
                let mut new_notification = recurring.notification.clone();
                new_notification.id = Uuid::new_v4(); // Give it a new ID
                new_notification.created_at = Utc::now();
                new_notification.scheduled_time = None; // It's being sent now
                new_notification.delivered_at = None;
                new_notification.read = false;
                
                due_notifications.push(new_notification);
                recurring.last_sent = Some(now);
                
                // Calculate next send time
                recurring.next_send = match recurring.recurrence {
                    Recurrence::None => {
                        // This shouldn't happen for recurring notifications
                        recurring.next_send + Duration::days(1)
                    }
                    Recurrence::Daily => recurring.next_send + Duration::days(1),
                    Recurrence::Weekly => recurring.next_send + Duration::weeks(1),
                    Recurrence::Monthly => {
                        // This is approximate - real implementation would need to handle month boundaries
                        recurring.next_send + Duration::days(30)
                    }
                    Recurrence::Yearly => {
                        // This is approximate - real implementation would need to handle leap years
                        recurring.next_send + Duration::days(365)
                    }
                    Recurrence::Custom(seconds) => recurring.next_send + Duration::seconds(seconds),
                };
            }
        }
        
        due_notifications
    }
    
    /// Get all scheduled notifications
    pub fn get_scheduled_notifications(&self) -> Vec<&ScheduledNotification> {
        self.scheduled_notifications.values().collect()
    }
    
    /// Get all recurring notifications
    pub fn get_recurring_notifications(&self) -> Vec<&RecurringNotification> {
        self.recurring_notifications.values().collect()
    }
}

impl Default for NotificationScheduler {
    fn default() -> Self {
        Self::new()
    }
}