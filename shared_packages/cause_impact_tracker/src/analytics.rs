//! Impact Analytics Dashboard
//!
//! This module provides analytics and dashboard functionality for cause coordinators
//! to understand the effectiveness of cause impact visualizations.

use serde::{Deserialize, Serialize};
use tracing::{debug, info};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::tracker::{ImpactMetrics, VisualizationEngagement, EngagementCorrelation, 
                     ContributionEffectiveness, CommunityValidation, VisualizationFeedback};
use cause_management::ml::{CauseData, ResourceAllocationRecord};
use skill_development::ml::CommunityData;
use learning_impact_tracker::analytics::{TrendDirection, FeedbackTheme, FeedbackTrend};

/// Analytics dashboard for cause coordinators
pub struct ImpactAnalyticsDashboard {
    /// Current metrics being analyzed
    metrics: ImpactMetrics,
}

/// Dashboard metrics summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardSummary {
    /// Overall engagement metrics
    pub engagement: EngagementMetrics,
    
    /// Cause effectiveness metrics
    pub cause_effectiveness: CauseEffectivenessMetrics,
    
    /// Community impact metrics
    pub community_impact: CommunityImpactMetrics,
    
    /// Feedback summary
    pub feedback: FeedbackSummary,
    
    /// Recommendations for coordinators
    pub recommendations: Vec<CoordinatorRecommendation>,
}

/// Engagement metrics for visualizations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngagementMetrics {
    /// Total visualization views
    pub total_views: u64,
    
    /// Average interaction time (seconds)
    pub avg_interaction_time: f64,
    
    /// Engagement quality score (0.0 to 1.0)
    pub quality_score: f64,
    
    /// Average decision confidence after using visualizations
    pub avg_decision_confidence: f64,
    
    /// Most popular visualizations
    pub popular_viz: Vec<PopularVisualization>,
    
    /// Engagement trends over time
    pub trends: Vec<EngagementTrend>,
}

/// Cause effectiveness metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CauseEffectivenessMetrics {
    /// Cause engagement rate with visualization usage
    pub engagement_rate_with_viz: f64,
    
    /// Cause engagement rate without visualization usage
    pub engagement_rate_without_viz: f64,
    
    /// Average contribution amount with visualization usage
    pub avg_contribution_with_viz: Option<f64>,
    
    /// Average contribution amount without visualization usage
    pub avg_contribution_without_viz: Option<f64>,
    
    /// Contribution effectiveness with visualization influence
    pub contribution_effectiveness_with_viz: f64,
    
    /// Contribution effectiveness without visualization influence
    pub contribution_effectiveness_without_viz: f64,
    
    /// Satisfaction correlation with visualization usage
    pub satisfaction_correlation: f64,
    
    /// Engagement trend analysis
    pub engagement_trends: Vec<EngagementTrend>,
}

/// Community impact metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityImpactMetrics {
    /// Community validation engagement
    pub validation_engagement: f64,
    
    /// Cause to community connection strength
    pub cause_community_connection: f64,
    
    /// Community understanding trends
    pub understanding_trends: Vec<UnderstandingTrend>,
    
    /// Contribution trends
    pub contribution_trends: Vec<ContributionTrend>,
}

/// Feedback summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackSummary {
    /// Average feedback rating
    pub avg_rating: f64,
    
    /// Percentage finding visualizations helpful
    pub helpful_percentage: f64,
    
    /// Average understanding improvement rating
    pub avg_understanding_improvement: f64,
    
    /// Average confidence improvement rating
    pub avg_confidence_improvement: f64,
    
    /// Common feedback themes
    pub common_themes: Vec<FeedbackTheme>,
    
    /// Feedback volume trends
    pub feedback_trends: Vec<FeedbackTrend>,
}

/// Popular visualization data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopularVisualization {
    /// Visualization identifier
    pub viz_id: String,
    
    /// View count
    pub view_count: u64,
    
    /// Engagement score
    pub engagement_score: f64,
}

