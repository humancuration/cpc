//! Integration with Broader Impact Ecosystem
//!
//! This module provides integration points with other impact tracking systems
//! to create a comprehensive view of community impact.

use serde::{Deserialize, Serialize};
use tracing::{debug, info};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

use crate::tracker::{ImpactMetrics, VisualizationEngagement, EngagementCorrelation, 
                     ContributionEffectiveness, CommunityValidation, VisualizationFeedback};
use learning_impact_tracker::tracker::{ImpactMetrics as LearningMetrics, VisualizationEngagement as LearningEngagement};
use volunteer_impact_tracker::tracker::{ImpactMetrics as VolunteerMetrics, VisualizationEngagement as VolunteerEngagement};
use financial_impact_tracker::tracker::{ImpactMetrics as FinancialMetrics, VisualizationEngagement as FinancialEngagement};
use cause_management::ml::{CauseData, ImpactMeasurement, EngagementMetric, OutcomeMeasurement};

/// Ecosystem integrator for cross-platform impact analysis
pub struct EcosystemIntegrator {
    /// Learning impact metrics
    learning_metrics: Option<LearningMetrics>,
    
    /// Volunteer impact metrics
    volunteer_metrics: Option<VolunteerMetrics>,
    
    /// Financial impact metrics
    financial_metrics: Option<FinancialMetrics>,
}

/// Integrated impact analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegratedImpactAnalysis {
    /// Overall community engagement score
    pub community_engagement_score: f64,
    
    /// Cross-platform correlation data
    pub cross_platform_correlations: HashMap<String, f64>,
    
    /// Holistic impact metrics
    pub holistic_metrics: HolisticMetrics,
    
    /// Integrated recommendations
    pub integrated_recommendations: Vec<IntegratedRecommendation>,
    
    /// Timestamp of analysis
    pub analyzed_at: DateTime<Utc>,
}

/// Holistic metrics combining all impact areas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolisticMetrics {
    /// Overall engagement across all platforms
    pub overall_engagement: f64,
    
    /// Cross-platform participation rate
    pub cross_platform_participation: f64,
    
    /// Community transformation index
    pub community_transformation_index: f64,
    
    /// Resource allocation effectiveness across platforms
    pub resource_allocation_effectiveness: f64,
    
    /// Community validation integration score
    pub community_validation_integration: f64,
}

/// Integrated recommendation combining insights from all platforms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegratedRecommendation {
    /// Recommendation identifier
    pub recommendation_id: String,
    
    /// Cross-platform recommendation description
    pub description: String,
    
    /// Platforms affected
    pub affected_platforms: Vec<AffectedPlatform>,
    
    /// Priority level
    pub priority: IntegrationPriority,
    
    /// Suggested coordinated action
    pub suggested_action: CoordinatedAction,
    
    /// Expected impact
    pub expected_impact: f64,
    
    /// Implementation complexity
    pub complexity: ImplementationComplexity,
}

/// Platforms that can be affected by recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AffectedPlatform {
    Learning,
    Volunteering,
    Financial,
    Cause,
    CrossPlatform,
}

/// Integration priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrationPriority {
    High,
    Medium,
    Low,
}

/// Coordinated actions across platforms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoordinatedAction {
    AlignVisualizationStrategies,
    ShareCommunityInsights,
    CoordinateResourceAllocation,
    HarmonizeFeedbackSystems,
    SynchronizeEngagementCampaigns,
    IntegrateDataCollection,
    UnifyUserExperience,
}

/// Implementation complexity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImplementationComplexity {
    Low,
    Medium,
    High,
}

impl EcosystemIntegrator {
    /// Create a new ecosystem integrator
    pub fn new() -> Self {
        info!("Initializing EcosystemIntegrator");
        Self {
            learning_metrics: None,
            volunteer_metrics: None,
            financial_metrics: None,
        }
    }
    
