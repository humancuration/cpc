//! Email marketing service for the advanced CRM module
//!
//! This module contains the application service for email marketing functionality.

use crate::domain::email_provider::{
    EmailProviderConfig, EmailProvider, EmailCampaign, CampaignStatus, 
    ContactList, WebhookEvent, EmailProviderError, TargetSegment, CampaignMetrics
};
use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;

#[derive(Debug, thiserror::Error)]
pub enum EmailMarketingServiceError {
    #[error("Email provider error: {0}")]
    EmailProviderError(#[from] EmailProviderError),
    #[error("Repository error: {0}")]
    RepositoryError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
}

#[async_trait]
pub trait EmailProviderRepository {
    async fn save_config(&self, config: EmailProviderConfig) -> Result<(), EmailProviderError>;
    async fn get_config(&self, user_id: Uuid, provider: EmailProvider) -> Result<Option<EmailProviderConfig>, EmailProviderError>;
    async fn get_configs(&self, user_id: Uuid) -> Result<Vec<EmailProviderConfig>, EmailProviderError>;
    async fn save_campaign(&self, campaign: EmailCampaign) -> Result<(), EmailProviderError>;
    async fn get_campaign(&self, id: Uuid) -> Result<Option<EmailCampaign>, EmailProviderError>;
    async fn get_campaigns(&self, user_id: Uuid) -> Result<Vec<EmailCampaign>, EmailProviderError>;
    async fn save_webhook_event(&self, event: WebhookEvent) -> Result<(), EmailProviderError>;
    async fn get_unprocessed_webhook_events(&self) -> Result<Vec<WebhookEvent>, EmailProviderError>;
}

#[async_trait]
pub trait EmailServiceProvider {
    async fn connect(&self, config: &EmailProviderConfig) -> Result<(), EmailProviderError>;
    async fn sync_contact_lists(&self, config: &EmailProviderConfig) -> Result<Vec<ContactList>, EmailProviderError>;
    async fn create_campaign(&self, config: &EmailProviderConfig, campaign: &EmailCampaign) -> Result<String, EmailProviderError>;
    async fn get_campaign_metrics(&self, config: &EmailProviderConfig, provider_campaign_id: &str) -> Result<CampaignMetrics, EmailProviderError>;
    async fn send_test_email(&self, config: &EmailProviderConfig, email: &str, campaign: &EmailCampaign) -> Result<(), EmailProviderError>;
}

pub struct NewCampaign {
    pub name: String,
    pub subject: String,
    pub content: String,
    pub target_segment: TargetSegment,
}

pub struct EmailMarketingService {
    repository: Arc<dyn EmailProviderRepository>,
    mailchimp_provider: Arc<dyn EmailServiceProvider>,
    sendgrid_provider: Arc<dyn EmailServiceProvider>,
}

impl EmailMarketingService {
    pub fn new(
        repository: Arc<dyn EmailProviderRepository>,
        mailchimp_provider: Arc<dyn EmailServiceProvider>,
        sendgrid_provider: Arc<dyn EmailServiceProvider>,
    ) -> Self {
        Self {
            repository,
            mailchimp_provider,
            sendgrid_provider,
        }
    }

    /// Establishes connection to Mailchimp/SendGrid
    pub async fn connect_provider(&self, config: EmailProviderConfig) -> Result<(), EmailMarketingServiceError> {
        // Save the configuration first
        self.repository.save_config(config.clone()).await?;

        // Test the connection
        let provider_result = match config.provider {
            EmailProvider::Mailchimp => {
                self.mailchimp_provider.connect(&config).await
            },
            EmailProvider::SendGrid => {
                self.sendgrid_provider.connect(&config).await
            },
            EmailProvider::CustomSmtp => {
                // Custom SMTP would need its own implementation
                return Err(EmailMarketingServiceError::ValidationError(
                    "Custom SMTP provider not yet implemented".to_string()
                ));
            }
        };

        provider_result.map_err(EmailMarketingServiceError::from)
    }

    /// Synchronizes contact lists from external providers
    pub async fn sync_contact_lists(&self, user_id: Uuid) -> Result<Vec<ContactList>, EmailMarketingServiceError> {
        let configs = self.repository.get_configs(user_id).await?;

        let mut all_contact_lists = Vec::new();

        for config in configs {
            let provider_result = match config.provider {
                EmailProvider::Mailchimp => {
                    self.mailchimp_provider.sync_contact_lists(&config).await
                },
                EmailProvider::SendGrid => {
                    self.sendgrid_provider.sync_contact_lists(&config).await
                },
                EmailProvider::CustomSmtp => {
                    // Custom SMTP doesn't typically have contact lists
                    Ok(Vec::new())
                }
            };

            match provider_result {
                Ok(mut contact_lists) => {
                    all_contact_lists.append(&mut contact_lists);
                },
                Err(e) => {
                    // Log the error but continue with other providers
                    eprintln!("Error syncing contact lists from {:?}: {}", config.provider, e);
                }
            }
        }

        Ok(all_contact_lists)
    }