/// Engagement trend data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngagementTrend {
    /// Time period
    pub period: String,
    
    /// Engagement score
    pub engagement_score: f64,
    
    /// View count
    pub view_count: u64,
}

/// Understanding trend data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnderstandingTrend {
    /// Skill/understanding area
    pub area: String,
    
    /// Trend direction
    pub trend: TrendDirection,
    
    /// Change magnitude
    pub magnitude: f64,
}

/// Contribution trend data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributionTrend {
    /// Resource category
    pub resource_category: String,
    
    /// Trend direction
    pub trend: TrendDirection,
    
    /// Contribution amount
    pub contribution_amount: Option<f64>,
}

/// Coordinator recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinatorRecommendation {
    /// Recommendation type
    pub rec_type: RecommendationType,
    
    /// Recommendation description
    pub description: String,
    
    /// Priority level
    pub priority: PriorityLevel,
    
    /// Supporting data
    pub data: Option<serde_json::Value>,
}

/// Recommendation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationType {
    CauseEngagementImprovement,
    VisualizationImprovement,
    ResourceAllocation,
    CommunityEngagement,
    CauseEducation,
    EngagementStrategy,
}

/// Priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PriorityLevel {
    High,
    Medium,
    Low,
}

impl ImpactAnalyticsDashboard {
    /// Create a new analytics dashboard
    pub fn new(metrics: ImpactMetrics) -> Self {
        info!("Initializing ImpactAnalyticsDashboard");
        Self { metrics }
    }
    
    /// Generate a comprehensive dashboard summary
    pub fn generate_summary(&self, community_data: &CommunityData) -> DashboardSummary {
        debug!("Generating dashboard summary");
        
        DashboardSummary {
            engagement: self.calculate_engagement_metrics(),
            cause_effectiveness: self.calculate_cause_effectiveness_metrics(),
            community_impact: self.calculate_community_impact_metrics(),
            feedback: self.calculate_feedback_summary(),
            recommendations: self.generate_recommendations(community_data),
        }
    }
    
    /// Calculate engagement metrics
    fn calculate_engagement_metrics(&self) -> EngagementMetrics {
        debug!("Calculating engagement metrics");
        
        let total_views = self.metrics.visualization_engagement.len() as u64;
        
        let (total_time, total_count, total_quality, total_confidence, confidence_count) = self.metrics.visualization_engagement
            .values()
            .fold((0.0, 0, 0.0, 0.0, 0), |(time, count, quality, confidence, conf_count), engagement| {
                let (new_confidence, new_conf_count) = if let Some(conf) = engagement.decision_confidence {
                    (confidence + conf, conf_count + 1)
                } else {
                    (confidence, conf_count)
                };
                (time + engagement.interaction_time, 
                 count + engagement.interaction_count, 
                 quality + engagement.quality_score,
                 new_confidence,
                 new_conf_count)
            });
        
        let avg_interaction_time = if total_views > 0 { total_time / total_views as f64 } else { 0.0 };
        let quality_score = if total_views > 0 { total_quality / total_views as f64 } else { 0.0 };
        let avg_decision_confidence = if confidence_count > 0 { total_confidence / confidence_count as f64 } else { 0.0 };
        
        // Calculate popular visualizations
        let mut viz_counts: HashMap<String, u64> = HashMap::new();
        for engagement in self.metrics.visualization_engagement.values() {
            *viz_counts.entry(engagement.component_id.clone()).or_insert(0) += 1;
        }
        
        let mut popular_viz: Vec<PopularVisualization> = viz_counts
            .into_iter()
            .map(|(viz_id, view_count)| PopularVisualization {
                viz_id,
                view_count,
                engagement_score: 0.0, // Would be calculated in real implementation
            })
            .collect();
        
        // Sort by view count
        popular_viz.sort_by(|a, b| b.view_count.cmp(&a.view_count));
        
        EngagementMetrics {
            total_views,
            avg_interaction_time,
            quality_score,
            avg_decision_confidence,
            popular_viz,
            trends: Vec::new(), // Would be calculated in real implementation
        }
    }
    
