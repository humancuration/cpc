//! Learning Impact Tracker
//!
//! This module provides the core functionality for tracking engagement with visualization
//! components and measuring their effectiveness.

use serde::{Deserialize, Serialize};
use tracing::{debug, info};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use anyhow::Result;
use std::collections::HashMap;

use learning_core::domain::{course::Course, enrollment::Enrollment};
use skill_development::ml::{LearnerProfile, CommunityData};
use volunteer_coordination::ml::{VolunteerProfile, VolunteerActivity};
use consent_manager::domain::consent::DataSharingLevel;
use impact_viz::core::{VisualizationType, VisualizationResult};

/// Main tracker for learning impact metrics
pub struct LearningImpactTracker {
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
    
    /// Course completion correlation data
    pub completion_correlation: Vec<CourseCompletionCorrelation>,
    
    /// Learning to volunteer transition tracking
    pub volunteer_transitions: Vec<VolunteerTransition>,
    
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

/// Course completion correlation data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseCompletionCorrelation {
    /// Unique identifier
    pub id: Uuid,
    
    /// Course identifier
    pub course_id: String,
    
    /// User identifier (hashed for privacy)
    pub user_id: String,
    
    /// Visualization usage during course
    pub viz_usage: Vec<String>,
    
    /// Course completion status
    pub completed: bool,
    
    /// Time to completion
    pub time_to_completion: Option<f64>, // in hours
    
    /// Satisfaction rating
    pub satisfaction: Option<u8>, // 1-10
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Volunteer transition tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolunteerTransition {
    /// Unique identifier
    pub id: Uuid,
    
    /// User identifier (hashed for privacy)
    pub user_id: String,
    
    /// Course that led to transition
    pub course_id: String,
    
    /// Visualization that influenced decision
    pub influencing_viz: Option<String>,
    
    /// Volunteer activity
    pub volunteer_activity: String,
    
    /// Time from course completion to volunteer start
    pub transition_time: f64, // in days
    
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
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl LearningImpactTracker {
    /// Create a new learning impact tracker with specified consent level
    pub fn new(consent_level: DataSharingLevel) -> Self {
        info!("Initializing LearningImpactTracker with consent level: {:?}", consent_level);
        Self {
            consent_level,
            metrics: ImpactMetrics {
                visualization_engagement: HashMap::new(),
                completion_correlation: Vec::new(),
                volunteer_transitions: Vec::new(),
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
    
    /// Track correlation between visualization usage and course completion
    pub fn track_course_completion_correlation(
        &mut self,
        user_id: &str,
        course_id: &str,
        viz_usage: Vec<String>,
        completed: bool,
        time_to_completion: Option<f64>,
        satisfaction: Option<u8>,
    ) -> Result<()> {
        debug!("Tracking course completion correlation for user: {}, course: {}", user_id, course_id);
        
        // Only collect data if consent level allows
        if self.consent_allows_data_collection() {
            let correlation = CourseCompletionCorrelation {
                id: Uuid::new_v4(),
                course_id: course_id.to_string(),
                user_id: self.hash_user_id(user_id),
                viz_usage,
                completed,
                time_to_completion,
                satisfaction,
                timestamp: Utc::now(),
            };
            
            self.metrics.completion_correlation.push(correlation);
        }
        
        Ok(())
    }
    
    /// Track transition from learning to volunteer activities
    pub fn track_volunteer_transition(
        &mut self,
        user_id: &str,
        course_id: &str,
        influencing_viz: Option<String>,
        volunteer_activity: &str,
        transition_time: f64,
    ) -> Result<()> {
        debug!("Tracking volunteer transition for user: {}, course: {}", user_id, course_id);
        
        // Only collect data if consent level allows
        if self.consent_allows_data_collection() {
            let transition = VolunteerTransition {
                id: Uuid::new_v4(),
                user_id: self.hash_user_id(user_id),
                course_id: course_id.to_string(),
                influencing_viz,
                volunteer_activity: volunteer_activity.to_string(),
                transition_time,
                timestamp: Utc::now(),
            };
            
            self.metrics.volunteer_transitions.push(transition);
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
        let tracker = LearningImpactTracker::new(DataSharingLevel::Standard);
        assert_eq!(tracker.consent_level, DataSharingLevel::Standard);
    }
    
    #[test]
    fn test_track_visualization_engagement() {
        let mut tracker = LearningImpactTracker::new(DataSharingLevel::Standard);
        let result = tracker.track_visualization_engagement(
            "user123",
            "skill_viz_1",
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
        let mut tracker = LearningImpactTracker::new(DataSharingLevel::None);
        let result = tracker.track_visualization_engagement(
            "user123",
            "skill_viz_1",
            VisualizationType::Comparative,
            120.5,
            15,
            0.85,
        );
        assert!(result.is_ok());
        assert!(tracker.metrics.visualization_engagement.is_empty());
    }
}