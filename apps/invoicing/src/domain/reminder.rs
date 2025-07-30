//! Reminder domain models for the invoicing module
//!
//! This module contains the core business entities for automatic payment reminders.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Payment reminder configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentReminderConfig {
    pub id: Uuid,
    pub user_id: Uuid,
    pub enabled: bool,
    pub first_reminder_days: i32,  // Days before due date for first reminder
    pub repeat_reminder_days: i32, // Days between repeat reminders
    pub max_reminders: u32,        // Maximum number of reminders to send
    pub reminder_template: String, // Template for reminder message
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Payment reminder instance
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentReminder {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub reminder_number: u32,
    pub scheduled_date: DateTime<Utc>,
    pub sent_date: Option<DateTime<Utc>>,
    pub status: ReminderStatus,
    pub channel: NotificationChannel,
    pub created_at: DateTime<Utc>,
}

/// Status of a payment reminder
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReminderStatus {
    Scheduled,
    Sent,
    Failed,
    Cancelled,
}

/// Notification channels for reminders
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationChannel {
    Email,
    Sms,
    P2P,
}

/// Error types for reminder operations
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ReminderError {
    #[error("Invalid reminder configuration: {0}")]
    InvalidConfig(String),
    #[error("Reminder not found: {0}")]
    ReminderNotFound(Uuid),
    #[error("Notification error: {0}")]
    NotificationError(String),
    #[error("Data access error: {0}")]
    DataAccessError(String),
}

impl PaymentReminderConfig {
    /// Create a new payment reminder configuration
    pub fn new(
        user_id: Uuid,
        first_reminder_days: i32,
        repeat_reminder_days: i32,
        max_reminders: u32,
        reminder_template: String,
    ) -> Result<Self, ReminderError> {
        if first_reminder_days <= 0 {
            return Err(ReminderError::InvalidConfig(
                "First reminder days must be positive".to_string()
            ));
        }
        
        if repeat_reminder_days <= 0 {
            return Err(ReminderError::InvalidConfig(
                "Repeat reminder days must be positive".to_string()
            ));
        }
        
        if max_reminders == 0 {
            return Err(ReminderError::InvalidConfig(
                "Maximum reminders must be at least 1".to_string()
            ));
        }

        let now = Utc::now();
        Ok(Self {
            id: Uuid::new_v4(),
            user_id,
            enabled: true,
            first_reminder_days,
            repeat_reminder_days,
            max_reminders,
            reminder_template,
            created_at: now,
            updated_at: now,
        })
    }

    /// Update the configuration
    pub fn update(
        &mut self,
        enabled: bool,
        first_reminder_days: i32,
        repeat_reminder_days: i32,
        max_reminders: u32,
        reminder_template: String,
    ) -> Result<(), ReminderError> {
        if first_reminder_days <= 0 {
            return Err(ReminderError::InvalidConfig(
                "First reminder days must be positive".to_string()
            ));
        }
        
        if repeat_reminder_days <= 0 {
            return Err(ReminderError::InvalidConfig(
                "Repeat reminder days must be positive".to_string()
            ));
        }
        
        if max_reminders == 0 {
            return Err(ReminderError::InvalidConfig(
                "Maximum reminders must be at least 1".to_string()
            ));
        }

        self.enabled = enabled;
        self.first_reminder_days = first_reminder_days;
        self.repeat_reminder_days = repeat_reminder_days;
        self.max_reminders = max_reminders;
        self.reminder_template = reminder_template;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Calculate the date for the first reminder
    pub fn calculate_first_reminder_date(&self, due_date: DateTime<Utc>) -> DateTime<Utc> {
        due_date - chrono::Duration::days(self.first_reminder_days as i64)
    }

    /// Calculate the date for the next reminder
    pub fn calculate_next_reminder_date(&self, last_reminder_date: DateTime<Utc>, reminder_number: u32) -> DateTime<Utc> {
        if reminder_number == 1 {
            // First reminder is based on due date
            self.calculate_first_reminder_date(last_reminder_date + chrono::Duration::days(self.first_reminder_days as i64))
        } else {
            // Subsequent reminders are based on the last reminder date
            last_reminder_date + chrono::Duration::days(self.repeat_reminder_days as i64)
        }
    }
}

impl PaymentReminder {
    /// Create a new payment reminder
    pub fn new(
        invoice_id: Uuid,
        reminder_number: u32,
        scheduled_date: DateTime<Utc>,
        channel: NotificationChannel,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            invoice_id,
            reminder_number,
            scheduled_date,
            sent_date: None,
            status: ReminderStatus::Scheduled,
            channel,
            created_at: now,
        }
    }

    /// Mark reminder as sent
    pub fn mark_as_sent(&mut self) {
        self.sent_date = Some(Utc::now());
        self.status = ReminderStatus::Sent;
    }

    /// Mark reminder as failed
    pub fn mark_as_failed(&mut self) {
        self.status = ReminderStatus::Failed;
    }

    /// Mark reminder as cancelled
    pub fn mark_as_cancelled(&mut self) {
        self.status = ReminderStatus::Cancelled;
    }

    /// Check if reminder is due to be sent
    pub fn is_due(&self) -> bool {
        self.status == ReminderStatus::Scheduled && Utc::now() >= self.scheduled_date
    }
}