    /// Calculate cause effectiveness metrics
    fn calculate_cause_effectiveness_metrics(&self) -> CauseEffectivenessMetrics {
        debug!("Calculating cause effectiveness metrics");
        
        let with_viz: Vec<&EngagementCorrelation> = self.metrics.engagement_correlation
            .iter()
            .filter(|correlation| !correlation.viz_usage.is_empty())
            .collect();
        
        let without_viz: Vec<&EngagementCorrelation> = self.metrics.engagement_correlation
            .iter()
            .filter(|correlation| correlation.viz_usage.is_empty())
            .collect();
        
        let engagement_rate_with_viz = if !with_viz.is_empty() {
            with_viz.iter()
                .filter(|correlation| correlation.engaged)
                .count() as f64 / with_viz.len() as f64
        } else {
            0.0
        };
        
        let engagement_rate_without_viz = if !without_viz.is_empty() {
            without_viz.iter()
                .filter(|correlation| correlation.engaged)
                .count() as f64 / without_viz.len() as f64
        } else {
            0.0
        };
        
        // Calculate average contributions
        let avg_contribution_with_viz = if !with_viz.is_empty() {
            let total: f64 = with_viz.iter()
                .filter_map(|c| c.contribution_amount)
                .sum();
            let count = with_viz.iter()
                .filter(|c| c.contribution_amount.is_some())
                .count();
            
            if count > 0 {
                Some(total / count as f64)
            } else {
                None
            }
        } else {
            None
        };
        
        let avg_contribution_without_viz = if !without_viz.is_empty() {
            let total: f64 = without_viz.iter()
                .filter_map(|c| c.contribution_amount)
                .sum();
            let count = without_viz.iter()
                .filter(|c| c.contribution_amount.is_some())
                .count();
            
            if count > 0 {
                Some(total / count as f64)
            } else {
                None
            }
        } else {
            None
        };
        
        // Calculate contribution effectiveness
        let with_viz_contrib: Vec<&ContributionEffectiveness> = self.metrics.contribution_effectiveness
            .iter()
            .filter(|contribution| contribution.influencing_viz.is_some())
            .collect();
        
        let without_viz_contrib: Vec<&ContributionEffectiveness> = self.metrics.contribution_effectiveness
            .iter()
            .filter(|contribution| contribution.influencing_viz.is_none())
            .collect();
        
        let contribution_effectiveness_with_viz = if !with_viz_contrib.is_empty() {
            with_viz_contrib.iter()
                .filter_map(|contribution| contribution.quality.map(|q| q as f64))
                .sum::<f64>() / with_viz_contrib.len() as f64
        } else {
            0.0
        };
        
        let contribution_effectiveness_without_viz = if !without_viz_contrib.is_empty() {
            without_viz_contrib.iter()
                .filter_map(|contribution| contribution.quality.map(|q| q as f64))
                .sum::<f64>() / without_viz_contrib.len() as f64
        } else {
            0.0
        };
        
        let satisfaction_correlation = 0.0; // Would be calculated in real implementation
        
        CauseEffectivenessMetrics {
            engagement_rate_with_viz,
            engagement_rate_without_viz,
            avg_contribution_with_viz,
            avg_contribution_without_viz,
            contribution_effectiveness_with_viz,
            contribution_effectiveness_without_viz,
            satisfaction_correlation,
            engagement_trends: Vec::new(), // Would be calculated in real implementation
        }
    }
    
