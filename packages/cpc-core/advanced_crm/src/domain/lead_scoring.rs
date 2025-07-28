//! Lead scoring models for the advanced CRM module
//!
//! This module contains the core business entities for lead scoring functionality.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

/// Lead score representation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LeadScore {
    pub lead_id: Uuid,
    pub base_score: u8,       // 0-100
    pub engagement_score: u8, // 0-100
    pub fit_score: u8,        // 0-100
    pub wellness_score: u8,   // 0-100 (new health integration)
    pub total_score: u8,      // 0-100
    pub scoring_factors: ScoringFactors,
    pub last_updated: DateTime<Utc>,
    pub scoring_model_id: Uuid, // References active scoring model
}

/// Factors that contribute to lead scoring
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScoringFactors {
    pub website_visits: u32,
    pub email_opens: u32,
    pub content_downloads: u32,
    pub social_engagement: u32,
    pub company_size: CompanySize,
    pub industry_fit: f32,
    pub wellness_metrics: WellnessMetrics, // New health integration
}

/// Wellness metrics from health module
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WellnessMetrics {
    pub stress_level: Option<u8>,
    pub focus_level: Option<u8>,
    pub burnout_risk: Option<f32>,
}

/// Company size categories for lead scoring
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CompanySize {
    Small,    // 1-50 employees
    Medium,   // 51-500 employees
    Large,    // 501-5000 employees
    Enterprise, // 5000+ employees
}

/// Engagement metrics for lead scoring
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EngagementMetrics {
    pub website_visits: u32,
    pub email_opens: u32,
    pub content_downloads: u32,
    pub social_engagement: u32,
}

/// Configurable scoring rules
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScoringRules {
    /// Weights for different scoring components (0.0 - 1.0)
    pub weights: ScoringWeights,
    /// Thresholds for different score categories
    pub thresholds: ScoringThresholds,
    /// Company size scoring multipliers
    pub company_size_multipliers: HashMap<CompanySize, f32>,
}

/// Weights for different scoring components
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScoringWeights {
    pub business_metrics: f32,  // 0.0 - 1.0 (60% recommended)
    pub wellness_metrics: f32,   // 0.0 - 1.0 (40% recommended)
    pub base_score: f32,        // Weight for base score component
    pub engagement_score: f32,  // Weight for engagement score component
    pub fit_score: f32,         // Weight for fit score component
}

/// Thresholds for different score categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScoringThresholds {
    pub hot_lead: u8,     // 0-100 (e.g., 80)
    pub warm_lead: u8,    // 0-100 (e.g., 60)
    pub cold_lead: u8,    // 0-100 (e.g., 40)
}

/// Scoring model definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScoringModel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub weights: ScoringWeights,
    pub thresholds: ScoringThresholds,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Error types for lead scoring operations
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ScoringError {
    #[error("Lead not found: {0}")]
    LeadNotFound(Uuid),
    #[error("Invalid scoring weight: {0}. Weights must be between 0.0 and 1.0")]
    InvalidWeight(f32),
    #[error("Invalid score threshold: {0}. Thresholds must be between 0 and 100")]
    InvalidThreshold(u8),
    #[error("Data access error: {0}")]
    DataAccessError(String),
    #[error("Health module integration error: {0}")]
    HealthIntegrationError(String),
}