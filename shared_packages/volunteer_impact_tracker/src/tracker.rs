//! Volunteer Impact Tracker
//!
//! This module provides the core functionality for tracking engagement with volunteer visualization
//! components and measuring their effectiveness.

use serde::{Deserialize, Serialize};
use tracing::{debug, info};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use anyhow::Result;
use std::collections::HashMap;

use volunteer_coordination::ml::{VolunteerEngagementData, VolunteerProfile, VolunteerTask, VolunteerActivity};
use learning_impact_tracker::tracker::{CourseCompletionCorrelation, CommunityValidation as LearningCommunityValidation};
use consent_manager::domain::consent::DataSharingLevel;
use impact_viz::core::{VisualizationType, VisualizationResult};
use skill_development::ml::CommunityData;

/// Main tracker for volunteer impact metrics
pub struct VolunteerImpactTracker {
    /// Consent-based data collection
    consent_level: DataSharingLevel,
    
    /// Collected metrics
    metrics: ImpactMetrics,
}

/// Comprehensive impact metrics collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactMetrics {
    /// Visualization engagement metrics
    pub visualization_engagement: HashMap<String, VisualizationEngagement>,
    
    /// Volunteer retention correlation data
    pub retention_correlation: Vec<RetentionCorrelation>,
    
    /// Task completion rate and quality tracking
    pub task_completion: Vec<TaskCompletion>,
    
    /// Community validation interactions
    pub community_validation: Vec<CommunityValidation>,
    
    /// Feedback collection data
    pub feedback_data: Vec<VisualizationFeedback>,
}

/// Visualization engagement tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationEngagement {
    /// Unique identifier for the engagement record
    pub id: Uuid,
    
    /// Type of visualization
    pub viz_type: VisualizationType,
    
    /// Component identifier
    pub component_id: String,
    
    /// User identifier (hashed for privacy)
    pub user_id: String,
    
    /// Time spent interacting with visualization
    pub interaction_time: f64, // in seconds
    
    /// Number of interactions
    pub interaction_count: u32,
    
    /// Timestamp of engagement
    pub timestamp: DateTime<Utc>,
    
    /// Engagement quality score
    pub quality_score: f64, // 0.0 to 1.0
}