    /// Set learning impact metrics
    pub fn set_learning_metrics(&mut self, metrics: LearningMetrics) {
        debug!("Setting learning impact metrics");
        self.learning_metrics = Some(metrics);
    }
    
    /// Set volunteer impact metrics
    pub fn set_volunteer_metrics(&mut self, metrics: VolunteerMetrics) {
        debug!("Setting volunteer impact metrics");
        self.volunteer_metrics = Some(metrics);
    }
    
    /// Set financial impact metrics
    pub fn set_financial_metrics(&mut self, metrics: FinancialMetrics) {
        debug!("Setting financial impact metrics");
        self.financial_metrics = Some(metrics);
    }
    
    /// Perform integrated impact analysis
    pub fn analyze_integrated_impact(&self, cause_metrics: &ImpactMetrics) -> IntegratedImpactAnalysis {
        debug!("Performing integrated impact analysis");
        
        let cross_platform_correlations = self.calculate_cross_platform_correlations(cause_metrics);
        let holistic_metrics = self.calculate_holistic_metrics(cause_metrics);
        let integrated_recommendations = self.generate_integrated_recommendations(cause_metrics);
        
        IntegratedImpactAnalysis {
            community_engagement_score: self.calculate_community_engagement_score(cause_metrics),
            cross_platform_correlations,
            holistic_metrics,
            integrated_recommendations,
            analyzed_at: Utc::now(),
        }
    }
    
    /// Calculate community engagement score across all platforms
    fn calculate_community_engagement_score(&self, cause_metrics: &ImpactMetrics) -> f64 {
        debug!("Calculating community engagement score");
        
        let mut total_score = 0.0;
        let mut score_count = 0.0;
        
        // Add cause engagement score
        let cause_engagement = cause_metrics.visualization_engagement.values()
            .map(|e| e.quality_score)
            .sum::<f64>() / cause_metrics.visualization_engagement.len().max(1) as f64;
        total_score += cause_engagement;
        score_count += 1.0;
        
        // Add learning engagement score if available
        if let Some(learning_metrics) = &self.learning_metrics {
            let learning_engagement = learning_metrics.visualization_engagement.values()
                .map(|e| e.quality_score)
                .sum::<f64>() / learning_metrics.visualization_engagement.len().max(1) as f64;
            total_score += learning_engagement;
            score_count += 1.0;
        }
        
        // Add volunteer engagement score if available
        if let Some(volunteer_metrics) = &self.volunteer_metrics {
            let volunteer_engagement = volunteer_metrics.visualization_engagement.values()
                .map(|e| e.quality_score)
                .sum::<f64>() / volunteer_metrics.visualization_engagement.len().max(1) as f64;
            total_score += volunteer_engagement;
            score_count += 1.0;
        }
        
        // Add financial engagement score if available
        if let Some(financial_metrics) = &self.financial_metrics {
            let financial_engagement = financial_metrics.visualization_engagement.values()
                .map(|e| e.quality_score)
                .sum::<f64>() / financial_metrics.visualization_engagement.len().max(1) as f64;
            total_score += financial_engagement;
            score_count += 1.0;
        }
        
        if score_count > 0.0 {
            total_score / score_count
        } else {
            0.0
        }
    }
    
    /// Calculate cross-platform correlations
    fn calculate_cross_platform_correlations(&self, cause_metrics: &ImpactMetrics) -> HashMap<String, f64> {
        debug!("Calculating cross-platform correlations");
        
        let mut correlations = HashMap::new();
        
        // Calculate correlations between cause engagement and other platforms
        if let Some(learning_metrics) = &self.learning_metrics {
            let correlation = self.calculate_engagement_correlation(
                &cause_metrics.visualization_engagement,
                &learning_metrics.visualization_engagement
            );
            correlations.insert("cause_learning".to_string(), correlation);
        }
        
        if let Some(volunteer_metrics) = &self.volunteer_metrics {
            let correlation = self.calculate_engagement_correlation(
                &cause_metrics.visualization_engagement,
                &volunteer_metrics.visualization_engagement
            );
            correlations.insert("cause_volunteer".to_string(), correlation);
        }
        
        if let Some(financial_metrics) = &self.financial_metrics {
            let correlation = self.calculate_engagement_correlation(
                &cause_metrics.visualization_engagement,
                &financial_metrics.visualization_engagement
            );
            correlations.insert("cause_financial".to_string(), correlation);
        }
        
        correlations
    }
    
