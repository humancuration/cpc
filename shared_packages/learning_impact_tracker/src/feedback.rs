//! Feedback Collection and Processing
//!
//! This module provides functionality for collecting and processing feedback
//! on visualization components from community members.

use serde::{Deserialize, Serialize};
use tracing::{debug, info};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use anyhow::Result;
use std::collections::HashMap;

use crate::tracker::{VisualizationFeedback, ValidationType};
use feedback_core::FeedbackError;

/// Feedback collector for visualization components
pub struct FeedbackCollector {
    /// Collected feedback data
    feedback_data: Vec<VisualizationFeedback>,
    
    /// Quick lookup for feedback by visualization
    feedback_by_viz: HashMap<String, Vec<Uuid>>,
}

/// Quick feedback request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickFeedbackRequest {
    /// User identifier
    pub user_id: String,
    
    /// Visualization identifier
    pub viz_id: String,
    
    /// Was the visualization helpful? (Yes/No)
    pub helpful: bool,
    
    /// Optional comment
    pub comment: Option<String>,
}

/// In-context suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InContextSuggestion {
    /// User identifier
    pub user_id: String,
    
    /// Visualization identifier
    pub viz_id: String,
    
    /// Suggestion content
    pub suggestion: String,
    
    /// Location/context of suggestion
    pub context: String,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Community voting on visualization effectiveness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityVote {
    /// Unique identifier
    pub id: Uuid,
    
    /// User identifier
    pub user_id: String,
    
    /// Visualization identifier
    pub viz_id: String,
    
    /// Vote value (-1, 0, 1)
    pub vote: i8,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Qualitative feedback on visualization impact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualitativeFeedback {
    /// Unique identifier
    pub id: Uuid,
    
    /// User identifier
    pub user_id: String,
    
    /// Visualization identifier
    pub viz_id: String,
    
    /// Feedback content
    pub content: String,
    
    /// How visualization affected learning decisions
    pub learning_impact: String,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Feedback processing result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackProcessingResult {
    /// Visualization identifier
    pub viz_id: String,
    
    /// Overall helpfulness score (0.0 to 1.0)
    pub helpfulness_score: f64,
    
    /// Common themes in feedback
    pub themes: Vec<FeedbackTheme>,
    
    /// Actionable insights
    pub insights: Vec<ActionableInsight>,
    
    /// Improvement suggestions
    pub suggestions: Vec<ImprovementSuggestion>,
}

/// Feedback theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackTheme {
    /// Theme description
    pub theme: String,
    
    /// Frequency of occurrence
    pub frequency: usize,
    
    /// Sentiment score (-1.0 to 1.0)
    pub sentiment: f64,
}

/// Actionable insight from feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionableInsight {
    /// Insight description
    pub description: String,
    
    /// Confidence level (0.0 to 1.0)
    pub confidence: f64,
    
    /// Priority level
    pub priority: PriorityLevel,
}

/// Improvement suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementSuggestion {
    /// Suggestion description
    pub description: String,
    
    /// Estimated impact score (0.0 to 1.0)
    pub impact_score: f64,
    
    /// Implementation difficulty
    pub difficulty: DifficultyLevel,
}

/// Priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PriorityLevel {
    High,
    Medium,
    Low,
}

/// Difficulty levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DifficultyLevel {
    Easy,
    Medium,
    Hard,
}

impl FeedbackCollector {
    /// Create a new feedback collector
    pub fn new() -> Self {
        info!("Initializing FeedbackCollector");
        Self {
            feedback_data: Vec::new(),
            feedback_by_viz: HashMap::new(),
        }
    }
    
    /// Collect quick "Was this helpful?" feedback
    pub fn collect_quick_feedback(&mut self, request: QuickFeedbackRequest) -> Result<(), FeedbackError> {
        debug!("Collecting quick feedback for viz: {}", request.viz_id);
        
        let feedback = VisualizationFeedback {
            id: Uuid::new_v4(),
            user_id: request.user_id,
            viz_id: request.viz_id.clone(),
            rating: if request.helpful { 5 } else { 1 },
            comment: request.comment,
            helpful: request.helpful,
            timestamp: Utc::now(),
        };
        
        self.store_feedback(feedback)?;
        Ok(())
    }
    
