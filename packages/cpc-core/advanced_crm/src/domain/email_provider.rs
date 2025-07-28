//! Email provider models for the advanced CRM module
//!
//! This module contains the core business entities for email marketing integration.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Email provider configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmailProviderConfig {
    pub id: Uuid,
    pub provider: EmailProvider,
    pub api_key: String, // This should be encrypted in storage
    pub user_id: Uuid,
    pub sync_contacts: bool,
    pub sync_campaigns: bool,
    pub last_sync: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Supported email providers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EmailProvider {
    Mailchimp,
    SendGrid,
    CustomSmtp,
}

/// Email campaign representation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmailCampaign {
    pub id: Uuid,
    pub name: String,
    pub subject: String,
    pub content: String,
    pub status: CampaignStatus,
    pub scheduled_time: Option<DateTime<Utc>>,
    pub target_segment: TargetSegment,
    pub metrics: CampaignMetrics,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Status of an email campaign
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CampaignStatus {
    Draft,
    Scheduled,
    Sending,
    Completed,
    Cancelled,
}

/// Target segment for a campaign
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TargetSegment {
    pub name: String,
    pub criteria: serde_json::Value, // Flexible criteria structure
}

/// Campaign metrics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CampaignMetrics {
    pub sent_count: u32,
    pub open_count: u32,
    pub click_count: u32,
    pub bounce_count: u32,
    pub unsubscribe_count: u32,
    pub open_rate: f32,
    pub click_rate: f32,
}

/// Contact list from an email provider
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContactList {
    pub id: String, // Provider-specific ID
    pub name: String,
    pub contact_count: u32,
    pub last_updated: DateTime<Utc>,
}

/// Webhook event from an email provider
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WebhookEvent {
    pub id: Uuid,
    pub provider: EmailProvider,
    pub event_type: WebhookEventType,
    pub payload: serde_json::Value,
    pub processed: bool,
    pub created_at: DateTime<Utc>,
}

/// Types of webhook events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WebhookEventType {
    Send,
    Open,
    Click,
    Bounce,
    Unsubscribe,
    SpamComplaint,
}

/// Error types for email provider operations
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum EmailProviderError {
    #[error("Provider not configured: {0}")]
    ProviderNotConfigured(EmailProvider),
    #[error("Invalid API key")]
    InvalidApiKey,
    #[error("API error: {0}")]
    ApiError(String),
    #[error("Data access error: {0}")]
    DataAccessError(String),
    #[error("Webhook processing error: {0}")]
    WebhookError(String),
}

impl EmailProviderConfig {
    /// Create a new email provider configuration
    pub fn new(
        provider: EmailProvider,
        api_key: String,
        user_id: Uuid,
        sync_contacts: bool,
        sync_campaigns: bool,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            provider,
            api_key,
            user_id,
            sync_contacts,
            sync_campaigns,
            last_sync: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Update the configuration
    pub fn update(&mut self, sync_contacts: bool, sync_campaigns: bool) {
        self.sync_contacts = sync_contacts;
        self.sync_campaigns = sync_campaigns;
        self.updated_at = Utc::now();
    }
}

impl EmailCampaign {
    /// Create a new email campaign
    pub fn new(
        name: String,
        subject: String,
        content: String,
        target_segment: TargetSegment,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            subject,
            content,
            status: CampaignStatus::Draft,
            scheduled_time: None,
            target_segment,
            metrics: CampaignMetrics::default(),
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Schedule the campaign
    pub fn schedule(&mut self, scheduled_time: DateTime<Utc>) {
        self.scheduled_time = Some(scheduled_time);
        self.status = CampaignStatus::Scheduled;
        self.updated_at = Utc::now();
    }
    
    /// Update campaign metrics
    pub fn update_metrics(&mut self, metrics: CampaignMetrics) {
        self.metrics = metrics;
        self.updated_at = Utc::now();
    }
}

impl Default for CampaignMetrics {
    fn default() -> Self {
        Self {
            sent_count: 0,
            open_count: 0,
            click_count: 0,
            bounce_count: 0,
            unsubscribe_count: 0,
            open_rate: 0.0,
            click_rate: 0.0,
        }
    }
}

impl CampaignMetrics {
    /// Calculate rates based on sent count
    pub fn calculate_rates(&mut self) {
        if self.sent_count > 0 {
            self.open_rate = self.open_count as f32 / self.sent_count as f32;
            self.click_rate = self.click_count as f32 / self.sent_count as f32;
        } else {
            self.open_rate = 0.0;
            self.click_rate = 0.0;
        }
    }
}