/// Volunteer retention correlation data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionCorrelation {
    /// Unique identifier
    pub id: Uuid,
    
    /// User identifier (hashed for privacy)
    pub user_id: String,
    
    /// Visualization usage history
    pub viz_usage: Vec<String>,
    
    /// Retention status (still active volunteer)
    pub retained: bool,
    
    /// Months of continued volunteering
    pub retention_months: Option<f64>,
    
    /// Satisfaction rating with volunteer experience
    pub satisfaction: Option<u8>, // 1-10
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Task completion tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCompletion {
    /// Unique identifier
    pub id: Uuid,
    
    /// User identifier (hashed for privacy)
    pub user_id: String,
    
    /// Task identifier
    pub task_id: String,
    
    /// Visualization that influenced task selection
    pub influencing_viz: Option<String>,
    
    /// Task completion status
    pub completed: bool,
    
    /// Quality of completion (1-10)
    pub quality: Option<u8>,
    
    /// Time to completion
    pub time_to_completion: Option<f64>, // in hours
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Community validation interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityValidation {
    /// Unique identifier
    pub id: Uuid,
    
    /// User identifier (hashed for privacy)
    pub user_id: String,
    
    /// Visualization involved
    pub viz_id: String,
    
    /// Validation type (endorsement, critique, suggestion)
    pub validation_type: ValidationType,
    
    /// Content of validation
    pub content: String,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Types of community validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationType {
    Endorsement,
    Critique,
    Suggestion,
    Question,
}

/// Visualization feedback data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationFeedback {
    /// Unique identifier
    pub id: Uuid,
    
    /// User identifier (hashed for privacy)
    pub user_id: String,
    
    /// Visualization identifier
    pub viz_id: String,
    
    /// Feedback rating (1-5 stars)
    pub rating: u8,
    
    /// Feedback comment
    pub comment: Option<String>,
    
    /// Whether visualization was helpful
    pub helpful: bool,
    
    /// How visualization affected volunteer decisions
    pub decision_impact: Option<String>,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl VolunteerImpactTracker {
    /// Create a new volunteer impact tracker with specified consent level
    pub fn new(consent_level: DataSharingLevel) -> Self {
        info!("Initializing VolunteerImpactTracker with consent level: {:?}", consent_level);
        Self {
            consent_level,
            metrics: ImpactMetrics {
                visualization_engagement: HashMap::new(),
                retention_correlation: Vec::new(),
                task_completion: Vec::new(),
                community_validation: Vec::new(),
                feedback_data: Vec::new(),
            },
        }
    }
    
    /// Track engagement with a visualization component
    pub fn track_visualization_engagement(
        &mut self,
        user_id: &str,
        component_id: &str,
        viz_type: VisualizationType,
        interaction_time: f64,
        interaction_count: u32,
        quality_score: f64,
    ) -> Result<()> {
        debug!("Tracking visualization engagement for user: {}, component: {}", user_id, component_id);
        
        // Only collect data if consent level allows
        if self.consent_allows_data_collection() {
            let engagement = VisualizationEngagement {
                id: Uuid::new_v4(),
                viz_type,
                component_id: component_id.to_string(),
                user_id: self.hash_user_id(user_id),
                interaction_time,
                interaction_count,
                timestamp: Utc::now(),
                quality_score,
            };
            
            self.metrics.visualization_engagement.insert(
                component_id.to_string(),
                engagement
            );
        }
        
        Ok(())
    }
    
    /// Track correlation between visualization usage and volunteer retention
    pub fn track_retention_correlation(
        &mut self,
        user_id: &str,
        viz_usage: Vec<String>,
        retained: bool,
        retention_months: Option<f64>,
        satisfaction: Option<u8>,
    ) -> Result<()> {
        debug!("Tracking retention correlation for user: {}", user_id);
        
        // Only collect data if consent level allows
        if self.consent_allows_data_collection() {
            let correlation = RetentionCorrelation {
                id: Uuid::new_v4(),
                user_id: self.hash_user_id(user_id),
                viz_usage,
                retained,
                retention_months,
                satisfaction,
                timestamp: Utc::now(),
            };
            
            self.metrics.retention_correlation.push(correlation);
        }
        
        Ok(())
    }
    
    /// Track task completion rates and quality
    pub fn track_task_completion(
        &mut self,
        user_id: &str,
        task_id: &str,
        influencing_viz: Option<String>,
        completed: bool,
        quality: Option<u8>,
        time_to_completion: Option<f64>,
    ) -> Result<()> {
        debug!("Tracking task completion for user: {}, task: {}", user_id, task_id);
        
        // Only collect data if consent level allows
        if self.consent_allows_data_collection() {
            let completion = TaskCompletion {
                id: Uuid::new_v4(),
                user_id: self.hash_user_id(user_id),
                task_id: task_id.to_string(),
                influencing_viz,
                completed,
                quality,
                time_to_completion,
                timestamp: Utc::now(),
            };
            
            self.metrics.task_completion.push(completion);
        }
        
        Ok(())
    }
    
    /// Record community validation interaction
    pub fn record_community_validation(
        &mut self,
        user_id: &str,
        viz_id: &str,
        validation_type: ValidationType,
        content: &str,
    ) -> Result<()> {
        debug!("Recording community validation for user: {}, viz: {}", user_id, viz_id);
        
        // Only collect data if consent level allows
        if self.consent_allows_data_collection() {
            let validation = CommunityValidation {
                id: Uuid::new_v4(),
                user_id: self.hash_user_id(user_id),
                viz_id: viz_id.to_string(),
                validation_type,
                content: content.to_string(),
                timestamp: Utc::now(),
            };
            
            self.metrics.community_validation.push(validation);
        }
        
        Ok(())
    }
    
    /// Record visualization feedback
    pub fn record_feedback(
        &mut self,
        user_id: &str,
        viz_id: &str,
        rating: u8,
        comment: Option<String>,
        helpful: bool,
        decision_impact: Option<String>,
    ) -> Result<()> {
        debug!("Recording feedback for user: {}, viz: {}", user_id, viz_id);
        
        // Only collect data if consent level allows
        if self.consent_allows_data_collection() {
            let feedback = VisualizationFeedback {
                id: Uuid::new_v4(),
                user_id: self.hash_user_id(user_id),
                viz_id: viz_id.to_string(),
                rating,
                comment,
                helpful,
                decision_impact,
                timestamp: Utc::now(),
            };
            
            self.metrics.feedback_data.push(feedback);
        }
        
        Ok(())
    }
    
    /// Get current impact metrics
    pub fn get_metrics(&self) -> &ImpactMetrics {
        &self.metrics
    }
    
    /// Check if consent level allows data collection
    fn consent_allows_data_collection(&self) -> bool {
        match self.consent_level {
            DataSharingLevel::None => false,
            DataSharingLevel::Minimal => true,
            DataSharingLevel::Standard => true,
            DataSharingLevel::Enhanced => true,
        }
    }
    
    /// Hash user ID for privacy preservation
    fn hash_user_id(&self, user_id: &str) -> String {
        // In a real implementation, this would use a proper hashing function
        // For now, we'll just prefix with "hashed_" to indicate the transformation
        format!("hashed_{}", user_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use consent_manager::domain::consent::DataSharingLevel;
    use impact_viz::core::VisualizationType;
    
    #[test]
    fn test_tracker_creation() {
        let tracker = VolunteerImpactTracker::new(DataSharingLevel::Standard);
        assert_eq!(tracker.consent_level, DataSharingLevel::Standard);
    }
    
    #[test]
    fn test_track_visualization_engagement() {
        let mut tracker = VolunteerImpactTracker::new(DataSharingLevel::Standard);
        let result = tracker.track_visualization_engagement(
            "user123",
            "volunteer_viz_1",
            VisualizationType::Comparative,
            120.5,
            15,
            0.85,
        );
        assert!(result.is_ok());
        assert!(!tracker.metrics.visualization_engagement.is_empty());
    }
    
    #[test]
    fn test_consent_respects_none_level() {
        let mut tracker = VolunteerImpactTracker::new(DataSharingLevel::None);
        let result = tracker.track_visualization_engagement(
            "user123",
            "volunteer_viz_1",
            VisualizationType::Comparative,
            120.5,
            15,
            0.85,
        );
        assert!(result.is_ok());
        assert!(tracker.metrics.visualization_engagement.is_empty());
    }
}