    /// Calculate community impact metrics
    fn calculate_community_impact_metrics(&self) -> CommunityImpactMetrics {
        debug!("Calculating community impact metrics");
        
        let total_engagement_records = self.metrics.engagement_correlation.len() as f64;
        let validation_engagement = self.metrics.community_validation.len() as f64;
        
        let cause_community_connection = if total_engagement_records > 0.0 {
            validation_engagement / total_engagement_records
        } else {
            0.0
        };
        
        CommunityImpactMetrics {
            validation_engagement,
            cause_community_connection,
            understanding_trends: Vec::new(), // Would be calculated in real implementation
            contribution_trends: Vec::new(), // Would be calculated in real implementation
        }
    }
    
    /// Calculate feedback summary
    fn calculate_feedback_summary(&self) -> FeedbackSummary {
        debug!("Calculating feedback summary");
        
        let total_feedback = self.metrics.feedback_data.len() as f64;
        
        let avg_rating = if !self.metrics.feedback_data.is_empty() {
            self.metrics.feedback_data.iter()
                .map(|feedback| feedback.rating as f64)
                .sum::<f64>() / total_feedback
        } else {
            0.0
        };
        
        let helpful_count = self.metrics.feedback_data.iter()
            .filter(|feedback| feedback.helpful)
            .count() as f64;
        
        let helpful_percentage = if total_feedback > 0.0 {
            helpful_count / total_feedback * 100.0
        } else {
            0.0
        };
        
        // Calculate understanding improvement
        let (total_understanding, understanding_count) = self.metrics.feedback_data.iter()
            .filter_map(|feedback| feedback.understanding_improvement.map(|u| u as f64))
            .fold((0.0, 0), |(sum, count), value| (sum + value, count + 1));
        
        let avg_understanding_improvement = if understanding_count > 0 {
            total_understanding / understanding_count as f64
        } else {
            0.0
        };
        
        // Calculate confidence improvement
        let (total_confidence, confidence_count) = self.metrics.feedback_data.iter()
            .filter_map(|feedback| feedback.confidence_improvement.map(|c| c as f64))
            .fold((0.0, 0), |(sum, count), value| (sum + value, count + 1));
        
        let avg_confidence_improvement = if confidence_count > 0 {
            total_confidence / confidence_count as f64
        } else {
            0.0
        };
        
        FeedbackSummary {
            avg_rating,
            helpful_percentage,
            avg_understanding_improvement,
            avg_confidence_improvement,
            common_themes: Vec::new(), // Would be calculated in real implementation
            feedback_trends: Vec::new(), // Would be calculated in real implementation
        }
    }
    
