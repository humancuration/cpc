//! Impact Analytics Dashboard
//!
//! This module provides analytics and dashboard functionality for volunteer coordinators
//! to understand the effectiveness of volunteer impact visualizations.

use serde::{Deserialize, Serialize};
use tracing::{debug, info};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::tracker::{ImpactMetrics, VisualizationEngagement, RetentionCorrelation, 
                     TaskCompletion, CommunityValidation, VisualizationFeedback};
use volunteer_coordination::ml::{VolunteerActivity, VolunteerEngagementData};
use skill_development::ml::CommunityData;
use learning_impact_tracker::analytics::{TrendDirection, FeedbackTheme, FeedbackTrend};

/// Analytics dashboard for volunteer coordinators
pub struct ImpactAnalyticsDashboard {
    /// Current metrics being analyzed
    metrics: ImpactMetrics,
}

/// Dashboard metrics summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardSummary {
    /// Overall engagement metrics
    pub engagement: EngagementMetrics,
    
    /// Volunteer effectiveness metrics
    pub volunteer_effectiveness: VolunteerEffectivenessMetrics,
    
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
    
    /// Most popular visualizations
    pub popular_viz: Vec<PopularVisualization>,
    
    /// Engagement trends over time
    pub trends: Vec<EngagementTrend>,
}

/// Volunteer effectiveness metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolunteerEffectivenessMetrics {
    /// Volunteer retention rate with visualization usage
    pub retention_rate_with_viz: f64,
    
    /// Volunteer retention rate without visualization usage
    pub retention_rate_without_viz: f64,
    
    /// Task completion rate with visualization influence
    pub completion_rate_with_viz: f64,
    
    /// Task completion rate without visualization influence
    pub completion_rate_without_viz: f64,
    
    /// Average task quality with visualization influence
    pub avg_task_quality_with_viz: f64,
    
    /// Average task quality without visualization influence
    pub avg_task_quality_without_viz: f64,
    
    /// Satisfaction correlation with visualization usage
    pub satisfaction_correlation: f64,
    
    /// Retention trend analysis
    pub retention_trends: Vec<RetentionTrend>,
}

/// Community impact metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityImpactMetrics {
    /// Community validation engagement
    pub validation_engagement: f64,
    
    /// Volunteer to community connection strength
    pub volunteer_community_connection: f64,
    
    /// Community skill trends
    pub skill_trends: Vec<SkillTrend>,
    
    /// Volunteer activity trends
    pub activity_trends: Vec<ActivityTrend>,
}

/// Feedback summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackSummary {
    /// Average feedback rating
    pub avg_rating: f64,
    
    /// Percentage finding visualizations helpful
    pub helpful_percentage: f64,
    
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

/// Retention trend data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionTrend {
    /// Time period
    pub period: String,
    
    /// Retention rate
    pub retention_rate: f64,
    
    /// Influencing visualizations
    pub influencing_viz: Vec<String>,
}

/// Skill trend data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTrend {
    /// Skill name
    pub skill: String,
    
    /// Trend direction
    pub trend: TrendDirection,
    
    /// Change magnitude
    pub magnitude: f64,
}

