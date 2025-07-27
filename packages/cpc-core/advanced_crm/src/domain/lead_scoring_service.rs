//! Lead scoring service trait for the advanced CRM module
//!
//! This module contains the service trait that defines the interface for lead scoring functionality.

use crate::domain::lead_scoring::{LeadScore, ScoringRules, EngagementMetrics, ScoringError};
use uuid::Uuid;

/// Service trait for calculating lead scores
#[async_trait::async_trait]
pub trait LeadScoringService {
    /// Calculate lead score using engagement data and other factors
    async fn calculate_score(&self, lead_id: Uuid, engagement_data: EngagementMetrics) -> Result<LeadScore, ScoringError>;
    
    /// Get scoring history for a lead
    async fn get_scoring_history(&self, lead_id: Uuid) -> Result<Vec<LeadScore>, ScoringError>;
    
    /// Update scoring rules
    async fn update_scoring_rules(&self, new_rules: ScoringRules) -> Result<(), ScoringError>;
}