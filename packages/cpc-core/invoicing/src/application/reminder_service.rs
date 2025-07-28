//! Reminder service for the invoicing module
//!
//! This module contains the application service for managing automatic payment reminders.

use crate::domain::reminder::{PaymentReminderConfig, PaymentReminder, ReminderStatus, NotificationChannel, ReminderError};
use crate::domain::payment::Invoice;
use crate::domain::status::PaymentStatus;
use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;

/// Error types for reminder service operations
#[derive(Debug, thiserror::Error)]
pub enum ReminderServiceError {
    #[error("Reminder error: {0}")]
    ReminderError(#[from] ReminderError),
    #[error("Repository error: {0}")]
    RepositoryError(String),
    #[error("Notification error: {0}")]
    NotificationError(String),
    #[error("Invoice not found: {0}")]
    InvoiceNotFound(Uuid),
}

#[async_trait]
pub trait ReminderRepository {
    async fn save_config(&self, config: &PaymentReminderConfig) -> Result<(), ReminderError>;
    async fn get_config(&self, user_id: Uuid) -> Result<Option<PaymentReminderConfig>, ReminderError>;
    async fn save_reminder(&self, reminder: &PaymentReminder) -> Result<(), ReminderError>;
    async fn get_pending_reminders(&self) -> Result<Vec<PaymentReminder>, ReminderError>;
    async fn get_reminders_for_invoice(&self, invoice_id: Uuid) -> Result<Vec<PaymentReminder>, ReminderError>;
    async fn update_reminder(&self, reminder: &PaymentReminder) -> Result<(), ReminderError>;
}

#[async_trait]
pub trait InvoiceRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Invoice, ReminderServiceError>;
    async fn get_overdue_invoices(&self) -> Result<Vec<Invoice>, ReminderServiceError>;
}

#[async_trait]
pub trait NotificationService {
    async fn send_email_reminder(&self, invoice: &Invoice, config: &PaymentReminderConfig) -> Result<(), ReminderServiceError>;
    async fn send_sms_reminder(&self, invoice: &Invoice, config: &PaymentReminderConfig) -> Result<(), ReminderServiceError>;
    async fn send_p2p_reminder(&self, invoice: &Invoice, config: &PaymentReminderConfig) -> Result<(), ReminderServiceError>;
}

pub struct ReminderService {
    reminder_repository: Arc<dyn ReminderRepository>,
    invoice_repository: Arc<dyn InvoiceRepository>,
    notification_service: Arc<dyn NotificationService>,
}

impl ReminderService {
    pub fn new(
        reminder_repository: Arc<dyn ReminderRepository>,
        invoice_repository: Arc<dyn InvoiceRepository>,
        notification_service: Arc<dyn NotificationService>,
    ) -> Self {
        Self {
            reminder_repository,
            invoice_repository,
            notification_service,
        }
    }

    /// Process pending reminders
    pub async fn process_pending_reminders(&self) -> Result<(), ReminderServiceError> {
        let pending_reminders = self.reminder_repository.get_pending_reminders().await
            .map_err(|e| ReminderServiceError::RepositoryError(e.to_string()))?;

        for mut reminder in pending_reminders {
            if reminder.is_due() {
                self.process_reminder(&mut reminder).await?;
            }
        }

        Ok(())
    }

    /// Schedule reminders for overdue invoices
    pub async fn schedule_overdue_reminders(&self) -> Result<(), ReminderServiceError> {
        let overdue_invoices = self.invoice_repository.get_overdue_invoices().await?;
        
        for invoice in overdue_invoices {
            if let Some(next_reminder_date) = invoice.next_reminder_date {
                if Utc::now() >= next_reminder_date {
                    self.schedule_reminder(&invoice).await?;
                }
            } else {
                // Schedule first reminder for new overdue invoices
                self.schedule_reminder(&invoice).await?;
            }
        }

        Ok(())
    }

