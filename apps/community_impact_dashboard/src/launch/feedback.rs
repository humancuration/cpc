//! Launch feedback integration
//!
//! This module provides enhanced feedback collection and analysis specifically
//! for the dashboard launch period, integrating with launch metrics and celebration features.

use crate::feedback::{UserFeedback, FeedbackCategory, FeedbackCollector};
use crate::launch::metrics::{LaunchMetrics, MetricCategory};
use tracing::info;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Launch feedback integration system
pub struct LaunchFeedbackIntegration {
    feedback_collector: FeedbackCollector,
    launch_metrics: LaunchMetrics,
    launch_feedback: Vec<LaunchFeedback>,
}

impl LaunchFeedbackIntegration {
    /// Create a new launch feedback integration system
    pub fn new(feedback_collector: FeedbackCollector, launch_metrics: LaunchMetrics) -> Self {
        Self {
            feedback_collector,
            launch_metrics,
            launch_feedback: Vec::new(),
        }
    }
    
    /// Collect launch-specific feedback and update metrics
    pub fn collect_launch_feedback(&mut self, feedback: UserFeedback) {
        // Store the original feedback
        self.feedback_collector.collect_feedback(feedback.clone());
        
        // Create launch-specific feedback record
        let launch_feedback = LaunchFeedback::from_user_feedback(feedback);
        self.launch_feedback.push(launch_feedback.clone());
        
        // Update launch metrics based on feedback
        self.update_metrics_from_feedback(&launch_feedback);
        
        info!("Collected launch feedback: {:?}", launch_feedback.feedback_type);
    }
    
    /// Update launch metrics based on feedback
    fn update_metrics_from_feedback(&mut self, launch_feedback: &LaunchFeedback) {
        match launch_feedback.feedback_type {
            LaunchFeedbackType::DashboardUnderstanding => {
                // Track understanding improvement metrics
                if let Some(rating) = launch_feedback.rating {
                    self.launch_metrics.record_metric(
                        "understanding_score",
                        rating as f64,
                        MetricCategory::Understanding
                    );
                }
            },
            LaunchFeedbackType::CommunityEngagement => {
                // Track engagement metrics
                self.launch_metrics.record_metric(
                    "engagement_feedback_count",
                    self.launch_feedback.iter()
                        .filter(|f| matches!(f.feedback_type, LaunchFeedbackType::CommunityEngagement))
                        .count() as f64,
                    MetricCategory::Engagement
                );
            },
            LaunchFeedbackType::ValidationParticipation => {
                // Track validation participation metrics
                self.launch_metrics.record_metric(
                    "validation_feedback_count",
                    self.launch_feedback.iter()
                        .filter(|f| matches!(f.feedback_type, LaunchFeedbackType::ValidationParticipation))
                        .count() as f64,
                    MetricCategory::Validation
                );
            },
            LaunchFeedbackType::TechnicalExperience => {
                // Track technical performance metrics
                if let Some(rating) = launch_feedback.rating {
                    self.launch_metrics.record_metric(
                        "technical_satisfaction",
                        rating as f64,
                        MetricCategory::Performance
                    );
                }
            },
        }
    }
    
    /// Get feedback statistics for the launch period
    pub fn get_launch_feedback_stats(&self) -> LaunchFeedbackStats {
        let total_feedback = self.launch_feedback.len();
        
        let mut feedback_by_type: HashMap<LaunchFeedbackType, usize> = HashMap::new();
        let mut ratings_sum = 0u32;
        let mut ratings_count = 0u32;
        
        for feedback in &self.launch_feedback {
            *feedback_by_type.entry(feedback.feedback_type).or_insert(0) += 1;
            
            if let Some(rating) = feedback.rating {
                ratings_sum += rating as u32;
                ratings_count += 1;
            }
        }
        
        let average_rating = if ratings_count > 0 {
            Some(ratings_sum as f64 / ratings_count as f64)
        } else {
            None
        };
        
        LaunchFeedbackStats {
            total_feedback,
            feedback_by_type,
            average_rating,
            positive_feedback: self.launch_feedback.iter()
                .filter(|f| f.rating.unwrap_or(3) >= 4)
                .count(),
            negative_feedback: self.launch_feedback.iter()
                .filter(|f| f.rating.unwrap_or(3) <= 2)
                .count(),
        }
    }
    