    /// Collect in-context suggestion
    pub fn collect_in_context_suggestion(&mut self, suggestion: InContextSuggestion) -> Result<(), FeedbackError> {
        debug!("Collecting in-context suggestion for viz: {}", suggestion.viz_id);
        
        let feedback = VisualizationFeedback {
            id: Uuid::new_v4(),
            user_id: suggestion.user_id,
            viz_id: suggestion.viz_id.clone(),
            rating: 5, // Assume positive for suggestions
            comment: Some(format!("Suggestion [{}]: {}", suggestion.context, suggestion.suggestion)),
            helpful: true,
            timestamp: suggestion.timestamp,
        };
        
        self.store_feedback(feedback)?;
        Ok(())
    }
    
    /// Collect community vote on visualization effectiveness
    pub fn collect_community_vote(&mut self, vote: CommunityVote) -> Result<(), FeedbackError> {
        debug!("Collecting community vote for viz: {}", vote.viz_id);
        
        let rating = match vote.vote {
            -1 => 1,
            0 => 3,
            1 => 5,
            _ => 3, // Default to neutral
        };
        
        let feedback = VisualizationFeedback {
            id: vote.id,
            user_id: vote.user_id,
            viz_id: vote.viz_id.clone(),
            rating,
            comment: Some(format!("Community vote: {}", vote.vote)),
            helpful: vote.vote > 0,
            timestamp: vote.timestamp,
        };
        
        self.store_feedback(feedback)?;
        Ok(())
    }
    
    /// Collect qualitative feedback on visualization impact
    pub fn collect_qualitative_feedback(&mut self, feedback: QualitativeFeedback) -> Result<(), FeedbackError> {
        debug!("Collecting qualitative feedback for viz: {}", feedback.viz_id);
        
        let viz_feedback = VisualizationFeedback {
            id: feedback.id,
            user_id: feedback.user_id,
            viz_id: feedback.viz_id.clone(),
            rating: 5, // Assume positive for detailed feedback
            comment: Some(format!("Impact: {}\n{}", feedback.learning_impact, feedback.content)),
            helpful: true,
            timestamp: feedback.timestamp,
        };
        
        self.store_feedback(viz_feedback)?;
        Ok(())
    }
    
    /// Store feedback and update indexes
    fn store_feedback(&mut self, feedback: VisualizationFeedback) -> Result<(), FeedbackError> {
        let feedback_id = feedback.id;
        let viz_id = feedback.viz_id.clone();
        
        self.feedback_data.push(feedback);
        self.feedback_by_viz.entry(viz_id).or_insert_with(Vec::new).push(feedback_id);
        
        Ok(())
    }
    
    /// Process feedback for a specific visualization
    pub fn process_feedback_for_viz(&self, viz_id: &str) -> Result<FeedbackProcessingResult, FeedbackError> {
        debug!("Processing feedback for viz: {}", viz_id);
        
        let feedback_ids = self.feedback_by_viz.get(viz_id).unwrap_or(&Vec::new());
        let viz_feedback: Vec<&VisualizationFeedback> = feedback_ids
            .iter()
            .filter_map(|id| self.feedback_data.iter().find(|f| f.id == *id))
            .collect();
        
        if viz_feedback.is_empty() {
            return Ok(FeedbackProcessingResult {
                viz_id: viz_id.to_string(),
                helpfulness_score: 0.0,
                themes: Vec::new(),
                insights: Vec::new(),
                suggestions: Vec::new(),
            });
        }
        
        // Calculate helpfulness score
        let helpful_count = viz_feedback.iter().filter(|f| f.helpful).count();
        let helpfulness_score = helpful_count as f64 / viz_feedback.len() as f64;
        
        // Extract themes (simplified implementation)
        let themes = self.extract_themes(&viz_feedback);
        
        // Generate insights (simplified implementation)
        let insights = self.generate_insights(&viz_feedback);
        
        // Generate suggestions (simplified implementation)
        let suggestions = self.generate_suggestions(&viz_feedback);
        
        Ok(FeedbackProcessingResult {
            viz_id: viz_id.to_string(),
            helpfulness_score,
            themes,
            insights,
            suggestions,
        })
    }
    
