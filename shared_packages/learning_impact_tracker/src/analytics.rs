//! Impact Analytics Dashboard
//!
//! This module provides analytics and dashboard functionality for educators and coordinators
//! to understand the effectiveness of learning impact visualizations.

use serde::{Deserialize, Serialize};
use tracing::{debug, info};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::tracker::{ImpactMetrics, VisualizationEngagement, CourseCompletionCorrelation, 
                     VolunteerTransition, CommunityValidation, VisualizationFeedback};
use skill_development::ml::CommunityData;
use volunteer_coordination::ml::VolunteerActivity;

/// Analytics dashboard for educators and coordinators
pub struct ImpactAnalyticsDashboard {
    /// Current metrics being analyzed
    metrics: ImpactMetrics,
}

/// Dashboard metrics summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardSummary {
    /// Overall engagement metrics
    pub engagement: EngagementMetrics,
    
    /// Learning effectiveness metrics
    pub learning_effectiveness: LearningEffectivenessMetrics,
    
    /// Community impact metrics
    pub community_impact: CommunityImpactMetrics,
    
    /// Feedback summary
    pub feedback: FeedbackSummary,
    
    /// Recommendations for educators
    pub recommendations: Vec<EducatorRecommendation>,
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

/// Learning effectiveness metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningEffectivenessMetrics {
    /// Course completion rate with visualization usage
    pub completion_rate_with_viz: f64,
    
    /// Course completion rate without visualization usage
    pub completion_rate_without_viz: f64,
    
    /// Average time to completion with visualization usage
    pub avg_completion_time_with_viz: f64,
    
    /// Average time to completion without visualization usage
    pub avg_completion_time_without_viz: f64,
    
    /// Satisfaction correlation with visualization usage
    pub satisfaction_correlation: f64,
    
    /// Skill gap analysis insights
    pub skill_gaps: Vec<SkillGapInsight>,
}

/// Community impact metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityImpactMetrics {
    /// Volunteer transition rate
    pub volunteer_transition_rate: f64,
    
    /// Community validation engagement
    pub validation_engagement: f64,
    
    /// Learning to impact connection strength
    pub learning_impact_connection: f64,
    
    /// Community skill trends
    pub skill_trends: Vec<SkillTrend>,
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

/// Skill gap insight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillGapInsight {
    /// Skill name
    pub skill: String,
    
    /// Gap size
    pub gap_size: f64,
    
    /// Visualization coverage
    pub viz_coverage: f64,
    
    /// Recommendation
    pub recommendation: String,
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

/// Trend direction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
}

/// Feedback theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackTheme {
    /// Theme description
    pub theme: String,
    
    /// Frequency of occurrence
    pub frequency: u64,
    
    /// Sentiment score (-1.0 to 1.0)
    pub sentiment: f64,
}

/// Feedback trend data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackTrend {
    /// Time period
    pub period: String,
    
    /// Average rating
    pub avg_rating: f64,
    
    /// Feedback count
    pub feedback_count: u64,
}