    /// Creates campaign on selected provider
    pub async fn create_campaign(
        &self,
        user_id: Uuid,
        new_campaign: NewCampaign,
        provider_type: EmailProvider,
    ) -> Result<EmailCampaign, EmailMarketingServiceError> {
        // Get the provider configuration
        let config = self.repository.get_config(user_id, provider_type).await?
            .ok_or_else(|| EmailMarketingServiceError::EmailProviderError(
                EmailProviderError::ProviderNotConfigured(provider_type)
            ))?;

        // Create the campaign entity
        let mut campaign = EmailCampaign::new(
            new_campaign.name,
            new_campaign.subject,
            new_campaign.content,
            new_campaign.target_segment,
        );

        // Create the campaign on the provider
        let provider_result = match provider_type {
            EmailProvider::Mailchimp => {
                self.mailchimp_provider.create_campaign(&config, &campaign).await
            },
            EmailProvider::SendGrid => {
                self.sendgrid_provider.create_campaign(&config, &campaign).await
            },
            EmailProvider::CustomSmtp => {
                return Err(EmailMarketingServiceError::ValidationError(
                    "Custom SMTP provider does not support campaign creation".to_string()
                ));
            }
        };

        let provider_campaign_id = provider_result?;

        // In a real implementation, we would store the provider_campaign_id
        // For now, we'll just save the campaign to our repository
        self.repository.save_campaign(campaign.clone()).await?;

        Ok(campaign)
    }

    /// Aggregates metrics from multiple providers
    pub async fn get_campaign_metrics(&self, campaign_id: Uuid) -> Result<CampaignMetrics, EmailMarketingServiceError> {
        let campaign = self.repository.get_campaign(campaign_id).await?
            .ok_or_else(|| EmailMarketingServiceError::EmailProviderError(
                EmailProviderError::DataAccessError("Campaign not found".to_string())
            ))?;

        // In a real implementation, we would get the provider campaign ID and fetch metrics
        // For now, we'll just return the stored metrics
        Ok(campaign.metrics)
    }

    /// Processes webhook events from email providers
    pub async fn handle_webhook_event(&self, event: WebhookEvent) -> Result<(), EmailMarketingServiceError> {
        // Save the webhook event
        self.repository.save_webhook_event(event.clone()).await?;

        // Process the event based on its type
        match event.event_type {
            crate::domain::email_provider::WebhookEventType::Send => {
                self.handle_send_event(&event).await?;
            },
            crate::domain::email_provider::WebhookEventType::Open => {
                self.handle_open_event(&event).await?;
            },
            crate::domain::email_provider::WebhookEventType::Click => {
                self.handle_click_event(&event).await?;
            },
            crate::domain::email_provider::WebhookEventType::Bounce => {
                self.handle_bounce_event(&event).await?;
            },
            crate::domain::email_provider::WebhookEventType::Unsubscribe => {
                self.handle_unsubscribe_event(&event).await?;
            },
            crate::domain::email_provider::WebhookEventType::SpamComplaint => {
                self.handle_spam_complaint_event(&event).await?;
            },
        }

        Ok(())
    }

    /// Processes all unprocessed webhook events
    pub async fn process_pending_webhook_events(&self) -> Result<(), EmailMarketingServiceError> {
        let events = self.repository.get_unprocessed_webhook_events().await?;

        for event in events {
            if let Err(e) = self.handle_webhook_event(event).await {
                eprintln!("Error processing webhook event: {}", e);
            }
        }

        Ok(())
    }

    async fn handle_send_event(&self, _event: &WebhookEvent) -> Result<(), EmailMarketingServiceError> {
        // Update campaign metrics for send event
        Ok(())
    }

    async fn handle_open_event(&self, _event: &WebhookEvent) -> Result<(), EmailMarketingServiceError> {
        // Update campaign metrics for open event
        Ok(())
    }

    async fn handle_click_event(&self, _event: &WebhookEvent) -> Result<(), EmailMarketingServiceError> {
        // Update campaign metrics for click event
        Ok(())
    }

    async fn handle_bounce_event(&self, _event: &WebhookEvent) -> Result<(), EmailMarketingServiceError> {
        // Update campaign metrics for bounce event
        Ok(())
    }

    async fn handle_unsubscribe_event(&self, _event: &WebhookEvent) -> Result<(), EmailMarketingServiceError> {
        // Update campaign metrics for unsubscribe event
        Ok(())
    }

    async fn handle_spam_complaint_event(&self, _event: &WebhookEvent) -> Result<(), EmailMarketingServiceError> {
        // Handle spam complaint event
        Ok(())
    }
}