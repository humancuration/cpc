//! Mailchimp email provider implementation
//!
//! This module contains the concrete implementation for integrating with Mailchimp.

use crate::domain::email_provider::{
    EmailProviderConfig, EmailProvider, ContactList, EmailProviderError,
    EmailCampaign, CampaignMetrics
};
use crate::application::email_marketing_service::EmailServiceProvider;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Mailchimp API client
pub struct MailchimpProvider {
    client: Client,
    base_url: String,
}

impl MailchimpProvider {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://us1.api.mailchimp.com/3.0".to_string(),
        }
    }

    fn get_auth_header(&self, api_key: &str) -> String {
        format!("Bearer {}", api_key)
    }

    fn get_base_url_for_key(&self, api_key: &str) -> String {
        // Extract data center from API key (format: xxxxxxxxxxxxxxxx-us1)
        if let Some(dc) = api_key.split('-').last() {
            format!("https://{}.api.mailchimp.com/3.0", dc)
        } else {
            self.base_url.clone()
        }
    }
}

#[async_trait]
impl EmailServiceProvider for MailchimpProvider {
    async fn connect(&self, config: &EmailProviderConfig) -> Result<(), EmailProviderError> {
        if config.provider != EmailProvider::Mailchimp {
            return Err(EmailProviderError::ProviderNotConfigured(EmailProvider::Mailchimp));
        }

        let url = format!("{}/ping", self.get_base_url_for_key(&config.api_key));
        
        let response = self.client
            .get(&url)
            .header("Authorization", self.get_auth_header(&config.api_key))
            .send()
            .await
            .map_err(|e| EmailProviderError::ApiError(format!("Failed to connect to Mailchimp: {}", e)))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(EmailProviderError::InvalidApiKey)
        }
    }

    async fn sync_contact_lists(&self, config: &EmailProviderConfig) -> Result<Vec<ContactList>, EmailProviderError> {
        if config.provider != EmailProvider::Mailchimp {
            return Err(EmailProviderError::ProviderNotConfigured(EmailProvider::Mailchimp));
        }

        let url = format!("{}/lists", self.get_base_url_for_key(&config.api_key));
        
        let response = self.client
            .get(&url)
            .header("Authorization", self.get_auth_header(&config.api_key))
            .send()
            .await
            .map_err(|e| EmailProviderError::ApiError(format!("Failed to fetch lists from Mailchimp: {}", e)))?;

        if !response.status().is_success() {
            return Err(EmailProviderError::ApiError(
                format!("Mailchimp API error: {}", response.status())
            ));
        }

        let lists_response: MailchimpListsResponse = response
            .json()
            .await
            .map_err(|e| EmailProviderError::ApiError(format!("Failed to parse Mailchimp response: {}", e)))?;

        let contact_lists = lists_response.lists.into_iter().map(|list| {
            ContactList {
                id: list.id,
                name: list.name,
                contact_count: list.stats.member_count,
                last_updated: Utc::now(), // Mailchimp doesn't provide this directly
            }
        }).collect();

        Ok(contact_lists)
    }

    async fn create_campaign(&self, config: &EmailProviderConfig, campaign: &EmailCampaign) -> Result<String, EmailProviderError> {
        if config.provider != EmailProvider::Mailchimp {
            return Err(EmailProviderError::ProviderNotConfigured(EmailProvider::Mailchimp));
        }

        // First, create the campaign
        let campaign_data = MailchimpCampaignCreate {
            campaign_type: "regular".to_string(),
            recipients: MailchimpRecipients {
                list_id: "".to_string(), // This would need to be determined from the target segment
            },
            settings: MailchimpCampaignSettings {
                subject_line: campaign.subject.clone(),
                title: campaign.name.clone(),
                from_name: "Your Company".to_string(), // This should be configurable
                reply_to: "noreply@yourcompany.com".to_string(), // This should be configurable
            },
        };

        let url = format!("{}/campaigns", self.get_base_url_for_key(&config.api_key));
        
        let response = self.client
            .post(&url)
            .header("Authorization", self.get_auth_header(&config.api_key))
            .json(&campaign_data)
            .send()
            .await
            .map_err(|e| EmailProviderError::ApiError(format!("Failed to create campaign in Mailchimp: {}", e)))?;

        if !response.status().is_success() {
            return Err(EmailProviderError::ApiError(
                format!("Mailchimp API error: {}", response.status())
            ));
        }

        let campaign_response: MailchimpCampaignResponse = response
            .json()
            .await
            .map_err(|e| EmailProviderError::ApiError(format!("Failed to parse Mailchimp campaign response: {}", e)))?;

        // Then set the content
        let content_data = MailchimpCampaignContent {
            html: campaign.content.clone(),
        };

        let content_url = format!("{}/campaigns/{}/content", self.get_base_url_for_key(&config.api_key), campaign_response.id);
        
        let content_response = self.client
            .put(&content_url)
            .header("Authorization", self.get_auth_header(&config.api_key))
            .json(&content_data)
            .send()
            .await
            .map_err(|e| EmailProviderError::ApiError(format!("Failed to set campaign content in Mailchimp: {}", e)))?;

        if !content_response.status().is_success() {
            return Err(EmailProviderError::ApiError(
                format!("Mailchimp API error setting content: {}", content_response.status())
            ));
        }

        Ok(campaign_response.id)
    }

    async fn get_campaign_metrics(&self, config: &EmailProviderConfig, provider_campaign_id: &str) -> Result<CampaignMetrics, EmailProviderError> {
        if config.provider != EmailProvider::Mailchimp {
            return Err(EmailProviderError::ProviderNotConfigured(EmailProvider::Mailchimp));
        }

        let url = format!("{}/reports/{}", self.get_base_url_for_key(&config.api_key), provider_campaign_id);
        
        let response = self.client
            .get(&url)
            .header("Authorization", self.get_auth_header(&config.api_key))
            .send()
            .await
            .map_err(|e| EmailProviderError::ApiError(format!("Failed to fetch campaign report from Mailchimp: {}", e)))?;

        if !response.status().is_success() {
            return Err(EmailProviderError::ApiError(
                format!("Mailchimp API error: {}", response.status())
            ));
        }

        let report_response: MailchimpReportResponse = response
            .json()
            .await
            .map_err(|e| EmailProviderError::ApiError(format!("Failed to parse Mailchimp report response: {}", e)))?;

        let mut metrics = CampaignMetrics {
            sent_count: report_response.sent_to.total as u32,
            open_count: report_response.opens.unique_opens as u32,
            click_count: report_response.clicks.unique_clicks as u32,
            bounce_count: report_response.bounces.hard_bounces as u32 + report_response.bounces.soft_bounces as u32,
            unsubscribe_count: report_response.unsubscribed as u32,
            open_rate: report_response.opens.open_rate as f32,
            click_rate: report_response.clicks.click_rate as f32,
        };

        metrics.calculate_rates();

        Ok(metrics)
    }

    async fn send_test_email(&self, config: &EmailProviderConfig, email: &str, campaign: &EmailCampaign) -> Result<(), EmailProviderError> {
        if config.provider != EmailProvider::Mailchimp {
            return Err(EmailProviderError::ProviderNotConfigured(EmailProvider::Mailchimp));
        }

        // This would require the campaign to already exist in Mailchimp
        // For now, we'll return an error indicating this functionality needs the campaign to be created first
        Err(EmailProviderError::ApiError(
            "Test email sending requires campaign to be created in Mailchimp first".to_string()
        ))
    }
}