    /// Get feedback trends over time
    pub fn get_feedback_trends(&self, hours: i64) -> FeedbackTrends {
        let cutoff = Utc::now() - chrono::Duration::hours(hours);
        
        let recent_feedback: Vec<&LaunchFeedback> = self.launch_feedback.iter()
            .filter(|feedback| feedback.submitted_at > cutoff)
            .collect();
        
        let mut hourly_counts: HashMap<i64, usize> = HashMap::new();
        let mut type_counts: HashMap<LaunchFeedbackType, usize> = HashMap::new();
        
        for feedback in recent_feedback {
            // Group by hour
            let hour_bucket = feedback.submitted_at.timestamp() / 3600;
            *hourly_counts.entry(hour_bucket).or_insert(0) += 1;
            
            // Count by type
            *type_counts.entry(feedback.feedback_type).or_insert(0) += 1;
        }
        
        FeedbackTrends {
            hourly_feedback_counts: hourly_counts,
            feedback_type_distribution: type_counts,
        }
    }
    
    /// Generate launch feedback report
    pub fn generate_launch_feedback_report(&self) -> LaunchFeedbackReport {
        let stats = self.get_launch_feedback_stats();
        let trends = self.get_feedback_trends(24); // Last 24 hours
        
        LaunchFeedbackReport {
            generated_at: Utc::now(),
            stats,
            trends,
            total_launch_feedback: self.launch_feedback.len(),
        }
    }
    
    /// Get the underlying feedback collector
    pub fn get_feedback_collector(&self) -> &FeedbackCollector {
        &self.feedback_collector
    }
    
    /// Get the underlying launch metrics
    pub fn get_launch_metrics(&self) -> &LaunchMetrics {
        &self.launch_metrics
    }
}

/// Launch-specific feedback structure
#[derive(Debug, Clone)]
pub struct LaunchFeedback {
    /// Unique identifier for this feedback
    pub id: Uuid,
    
    /// User who provided the feedback
    pub user_id: String,
    
    /// Type of launch feedback
    pub feedback_type: LaunchFeedbackType,
    
    /// Feedback message
    pub message: String,
    
    /// Rating (1-5 stars)
    pub rating: Option<u8>,
    
    /// Additional context or suggestions
    pub context: Option<String>,
    
    /// When the feedback was submitted
    pub submitted_at: DateTime<Utc>,
}

impl LaunchFeedback {
    /// Create from UserFeedback
    pub fn from_user_feedback(user_feedback: UserFeedback) -> Self {
        let feedback_type = match user_feedback.category {
            FeedbackCategory::LaunchExperience => LaunchFeedbackType::DashboardUnderstanding,
            FeedbackCategory::CommunityValidation => LaunchFeedbackType::ValidationParticipation,
            FeedbackCategory::Onboarding => LaunchFeedbackType::DashboardUnderstanding,
            FeedbackCategory::Usability | FeedbackCategory::Performance => {
                LaunchFeedbackType::TechnicalExperience
            },
            FeedbackCategory::General => LaunchFeedbackType::CommunityEngagement,
            _ => LaunchFeedbackType::TechnicalExperience,
        };
        
        Self {
            id: user_feedback.id,
            user_id: user_feedback.user_id,
            feedback_type,
            message: user_feedback.message,
            rating: user_feedback.rating,
            context: None,
            submitted_at: user_feedback.timestamp,
        }
    }
}

/// Types of launch feedback
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LaunchFeedbackType {
    /// Feedback on understanding of interconnected impact
    DashboardUnderstanding,
    
    /// Feedback on community engagement features
    CommunityEngagement,
    
    /// Feedback on validation participation experience
    ValidationParticipation,
    
    /// Feedback on technical experience and performance
    TechnicalExperience,
}

