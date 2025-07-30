//! SMS notification implementation for payment reminders
//!
//! This module contains the implementation for sending payment reminders via SMS.

use crate::application::reminder_service::NotificationService;
use crate::domain::reminder::PaymentReminderConfig;
use crate::domain::payment::Invoice;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Error types for SMS notification operations
#[derive(Debug, thiserror::Error)]
pub enum SmsNotificationError {
    #[error("SMS configuration error: {0}")]
    ConfigError(String),
    #[error("SMS sending error: {0}")]
    SendError(String),
    #[error("Template rendering error: {0}")]
    TemplateError(String),
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
}

/// SMS notification service
pub struct SmsNotificationService {
    client: Client,
    api_key: String,
    api_url: String,
    from_number: String,
}

impl SmsNotificationService {
    pub fn new(api_key: String, api_url: String, from_number: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            api_url,
            from_number,
        }
    }
}

#[async_trait]
impl NotificationService for SmsNotificationService {
    async fn send_email_reminder(&self, _invoice: &Invoice, _config: &PaymentReminderConfig) -> Result<(), crate::application::reminder_service::ReminderServiceError> {
        // Email reminders would use a different service
        Err(crate::application::reminder_service::ReminderServiceError::NotificationError(
            "Email reminders not supported by SMS service".to_string()
        ))
    }

    async fn send_sms_reminder(&self, invoice: &Invoice, config: &PaymentReminderConfig) -> Result<(), crate::application::reminder_service::ReminderServiceError> {
        // Render the SMS template
        let body = self.render_template(config, invoice)
            .map_err(|e| crate::application::reminder_service::ReminderServiceError::NotificationError(e.to_string()))?;

        // Send the SMS using a generic SMS API
        self.send_sms(&invoice.client_email, &body).await // Using client_email as phone number for demo
    }

    async fn send_p2p_reminder(&self, _invoice: &Invoice, _config: &PaymentReminderConfig) -> Result<(), crate::application::reminder_service::ReminderServiceError> {
        // P2P reminders would use a different service
        Err(crate::application::reminder_service::ReminderServiceError::NotificationError(
            "P2P reminders not supported by SMS service".to_string()
        ))
    }
}

impl SmsNotificationService {
    /// Render the reminder template with invoice data
    fn render_template(&self, config: &PaymentReminderConfig, invoice: &Invoice) -> Result<String, SmsNotificationError> {
        let mut template = config.reminder_template.clone();
        
        // Replace placeholders with actual values
        template = template.replace("{invoice_id}", &invoice.id.to_string());
        template = template.replace("{due_date}", &invoice.due_date.format("%Y-%m-%d").to_string());
        template = template.replace("{client_name}", &invoice.client_name);
        template = template.replace("{total_amount}", &format!("{:.2}", invoice.total_amount));
        
        // SMS messages should be concise
        if template.len() > 160 {
            template = template.chars().take(157).collect::<String>() + "...";
        }
        
        Ok(template)
    }

    /// Send SMS using a generic SMS API
    async fn send_sms(&self, to: &str, message: &str) -> Result<(), crate::application::reminder_service::ReminderServiceError> {
        // This is a generic implementation that would need to be adapted for specific SMS providers
        // Example for Twilio-like API:
        
        let mut params = HashMap::new();
        params.insert("From", self.from_number.clone());
        params.insert("To", to.to_string());
        params.insert("Body", message.to_string());

        let response = self.client
            .post(&self.api_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .form(&params)
            .send()
            .await
            .map_err(|e| crate::application::reminder_service::ReminderServiceError::NotificationError(
                format!("Failed to send SMS: {}", e)
            ))?;

        if !response.status().is_success() {
            let error_text = response.text().await
                .map_err(|e| crate::application::reminder_service::ReminderServiceError::NotificationError(
                    format!("Failed to read error response: {}", e)
                ))?;
            return Err(crate::application::reminder_service::ReminderServiceError::NotificationError(
                format!("SMS API error: {}", error_text)
            ));
        }

        Ok(())
    }
}

/// SMS service configuration
#[derive(Debug, Clone)]
pub struct SmsConfig {
    pub api_key: String,
    pub api_url: String,
    pub from_number: String,
}

/// Twilio SMS service implementation
pub struct TwilioSmsService {
    client: Client,
    account_sid: String,
    auth_token: String,
    from_number: String,
}

impl TwilioSmsService {
    pub fn new(account_sid: String, auth_token: String, from_number: String) -> Self {
        Self {
            client: Client::new(),
            account_sid,
            auth_token,
            from_number,
        }
    }
}

#[async_trait]
impl NotificationService for TwilioSmsService {
    async fn send_email_reminder(&self, _invoice: &Invoice, _config: &PaymentReminderConfig) -> Result<(), crate::application::reminder_service::ReminderServiceError> {
        Err(crate::application::reminder_service::ReminderServiceError::NotificationError(
            "Email reminders not supported by Twilio SMS service".to_string()
        ))
    }

    async fn send_sms_reminder(&self, invoice: &Invoice, config: &PaymentReminderConfig) -> Result<(), crate::application::reminder_service::ReminderServiceError> {
        let body = self.render_template(config, invoice)
            .map_err(|e| crate::application::reminder_service::ReminderServiceError::NotificationError(e.to_string()))?;

        let url = format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
            self.account_sid
        );

        let mut params = HashMap::new();
        params.insert("From", self.from_number.clone());
        params.insert("To", invoice.client_email.clone()); // Using client_email as phone number for demo
        params.insert("Body", body);

        let response = self.client
            .post(&url)
            .basic_auth(&self.account_sid, Some(&self.auth_token))
            .form(&params)
            .send()
            .await
            .map_err(|e| crate::application::reminder_service::ReminderServiceError::NotificationError(
                format!("Failed to send SMS via Twilio: {}", e)
            ))?;

        if !response.status().is_success() {
            let error_text = response.text().await
                .map_err(|e| crate::application::reminder_service::ReminderServiceError::NotificationError(
                    format!("Failed to read error response: {}", e)
                ))?;
            return Err(crate::application::reminder_service::ReminderServiceError::NotificationError(
                format!("Twilio API error: {}", error_text)
            ));
        }

        Ok(())
    }

    async fn send_p2p_reminder(&self, _invoice: &Invoice, _config: &PaymentReminderConfig) -> Result<(), crate::application::reminder_service::ReminderServiceError> {
        Err(crate::application::reminder_service::ReminderServiceError::NotificationError(
            "P2P reminders not supported by Twilio SMS service".to_string()
        ))
    }
}

impl TwilioSmsService {
    /// Render the reminder template with invoice data
    fn render_template(&self, config: &PaymentReminderConfig, invoice: &Invoice) -> Result<String, SmsNotificationError> {
        let mut template = config.reminder_template.clone();
        
        // Replace placeholders with actual values
        template = template.replace("{invoice_id}", &invoice.id.to_string());
        template = template.replace("{due_date}", &invoice.due_date.format("%Y-%m-%d").to_string());
        template = template.replace("{client_name}", &invoice.client_name);
        template = template.replace("{total_amount}", &format!("{:.2}", invoice.total_amount));
        
        // SMS messages should be concise
        if template.len() > 160 {
            template = template.chars().take(157).collect::<String>() + "...";
        }
        
        Ok(template)
    }
}