    /// Calculate engagement correlation between two platforms
    fn calculate_engagement_correlation(
        &self,
        platform_a: &HashMap<String, VisualizationEngagement>,
        platform_b: &HashMap<String, impl Clone>,
    ) -> f64 {
        // In a real implementation, this would calculate actual statistical correlation
        // For now, we'll use a simplified approach based on engagement volume
        
        let a_engagement_count = platform_a.len() as f64;
        let b_engagement_count = platform_b.len() as f64;
        
        if a_engagement_count > 0.0 && b_engagement_count > 0.0 {
            (a_engagement_count.min(b_engagement_count) / a_engagement_count.max(b_engagement_count)).sqrt()
        } else {
            0.0
        }
    }
    
    /// Calculate holistic metrics combining all impact areas
    fn calculate_holistic_metrics(&self, cause_metrics: &ImpactMetrics) -> HolisticMetrics {
        debug!("Calculating holistic metrics");
        
        // Overall engagement across all platforms
        let overall_engagement = self.calculate_community_engagement_score(cause_metrics);
        
        // Cross-platform participation rate
        let cross_platform_participation = self.calculate_cross_platform_participation(cause_metrics);
        
        // Community transformation index
        let community_transformation_index = self.calculate_community_transformation_index(cause_metrics);
        
        // Resource allocation effectiveness across platforms
        let resource_allocation_effectiveness = self.calculate_resource_allocation_effectiveness(cause_metrics);
        
        // Community validation integration score
        let community_validation_integration = self.calculate_community_validation_integration(cause_metrics);
        
        HolisticMetrics {
            overall_engagement,
            cross_platform_participation,
            community_transformation_index,
            resource_allocation_effectiveness,
            community_validation_integration,
        }
    }
    
    /// Calculate cross-platform participation rate
    fn calculate_cross_platform_participation(&self, cause_metrics: &ImpactMetrics) -> f64 {
        let mut participating_platforms = 1.0; // At least cause platform
        
        if self.learning_metrics.is_some() {
            participating_platforms += 1.0;
        }
        
        if self.volunteer_metrics.is_some() {
            participating_platforms += 1.0;
        }
        
        if self.financial_metrics.is_some() {
            participating_platforms += 1.0;
        }
        
        participating_platforms / 4.0 // Max 4 platforms
    }
    
    /// Calculate community transformation index
    fn calculate_community_transformation_index(&self, cause_metrics: &ImpactMetrics) -> f64 {
        // This would combine various transformation metrics from all platforms
        // For now, we'll use a weighted average of key metrics
        
        let mut index = 0.0;
        let mut weight_sum = 0.0;
        
        // Cause engagement quality (weight: 0.3)
        let cause_quality = cause_metrics.visualization_engagement.values()
            .map(|e| e.quality_score)
            .sum::<f64>() / cause_metrics.visualization_engagement.len().max(1) as f64;
        index += cause_quality * 0.3;
        weight_sum += 0.3;
        
        // Feedback helpfulness (weight: 0.2)
        let feedback_helpfulness = if !cause_metrics.feedback_data.is_empty() {
            cause_metrics.feedback_data.iter()
                .filter(|f| f.helpful)
                .count() as f64 / cause_metrics.feedback_data.len() as f64
        } else {
            0.0
        };
        index += feedback_helpfulness * 0.2;
        weight_sum += 0.2;
        
        // Community validation engagement (weight: 0.2)
        let validation_engagement = cause_metrics.community_validation.len() as f64 /
            cause_metrics.engagement_correlation.len().max(1) as f64;
        index += validation_engagement.min(1.0) * 0.2;
        weight_sum += 0.2;
        
        // Contribution effectiveness (weight: 0.3)
        let contribution_effectiveness = if !cause_metrics.contribution_effectiveness.is_empty() {
            cause_metrics.contribution_effectiveness.iter()
                .filter_map(|c| c.quality.map(|q| q as f64))
                .sum::<f64>() / cause_metrics.contribution_effectiveness.len() as f64 / 10.0 // Normalize to 0-1
        } else {
            0.0
        };
        index += contribution_effectiveness * 0.3;
        weight_sum += 0.3;
        
        if weight_sum > 0.0 {
            index / weight_sum
        } else {
            0.0
        }
    }
    
