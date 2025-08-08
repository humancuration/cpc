//! Feedback Collection and Processing
//!
//! This module provides functionality for collecting and processing feedback
//! on financial impact visualizations.

use serde::{Deserialize, Serialize};
use tracing::{debug, info};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use anyhow::Result;
use std::collections::HashMap;

use crate::tracker::{VisualizationFeedback, ValidationType};
use feedback_core::domain::feedback::{Feedback, FeedbackType, FeedbackPriority};
use consent_manager::domain::consent::DataSharingLevel;

/// Feedback collector for financial impact visualizations
pub struct FeedbackCollector {
    /// Consent-based data collection
    consent_level: DataSharingLevel,
    
    /// Collected feedback
    feedback: Vec<VisualizationFeedback>,
}

/// Feedback processing service
pub struct FeedbackProcessor {
    /// Feedback data to process
    feedback_data: Vec<VisualizationFeedback>,
}

/// Processed feedback insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackInsights {
    /// Overall helpfulness percentage
    pub helpfulness_percentage: f64,
    
    /// Average rating
    pub avg_rating: f64,
    
    /// Common themes in feedback
    pub common_themes: Vec<FeedbackTheme>,
    
    /// Sentiment analysis
    pub sentiment: SentimentAnalysis,
    
    /// Improvement suggestions
    pub suggestions: Vec<ImprovementSuggestion>,
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

/// Sentiment analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentAnalysis {
    /// Overall sentiment score (-1.0 to 1.0)
    pub overall_sentiment: f64,
    
    /// Positive feedback count
    pub positive_count: u64,
    
    /// Neutral feedback count
    pub neutral_count: u64,
    
    /// Negative feedback count
    pub negative_count: u64,
}

/// Improvement suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementSuggestion {
    /// Suggestion description
    pub description: String,
    
    /// Priority level
    pub priority: FeedbackPriority,
    
    /// Affected visualization components
    pub affected_components: Vec<String>,
    
    /// Supporting feedback count
    pub supporting_feedback: u64,
}

impl FeedbackCollector {
    /// Create a new feedback collector with specified consent level
    pub fn new(consent_level: DataSharingLevel) -> Self {
        info!("Initializing FeedbackCollector with consent level: {:?}", consent_level);
        Self {
            consent_level,
            feedback: Vec::new(),
        }
    }
    
    /// Collect quick feedback on visualization helpfulness
    pub fn collect_quick_feedback(
        &mut self,
        user_id: &str,
        viz_id: &str,
        helpful: bool,
    ) -> Result<()> {
        debug!("Collecting quick feedback for user: {}, viz: {}", user_id, viz_id);
        
        // Only collect data if consent level allows
        if self.consent_allows_data_collection() {
            let feedback = VisualizationFeedback {
                id: Uuid::new_v4(),
                user_id: self.hash_user_id(user_id),
                viz_id: viz_id.to_string(),
                rating: if helpful { 5 } else { 1 }, // Simple 5-star or 1-star rating
                comment: None,
                helpful,
                decision_impact: None,
                understanding_improvement: None,
                confidence_improvement: None,
                timestamp: Utc::now(),
            };
            
            self.feedback.push(feedback);
        }
        
        Ok(())
    }
    
    /// Collect detailed feedback
    pub fn collect_detailed_feedback(
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
        debug!("Collecting detailed feedback for user: {}, viz: {}", user_id, viz_id);
        
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
            
            self.feedback.push(feedback);
        }
        