    /// Schedule a reminder for an invoice
    pub async fn schedule_reminder(&self, invoice: &Invoice) -> Result<(), ReminderServiceError> {
        let config = self.reminder_repository.get_config(invoice.client_id).await
            .map_err(|e| ReminderServiceError::RepositoryError(e.to_string()))?
            .unwrap_or_else(|| {
                // Default configuration if none exists
                PaymentReminderConfig::new(
                    invoice.client_id,
                    3,  // 3 days before due date
                    7,  // 7 days between reminders
                    3,  // Maximum 3 reminders
                    "This is a reminder that your invoice #{invoice_id} is due on {due_date}. Please make payment at your earliest convenience.".to_string(),
                ).unwrap_or_else(|_| PaymentReminderConfig {
                    id: Uuid::new_v4(),
                    user_id: invoice.client_id,
                    enabled: true,
                    first_reminder_days: 3,
                    repeat_reminder_days: 7,
                    max_reminders: 3,
                    reminder_template: "This is a reminder that your invoice #{invoice_id} is due on {due_date}. Please make payment at your earliest convenience.".to_string(),
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                })
            });

        if !config.enabled {
            return Ok(());
        }

        // Get existing reminders for this invoice
        let existing_reminders = self.reminder_repository.get_reminders_for_invoice(invoice.id).await
            .map_err(|e| ReminderServiceError::RepositoryError(e.to_string()))?;

        // Determine the next reminder number
        let next_reminder_number = existing_reminders.len() as u32 + 1;

        // Check if we've reached the maximum number of reminders
        if next_reminder_number > config.max_reminders {
            return Ok(());
        }

        // Calculate the scheduled date
        let scheduled_date = if existing_reminders.is_empty() {
            // First reminder
            config.calculate_first_reminder_date(invoice.due_date)
        } else {
            // Subsequent reminder
            let last_reminder = existing_reminders.last().unwrap();
            config.calculate_next_reminder_date(last_reminder.scheduled_date, next_reminder_number)
        };

        // Create and save the reminder
        let reminder = PaymentReminder::new(
            invoice.id,
            next_reminder_number,
            scheduled_date,
            NotificationChannel::Email, // Default to email
        );

        self.reminder_repository.save_reminder(&reminder).await
            .map_err(|e| ReminderServiceError::RepositoryError(e.to_string()))?;

        Ok(())
    }

    /// Process an individual reminder
    async fn process_reminder(&self, reminder: &mut PaymentReminder) -> Result<(), ReminderServiceError> {
        // Get the invoice for this reminder
        let invoice = self.invoice_repository.find_by_id(reminder.invoice_id).await?;

        // Get the reminder configuration
        let config = self.reminder_repository.get_config(invoice.client_id).await
            .map_err(|e| ReminderServiceError::RepositoryError(e.to_string()))?
            .ok_or_else(|| ReminderServiceError::RepositoryError(
                "No reminder configuration found for user".to_string()
            ))?;

        // Send the reminder based on the configured channel
        let result = match reminder.channel {
            NotificationChannel::Email => {
                self.notification_service.send_email_reminder(&invoice, &config).await
            },
            NotificationChannel::Sms => {
                self.notification_service.send_sms_reminder(&invoice, &config).await
            },
            NotificationChannel::P2P => {
                self.notification_service.send_p2p_reminder(&invoice, &config).await
            },
        };

        match result {
            Ok(_) => {
                // Mark reminder as sent
                reminder.mark_as_sent();
            },
            Err(e) => {
                // Mark reminder as failed
                reminder.mark_as_failed();
                eprintln!("Failed to send reminder {}: {}", reminder.id, e);
            }
        }

        // Update the reminder in the repository
        self.reminder_repository.update_reminder(reminder).await
            .map_err(|e| ReminderServiceError::RepositoryError(e.to_string()))?;

        // Schedule the next reminder if needed
        if reminder.status == ReminderStatus::Sent {
            self.schedule_next_reminder(&invoice, &config, reminder.reminder_number).await?;
        }

        Ok(())
    }

    /// Schedule the next reminder if needed
    async fn schedule_next_reminder(&self, invoice: &Invoice, config: &PaymentReminderConfig, last_reminder_number: u32) -> Result<(), ReminderServiceError> {
        // Check if we've reached the maximum number of reminders
        if last_reminder_number >= config.max_reminders {
            return Ok(());
        }

        // Calculate the next reminder date
        let last_reminder_date = Utc::now(); // Use current time as last reminder date
        let next_reminder_date = config.calculate_next_reminder_date(last_reminder_date, last_reminder_number + 1);

        // Create and save the next reminder
        let next_reminder = PaymentReminder::new(
            invoice.id,
            last_reminder_number + 1,
            next_reminder_date,
            NotificationChannel::Email, // Default to email
        );

        self.reminder_repository.save_reminder(&next_reminder).await
            .map_err(|e| ReminderServiceError::RepositoryError(e.to_string()))?;

        // Update the invoice with the next reminder date
        // Note: This would require updating the invoice repository with an update method
        // that can set the next_reminder_date field

        Ok(())
    }
}