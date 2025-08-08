//! Cause Impact Tracker
//!
//! This module provides the core functionality for tracking engagement with cause visualization
//! components and measuring their effectiveness.

use serde::{Deserialize, Serialize};
use tracing::{debug, info};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use anyhow::Result;
use std::collections::HashMap;

use cause_management::ml::{CauseData, EngagementMetric, OutcomeMeasurement, ResourceAllocationRecord, ImpactMeasurement};
use consent_manager::domain::consent::DataSharingLevel;
use impact_viz::core::{VisualizationType, VisualizationResult};
use volunteer_impact_tracker::tracker::{TaskCompletion, CommunityValidation as VolunteerCommunityValidation};
use learning_impact_tracker::tracker::{CourseCompletionCorrelation, CommunityValidation as LearningCommunityValidation};
use financial_impact_tracker::tracker::{Transaction, ResourceData};

/// Main tracker for cause impact metrics
pub struct CauseImpactTracker {
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
    
    /// Cause engagement correlation data
    pub engagement_correlation: Vec<EngagementCorrelation>,
    
    /// Contribution effectiveness tracking
    pub contribution_effectiveness: Vec<ContributionEffectiveness>,
    
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
    
    /// Cause decision confidence level after using visualization
    pub decision_confidence: Option<f64>, // 0.0 to 1.0
}

/// Cause engagement correlation data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngagementCorrelation {
    /// Unique identifier
    pub id: Uuid,
    
    /// User identifier (hashed for privacy)
    pub user_id: String,
    
    /// Visualization usage history
    pub viz_usage: Vec<String>,
    
    /// Cause engagement status (active contributor)
    pub engaged: bool,
    
    /// Months of continued cause engagement
    pub engagement_months: Option<f64>,
    
    /// Satisfaction rating with cause engagement experience
    pub satisfaction: Option<u8>, // 1-10
    
    /// Contribution amount
    pub contribution_amount: Option<f64>,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Contribution effectiveness tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributionEffectiveness {
    /// Unique identifier
    pub id: Uuid,
    
    /// User identifier (hashed for privacy)
    pub user_id: String,
    
    /// Visualization that influenced contribution decision
    pub influencing_viz: Option<String>,
    
    /// Contribution decision
    pub contribution_decision: String,
    
    /// Contribution quality score (1-10)
    pub quality: Option<u8>,
    
    /// Impact of contribution decision
    pub impact: Option<f64>,
    
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
    
    /// Cause context of validation
    pub cause_context: Option<String>,
    
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
    
    /// How visualization affected cause decisions
    pub decision_impact: Option<String>,
    
    /// Understanding improvement
    pub understanding_improvement: Option<u8>, // 1-10 scale
    
    /// Confidence in cause decisions
    pub confidence_improvement: Option<u8>, // 1-10 scale
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

impl CauseImpactTracker {
    /// Create a new cause impact tracker with specified consent level
    pub fn new(consent_level: DataSharingLevel) -> Self {
        info!("Initializing CauseImpactTracker with consent level: {:?}", consent_level);
        Self {
            consent_level,
            metrics: ImpactMetrics {
                visualization_engagement: HashMap::new(),
                engagement_correlation: Vec::new(),
                contribution_effectiveness: Vec::new(),
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
        decision_confidence: Option<f64>,
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
                decision_confidence,
            };
            
            self.metrics.visualization_engagement.insert(
                component_id.to_string(),
                engagement
            );
        }
        
        Ok(())
    }
    
    /// Track correlation between visualization usage and cause engagement
    pub fn track_engagement_correlation(
        &mut self,
        user_id: &str,
        viz_usage: Vec<String>,
        engaged: bool,
        engagement_months: Option<f64>,
        satisfaction: Option<u8>,
        contribution_amount: Option<f64>,
    ) -> Result<()> {
        debug!("Tracking engagement correlation for user: {}", user_id);
        
        // Only collect data if consent level allows
        if self.consent_allows_data_collection() {
            let correlation = EngagementCorrelation {
                id: Uuid::new_v4(),
                user_id: self.hash_user_id(user_id),
                viz_usage,
                engaged,
                engagement_months,
                satisfaction,
                contribution_amount,
                timestamp: Utc::now(),
            };
            
            self.metrics.engagement_correlation.push(correlation);
        }
        
        Ok(())
    }
    
    /// Track contribution effectiveness
    pub fn track_contribution_effectiveness(
        &mut self,
        user_id: &str,
        influencing_viz: Option<String>,
        contribution_decision: &str,
        quality: Option<u8>,
        impact: Option<f64>,
    ) -> Result<()> {
        debug!("Tracking contribution effectiveness for user: {}", user_id);
        
        // Only collect data if consent level allows
        if self.consent_allows_data_collection() {
            let effectiveness = ContributionEffectiveness {
                id: Uuid::new_v4(),
                user_id: self.hash_user_id(user_id),
                influencing_viz,
                contribution_decision: contribution_decision.to_string(),
                quality,
                impact,
                timestamp: Utc::now(),
            };
            
            self.metrics.contribution_effectiveness.push(effectiveness);
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
        cause_context: Option<String>,
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
                cause_context,
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
        understanding_improvement: Option<u8>,
        confidence_improvement: Option<u8>,
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
                understanding_improvement,
                confidence_improvement,
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
        let tracker = CauseImpactTracker::new(DataSharingLevel::Standard);
        assert_eq!(tracker.consent_level, DataSharingLevel::Standard);
    }
    
    #[test]
    fn test_track_visualization_engagement() {
        let mut tracker = CauseImpactTracker::new(DataSharingLevel::Standard);
        let result = tracker.track_visualization_engagement(
            "user123",
            "cause_viz_1",
            VisualizationType::Comparative,
            120.5,
            15,
            0.85,
            Some(0.9),
        );
        assert!(result.is_ok());
        assert!(!tracker.metrics.visualization_engagement.is_empty());
    }
    
    #[test]
    fn test_consent_respects_none_level() {
        let mut tracker = CauseImpactTracker::new(DataSharingLevel::None);
        let result = tracker.track_visualization_engagement(
            "user123",
            "cause_viz_1",
            VisualizationType::Comparative,
            120.5,
            15,
            0.85,
            Some(0.9),
        );
        assert!(result.is_ok());
        assert!(tracker.metrics.visualization_engagement.is_empty());
    }
    
    #[test]
    fn test_track_engagement_correlation() {
        let mut tracker = CauseImpactTracker::new(DataSharingLevel::Standard);
        let result = tracker.track_engagement_correlation(
            "user123",
            vec!["viz1".to_string(), "viz2".to_string()],
            true,
            Some(6.5),
            Some(8),
            Some(100.50),
        );
        assert!(result.is_ok());
        assert_eq!(tracker.metrics.engagement_correlation.len(), 1);
    }
}