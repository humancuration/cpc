//! Email notification implementation for payment reminders
//!
//! This module contains the implementation for sending payment reminders via email.

use crate::application::reminder_service::NotificationService;
use crate::domain::reminder::PaymentReminderConfig;
use crate::domain::payment::Invoice;
use async_trait::async_trait;
use lettre::{
    transport::smtp::SmtpTransport,
    Message, Transport,
};
use uuid::Uuid;

/// Error types for email notification operations
#[derive(Debug, thiserror::Error)]
pub enum EmailNotificationError {
    #[error("Email configuration error: {0}")]
    ConfigError(String),
    #[error("Email sending error: {0}")]
    SendError(String),
    #[error("Template rendering error: {0}")]
    TemplateError(String),
}

/// Email notification service
pub struct EmailNotificationService {
    smtp_transport: SmtpTransport,
    from_address: String,
}

impl EmailNotificationService {
    pub fn new(smtp_transport: SmtpTransport, from_address: String) -> Self {
        Self {
            smtp_transport,
            from_address,
        }
    }
}

#[async_trait]
impl NotificationService for EmailNotificationService {
    async fn send_email_reminder(&self, invoice: &Invoice, config: &PaymentReminderConfig) -> Result<(), crate::application::reminder_service::ReminderServiceError> {
        // Render the email template
        let subject = "Payment Reminder";
        let body = self.render_template(config, invoice)
            .map_err(|e| crate::application::reminder_service::ReminderServiceError::NotificationError(e.to_string()))?;

        // Create the email message
        let email = Message::builder()
            .from(self.from_address.parse().map_err(|e| 
                crate::application::reminder_service::ReminderServiceError::NotificationError(
                    format!("Invalid from address: {}", e)
                )
            )?)
            .to(invoice.client_email.parse().map_err(|e| 
                crate::application::reminder_service::ReminderServiceError::NotificationError(
                    format!("Invalid client email: {}", e)
                )
            )?)
            .subject(subject)
            .body(body)
            .map_err(|e| 
                crate::application::reminder_service::ReminderServiceError::NotificationError(
                    format!("Failed to create email message: {}", e)
                )
            )?;

        // Send the email
        self.smtp_transport.send(&email)
            .map_err(|e| 
                crate::application::reminder_service::ReminderServiceError::NotificationError(
                    format!("Failed to send email: {}", e)
                )
            )?;

        Ok(())
    }

    async fn send_sms_reminder(&self, _invoice: &Invoice, _config: &PaymentReminderConfig) -> Result<(), crate::application::reminder_service::ReminderServiceError> {
        // SMS reminders would use a different service
        Err(crate::application::reminder_service::ReminderServiceError::NotificationError(
            "SMS reminders not supported by email service".to_string()
        ))
    }

    async fn send_p2p_reminder(&self, _invoice: &Invoice, _config: &PaymentReminderConfig) -> Result<(), crate::application::reminder_service::ReminderServiceError> {
        // P2P reminders would use a different service
        Err(crate::application::reminder_service::ReminderServiceError::NotificationError(
            "P2P reminders not supported by email service".to_string()
        ))
    }
}

impl EmailNotificationService {
    /// Render the reminder template with invoice data
    fn render_template(&self, config: &PaymentReminderConfig, invoice: &Invoice) -> Result<String, EmailNotificationError> {
        let mut template = config.reminder_template.clone();
        
        // Replace placeholders with actual values
        template = template.replace("{invoice_id}", &invoice.id.to_string());
        template = template.replace("{due_date}", &invoice.due_date.format("%Y-%m-%d").to_string());
        template = template.replace("{client_name}", &invoice.client_name);
        template = template.replace("{total_amount}", &format!("{:.2}", invoice.total_amount));
        
        Ok(template)
    }
}

/// Email service configuration
#[derive(Debug, Clone)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_address: String,
}

/// Create SMTP transport from configuration
pub fn create_smtp_transport(config: &EmailConfig) -> Result<SmtpTransport, EmailNotificationError> {
    use lettre::transport::smtp::authentication::Credentials;
    
    let credentials = Credentials::new(
        config.smtp_username.clone(),
        config.smtp_password.clone(),
    );

    let transport = SmtpTransport::relay(&config.smtp_host)
        .map_err(|e| EmailNotificationError::ConfigError(format!("Failed to create SMTP relay: {}", e)))?
        .credentials(credentials)
        .build();

    Ok(transport)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use rust_decimal::Decimal;
    use uuid::Uuid;

    #[test]
    fn test_render_template() {
        let service = EmailNotificationService {
            smtp_transport: SmtpTransport::unencrypted_localhost(),
            from_address: "test@example.com".to_string(),
        };

        let config = PaymentReminderConfig::new(
            Uuid::new_v4(),
            3,
            7,
            3,
            "Hello {client_name}, this is a reminder for invoice {invoice_id} due on {due_date}. Amount: ${total_amount}".to_string(),
        ).unwrap();

        let invoice = Invoice::new(
            Uuid::new_v4(),
            "John Doe".to_string(),
            "john@example.com".to_string(),
            vec![],
            Decimal::new(10000, 2), // $100.00
            Utc::now() + chrono::Duration::days(7),
        );

        let result = service.render_template(&config, &invoice);
        assert!(result.is_ok());
        let rendered = result.unwrap();
        assert!(rendered.contains("John Doe"));
        assert!(rendered.contains(&invoice.id.to_string()));
        assert!(rendered.contains("$100.00"));
    }
}