    /// Generate recommendations for coordinators
    fn generate_recommendations(&self, _community_data: &CommunityData) -> Vec<CoordinatorRecommendation> {
        debug!("Generating coordinator recommendations");
        
        let mut recommendations = Vec::new();
        
        // Check for low engagement visualizations
        for (viz_id, engagement) in &self.metrics.visualization_engagement {
            if engagement.quality_score < 0.5 {
                recommendations.push(CoordinatorRecommendation {
                    rec_type: RecommendationType::VisualizationImprovement,
                    description: format!("Visualization '{}' has low engagement quality score. Consider revising.", viz_id),
                    priority: PriorityLevel::High,
                    data: None,
                });
            }
        }
        
        // Check for low engagement rates with visualization usage
        let effectiveness = self.calculate_cause_effectiveness_metrics();
        if effectiveness.engagement_rate_with_viz < 0.5 {
            recommendations.push(CoordinatorRecommendation {
                rec_type: RecommendationType::EngagementStrategy,
                description: "Cause participants using visualizations have low engagement rates. Review integration approach.".to_string(),
                priority: PriorityLevel::High,
                data: None,
            });
        }
        
        // Check for low contribution effectiveness with visualization influence
        if effectiveness.contribution_effectiveness_with_viz < 7.0 {
            recommendations.push(CoordinatorRecommendation {
                rec_type: RecommendationType::ResourceAllocation,
                description: "Contributions influenced by visualizations have low effectiveness scores. Review visualization impact.".to_string(),
                priority: PriorityLevel::Medium,
                data: None,
            });
        }
        
        // Check for low feedback helpfulness
        let feedback_summary = self.calculate_feedback_summary();
        if feedback_summary.helpful_percentage < 70.0 {
            recommendations.push(CoordinatorRecommendation {
                rec_type: RecommendationType::VisualizationImprovement,
                description: "Low percentage of participants find visualizations helpful. Consider user research.".to_string(),
                priority: PriorityLevel::Medium,
                data: None,
            });
        }
        
        // Check for low understanding improvement
        if feedback_summary.avg_understanding_improvement < 5.0 {
            recommendations.push(CoordinatorRecommendation {
                rec_type: RecommendationType::CauseEducation,
                description: "Low cause understanding improvement reported. Consider enhancing educational aspects of visualizations.".to_string(),
                priority: PriorityLevel::Medium,
                data: None,
            });
        }
        
        recommendations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tracker::{VisualizationEngagement, EngagementCorrelation, VisualizationType};
    use std::collections::HashMap;
    use uuid::Uuid;
    use chrono::Utc;
    
    #[test]
    fn test_dashboard_creation() {
        let metrics = ImpactMetrics {
            visualization_engagement: HashMap::new(),
            engagement_correlation: Vec::new(),
            contribution_effectiveness: Vec::new(),
            community_validation: Vec::new(),
            feedback_data: Vec::new(),
        };
        
        let dashboard = ImpactAnalyticsDashboard::new(metrics);
        assert!(true); // Dashboard should be created successfully
    }
    
    #[test]
    fn test_engagement_metrics_calculation() {
        let mut engagement_map = HashMap::new();
        engagement_map.insert("viz1".to_string(), VisualizationEngagement {
            id: Uuid::new_v4(),
            viz_type: VisualizationType::Comparative,
            component_id: "viz1".to_string(),
            user_id: "user1".to_string(),
            interaction_time: 120.0,
            interaction_count: 10,
            timestamp: Utc::now(),
            quality_score: 0.8,
            decision_confidence: Some(0.9),
        });
        
        let metrics = ImpactMetrics {
            visualization_engagement: engagement_map,
            engagement_correlation: Vec::new(),
            contribution_effectiveness: Vec::new(),
            community_validation: Vec::new(),
            feedback_data: Vec::new(),
        };
        
        let dashboard = ImpactAnalyticsDashboard::new(metrics);
        let engagement_metrics = dashboard.calculate_engagement_metrics();
        
        assert_eq!(engagement_metrics.total_views, 1);
        assert_eq!(engagement_metrics.avg_interaction_time, 120.0);
        assert_eq!(engagement_metrics.quality_score, 0.8);
        assert_eq!(engagement_metrics.avg_decision_confidence, 0.9);
    }
    
    #[test]
    fn test_cause_effectiveness_metrics_calculation() {
        let correlation = EngagementCorrelation {
            id: Uuid::new_v4(),
            user_id: "user1".to_string(),
            viz_usage: vec!["viz1".to_string()],
            engaged: true,
            engagement_months: Some(6.0),
            satisfaction: Some(8),
            contribution_amount: Some(100.50),
            timestamp: Utc::now(),
        };
        
        let metrics = ImpactMetrics {
            visualization_engagement: HashMap::new(),
            engagement_correlation: vec![correlation],
            contribution_effectiveness: Vec::new(),
            community_validation: Vec::new(),
            feedback_data: Vec::new(),
        };
        
        let dashboard = ImpactAnalyticsDashboard::new(metrics);
        let effectiveness_metrics = dashboard.calculate_cause_effectiveness_metrics();
        
        assert_eq!(effectiveness_metrics.engagement_rate_with_viz, 1.0);
        assert!(effectiveness_metrics.avg_contribution_with_viz.is_some());
    }
}