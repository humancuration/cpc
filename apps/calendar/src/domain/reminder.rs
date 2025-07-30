//! Event reminder domain model with escalation system

use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents an event reminder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventReminder {
    pub id: Uuid,
    pub event_id: Uuid,
    pub user_id: Uuid,
    pub trigger_time: DateTime<Utc>,
    pub method: ReminderMethod,
    pub escalation_level: u8,
    pub status: ReminderStatus,
    pub created_at: DateTime<Utc>,
    pub message: String,
}

impl EventReminder {
    /// Create a new event reminder
    pub fn new(
        event_id: Uuid,
        user_id: Uuid,
        trigger_time: DateTime<Utc>,
        method: ReminderMethod,
        message: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_id,
            user_id,
            trigger_time,
            method,
            escalation_level: 0,
            status: ReminderStatus::Pending,
            created_at: Utc::now(),
            message,
        }
    }

    /// Check if the reminder is due
    pub fn is_due(&self, current_time: DateTime<Utc>) -> bool {
        self.trigger_time <= current_time && self.status == ReminderStatus::Pending
    }

    /// Mark the reminder as sent
    pub fn mark_sent(&mut self, escalation_level: u8) {
        self.status = ReminderStatus::Sent(escalation_level);
    }

    /// Mark the reminder as failed
    pub fn mark_failed(&mut self) {
        self.status = ReminderStatus::Failed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Utc, Duration};
    use uuid::Uuid;

    #[test]
    fn test_create_event_reminder() {
        let event_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let trigger_time = Utc::now() + Duration::hours(1);
        
        let reminder = EventReminder::new(
            event_id,
            user_id,
            trigger_time,
            ReminderMethod::Email,
            "Test reminder".to_string(),
        );
        
        assert_eq!(reminder.event_id, event_id);
        assert_eq!(reminder.user_id, user_id);
        assert_eq!(reminder.trigger_time, trigger_time);
        assert_eq!(reminder.method, ReminderMethod::Email);
        assert_eq!(reminder.escalation_level, 0);
        assert_eq!(reminder.status, ReminderStatus::Pending);
        assert_eq!(reminder.message, "Test reminder");
    }

    #[test]
    fn test_is_due() {
        let event_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let trigger_time = Utc::now() - Duration::minutes(10); // Past time
        
        let reminder = EventReminder::new(
            event_id,
            user_id,
            trigger_time,
            ReminderMethod::Email,
            "Test reminder".to_string(),
        );
        
        assert!(reminder.is_due(Utc::now()));
        
        // Test with future trigger time
        let future_reminder = EventReminder::new(
            event_id,
            user_id,
            Utc::now() + Duration::minutes(10),
            ReminderMethod::Email,
            "Future reminder".to_string(),
        );
        
        assert!(!future_reminder.is_due(Utc::now()));
    }

    #[test]
    fn test_mark_sent() {
        let event_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let trigger_time = Utc::now();
        
        let mut reminder = EventReminder::new(
            event_id,
            user_id,
            trigger_time,
            ReminderMethod::Email,
            "Test reminder".to_string(),
        );
        
        reminder.mark_sent(1);
        assert_eq!(reminder.status, ReminderStatus::Sent(1));
    }

    #[test]
    fn test_reminder_escalation() {
        let base_reminder = Duration::hours(1);
        let mut escalation = ReminderEscalation::new(base_reminder);
        
        escalation = escalation.add_step(EscalationStep::new(
            Duration::minutes(30),
            ReminderMethod::SMS,
            3,
        ));
        
        assert_eq!(escalation.escalation_steps.len(), 1);
        
        let event_time = Utc::now() + Duration::hours(2);
        let reminder_times = escalation.calculate_reminder_times(event_time);
        
        assert_eq!(reminder_times.len(), 2);
        assert_eq!(reminder_times[0], event_time - base_reminder);
        assert_eq!(reminder_times[1], event_time - base_reminder - Duration::minutes(30));
    }
}

/// Methods for delivering reminders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReminderMethod {
    PushNotification,
    Email,
    SMS,
    InApp,
    LocationBased(String), // Geofence name
}

/// Status of a reminder
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReminderStatus {
    Pending,
    Sent(u8), // Escalation level when sent
    Failed,
}

/// Escalation configuration for reminders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReminderEscalation {
    pub base_reminder: Duration, // Initial reminder time before event
    pub escalation_steps: Vec<EscalationStep>,
}

impl ReminderEscalation {
    /// Create a new reminder escalation configuration
    pub fn new(base_reminder: Duration) -> Self {
        Self {
            base_reminder,
            escalation_steps: Vec::new(),
        }
    }

    /// Add an escalation step
    pub fn add_step(mut self, step: EscalationStep) -> Self {
        self.escalation_steps.push(step);
        self
    }

    /// Calculate all reminder times for an event
    pub fn calculate_reminder_times(&self, event_time: DateTime<Utc>) -> Vec<DateTime<Utc>> {
        let mut times = Vec::new();
        
        // Base reminder
        times.push(event_time - self.base_reminder);
        
        // Escalation reminders
        let mut cumulative_delay = self.base_reminder;
        for step in &self.escalation_steps {
            cumulative_delay = cumulative_delay + step.delay;
            times.push(event_time - cumulative_delay);
        }
        
        times
    }
}

/// An escalation step in the reminder system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationStep {
    pub delay: Duration, // Time after previous step
    pub method: ReminderMethod,
    pub max_attempts: u8,
}

impl EscalationStep {
    /// Create a new escalation step
    pub fn new(delay: Duration, method: ReminderMethod, max_attempts: u8) -> Self {
        Self {
            delay,
            method,
            max_attempts,
        }
    }
}