        Ok(())
    }
    
    /// Collect in-context suggestions
    pub fn collect_suggestion(
        &mut self,
        user_id: &str,
        viz_id: &str,
        suggestion: &str,
    ) -> Result<()> {
        debug!("Collecting suggestion for user: {}, viz: {}", user_id, viz_id);
        
        // Only collect data if consent level allows
        if self.consent_allows_data_collection() {
            let feedback = VisualizationFeedback {
                id: Uuid::new_v4(),
                user_id: self.hash_user_id(user_id),
                viz_id: viz_id.to_string(),
                rating: 3, // Neutral rating for suggestions
                comment: Some(suggestion.to_string()),
                helpful: true, // Suggestions are implicitly helpful
                decision_impact: None,
                understanding_improvement: None,
                confidence_improvement: None,
                timestamp: Utc::now(),
            };
            
            self.feedback.push(feedback);
        }
        
        Ok(())
    }
    
    /// Collect community voting on visualization effectiveness
    pub fn collect_community_vote(
        &mut self,
        user_id: &str,
        viz_id: &str,
        vote: i8, // -1, 0, or 1
    ) -> Result<()> {
        debug!("Collecting community vote for user: {}, viz: {}", user_id, viz_id);
        
        // Only collect data if consent level allows
        if self.consent_allows_data_collection() {
            let rating = match vote {
                -1 => 1,
                0 => 3,
                1 => 5,
                _ => 3, // Default to neutral for invalid votes
            };
            
            let feedback = VisualizationFeedback {
                id: Uuid::new_v4(),
                user_id: self.hash_user_id(user_id),
                viz_id: viz_id.to_string(),
                rating,
                comment: if vote == 0 { Some("Neutral vote".to_string()) } else { None },
                helpful: vote >= 0,
                decision_impact: None,
                understanding_improvement: None,
                confidence_improvement: None,
                timestamp: Utc::now(),
            };
            
            self.feedback.push(feedback);
        }
        
        Ok(())
    }
    
    /// Get collected feedback
    pub fn get_feedback(&self) -> &Vec<VisualizationFeedback> {
        &self.feedback
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

impl FeedbackProcessor {
    /// Create a new feedback processor
    pub fn new(feedback_data: Vec<VisualizationFeedback>) -> Self {
        info!("Initializing FeedbackProcessor");
        Self { feedback_data }
    }
    
    /// Process feedback and generate insights
    pub fn process_feedback(&self) -> FeedbackInsights {
        debug!("Processing feedback data");
        
        let total_feedback = self.feedback_data.len() as f64;
        
        // Calculate helpfulness percentage
        let helpful_count = self.feedback_data.iter()
            .filter(|feedback| feedback.helpful)
            .count() as f64;
        
        let helpfulness_percentage = if total_feedback > 0.0 {
            helpful_count / total_feedback * 100.0
        } else {
            0.0
        };
        
        // Calculate average rating
        let avg_rating = if !self.feedback_data.is_empty() {
            self.feedback_data.iter()
                .map(|feedback| feedback.rating as f64)
                .sum::<f64>() / total_feedback
        } else {
            0.0
        };
        
        // Extract themes from comments (simplified implementation)
        let common_themes = self.extract_common_themes();
        
        // Perform sentiment analysis (simplified implementation)
        let sentiment = self.analyze_sentiment();
        
        // Generate improvement suggestions
        let suggestions = self.generate_suggestions();
        
        FeedbackInsights {
            helpfulness_percentage,
            avg_rating,
            common_themes,
            sentiment,
            suggestions,
        }
    }
    
    /// Extract common themes from feedback comments
    fn extract_common_themes(&self) -> Vec<FeedbackTheme> {
        debug!("Extracting common themes from feedback");
        
        let mut themes: HashMap<String, (u64, f64)> = HashMap::new();
        
        // Simple keyword-based theme extraction
        let positive_keywords = ["good", "great", "helpful", "useful", "clear", "understandable"];
        let negative_keywords = ["confusing", "difficult", "unclear", "complex", "hard"];
        let improvement_keywords = ["improve", "better", "enhance", "add", "include", "more"];
        
        for feedback in &self.feedback_data {
            if let Some(comment) = &feedback.comment {
                let comment_lower = comment.to_lowercase();
                
                // Check for positive themes
                for keyword in &positive_keywords {
                    if comment_lower.contains(keyword) {
                        let entry = themes.entry("Positive Experience".to_string()).or_insert((0, 0.0));
                        entry.0 += 1;
                        entry.1 += 0.5; // Positive sentiment
                    }
                }
                
                // Check for negative themes
                for keyword in &negative_keywords {
                    if comment_lower.contains(keyword) {
                        let entry = themes.entry("Negative Experience".to_string()).or_insert((0, 0.0));
                        entry.0 += 1;
                        entry.1 -= 0.5; // Negative sentiment
                    }
                }
                
                // Check for improvement suggestions
                for keyword in &improvement_keywords {
                    if comment_lower.contains(keyword) {
                        let entry = themes.entry("Improvement Suggestion".to_string()).or_insert((0, 0.0));
                        entry.0 += 1;
                        entry.1 += 0.0; // Neutral sentiment for suggestions
                    }
                }
            }
        }
        
        themes.into_iter()
            .map(|(theme, (frequency, sentiment_sum))| {
                let avg_sentiment = if frequency > 0 {
                    sentiment_sum / frequency as f64
                } else {
                    0.0
                };
                
                FeedbackTheme {
                    theme,
                    frequency,
                    sentiment: avg_sentiment,
                }
            })
            .collect()
    }
    
    /// Analyze sentiment of feedback
    fn analyze_sentiment(&self) -> SentimentAnalysis {
        debug!("Analyzing sentiment of feedback");
        
        let mut positive_count = 0u64;
        let mut neutral_count = 0u64;
        let mut negative_count = 0u64;
        
        for feedback in &self.feedback_data {
            match feedback.rating {
                4..=5 => positive_count += 1,
                2..=3 => neutral_count += 1,
                1 => negative_count += 1,
                _ => neutral_count += 1, // For any unexpected ratings
            }
        }
        
        let total = positive_count + neutral_count + negative_count;
        let overall_sentiment = if total > 0 {
            ((positive_count as f64 - negative_count as f64) / total as f64)
        } else {
            0.0
        };
        
        SentimentAnalysis {
            overall_sentiment,
            positive_count,
            neutral_count,
            negative_count,
        }
    }
    
    /// Generate improvement suggestions based on feedback
    fn generate_suggestions(&self) -> Vec<ImprovementSuggestion> {
        debug!("Generating improvement suggestions");
        
        let mut suggestions: HashMap<String, (FeedbackPriority, Vec<String>, u64)> = HashMap::new();
        
        // Generate suggestions based on low ratings
        for feedback in &self.feedback_data {
            if feedback.rating <= 2 {
                let suggestion_key = "Improve visualization clarity and usability";
                let entry = suggestions.entry(suggestion_key.to_string()).or_insert((
                    FeedbackPriority::High,
                    Vec::new(),
                    0
                ));
                entry.1.push(feedback.viz_id.clone());
                entry.2 += 1;
            }
            
            if let Some(comment) = &feedback.comment {
                if comment.to_lowercase().contains("confusing") || 
                   comment.to_lowercase().contains("unclear") {
                    let suggestion_key = "Simplify complex visual elements";
                    let entry = suggestions.entry(suggestion_key.to_string()).or_insert((
                        FeedbackPriority::High,
                        Vec::new(),
                        0
                    ));
                    entry.1.push(feedback.viz_id.clone());
                    entry.2 += 1;
                }
                
                if comment.to_lowercase().contains("more") || 
                   comment.to_lowercase().contains("add") {
                    let suggestion_key = "Add requested features or information";
                    let entry = suggestions.entry(suggestion_key.to_string()).or_insert((
                        FeedbackPriority::Medium,
                        Vec::new(),
                        0
                    ));
                    entry.1.push(feedback.viz_id.clone());
                    entry.2 += 1;
                }
            }
        }
        
        suggestions.into_iter()
            .map(|(description, (priority, components, supporting_feedback))| {
                // Deduplicate components
                let unique_components: Vec<String> = components.into_iter().collect::<std::collections::HashSet<_>>().into_iter().collect();
                
                ImprovementSuggestion {
                    description,
                    priority,
                    affected_components: unique_components,
                    supporting_feedback,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use consent_manager::domain::consent::DataSharingLevel;
    
    #[test]
    fn test_feedback_collector_creation() {
        let collector = FeedbackCollector::new(DataSharingLevel::Standard);
        assert_eq!(collector.consent_level, DataSharingLevel::Standard);
    }
    
    #[test]
    fn test_collect_quick_feedback() {
        let mut collector = FeedbackCollector::new(DataSharingLevel::Standard);
        let result = collector.collect_quick_feedback(
            "user123",
            "financial_viz_1",
            true,
        );
        assert!(result.is_ok());
        assert_eq!(collector.feedback.len(), 1);
        assert!(collector.feedback[0].helpful);
        assert_eq!(collector.feedback[0].rating, 5);
    }
    
    #[test]
    fn test_collect_detailed_feedback() {
        let mut collector = FeedbackCollector::new(DataSharingLevel::Standard);
        let result = collector.collect_detailed_feedback(
            "user123",
            "financial_viz_1",
            4,
            Some("Very helpful visualization".to_string()),
            true,
            Some("Helped me understand my budget".to_string()),
            Some(8),
            Some(9),
        );
        assert!(result.is_ok());
        assert_eq!(collector.feedback.len(), 1);
        assert_eq!(collector.feedback[0].rating, 4);
        assert!(collector.feedback[0].comment.is_some());
        assert_eq!(collector.feedback[0].comment.as_ref().unwrap(), "Very helpful visualization");
    }
    
    #[test]
    fn test_feedback_processor() {
        let feedback_data = vec![VisualizationFeedback {
            id: Uuid::new_v4(),
            user_id: "hashed_user123".to_string(),
            viz_id: "financial_viz_1".to_string(),
            rating: 5,
            comment: Some("Great visualization!".to_string()),
            helpful: true,
            decision_impact: Some("Helped me make better financial decisions".to_string()),
            understanding_improvement: Some(9),
            confidence_improvement: Some(8),
            timestamp: Utc::now(),
        }];
        
        let processor = FeedbackProcessor::new(feedback_data);
        let insights = processor.process_feedback();
        
        assert_eq!(insights.helpfulness_percentage, 100.0);
        assert_eq!(insights.avg_rating, 5.0);
        assert!(insights.sentiment.positive_count > 0);
    }
}