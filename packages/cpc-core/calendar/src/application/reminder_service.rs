//! Reminder service for calendar events

use crate::domain::{
    EventReminder, ReminderMethod, ReminderEscalation, CalendarError, CalendarEvent
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Repository trait for event reminders
#[async_trait]
pub trait ReminderRepository: Send + Sync {
    async fn save(&self, reminder: &EventReminder) -> Result<(), CalendarError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<EventReminder>, CalendarError>;
    async fn find_due_reminders(&self, current_time: DateTime<Utc>) -> Result<Vec<EventReminder>, CalendarError>;
    async fn update_status(&self, id: Uuid, status: crate::domain::ReminderStatus) -> Result<(), CalendarError>;
    async fn delete(&self, id: Uuid) -> Result<(), CalendarError>;
}

/// Notification service trait
#[async_trait]
pub trait NotificationService: Send + Sync {
    async fn send_notification(
        &self,
        user_id: &Uuid,
        message: &str,
        method: &ReminderMethod,
    ) -> Result<(), CalendarError>;
}

/// Reminder service for processing event reminders
pub struct ReminderService {
    repository: Arc<dyn ReminderRepository>,
    notification_service: Arc<dyn NotificationService>,
}

impl ReminderService {
    /// Create a new reminder service
    pub fn new(
        repository: Arc<dyn ReminderRepository>,
        notification_service: Arc<dyn NotificationService>,
    ) -> Self {
        Self {
            repository,
            notification_service,
        }
    }

    /// Process all due reminders
    pub async fn process_reminders(&self) -> Result<(), CalendarError> {
        let now = Utc::now();
        let due_reminders = self.repository.find_due_reminders(now).await?;
        
        for mut reminder in due_reminders {
            // Send notification through appropriate channel
            if let Err(_) = self.notification_service.send_notification(
                &reminder.user_id,
                &reminder.message,
                &reminder.method,
            ).await {
                // If notification fails, mark reminder as failed
                reminder.mark_failed();
                self.repository.update_status(reminder.id, reminder.status.clone()).await?;
                continue;
            }
            
            // Mark reminder as sent
            reminder.mark_sent(reminder.escalation_level);
            self.repository.update_status(reminder.id, reminder.status.clone()).await?;
        }
        
        Ok(())
    }

    /// Create reminders for an event based on escalation rules
    pub async fn create_event_reminders(
        &self,
        event: &CalendarEvent,
        escalation: &ReminderEscalation,
    ) -> Result<Vec<EventReminder>, CalendarError> {
        let mut reminders = Vec::new();
        
        // Calculate all reminder times
        let reminder_times = escalation.calculate_reminder_times(event.start);
        
        // Create base reminder
        if let Some(base_time) = reminder_times.first() {
            let reminder = EventReminder::new(
                event.id,
                event.user_id,
                *base_time,
                ReminderMethod::InApp, // Default method
                format!("Reminder: {}", event.title),
            );
            
            self.repository.save(&reminder).await?;
            reminders.push(reminder);
        }
        
        // Create escalation reminders
        for (i, time) in reminder_times.iter().enumerate().skip(1) {
            if let Some(step) = escalation.escalation_steps.get(i - 1) {
                let reminder = EventReminder::new(
                    event.id,
                    event.user_id,
                    *time,
                    step.method.clone(),
                    format!("Escalation reminder: {}", event.title),
                );
                
                self.repository.save(&reminder).await?;
                reminders.push(reminder);
            }
        }
        
        Ok(reminders)
    }

    /// Get reminders for a user
    pub async fn get_user_reminders(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<EventReminder>, CalendarError> {
        // In a real implementation, we would query the repository for reminders by user_id
        // For now, we'll return an empty vector
        Ok(Vec::new())
    }

    /// Cancel a reminder
    pub async fn cancel_reminder(
        &self,
        reminder_id: Uuid,
    ) -> Result<(), CalendarError> {
        self.repository.delete(reminder_id).await
    }

    /// Determine the next escalation step for a reminder
    fn determine_next_escalation_step(
        &self,
        reminder: &EventReminder,
        escalation: &ReminderEscalation,
    ) -> Option<crate::domain::EscalationStep> {
        if reminder.escalation_level as usize >= escalation.escalation_steps.len() {
            return None;
        }
        
        escalation.escalation_steps.get(reminder.escalation_level as usize).cloned()
    }
}