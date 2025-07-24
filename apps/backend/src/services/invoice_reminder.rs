use chrono::{Utc, Duration};
use tokio::time::{sleep, interval};
use std::sync::Arc;
use crate::{
    services::invoicing::InvoiceService,
    db::DbPool,
    notifications::{NotificationService, NotificationType},
};

pub struct InvoiceReminderScheduler {
    db: DbPool,
    invoice_service: InvoiceService,
    notification_service: Arc<NotificationService>,
}

impl InvoiceReminderScheduler {
    pub fn new(
        db: DbPool,
        invoice_service: InvoiceService,
        notification_service: Arc<NotificationService>
    ) -> Self {
        Self { db, invoice_service, notification_service }
    }

    pub async fn run(&self) {
        let mut interval = interval(std::time::Duration::from_secs(60 * 60)); // Run hourly
        loop {
            interval.tick().await;
            if let Err(e) = self.check_reminders().await {
                log::error!("Error in reminder scheduler: {}", e);
            }
        }
    }

    async fn check_reminders(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let now = Utc::now();
        let due_soon = now + Duration::days(3);
        let due_tomorrow = now + Duration::days(1);
        
        // Get invoices due in 3 days
        let invoices = self.invoice_service.get_invoices_due_between(now, due_soon).await?;
        for invoice in invoices {
            if let Err(e) = self.notification_service.send_notification(
                invoice.issuer_id,
                NotificationType::PaymentReminder {
                    invoice_id: invoice.id,
                    days_until_due: 3,
                },
                serde_json::json!({}),
            ).await {
                tracing::error!("Failed to send notification for invoice {}: {}", invoice.id, e);
            }
        }
        
        // Get invoices due tomorrow
        let invoices = self.invoice_service.get_invoices_due_between(due_tomorrow, due_tomorrow + Duration::days(1)).await?;
        for invoice in invoices {
            if let Err(e) = self.notification_service.send_notification(
                invoice.issuer_id,
                NotificationType::PaymentReminder {
                    invoice_id: invoice.id,
                    days_until_due: 1,
                },
                serde_json::json!({}),
            ).await {
                tracing::error!("Failed to send notification for invoice {}: {}", invoice.id, e);
            }
        }
        
        // Get overdue invoices
        let overdue_invoices = self.invoice_service.get_overdue_invoices().await?;
        for invoice in overdue_invoices {
            if let Err(e) = self.notification_service.send_notification(
                invoice.issuer_id,
                NotificationType::PaymentReminder {
                    invoice_id: invoice.id,
                    days_until_due: 0,
                },
                serde_json::json!({}),
            ).await {
                tracing::error!("Failed to send notification for invoice {}: {}", invoice.id, e);
            }
        }
        
        Ok(())
    }
}