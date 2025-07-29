//! Reminder scheduler for the invoicing module
//!
//! This module contains the scheduler for processing payment reminders at regular intervals.

use crate::application::reminder_service::ReminderService;
use std::sync::Arc;
use tokio::time::{interval, Duration, Interval};
use tracing::{info, error};

/// Reminder scheduler configuration
#[derive(Debug, Clone)]
pub struct SchedulerConfig {
    /// Interval between processing runs (in seconds)
    pub processing_interval: u64,
    /// Whether to enable the scheduler
    pub enabled: bool,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            processing_interval: 300, // 5 minutes
            enabled: true,
        }
    }
}

/// Reminder scheduler
pub struct ReminderScheduler {
    reminder_service: Arc<ReminderService>,
    config: SchedulerConfig,
    interval: Interval,
}

impl ReminderScheduler {
    pub fn new(reminder_service: Arc<ReminderService>, config: SchedulerConfig) -> Self {
        let interval = interval(Duration::from_secs(config.processing_interval));
        Self {
            reminder_service,
            config,
            interval,
        }
    }

    /// Start the scheduler
    pub async fn start(&mut self) {
        if !self.config.enabled {
            info!("Reminder scheduler is disabled");
            return;
        }

        info!("Starting reminder scheduler with {} second interval", self.config.processing_interval);
        
        loop {
            self.interval.tick().await;
            
            if let Err(e) = self.process_reminders().await {
                error!("Error processing reminders: {}", e);
            }
        }
    }

    /// Process pending reminders
    async fn process_reminders(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Processing pending reminders");
        
        // Process pending reminders
        self.reminder_service.process_pending_reminders().await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        
        // Schedule reminders for overdue invoices
        self.reminder_service.schedule_overdue_reminders().await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        
        info!("Finished processing reminders");
        Ok(())
    }

    /// Run a single processing cycle
    pub async fn run_once(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.process_reminders().await
    }
}

/// Background job processor for reminders
pub struct ReminderJobProcessor {
    reminder_service: Arc<ReminderService>,
}

impl ReminderJobProcessor {
    pub fn new(reminder_service: Arc<ReminderService>) -> Self {
        Self { reminder_service }
    }

    /// Process all pending reminders
    pub async fn process_pending_reminders(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.reminder_service.process_pending_reminders().await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Schedule reminders for overdue invoices
    pub async fn schedule_overdue_reminders(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.reminder_service.schedule_overdue_reminders().await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }
}

/// Manual trigger for reminder processing
pub struct ReminderProcessorTrigger {
    job_processor: Arc<ReminderJobProcessor>,
}

impl ReminderProcessorTrigger {
    pub fn new(job_processor: Arc<ReminderJobProcessor>) -> Self {
        Self { job_processor }
    }

    /// Trigger immediate processing of reminders
    pub async fn trigger_processing(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Manually triggering reminder processing");
        
        self.job_processor.process_pending_reminders().await?;
        self.job_processor.schedule_overdue_reminders().await?;
        
        info!("Manual reminder processing completed");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::reminder_service::{ReminderRepository, InvoiceRepository, NotificationService};
    use crate::domain::reminder::{PaymentReminderConfig, PaymentReminder, ReminderStatus, NotificationChannel};
    use crate::domain::payment::Invoice;
    use async_trait::async_trait;
    use uuid::Uuid;
    use chrono::Utc;

    struct MockReminderRepository;
    struct MockInvoiceRepository;
    struct MockNotificationService;

    #[async_trait]
    impl ReminderRepository for MockReminderRepository {
        async fn save_config(&self, _config: &PaymentReminderConfig) -> Result<(), crate::domain::reminder::ReminderError> {
            Ok(())
        }

        async fn get_config(&self, _user_id: Uuid) -> Result<Option<PaymentReminderConfig>, crate::domain::reminder::ReminderError> {
            Ok(None)
        }

        async fn save_reminder(&self, _reminder: &PaymentReminder) -> Result<(), crate::domain::reminder::ReminderError> {
            Ok(())
        }

        async fn get_pending_reminders(&self) -> Result<Vec<PaymentReminder>, crate::domain::reminder::ReminderError> {
            Ok(vec![])
        }

        async fn get_reminders_for_invoice(&self, _invoice_id: Uuid) -> Result<Vec<PaymentReminder>, crate::domain::reminder::ReminderError> {
            Ok(vec![])
        }

        async fn update_reminder(&self, _reminder: &PaymentReminder) -> Result<(), crate::domain::reminder::ReminderError> {
            Ok(())
        }
    }

    #[async_trait]
    impl InvoiceRepository for MockInvoiceRepository {
        async fn find_by_id(&self, _id: Uuid) -> Result<Invoice, crate::application::reminder_service::ReminderServiceError> {
            Err(crate::application::reminder_service::ReminderServiceError::InvoiceNotFound(_id))
        }

        async fn get_overdue_invoices(&self) -> Result<Vec<Invoice>, crate::application::reminder_service::ReminderServiceError> {
            Ok(vec![])
        }
    }

    #[async_trait]
    impl NotificationService for MockNotificationService {
        async fn send_email_reminder(&self, _invoice: &Invoice, _config: &PaymentReminderConfig) -> Result<(), crate::application::reminder_service::ReminderServiceError> {
            Ok(())
        }

        async fn send_sms_reminder(&self, _invoice: &Invoice, _config: &PaymentReminderConfig) -> Result<(), crate::application::reminder_service::ReminderServiceError> {
            Ok(())
        }

        async fn send_p2p_reminder(&self, _invoice: &Invoice, _config: &PaymentReminderConfig) -> Result<(), crate::application::reminder_service::ReminderServiceError> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_reminder_scheduler_creation() {
        let reminder_service = Arc::new(ReminderService::new(
            Arc::new(MockReminderRepository),
            Arc::new(MockInvoiceRepository),
            Arc::new(MockNotificationService),
        ));

        let config = SchedulerConfig {
            processing_interval: 1, // 1 second for testing
            enabled: true,
        };

        let scheduler = ReminderScheduler::new(reminder_service, config);
        assert_eq!(scheduler.config.processing_interval, 1);
        assert_eq!(scheduler.config.enabled, true);
    }

    #[tokio::test]
    async fn test_reminder_job_processor() {
        let reminder_service = Arc::new(ReminderService::new(
            Arc::new(MockReminderRepository),
            Arc::new(MockInvoiceRepository),
            Arc::new(MockNotificationService),
        ));

        let job_processor = ReminderJobProcessor::new(reminder_service);
        
        // These should not panic
        let _ = job_processor.process_pending_reminders().await;
        let _ = job_processor.schedule_overdue_reminders().await;
    }
}