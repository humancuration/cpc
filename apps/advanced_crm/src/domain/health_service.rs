//! Health service integration for the advanced CRM module
//!
//! This module contains the service trait that defines the interface for integrating
//! with the Health module to retrieve wellness data for lead scoring.

use crate::domain::integration_points::WellnessData;
use crate::domain::lead_scoring::ScoringError;
use uuid::Uuid;
use chrono::NaiveDate;

/// Service trait for integrating with the Health module
#[async_trait::async_trait]
pub trait HealthService {
    /// Get wellness data for a user on a specific date
    async fn get_wellness_data(&self, user_id: Uuid, date: NaiveDate) -> Result<WellnessData, ScoringError>;
    
    /// Get average wellness score for a user over a date range
    async fn get_average_wellness_score(&self, user_id: Uuid, start_date: NaiveDate, end_date: NaiveDate) -> Result<u8, ScoringError>;
}