/// Educator recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EducatorRecommendation {
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
    CourseAdjustment,
    VisualizationImprovement,
    ResourceAllocation,
    CommunityEngagement,
    SkillDevelopment,
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
            learning_effectiveness: self.calculate_learning_effectiveness_metrics(),
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
    
    /// Calculate learning effectiveness metrics
    fn calculate_learning_effectiveness_metrics(&self) -> LearningEffectivenessMetrics {
        debug!("Calculating learning effectiveness metrics");
        
        let with_viz: Vec<&CourseCompletionCorrelation> = self.metrics.completion_correlation
            .iter()
            .filter(|correlation| !correlation.viz_usage.is_empty())
            .collect();
        
        let without_viz: Vec<&CourseCompletionCorrelation> = self.metrics.completion_correlation
            .iter()
            .filter(|correlation| correlation.viz_usage.is_empty())
            .collect();
        
        let completion_rate_with_viz = if !with_viz.is_empty() {
            with_viz.iter()
                .filter(|correlation| correlation.completed)
                .count() as f64 / with_viz.len() as f64
        } else {
            0.0
        };
        
        let completion_rate_without_viz = if !without_viz.is_empty() {
            without_viz.iter()
                .filter(|correlation| correlation.completed)
                .count() as f64 / without_viz.len() as f64
        } else {
            0.0
        };
        
        let avg_completion_time_with_viz = if !with_viz.is_empty() {
            with_viz.iter()
                .filter_map(|correlation| correlation.time_to_completion)
                .sum::<f64>() / with_viz.len() as f64
        } else {
            0.0
        };
        
        let avg_completion_time_without_viz = if !without_viz.is_empty() {
            without_viz.iter()
                .filter_map(|correlation| correlation.time_to_completion)
                .sum::<f64>() / without_viz.len() as f64
        } else {
            0.0
        };
        
        let satisfaction_correlation = 0.0; // Would be calculated in real implementation
        
        LearningEffectivenessMetrics {
            completion_rate_with_viz,
            completion_rate_without_viz,
            avg_completion_time_with_viz,
            avg_completion_time_without_viz,
            satisfaction_correlation,
            skill_gaps: Vec::new(), // Would be calculated in real implementation
        }
    }
    
    /// Calculate community impact metrics
    fn calculate_community_impact_metrics(&self) -> CommunityImpactMetrics {
        debug!("Calculating community impact metrics");
        
        let total_completions = self.metrics.completion_correlation.len() as f64;
        let volunteer_transitions = self.metrics.volunteer_transitions.len() as f64;
        
        let volunteer_transition_rate = if total_completions > 0.0 {
            volunteer_transitions / total_completions
        } else {
            0.0
        };
        
        let validation_engagement = self.metrics.community_validation.len() as f64;
        
        let learning_impact_connection = if total_completions > 0.0 {
            (volunteer_transitions + validation_engagement) / total_completions
        } else {
            0.0
        };
        
        CommunityImpactMetrics {
            volunteer_transition_rate,
            validation_engagement,
            learning_impact_connection,
            skill_trends: Vec::new(), // Would be calculated in real implementation
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
    
    /// Generate recommendations for educators
    fn generate_recommendations(&self, _community_data: &CommunityData) -> Vec<EducatorRecommendation> {
        debug!("Generating educator recommendations");
        
        let mut recommendations = Vec::new();
        
        // Check for low engagement visualizations
        for (viz_id, engagement) in &self.metrics.visualization_engagement {
            if engagement.quality_score < 0.5 {
                recommendations.push(EducatorRecommendation {
                    rec_type: RecommendationType::VisualizationImprovement,
                    description: format!("Visualization '{}' has low engagement quality score. Consider revising.", viz_id),
                    priority: PriorityLevel::High,
                    data: None,
                });
            }
        }
        
        // Check for low completion rates with visualization usage
        let effectiveness = self.calculate_learning_effectiveness_metrics();
        if effectiveness.completion_rate_with_viz < 0.5 {
            recommendations.push(EducatorRecommendation {
                rec_type: RecommendationType::CourseAdjustment,
                description: "Courses using visualizations have low completion rates. Review integration approach.".to_string(),
                priority: PriorityLevel::High,
                data: None,
            });
        }
        
        // Check for low feedback helpfulness
        let feedback_summary = self.calculate_feedback_summary();
        if feedback_summary.helpful_percentage < 70.0 {
            recommendations.push(EducatorRecommendation {
                rec_type: RecommendationType::VisualizationImprovement,
                description: "Low percentage of users find visualizations helpful. Consider user research.".to_string(),
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
    use crate::tracker::{VisualizationEngagement, CourseCompletionCorrelation, VisualizationType};
    use std::collections::HashMap;
    use uuid::Uuid;
    use chrono::Utc;
    
    #[test]
    fn test_dashboard_creation() {
        let metrics = ImpactMetrics {
            visualization_engagement: HashMap::new(),
            completion_correlation: Vec::new(),
            volunteer_transitions: Vec::new(),
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
            completion_correlation: Vec::new(),
            volunteer_transitions: Vec::new(),
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