    /// Calculate resource allocation effectiveness across platforms
    fn calculate_resource_allocation_effectiveness(&self, _cause_metrics: &ImpactMetrics) -> f64 {
        // This would analyze how effectively resources are allocated across all platforms
        // For now, we'll return a placeholder value
        0.75 // Placeholder - would be calculated in real implementation
    }
    
    /// Calculate community validation integration score
    fn calculate_community_validation_integration(&self, cause_metrics: &ImpactMetrics) -> f64 {
        // This measures how well community validation is integrated across platforms
        // For now, we'll base it on the volume and quality of validation
        
        let validation_count = cause_metrics.community_validation.len() as f64;
        let engagement_count = cause_metrics.engagement_correlation.len() as f64;
        
        if engagement_count > 0.0 {
            let validation_rate = validation_count / engagement_count;
            // Normalize to 0-1 range (assuming max validation rate of 0.5)
            validation_rate.min(1.0)
        } else {
            0.0
        }
    }
    
    /// Generate integrated recommendations across platforms
    fn generate_integrated_recommendations(&self, _cause_metrics: &ImpactMetrics) -> Vec<IntegratedRecommendation> {
        debug!("Generating integrated recommendations");
        
        let mut recommendations = Vec::new();
        
        // Add a recommendation to align visualization strategies across platforms
        recommendations.push(IntegratedRecommendation {
            recommendation_id: Uuid::new_v4().to_string(),
            description: "Align visualization strategies across learning, volunteering, financial, and cause platforms for consistent user experience".to_string(),
            affected_platforms: vec![
                AffectedPlatform::Learning,
                AffectedPlatform::Volunteering,
                AffectedPlatform::Financial,
                AffectedPlatform::Cause,
            ],
            priority: IntegrationPriority::High,
            suggested_action: CoordinatedAction::AlignVisualizationStrategies,
            expected_impact: 0.8,
            complexity: ImplementationComplexity::High,
        });
        
        // Add a recommendation to share community insights
        recommendations.push(IntegratedRecommendation {
            recommendation_id: Uuid::new_v4().to_string(),
            description: "Share community validation insights across platforms to improve overall impact".to_string(),
            affected_platforms: vec![
                AffectedPlatform::Learning,
                AffectedPlatform::Volunteering,
                AffectedPlatform::Financial,
                AffectedPlatform::Cause,
                AffectedPlatform::CrossPlatform,
            ],
            priority: IntegrationPriority::Medium,
            suggested_action: CoordinatedAction::ShareCommunityInsights,
            expected_impact: 0.7,
            complexity: ImplementationComplexity::Medium,
        });
        
        // Add a recommendation to coordinate resource allocation
        recommendations.push(IntegratedRecommendation {
            recommendation_id: Uuid::new_v4().to_string(),
            description: "Coordinate resource allocation decisions across platforms based on integrated impact analysis".to_string(),
            affected_platforms: vec![
                AffectedPlatform::Financial,
                AffectedPlatform::Cause,
                AffectedPlatform::CrossPlatform,
            ],
            priority: IntegrationPriority::Medium,
            suggested_action: CoordinatedAction::CoordinateResourceAllocation,
            expected_impact: 0.75,
            complexity: ImplementationComplexity::High,
        });
        
        recommendations
    }
}