/// Activity trend data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityTrend {
    /// Activity type
    pub activity_type: String,
    
    /// Trend direction
    pub trend: TrendDirection,
    
    /// Participation count
    pub participation_count: u64,
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
    VolunteerOpportunityAdjustment,
    VisualizationImprovement,
    ResourceAllocation,
    CommunityEngagement,
    SkillDevelopment,
    RetentionStrategy,
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
            volunteer_effectiveness: self.calculate_volunteer_effectiveness_metrics(),
            community_impact: self.calculate_community_impact_metrics(),
            feedback: self.calculate_feedback_summary(),
            recommendations: self.generate_recommendations(community_data),
        }
    }
    
    /// Calculate engagement metrics
    fn calculate_engagement_metrics(&self) -> EngagementMetrics {
        debug!("Calculating engagement metrics");
        
        let total_views = self.metrics.visualization_engagement.len() as u64;
        
        let (total_time, total_count, total_quality) = self.metrics.visualization_engagement
            .values()
            .fold((0.0, 0, 0.0), |(time, count, quality), engagement| {
                (time + engagement.interaction_time, 
                 count + engagement.interaction_count, 
                 quality + engagement.quality_score)
            });
        
        let avg_interaction_time = if total_views > 0 { total_time / total_views as f64 } else { 0.0 };
        let quality_score = if total_views > 0 { total_quality / total_views as f64 } else { 0.0 };
        
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
            popular_viz,
            trends: Vec::new(), // Would be calculated in real implementation
        }
    }
    
    /// Calculate volunteer effectiveness metrics
    fn calculate_volunteer_effectiveness_metrics(&self) -> VolunteerEffectivenessMetrics {
        debug!("Calculating volunteer effectiveness metrics");
        
        let with_viz: Vec<&RetentionCorrelation> = self.metrics.retention_correlation
            .iter()
            .filter(|correlation| !correlation.viz_usage.is_empty())
            .collect();
        
        let without_viz: Vec<&RetentionCorrelation> = self.metrics.retention_correlation
            .iter()
            .filter(|correlation| correlation.viz_usage.is_empty())
            .collect();
        
        let retention_rate_with_viz = if !with_viz.is_empty() {
            with_viz.iter()
                .filter(|correlation| correlation.retained)
                .count() as f64 / with_viz.len() as f64
        } else {
            0.0
        };
        
        let retention_rate_without_viz = if !without_viz.is_empty() {
            without_viz.iter()
                .filter(|correlation| correlation.retained)
                .count() as f64 / without_viz.len() as f64
        } else {
            0.0
        };
        
        let with_viz_tasks: Vec<&TaskCompletion> = self.metrics.task_completion
            .iter()
            .filter(|completion| completion.influencing_viz.is_some())
            .collect();
        
        let without_viz_tasks: Vec<&TaskCompletion> = self.metrics.task_completion
            .iter()
            .filter(|completion| completion.influencing_viz.is_none())
            .collect();
        
        let completion_rate_with_viz = if !with_viz_tasks.is_empty() {
            with_viz_tasks.iter()
                .filter(|completion| completion.completed)
                .count() as f64 / with_viz_tasks.len() as f64
        } else {
            0.0
        };
        
        let completion_rate_without_viz = if !without_viz_tasks.is_empty() {
            without_viz_tasks.iter()
                .filter(|completion| completion.completed)
                .count() as f64 / without_viz_tasks.len() as f64
        } else {
            0.0
        };
        
        let avg_task_quality_with_viz = if !with_viz_tasks.is_empty() {
            with_viz_tasks.iter()
                .filter_map(|completion| completion.quality.map(|q| q as f64))
                .sum::<f64>() / with_viz_tasks.len() as f64
        } else {
            0.0
        };
        
        let avg_task_quality_without_viz = if !without_viz_tasks.is_empty() {
            without_viz_tasks.iter()
                .filter_map(|completion| completion.quality.map(|q| q as f64))
                .sum::<f64>() / without_viz_tasks.len() as f64
        } else {
            0.0
        };
        
        let satisfaction_correlation = 0.0; // Would be calculated in real implementation
        
        VolunteerEffectivenessMetrics {
            retention_rate_with_viz,
            retention_rate_without_viz,
            completion_rate_with_viz,
            completion_rate_without_viz,
            avg_task_quality_with_viz,
            avg_task_quality_without_viz,
            satisfaction_correlation,
            retention_trends: Vec::new(), // Would be calculated in real implementation
        }
    }
    
    /// Calculate community impact metrics
    fn calculate_community_impact_metrics(&self) -> CommunityImpactMetrics {
        debug!("Calculating community impact metrics");
        
        let total_retention_records = self.metrics.retention_correlation.len() as f64;
        let validation_engagement = self.metrics.community_validation.len() as f64;
        
        let volunteer_community_connection = if total_retention_records > 0.0 {
            validation_engagement / total_retention_records
        } else {
            0.0
        };
        
        CommunityImpactMetrics {
            validation_engagement,
            volunteer_community_connection,
            skill_trends: Vec::new(), // Would be calculated in real implementation
            activity_trends: Vec::new(), // Would be calculated in real implementation
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
        
        FeedbackSummary {
            avg_rating,
            helpful_percentage,
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
        
        // Check for low retention rates with visualization usage
        let effectiveness = self.calculate_volunteer_effectiveness_metrics();
        if effectiveness.retention_rate_with_viz < 0.5 {
            recommendations.push(CoordinatorRecommendation {
                rec_type: RecommendationType::RetentionStrategy,
                description: "Volunteers using visualizations have low retention rates. Review integration approach.".to_string(),
                priority: PriorityLevel::High,
                data: None,
            });
        }
        
        // Check for low task completion rates with visualization influence
        if effectiveness.completion_rate_with_viz < 0.7 {
            recommendations.push(CoordinatorRecommendation {
                rec_type: RecommendationType::VolunteerOpportunityAdjustment,
                description: "Volunteers influenced by visualizations have low task completion rates. Review task matching.".to_string(),
                priority: PriorityLevel::Medium,
                data: None,
            });
        }
        
        // Check for low feedback helpfulness
        let feedback_summary = self.calculate_feedback_summary();
        if feedback_summary.helpful_percentage < 70.0 {
            recommendations.push(CoordinatorRecommendation {
                rec_type: RecommendationType::VisualizationImprovement,
                description: "Low percentage of volunteers find visualizations helpful. Consider user research.".to_string(),
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
    use crate::tracker::{VisualizationEngagement, RetentionCorrelation, VisualizationType};
    use std::collections::HashMap;
    use uuid::Uuid;
    use chrono::Utc;
    
    #[test]
    fn test_dashboard_creation() {
        let metrics = ImpactMetrics {
            visualization_engagement: HashMap::new(),
            retention_correlation: Vec::new(),
            task_completion: Vec::new(),
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
        });
        
        let metrics = ImpactMetrics {
            visualization_engagement: engagement_map,
            retention_correlation: Vec::new(),
            task_completion: Vec::new(),
            community_validation: Vec::new(),
            feedback_data: Vec::new(),
        };
        
        let dashboard = ImpactAnalyticsDashboard::new(metrics);
        let engagement_metrics = dashboard.calculate_engagement_metrics();
        
        assert_eq!(engagement_metrics.total_views, 1);
        assert_eq!(engagement_metrics.avg_interaction_time, 120.0);
        assert_eq!(engagement_metrics.quality_score, 0.8);
    }
}