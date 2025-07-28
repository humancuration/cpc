//! Lead scoring service for the advanced CRM module
//!
//! This module contains the application service for calculating lead scores.

use crate::domain::lead_scoring::{LeadScore, ScoringFactors, WellnessMetrics, ScoringModel, ScoringError};
use crate::domain::health_service::HealthService;
use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tracing::info;

#[derive(Debug, thiserror::Error)]
pub enum LeadScoringServiceError {
    #[error("Scoring error: {0}")]
    ScoringError(#[from] ScoringError),
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

#[async_trait]
pub trait LeadScoringRepository {
    async fn get_lead_score(&self, lead_id: Uuid) -> Result<Option<LeadScore>, ScoringError>;
    async fn save_lead_score(&self, score: LeadScore) -> Result<(), ScoringError>;
    async fn get_scoring_history(&self, lead_id: Uuid) -> Result<Vec<LeadScore>, ScoringError>;
    async fn get_active_scoring_model(&self) -> Result<ScoringModel, ScoringError>;
    async fn update_scoring_model(&self, model: ScoringModel) -> Result<(), ScoringError>;
}

pub struct ExternalLeadData {
    pub website_visits: u32,
    pub email_opens: u32,
    pub content_downloads: u32,
    pub social_engagement: u32,
    pub company_size: String,
    pub industry: String,
}

pub struct LeadFilter {
    pub min_score: Option<u8>,
    pub max_score: Option<u8>,
    pub company_size: Option<String>,
    pub industry: Option<String>,
}

pub struct LeadScoringService {
    repository: Arc<dyn LeadScoringRepository>,
    health_service: Arc<dyn HealthService>,
}

impl LeadScoringService {
    pub fn new(
        repository: Arc<dyn LeadScoringRepository>,
        health_service: Arc<dyn HealthService>,
    ) -> Self {
        Self {
            repository,
            health_service,
        }
    }

    /// Calculates lead score using multiple factors including external data sources and wellness data
    pub async fn calculate_lead_score(
        &self,
        lead_id: Uuid,
        external_data: Option<ExternalLeadData>,
    ) -> Result<LeadScore, LeadScoringServiceError> {
        let scoring_model = self.repository.get_active_scoring_model().await?;
        
        // Get existing lead data or use default values
        let (base_score, engagement_score, fit_score) = if let Some(data) = external_data {
            (
                50, // Default base score
                self.calculate_engagement_score(&data),
                self.calculate_fit_score(&data, &scoring_model),
            )
        } else {
            (50, 50, 50) // Default values if no external data
        };

        // Get wellness data for the lead (if available)
        let wellness_metrics = self.get_wellness_metrics(lead_id).await?;
        let wellness_score = self.calculate_wellness_score(&wellness_metrics, &scoring_model);

        // Calculate total score using the model weights
        let total_score = self.calculate_total_score(
            base_score,
            engagement_score,
            fit_score,
            wellness_score,
            &scoring_model.weights,
        );

        let scoring_factors = ScoringFactors {
            website_visits: external_data.as_ref().map(|d| d.website_visits).unwrap_or(0),
            email_opens: external_data.as_ref().map(|d| d.email_opens).unwrap_or(0),
            content_downloads: external_data.as_ref().map(|d| d.content_downloads).unwrap_or(0),
            social_engagement: external_data.as_ref().map(|d| d.social_engagement).unwrap_or(0),
            company_size: self.parse_company_size(external_data.as_ref().and_then(|d| d.company_size.parse().ok()).unwrap_or_default()),
            industry_fit: external_data.as_ref().map(|d| self.calculate_industry_fit(&d.industry)).unwrap_or(0.5),
            wellness_metrics,
        };

        let lead_score = LeadScore {
            lead_id,
            base_score,
            engagement_score,
            fit_score,
            wellness_score,
            total_score,
            scoring_factors,
            last_updated: Utc::now(),
            scoring_model_id: scoring_model.id,
        };

        // Save the calculated score
        self.repository.save_lead_score(lead_score.clone()).await?;

        Ok(lead_score)
    }

    /// Gets highest scoring leads with filtering capability
    pub async fn get_top_leads(
        &self,
        limit: u32,
        filter: LeadFilter,
    ) -> Result<Vec<(Uuid, LeadScore)>, LeadScoringServiceError> {
        // This would typically query the repository with filters
        // For now, we'll return an empty vector as a placeholder
        Ok(Vec::new())
    }

    /// Updates scoring algorithm with validation
    pub async fn update_scoring_model(
        &self,
        model: ScoringModel,
    ) -> Result<(), LeadScoringServiceError> {
        // Validate the model weights
        if model.weights.business_metrics < 0.0 || model.weights.business_metrics > 1.0 {
            return Err(LeadScoringServiceError::ScoringError(
                ScoringError::InvalidWeight(model.weights.business_metrics)
            ));
        }
        
        if model.weights.wellness_metrics < 0.0 || model.weights.wellness_metrics > 1.0 {
            return Err(LeadScoringServiceError::ScoringError(
                ScoringError::InvalidWeight(model.weights.wellness_metrics)
            ));
        }

        self.repository.update_scoring_model(model).await?;
        Ok(())
    }

    /// Retrieves historical scoring data for trend analysis
    pub async fn get_scoring_history(
        &self,
        lead_id: Uuid,
    ) -> Result<Vec<LeadScore>, LeadScoringServiceError> {
        let history = self.repository.get_scoring_history(lead_id).await?;
        Ok(history)
    }

    fn calculate_engagement_score(&self, data: &ExternalLeadData) -> u8 {
        // Simple calculation based on engagement metrics
        let score = (data.website_visits + data.email_opens + data.content_downloads + data.social_engagement) as f32;
        (score.min(100.0)) as u8
    }

    fn calculate_fit_score(&self, data: &ExternalLeadData, model: &ScoringModel) -> u8 {
        let company_size_score = match data.company_size.as_str() {
            "Enterprise" => 100,
            "Large" => 80,
            "Medium" => 60,
            "Small" => 40,
            _ => 50,
        };
        
        let industry_fit = self.calculate_industry_fit(&data.industry);
        
        // Weighted average
        let fit_score = (company_size_score as f32 * 0.6 + industry_fit * 100.0 * 0.4) as u8;
        fit_score.min(100)
    }

    fn calculate_industry_fit(&self, industry: &str) -> f32 {
        // Simple matching algorithm - in a real implementation this would be more complex
        match industry.to_lowercase().as_str() {
            "technology" | "software" | "it" => 1.0,
            "finance" | "banking" => 0.9,
            "healthcare" => 0.8,
            "education" => 0.7,
            _ => 0.5,
        }
    }

    async fn get_wellness_metrics(&self, user_id: Uuid) -> Result<WellnessMetrics, LeadScoringServiceError> {
        // Try to get wellness data from the health service
        match self.health_service.get_average_wellness_score(user_id, chrono::Utc::now().date_naive() - chrono::Duration::days(30), chrono::Utc::now().date_naive()).await {
            Ok(wellness_score) => {
                // For now, we'll create dummy values based on the wellness score
                Ok(WellnessMetrics {
                    stress_level: Some(100 - wellness_score),
                    focus_level: Some(wellness_score),
                    burnout_risk: Some((100 - wellness_score) as f32 / 100.0),
                })
            },
            Err(_) => {
                // Return default values if wellness data is not available
                Ok(WellnessMetrics {
                    stress_level: None,
                    focus_level: None,
                    burnout_risk: None,
                })
            }
        }
    }

    fn calculate_wellness_score(&self, metrics: &WellnessMetrics, model: &ScoringModel) -> u8 {
        if let (Some(stress), Some(focus), Some(burnout)) = (metrics.stress_level, metrics.focus_level, metrics.burnout_risk) {
            // Wellness score calculation based on metrics
            // Lower stress and burnout risk, higher focus = higher wellness score
            let wellness_score = (focus as f32 * 0.4 + (100 - stress) as f32 * 0.3 + (1.0 - burnout) * 100.0 * 0.3) as u8;
            wellness_score.min(100)
        } else {
            // Default score if wellness data is not available
            50
        }
    }

    fn calculate_total_score(
        &self,
        base_score: u8,
        engagement_score: u8,
        fit_score: u8,
        wellness_score: u8,
        weights: &crate::domain::lead_scoring::ScoringWeights,
    ) -> u8 {
        let business_component = (
            base_score as f32 * weights.base_score +
            engagement_score as f32 * weights.engagement_score +
            fit_score as f32 * weights.fit_score
        ) * weights.business_metrics;

        let wellness_component = wellness_score as f32 * weights.wellness_metrics;

        let total = business_component + wellness_component;
        total.min(100.0) as u8
    }

    fn parse_company_size(&self, size_str: String) -> crate::domain::lead_scoring::CompanySize {
        match size_str.to_lowercase().as_str() {
            "small" => crate::domain::lead_scoring::CompanySize::Small,
            "medium" => crate::domain::lead_scoring::CompanySize::Medium,
            "large" => crate::domain::lead_scoring::CompanySize::Large,
            "enterprise" => crate::domain::lead_scoring::CompanySize::Enterprise,
            _ => crate::domain::lead_scoring::CompanySize::Small,
        }
    }
}