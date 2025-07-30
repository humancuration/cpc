//! Email campaign models for the advanced CRM module
//!
//! This module contains the core business entities for email marketing functionality.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

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

/// Target segment for email campaigns
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TargetSegment {
    pub tags: Vec<String>,
    pub min_score: Option<u8>,
    pub max_score: Option<u8>,
}

/// Metrics for email campaigns
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CampaignMetrics {
    pub sent_count: u32,
    pub opened_count: u32,
    pub clicked_count: u32,
    pub bounced_count: u32,
    pub unsubscribed_count: u32,
}