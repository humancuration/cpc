use chrono::Utc;
use tokio::time::{sleep, Duration};
use crate::graphql::user_testing::send_weekly_summary;
use crate::services::invoice_reminder::InvoiceReminderScheduler;
use crate::services::invoicing::{InvoiceService, DbPool};
use crate::notifications::NotificationService;
use std::sync::Arc;

pub async fn start_scheduled_jobs() {
    // Start weekly summary job
    tokio::spawn(async move {
        // Weekly summary job: every Monday at 9:00 AM
        loop {
            let now = Utc::now();
            let next_monday = if now.weekday().num_days_from_monday() == 0 {
                // If today is Monday, schedule for next Monday
                now + chrono::Duration::days(7)
            } else {
                // Calculate days until next Monday
                let days_until_monday = 7 - now.weekday().num_days_from_monday();
                now + chrono::Duration::days(days_until_monday as i64)
            };
            
            let next_monday_9am = next_monday.date().and_hms_opt(9, 0, 0).unwrap();
            let duration_until_next = (next_monday_9am - now).to_std().unwrap();
            
            sleep(duration_until_next).await;
            
            // Execute the weekly summary
            send_weekly_summary().await;
            
            // Sleep for a week after completing
            sleep(Duration::from_secs(7 * 24 * 60 * 60)).await;
        }
    });
}

pub async fn start_invoice_reminder_scheduler(
    db: DbPool,
    invoice_service: InvoiceService,
    notification_service: Arc<NotificationService>,
) {
    let scheduler = InvoiceReminderScheduler::new(
        db,
        invoice_service,
        notification_service,
    );
    
    tokio::spawn(async move {
        scheduler.run().await;
    });
}