impl Default for EcosystemIntegrator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tracker::{VisualizationEngagement, VisualizationType};
    use std::collections::HashMap;
    use uuid::Uuid;
    use chrono::Utc;
    
    #[test]
    fn test_ecosystem_integrator_creation() {
        let integrator = EcosystemIntegrator::new();
        assert!(integrator.learning_metrics.is_none());
        assert!(integrator.volunteer_metrics.is_none());
        assert!(integrator.financial_metrics.is_none());
    }
    
    #[test]
    fn test_set_metrics() {
        let mut integrator = EcosystemIntegrator::new();
        
        let learning_metrics = LearningMetrics {
            visualization_engagement: HashMap::new(),
            course_completion_correlation: Vec::new(),
            learning_effectiveness: Vec::new(),
            community_validation: Vec::new(),
            feedback_data: Vec::new(),
        };
        
        integrator.set_learning_metrics(learning_metrics);
        assert!(integrator.learning_metrics.is_some());
    }
    
    #[test]
    fn test_calculate_community_engagement_score() {
        let mut integrator = EcosystemIntegrator::new();
        
        // Set up learning metrics
        let mut learning_engagement = HashMap::new();
        learning_engagement.insert("learn_viz_1".to_string(), LearningEngagement {
            id: Uuid::new_v4(),
            viz_type: learning_impact_tracker::tracker::VisualizationType::Comparative,
            component_id: "learn_viz_1".to_string(),
            user_id: "user1".to_string(),
            interaction_time: 120.0,
            interaction_count: 10,
            timestamp: Utc::now(),
            quality_score: 0.8,
            learning_outcome_improvement: Some(0.7),
        });
        
        let learning_metrics = LearningMetrics {
            visualization_engagement: learning_engagement,
            course_completion_correlation: Vec::new(),
            learning_effectiveness: Vec::new(),
            community_validation: Vec::new(),
            feedback_data: Vec::new(),
        };
        
        integrator.set_learning_metrics(learning_metrics);
        
        // Set up cause metrics
        let mut cause_engagement = HashMap::new();
        cause_engagement.insert("cause_viz_1".to_string(), VisualizationEngagement {
            id: Uuid::new_v4(),
            viz_type: VisualizationType::Comparative,
            component_id: "cause_viz_1".to_string(),
            user_id: "user1".to_string(),
            interaction_time: 150.0,
            interaction_count: 12,
            timestamp: Utc::now(),
            quality_score: 0.85,
            decision_confidence: Some(0.9),
        });
        
        let cause_metrics = ImpactMetrics {
            visualization_engagement: cause_engagement,
            engagement_correlation: Vec::new(),
            contribution_effectiveness: Vec::new(),
            community_validation: Vec::new(),
            feedback_data: Vec::new(),
        };
        
        let engagement_score = integrator.calculate_community_engagement_score(&cause_metrics);
        assert!(engagement_score > 0.0);
        assert!(engagement_score <= 1.0);
    }
    
    #[test]
    fn test_analyze_integrated_impact() {
        let mut integrator = EcosystemIntegrator::new();
        
        // Set up cause metrics
        let cause_metrics = ImpactMetrics {
            visualization_engagement: HashMap::new(),
            engagement_correlation: Vec::new(),
            contribution_effectiveness: Vec::new(),
            community_validation: Vec::new(),
            feedback_data: Vec::new(),
        };
        
        let analysis = integrator.analyze_integrated_impact(&cause_metrics);
        assert!(analysis.community_engagement_score >= 0.0);
        assert!(!analysis.integrated_recommendations.is_empty());
    }
}