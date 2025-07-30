//! Webhook event handler for email providers
//!
//! This module contains the handler for processing webhook events from email providers.

use crate::domain::email_provider::{WebhookEvent, EmailProvider, WebhookEventType};
use crate::application::email_marketing_service::EmailMarketingService;
use async_trait::async_trait;
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use warp::Filter;

/// Webhook handler for email provider events
pub struct EmailWebhookHandler {
    email_service: Arc<EmailMarketingService>,
}

impl EmailWebhookHandler {
    pub fn new(email_service: Arc<EmailMarketingService>) -> Self {
        Self { email_service }
    }

    /// Create Warp filters for handling webhook events
    pub fn webhook_routes(&self) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        // Mailchimp webhook endpoint
        let mailchimp_handler = self.clone();
        let mailchimp_webhook = warp::path!("webhook" / "mailchimp")
            .and(warp::post())
            .and(warp::body::json())
            .and_then(move |payload: MailchimpWebhookPayload| {
                let handler = mailchimp_handler.clone();
                async move {
                    match handler.handle_mailchimp_webhook(payload).await {
                        Ok(_) => Ok(warp::reply::with_status("OK", warp::http::StatusCode::OK)),
                        Err(e) => {
                            eprintln!("Error handling Mailchimp webhook: {}", e);
                            Ok(warp::reply::with_status(
                                "Internal Server Error",
                                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                            ))
                        }
                    }
                }
            });

        // SendGrid webhook endpoint
        let sendgrid_handler = self.clone();
        let sendgrid_webhook = warp::path!("webhook" / "sendgrid")
            .and(warp::post())
            .and(warp::body::json())
            .and_then(move |payload: SendGridWebhookPayload| {
                let handler = sendgrid_handler.clone();
                async move {
                    match handler.handle_sendgrid_webhook(payload).await {
                        Ok(_) => Ok(warp::reply::with_status("OK", warp::http::StatusCode::OK)),
                        Err(e) => {
                            eprintln!("Error handling SendGrid webhook: {}", e);
                            Ok(warp::reply::with_status(
                                "Internal Server Error",
                                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                            ))
                        }
                    }
                }
            });

        mailchimp_webhook.or(sendgrid_webhook)
    }

    /// Handle Mailchimp webhook events
    async fn handle_mailchimp_webhook(&self, payload: MailhookPayload) -> Result<(), WebhookHandlerError> {
        // Convert Mailchimp payload to our internal WebhookEvent
        let event_type = match payload.event_type.as_str() {
            "send" => WebhookEventType::Send,
            "open" => WebhookEventType::Open,
            "click" => WebhookEventType::Click,
            "bounce" => WebhookEventType::Bounce,
            "unsubscribe" => WebhookEventType::Unsubscribe,
            "spam" => WebhookEventType::SpamComplaint,
            _ => return Err(WebhookHandlerError::UnknownEventType(payload.event_type)),
        };

        let webhook_event = WebhookEvent {
            id: Uuid::new_v4(),
            provider: EmailProvider::Mailchimp,
            event_type,
            payload: serde_json::to_value(&payload)?,
            processed: false,
            created_at: Utc::now(),
        };

        // Process the event through the email service
        self.email_service.handle_webhook_event(webhook_event).await?;

        Ok(())
    }

    /// Handle SendGrid webhook events
    async fn handle_sendgrid_webhook(&self, payload: SendGridWebhookPayload) -> Result<(), WebhookHandlerError> {
        // Process each event in the payload
        for sg_event in payload {
            let event_type = match sg_event.event.as_str() {
                "processed" => WebhookEventType::Send,
                "open" => WebhookEventType::Open,
                "click" => WebhookEventType::Click,
                "bounce" => WebhookEventType::Bounce,
                "unsubscribe" => WebhookEventType::Unsubscribe,
                "spam_report" => WebhookEventType::SpamComplaint,
                _ => continue, // Skip unknown event types
            };

            let webhook_event = WebhookEvent {
                id: Uuid::new_v4(),
                provider: EmailProvider::SendGrid,
                event_type,
                payload: serde_json::to_value(&sg_event)?,
                processed: false,
                created_at: Utc::now(),
            };

            // Process the event through the email service
            self.email_service.handle_webhook_event(webhook_event).await?;
        }

        Ok(())
    }
}

/// Error types for webhook handling
#[derive(Debug, thiserror::Error)]
pub enum WebhookHandlerError {
    #[error("Unknown event type: {0}")]
    UnknownEventType(String),
    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Email service error: {0}")]
    EmailServiceError(String),
}

impl From<crate::application::email_marketing_service::EmailMarketingServiceError> for WebhookHandlerError {
    fn from(error: crate::application::email_marketing_service::EmailMarketingServiceError) -> Self {
        WebhookHandlerError::EmailServiceError(error.to_string())
    }
}

// Mailchimp webhook payload structures

#[derive(Debug, Deserialize, Serialize)]
pub struct MailchimpWebhookPayload {
    #[serde(rename = "type")]
    pub event_type: String,
    pub fired_at: String,
    pub data: MailchimpWebhookData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MailchimpWebhookData {
    pub id: String,
    pub email: String,
    pub campaign_id: String,
    pub subject: Option<String>,
    pub reason: Option<String>,
    pub list_id: Option<String>,
}

// SendGrid webhook payload structures

#[derive(Debug, Deserialize, Serialize)]
pub struct SendGridEvent {
    pub email: String,
    pub event: String,
    pub campaign_id: Option<String>,
    pub timestamp: i64,
    pub sg_event_id: String,
    pub sg_message_id: String,
    pub useragent: Option<String>,
    pub ip: Option<String>,
    pub reason: Option<String>,
    pub url: Option<String>,
}

pub type SendGridWebhookPayload = Vec<SendGridEvent>;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_mailchimp_webhook_payload_deserialization() {
        let json_data = json!({
            "type": "open",
            "fired_at": "2024-07-28 12:00:00",
            "data": {
                "id": "12345",
                "email": "test@example.com",
                "campaign_id": "campaign123",
                "subject": "Test Subject",
                "list_id": "list123"
            }
        });

        let payload: Result<MailchimpWebhookPayload, _> = serde_json::from_value(json_data);
        assert!(payload.is_ok());
    }

    #[test]
    fn test_sendgrid_webhook_payload_deserialization() {
        let json_data = json!([
            {
                "email": "test@example.com",
                "event": "open",
                "campaign_id": "campaign123",
                "timestamp": 1609459200,
                "sg_event_id": "event123",
                "sg_message_id": "message123",
                "useragent": "Mozilla/5.0",
                "ip": "192.168.1.1"
            }
        ]);

        let payload: Result<SendGridWebhookPayload, _> = serde_json::from_value(json_data);
        assert!(payload.is_ok());
    }
}