// Mailchimp API response structures

#[derive(Debug, Deserialize)]
struct MailchimpListsResponse {
    lists: Vec<MailchimpList>,
}

#[derive(Debug, Deserialize)]
struct MailchimpList {
    id: String,
    name: String,
    stats: MailchimpListStats,
}

#[derive(Debug, Deserialize)]
struct MailchimpListStats {
    member_count: u32,
}

#[derive(Debug, Serialize)]
struct MailchimpCampaignCreate {
    #[serde(rename = "type")]
    campaign_type: String,
    recipients: MailchimpRecipients,
    settings: MailchimpCampaignSettings,
}

#[derive(Debug, Serialize)]
struct MailchimpRecipients {
    #[serde(rename = "list_id")]
    list_id: String,
}

#[derive(Debug, Serialize)]
struct MailchimpCampaignSettings {
    #[serde(rename = "subject_line")]
    subject_line: String,
    title: String,
    #[serde(rename = "from_name")]
    from_name: String,
    #[serde(rename = "reply_to")]
    reply_to: String,
}

#[derive(Debug, Deserialize)]
struct MailchimpCampaignResponse {
    id: String,
}

#[derive(Debug, Serialize)]
struct MailchimpCampaignContent {
    html: String,
}

#[derive(Debug, Deserialize)]
struct MailchimpReportResponse {
    #[serde(rename = "sent_to")]
    sent_to: MailchimpSentTo,
    opens: MailchimpOpens,
    clicks: MailchimpClicks,
    bounces: MailchimpBounces,
    #[serde(rename = "unsubscribed")]
    unsubscribed: u32,
}

#[derive(Debug, Deserialize)]
struct MailchimpSentTo {
    total: u32,
}

#[derive(Debug, Deserialize)]
struct MailchimpOpens {
    #[serde(rename = "unique_opens")]
    unique_opens: u32,
    #[serde(rename = "open_rate")]
    open_rate: f64,
}

#[derive(Debug, Deserialize)]
struct MailchimpClicks {
    #[serde(rename = "unique_clicks")]
    unique_clicks: u32,
    #[serde(rename = "click_rate")]
    click_rate: f64,
}

#[derive(Debug, Deserialize)]
struct MailchimpBounces {
    #[serde(rename = "hard_bounces")]
    hard_bounces: u32,
    #[serde(rename = "soft_bounces")]
    soft_bounces: u32,
}