/// Launch feedback statistics
#[derive(Debug, Clone)]
pub struct LaunchFeedbackStats {
    /// Total launch feedback collected
    pub total_feedback: usize,
    
    /// Feedback distribution by type
    pub feedback_by_type: HashMap<LaunchFeedbackType, usize>,
    
    /// Average rating (if any ratings provided)
    pub average_rating: Option<f64>,
    
    /// Positive feedback (4-5 stars)
    pub positive_feedback: usize,
    
    /// Negative feedback (1-2 stars)
    pub negative_feedback: usize,
}

/// Feedback trends analysis
#[derive(Debug, Clone)]
pub struct FeedbackTrends {
    /// Feedback counts by hour
    pub hourly_feedback_counts: HashMap<i64, usize>,
    
    /// Distribution of feedback by type
    pub feedback_type_distribution: HashMap<LaunchFeedbackType, usize>,
}

/// Launch feedback report
#[derive(Debug, Clone)]
pub struct LaunchFeedbackReport {
    /// When the report was generated
    pub generated_at: DateTime<Utc>,
    
    /// Feedback statistics
    pub stats: LaunchFeedbackStats,
    
    /// Feedback trends
    pub trends: FeedbackTrends,
    
    /// Total launch feedback collected
    pub total_launch_feedback: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feedback::UserFeedback;
    
    #[test]
    fn test_collect_launch_feedback() {
        let feedback_collector = FeedbackCollector::new();
        let launch_metrics = LaunchMetrics::new();
        let mut integration = LaunchFeedbackIntegration::new(feedback_collector, launch_metrics);
        
        let user_feedback = UserFeedback {
            id: Uuid::new_v4(),
            user_id: "test_user".to_string(),
            category: FeedbackCategory::LaunchExperience,
            message: "Great introduction to interconnected impact".to_string(),
            rating: Some(5),
            timestamp: Utc::now(),
        };
        
        integration.collect_launch_feedback(user_feedback);
        assert_eq!(integration.launch_feedback.len(), 1);
    }
    
    #[test]
    fn test_feedback_stats() {
        let feedback_collector = FeedbackCollector::new();
        let launch_metrics = LaunchMetrics::new();
        let mut integration = LaunchFeedbackIntegration::new(feedback_collector, launch_metrics);
        
        // Add some feedback
        let feedback1 = UserFeedback {
            id: Uuid::new_v4(),
            user_id: "user1".to_string(),
            category: FeedbackCategory::LaunchExperience,
            message: "Good".to_string(),
            rating: Some(4),
            timestamp: Utc::now(),
        };
        
        let feedback2 = UserFeedback {
            id: Uuid::new_v4(),
            user_id: "user2".to_string(),
            category: FeedbackCategory::CommunityValidation,
            message: "Needs improvement".to_string(),
            rating: Some(2),
            timestamp: Utc::now(),
        };
        
        integration.collect_launch_feedback(feedback1);
        integration.collect_launch_feedback(feedback2);
        
        let stats = integration.get_launch_feedback_stats();
        assert_eq!(stats.total_feedback, 2);
        assert_eq!(stats.positive_feedback, 1);
        assert_eq!(stats.negative_feedback, 1);
    }
    
    #[test]
    fn test_feedback_trends() {
        let feedback_collector = FeedbackCollector::new();
        let launch_metrics = LaunchMetrics::new();
        let mut integration = LaunchFeedbackIntegration::new(feedback_collector, launch_metrics);
        
        let user_feedback = UserFeedback {
            id: Uuid::new_v4(),
            user_id: "test_user".to_string(),
            category: FeedbackCategory::LaunchExperience,
            message: "Test feedback".to_string(),
            rating: Some(5),
            timestamp: Utc::now(),
        };
        
        integration.collect_launch_feedback(user_feedback);
        
        let trends = integration.get_feedback_trends(24);
        assert!(!trends.hourly_feedback_counts.is_empty());
        assert!(!trends.feedback_type_distribution.is_empty());
    }
}