    /// Extract themes from feedback
    fn extract_themes(&self, feedback: &[&VisualizationFeedback]) -> Vec<FeedbackTheme> {
        let mut themes: HashMap<String, (usize, f64)> = HashMap::new();
        
        for fb in feedback {
            if let Some(comment) = &fb.comment {
                // Simple keyword-based theme extraction (would be more sophisticated in real implementation)
                let words: Vec<&str> = comment.split_whitespace().collect();
                for word in words {
                    let word = word.to_lowercase();
                    if word.len() > 3 { // Only consider words longer than 3 characters
                        let entry = themes.entry(word).or_insert((0, 0.0));
                        entry.0 += 1; // Increment frequency
                        entry.1 += if fb.helpful { 0.1 } else { -0.1 }; // Adjust sentiment
                    }
                }
            }
        }
        
        themes.into_iter()
            .map(|(theme, (frequency, sentiment_sum))| FeedbackTheme {
                theme,
                frequency,
                sentiment: if frequency > 0 { sentiment_sum / frequency as f64 } else { 0.0 },
            })
            .take(5) // Top 5 themes
            .collect()
    }
    
    /// Generate insights from feedback
    fn generate_insights(&self, feedback: &[&VisualizationFeedback]) -> Vec<ActionableInsight> {
        let mut insights = Vec::new();
        
        let helpful_percentage = feedback.iter().filter(|f| f.helpful).count() as f64 / feedback.len() as f64;
        
        if helpful_percentage < 0.5 {
            insights.push(ActionableInsight {
                description: "Visualization is not helpful to majority of users".to_string(),
                confidence: 0.8,
                priority: PriorityLevel::High,
            });
        } else if helpful_percentage < 0.7 {
            insights.push(ActionableInsight {
                description: "Visualization could be more helpful to users".to_string(),
                confidence: 0.7,
                priority: PriorityLevel::Medium,
            });
        }
        
        insights
    }
    
    /// Generate improvement suggestions from feedback
    fn generate_suggestions(&self, feedback: &[&VisualizationFeedback]) -> Vec<ImprovementSuggestion> {
        let mut suggestions = Vec::new();
        
        let helpful_percentage = feedback.iter().filter(|f| f.helpful).count() as f64 / feedback.len() as f64;
        
        if helpful_percentage < 0.7 {
            suggestions.push(ImprovementSuggestion {
                description: "Consider simplifying visualization design".to_string(),
                impact_score: 0.7,
                difficulty: DifficultyLevel::Medium,
            });
            
            suggestions.push(ImprovementSuggestion {
                description: "Add more interactive elements to engage users".to_string(),
                impact_score: 0.6,
                difficulty: DifficultyLevel::Hard,
            });
        }
        
        suggestions
    }
    
    /// Get all feedback for a specific visualization
    pub fn get_feedback_for_viz(&self, viz_id: &str) -> Vec<&VisualizationFeedback> {
        let feedback_ids = self.feedback_by_viz.get(viz_id).unwrap_or(&Vec::new());
        feedback_ids
            .iter()
            .filter_map(|id| self.feedback_data.iter().find(|f| f.id == *id))
            .collect()
    }
}

impl Default for FeedbackCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_feedback_collector_creation() {
        let collector = FeedbackCollector::new();
        assert!(collector.feedback_data.is_empty());
        assert!(collector.feedback_by_viz.is_empty());
    }
    
    #[test]
    fn test_collect_quick_feedback() {
        let mut collector = FeedbackCollector::new();
        let request = QuickFeedbackRequest {
            user_id: "user123".to_string(),
            viz_id: "skill_viz_1".to_string(),
            helpful: true,
            comment: Some("Very helpful!".to_string()),
        };
        
        let result = collector.collect_quick_feedback(request);
        assert!(result.is_ok());
        assert_eq!(collector.feedback_data.len(), 1);
        assert!(collector.feedback_by_viz.contains_key("skill_viz_1"));
    }
    
    #[test]
    fn test_process_feedback_for_viz() {
        let mut collector = FeedbackCollector::new();
        
        // Add some feedback
        let request = QuickFeedbackRequest {
            user_id: "user123".to_string(),
            viz_id: "skill_viz_1".to_string(),
            helpful: true,
            comment: Some("Very helpful!".to_string()),
        };
        
        let _ = collector.collect_quick_feedback(request);
        
        let result = collector.process_feedback_for_viz("skill_viz_1");
        assert!(result.is_ok());
        
        let processing_result = result.unwrap();
        assert_eq!(processing_result.viz_id, "skill_viz_1");
        assert_eq!(processing_result.helpfulness_score, 1.0);
    }
}