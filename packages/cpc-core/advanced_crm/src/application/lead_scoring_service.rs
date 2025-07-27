//! Lead scoring service for the advanced CRM module
//!
//! This module contains the service implementation for lead scoring functionality.

use crate::domain::lead_scoring::{
    LeadScore, ScoringFactors, ScoringRules, EngagementMetrics,
    ScoringError, ScoringWeights, ScoringThresholds, CompanySize
};
use crate::domain::lead_scoring_service::LeadScoringService as LeadScoringServiceTrait;
use crate::domain::integration_points::SalesPerformanceData;
use uuid::Uuid;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::{info, debug, warn};

/// Service for calculating lead scores
pub struct LeadScoringService {
    /// Current scoring rules
    rules: Arc<RwLock<ScoringRules>>,
    /// Cache for lead scores
    score_cache: Arc<RwLock<HashMap<Uuid, Vec<LeadScore>>>>,
}

impl LeadScoringService {
    /// Create a new lead scoring service with default rules
    pub fn new() -> Self {
        let mut company_size_multipliers = HashMap::new();
        company_size_multipliers.insert(CompanySize::Small, 0.8);
        company_size_multipliers.insert(CompanySize::Medium, 1.0);
        company_size_multipliers.insert(CompanySize::Large, 1.2);
        company_size_multipliers.insert(CompanySize::Enterprise, 1.5);
        
        let default_rules = ScoringRules {
            weights: ScoringWeights {
                business_metrics: 0.6,
                wellness_metrics: 0.4,
                base_score: 0.4,
                engagement_score: 0.4,
                fit_score: 0.2,
            },
            thresholds: ScoringThresholds {
                hot_lead: 80,
                warm_lead: 60,
                cold_lead: 40,
            },
            company_size_multipliers,
        };
        
        Self {
            rules: Arc::new(RwLock::new(default_rules)),
            score_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Validate scoring rules
    fn validate_rules(rules: &ScoringRules) -> Result<(), ScoringError> {
        // Validate weights
        if rules.weights.business_metrics < 0.0 || rules.weights.business_metrics > 1.0 {
            return Err(ScoringError::InvalidWeight(rules.weights.business_metrics));
        }
        if rules.weights.wellness_metrics < 0.0 || rules.weights.wellness_metrics > 1.0 {
            return Err(ScoringError::InvalidWeight(rules.weights.wellness_metrics));
        }
        if rules.weights.base_score < 0.0 || rules.weights.base_score > 1.0 {
            return Err(ScoringError::InvalidWeight(rules.weights.base_score));
        }
        if rules.weights.engagement_score < 0.0 || rules.weights.engagement_score > 1.0 {
            return Err(ScoringError::InvalidWeight(rules.weights.engagement_score));
        }
        if rules.weights.fit_score < 0.0 || rules.weights.fit_score > 1.0 {
            return Err(ScoringError::InvalidWeight(rules.weights.fit_score));
        }
        
        // Validate thresholds
        if rules.thresholds.hot_lead > 100 {
            return Err(ScoringError::InvalidThreshold(rules.thresholds.hot_lead));
        }
        if rules.thresholds.warm_lead > 100 {
            return Err(ScoringError::InvalidThreshold(rules.thresholds.warm_lead));
        }
        if rules.thresholds.cold_lead > 100 {
            return Err(ScoringError::InvalidThreshold(rules.thresholds.cold_lead));
        }
        
        Ok(())
    }
    
    /// Calculate base score based on company size and industry fit
    fn calculate_base_score(&self, company_size: CompanySize, industry_fit: f32) -> u8 {
        let rules = self.rules.read().unwrap();
        let size_multiplier = rules.company_size_multipliers.get(&company_size).copied().unwrap_or(1.0);
        
        // Base score is a combination of company size and industry fit
        let base = (50.0 * size_multiplier * industry_fit) as u8;
        base.min(100) // Cap at 100
    }
    
    /// Calculate engagement score based on engagement metrics
    fn calculate_engagement_score(&self, engagement_data: &EngagementMetrics) -> u8 {
        // Normalize engagement metrics to 0-100 scale
        let website_score = (engagement_data.website_visits.min(100) as f32) * 1.0;
        let email_score = (engagement_data.email_opens.min(50) as f32) * 2.0;
        let content_score = (engagement_data.content_downloads.min(20) as f32) * 5.0;
        let social_score = (engagement_data.social_engagement.min(50) as f32) * 2.0;
        
        let total = (website_score + email_score + content_score + social_score) as u8;
        total.min(100) // Cap at 100
    }
    
    /// Calculate fit score based on content and social engagement
    fn calculate_fit_score(&self, engagement_data: &EngagementMetrics) -> u8 {
        // Fit score is based on content downloads and social engagement
        let content_fit = (engagement_data.content_downloads.min(20) as f32) * 5.0;
        let social_fit = (engagement_data.social_engagement.min(50) as f32) * 2.0;
        
        let total = (content_fit + social_fit) as u8;
        total.min(100) // Cap at 100
    }
    
    /// Get wellness score from Health module (stub implementation)
    async fn get_wellness_score(&self, user_id: Uuid) -> Result<u8, ScoringError> {
        // In a real implementation, this would integrate with the Health module
        // For now, we'll return a default value
        // This would typically call something like:
        // health_service.get_wellness_data(user_id, Utc::now().date_naive())
        //    .await
        //    .map(|data| data.wellness_score)
        //    .map_err(|e| ScoringError::HealthIntegrationError(e.to_string()))
        Ok(75) // Default wellness score
    }
}

#[async_trait::async_trait]
impl LeadScoringServiceTrait for LeadScoringService {
    async fn calculate_score(&self, lead_id: Uuid, engagement_data: EngagementMetrics) -> Result<LeadScore, ScoringError> {
        info!(lead_id = %lead_id, "Calculating lead score");
        
        // For this implementation, we'll use default values for company size and industry fit
        // In a real implementation, these would be fetched from the CRM
        let company_size = CompanySize::Medium;
        let industry_fit = 0.8;
        
        // Calculate component scores
        let base_score = self.calculate_base_score(company_size.clone(), industry_fit);
        let engagement_score = self.calculate_engagement_score(&engagement_data);
        let fit_score = self.calculate_fit_score(&engagement_data);
        
        // Get wellness score from Health module
        // In a real implementation, this would be the salesperson's user_id
        // For now, we're using the lead_id as a placeholder
        let wellness_score = self.get_wellness_score(lead_id).await?;
        
        // Apply weights
        let rules = self.rules.read().unwrap();
        let business_component = (
            (base_score as f32 * rules.weights.base_score) +
            (engagement_score as f32 * rules.weights.engagement_score) +
            (fit_score as f32 * rules.weights.fit_score)
        ) as u8;
        
        let total_score = (
            (business_component as f32 * rules.weights.business_metrics) +
            (wellness_score as f32 * rules.weights.wellness_metrics)
        ) as u8;
        
        let scoring_factors = ScoringFactors {
            website_visits: engagement_data.website_visits,
            email_opens: engagement_data.email_opens,
            content_downloads: engagement_data.content_downloads,
            social_engagement: engagement_data.social_engagement,
            company_size,
            industry_fit,
        };
        
        let score = LeadScore {
            lead_id,
            base_score,
            engagement_score,
            fit_score,
            total_score,
            scoring_factors,
            last_updated: Utc::now(),
        };
        
        // Cache the score
        {
            let mut cache = self.score_cache.write().unwrap();
            let scores = cache.entry(lead_id).or_insert_with(Vec::new);
            scores.push(score.clone());
            
            // Keep only the last 10 scores for each lead
            if scores.len() > 10 {
                scores.drain(0..scores.len()-10);
            }
        }
        
        debug!(lead_id = %lead_id, total_score = score.total_score, "Lead score calculated");
        
        Ok(score)
    }
    
    async fn get_scoring_history(&self, lead_id: Uuid) -> Result<Vec<LeadScore>, ScoringError> {
        info!(lead_id = %lead_id, "Retrieving scoring history");
        
        let cache = self.score_cache.read().unwrap();
        let history = cache.get(&lead_id).cloned().unwrap_or_else(Vec::new);
        
        Ok(history)
    }
    
    async fn update_scoring_rules(&self, new_rules: ScoringRules) -> Result<(), ScoringError> {
        info!("Updating scoring rules");
        
        // Validate the new rules
        Self::validate_rules(&new_rules)?;
        
        // Update the rules
        {
            let mut rules = self.rules.write().unwrap();
            *rules = new_rules;
        }
        
        // Clear the cache since rules have changed
        {
            let mut cache = self.score_cache.write().unwrap();
            cache.clear();
        }
        
        info!("Scoring rules updated successfully");
        
        Ok(())
    }
}

impl Default for LeadScoringService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;
    
    #[test]
    fn test_new_service() {
        let service = LeadScoringService::new();
        assert!(service.rules.read().unwrap().company_size_multipliers.len() > 0);
    }
    
    #[test]
    fn test_validate_rules_valid() {
        let mut company_size_multipliers = HashMap::new();
        company_size_multipliers.insert(CompanySize::Small, 0.8);
        
        let rules = ScoringRules {
            weights: ScoringWeights {
                business_metrics: 0.6,
                wellness_metrics: 0.4,
                base_score: 0.4,
                engagement_score: 0.4,
                fit_score: 0.2,
            },
            thresholds: ScoringThresholds {
                hot_lead: 80,
                warm_lead: 60,
                cold_lead: 40,
            },
            company_size_multipliers,
        };
        
        assert!(LeadScoringService::validate_rules(&rules).is_ok());
    }
    
    #[test]
    fn test_validate_rules_invalid_weight() {
        let mut company_size_multipliers = HashMap::new();
        company_size_multipliers.insert(CompanySize::Small, 0.8);
        
        let rules = ScoringRules {
            weights: ScoringWeights {
                business_metrics: 1.5, // Invalid - should be 0.0-1.0
                wellness_metrics: 0.4,
                base_score: 0.4,
                engagement_score: 0.4,
                fit_score: 0.2,
            },
            thresholds: ScoringThresholds {
                hot_lead: 80,
                warm_lead: 60,
                cold_lead: 40,
            },
            company_size_multipliers,
        };
        
        assert!(matches!(LeadScoringService::validate_rules(&rules), Err(ScoringError::InvalidWeight(1.5))));
    }
    
    #[test]
    fn test_validate_rules_invalid_threshold() {
        let mut company_size_multipliers = HashMap::new();
        company_size_multipliers.insert(CompanySize::Small, 0.8);
        
        let rules = ScoringRules {
            weights: ScoringWeights {
                business_metrics: 0.6,
                wellness_metrics: 0.4,
                base_score: 0.4,
                engagement_score: 0.4,
                fit_score: 0.2,
            },
            thresholds: ScoringThresholds {
                hot_lead: 150, // Invalid - should be 0-100
                warm_lead: 60,
                cold_lead: 40,
            },
            company_size_multipliers,
        };
        
        assert!(matches!(LeadScoringService::validate_rules(&rules), Err(ScoringError::InvalidThreshold(150))));
    }
    
    #[test]
    fn test_calculate_base_score() {
        let service = LeadScoringService::new();
        let score = service.calculate_base_score(CompanySize::Medium, 0.8);
        assert_eq!(score, 40); // 50 * 1.0 * 0.8 = 40
    }
    
    #[test]
    fn test_calculate_engagement_score() {
        let service = LeadScoringService::new();
        let engagement_data = EngagementMetrics {
            website_visits: 50,
            email_opens: 25,
            content_downloads: 10,
            social_engagement: 20,
        };
        let score = service.calculate_engagement_score(&engagement_data);
        assert_eq!(score, 100); // (50*1) + (25*2) + (10*5) + (20*2) = 190, capped at 100
    }
    
    #[test]
    fn test_calculate_fit_score() {
        let service = LeadScoringService::new();
        let engagement_data = EngagementMetrics {
            website_visits: 10,
            email_opens: 5,
            content_downloads: 5,
            social_engagement: 10,
        };
        let score = service.calculate_fit_score(&engagement_data);
        assert_eq!(score, 45); // (5*5) + (10*2) = 45
    }
    
    #[tokio::test]
    async fn test_calculate_score() {
        let service = LeadScoringService::new();
        let lead_id = Uuid::new_v4();
        let engagement_data = EngagementMetrics {
            website_visits: 20,
            email_opens: 10,
            content_downloads: 3,
            social_engagement: 5,
        };
        
        let result = service.calculate_score(lead_id, engagement_data).await;
        assert!(result.is_ok());
        
        let score = result.unwrap();
        assert_eq!(score.lead_id, lead_id);
        assert!(score.total_score <= 100);
    }
    
    #[tokio::test]
    async fn test_get_scoring_history() {
        let service = LeadScoringService::new();
        let lead_id = Uuid::new_v4();
        let engagement_data = EngagementMetrics {
            website_visits: 10,
            email_opens: 5,
            content_downloads: 2,
            social_engagement: 3,
        };
        
        // Calculate a score to add to history
        let _ = service.calculate_score(lead_id, engagement_data).await;
        
        // Get the history
        let history = service.get_scoring_history(lead_id).await.unwrap();
        assert_eq!(history.len(), 1);
    }
    
    #[tokio::test]
    async fn test_update_scoring_rules() {
        let service = LeadScoringService::new();
        let mut company_size_multipliers = HashMap::new();
        company_size_multipliers.insert(CompanySize::Small, 1.0);
        
        let new_rules = ScoringRules {
            weights: ScoringWeights {
                business_metrics: 0.7,
                wellness_metrics: 0.3,
                base_score: 0.5,
                engagement_score: 0.3,
                fit_score: 0.2,
            },
            thresholds: ScoringThresholds {
                hot_lead: 85,
                warm_lead: 65,
                cold_lead: 35,
            },
            company_size_multipliers,
        };
        
        let result = service.update_scoring_rules(new_rules).await;
        assert!(result.is_ok());
        
        // Check that the rules were updated
        let rules = service.rules.read().unwrap();
        assert_eq!(rules.weights.business_metrics, 0.7);
    }
}