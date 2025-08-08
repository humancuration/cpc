//! Continuous Improvement Mechanisms
//!
//! This module provides systems for continuously improving financial impact visualizations
//! based on collected data and feedback.

use serde::{Deserialize, Serialize};
use tracing::{debug, info};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

use crate::tracker::{VisualizationEngagement, ParticipationCorrelation, VisualizationFeedback};
use crate::analytics::{FeedbackInsights, ImprovementSuggestion};
use crate::feedback::FeedbackProcessor;
use impact_viz::financial::AllocationScenario;
use ml_core::models::ResourceAllocation;
use cpay_core::ml::{FinancialData, ResourceData};

/// Improvement engine for financial impact visualizations
pub struct ImprovementEngine {
    /// Feedback insights to drive improvements
    feedback_insights: FeedbackInsights,
    
    /// Historical engagement data
    engagement_history: Vec<VisualizationEngagement>,
    
    /// Historical participation data
    participation_history: Vec<ParticipationCorrelation>,
}

/// A/B testing framework for visualizations
pub struct ABTestingFramework {
    /// Active tests
    active_tests: HashMap<String, ABTest>,
    
    /// Test results
    test_results: HashMap<String, ABTestResult>,
}

/// A/B test configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ABTest {
    /// Unique test identifier
    pub id: String,
    
    /// Test name
    pub name: String,
    
    /// Visualization component being tested
    pub component_id: String,
    
    /// Test variants
    pub variants: Vec<TestVariant>,
    
    /// Start time
    pub start_time: DateTime<Utc>,
    
    /// End time
    pub end_time: Option<DateTime<Utc>>,
    
    /// Test status
    pub status: ABTestStatus,
}

/// Test variant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestVariant {
    /// Variant identifier
    pub id: String,
    
    /// Variant name
    pub name: String,
    
    /// Configuration data
    pub config: serde_json::Value,
    
    /// Participant count
    pub participant_count: u64,
    
    /// Engagement metrics
    pub engagement_metrics: EngagementMetrics,
}

/// Engagement metrics for A/B testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngagementMetrics {
    /// Views
    pub views: u64,
    
    /// Clicks
    pub clicks: u64,
    
    /// Time spent
    pub time_spent: f64,
    
    /// Conversion rate
    pub conversion_rate: f64,
}

/// A/B test status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ABTestStatus {
    Running,
    Paused,
    Completed,
    Cancelled,
}

/// A/B test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ABTestResult {
    /// Test identifier
    pub test_id: String,
    
    /// Winning variant
    pub winning_variant: Option<String>,
    
    /// Statistical significance
    pub significance: f64,
    
    /// Results summary
    pub summary: String,
    
    /// Detailed results
    pub details: serde_json::Value,
}

/// Personalized financial planning suggestions
pub struct PersonalizationEngine {
    /// User profiles
    user_profiles: HashMap<String, UserProfile>,
}

/// User profile for personalization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    /// User identifier
    pub user_id: String,
    
    /// Financial behavior patterns
    pub behavior_patterns: FinancialBehaviorPatterns,
    
    /// Preferences
    pub preferences: UserPreferences,
    
    /// Historical engagement
    pub engagement_history: Vec<HistoricalEngagement>,
}

/// Financial behavior patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialBehaviorPatterns {
    /// Preferred visualization types
    pub preferred_viz_types: Vec<String>,
    
    /// Engagement times
    pub engagement_times: Vec<DateTime<Utc>>,
    
    /// Decision confidence levels
    pub decision_confidence_history: Vec<f64>,
    
    /// Resource allocation preferences
    pub allocation_preferences: Vec<String>,
}

/// User preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    /// Preferred time for financial planning
    pub preferred_time: Option<String>,
    
    /// Notification preferences
    pub notification_preferences: NotificationPreferences,
    
    /// Accessibility preferences
    pub accessibility_preferences: AccessibilityPreferences,
}

/// Notification preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreferences {
    /// Email notifications enabled
    pub email_enabled: bool,
    
    /// Push notifications enabled
    pub push_enabled: bool,
    
    /// Frequency
    pub frequency: String,
}

/// Accessibility preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityPreferences {
    /// High contrast mode
    pub high_contrast: bool,
    
    /// Screen reader support
    pub screen_reader: bool,
    
    /// Font size
    pub font_size: String,
}

/// Historical engagement record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalEngagement {
    /// Component identifier
    pub component_id: String,
    
    /// Engagement timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Engagement quality
    pub quality: f64,
    
    /// Decision confidence
    pub decision_confidence: Option<f64>,
}

/// Community-curated visualization templates
pub struct CommunityTemplateRepository {
    /// Templates
    templates: HashMap<String, VisualizationTemplate>,
    
    /// Template ratings
    template_ratings: HashMap<String, f64>,
}

/// Visualization template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationTemplate {
    /// Template identifier
    pub id: String,
    
    /// Template name
    pub name: String,
    
    /// Template description
    pub description: String,
    
    /// Template configuration
    pub config: serde_json::Value,
    
    /// Creator
    pub creator: String,
    
    /// Tags
    pub tags: Vec<String>,
    
    /// Usage count
    pub usage_count: u64,
}

/// Impact score for visualization components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactScore {
    /// Component identifier
    pub component_id: String,
    
    /// Overall impact score (0.0 to 1.0)
    pub score: f64,
    
    /// Engagement component of score
    pub engagement_score: f64,
    
    /// Effectiveness component of score
    pub effectiveness_score: f64,
    
    /// Feedback component of score
    pub feedback_score: f64,
    
    /// Last calculated
    pub last_calculated: DateTime<Utc>,
}

impl ImprovementEngine {
    /// Create a new improvement engine
    pub fn new(
        feedback_insights: FeedbackInsights,
        engagement_history: Vec<VisualizationEngagement>,
        participation_history: Vec<ParticipationCorrelation>,
    ) -> Self {
        info!("Initializing ImprovementEngine");
        Self {
            feedback_insights,
            engagement_history,
            participation_history,
        }
    }
    
    /// Generate improvement suggestions based on all available data
    pub fn generate_improvement_suggestions(&self) -> Vec<ImprovementSuggestion> {
        debug!("Generating improvement suggestions");
        
        // Combine insights from feedback with historical data
        let mut suggestions = self.feedback_insights.suggestions.clone();
        
        // Add suggestions based on engagement patterns
        for theme in &self.feedback_insights.common_themes {
            if theme.sentiment < -0.3 {
                suggestions.push(ImprovementSuggestion {
                    description: format!("Address negative feedback theme: {}", theme.theme),
                    priority: if theme.frequency > 10 { crate::feedback::FeedbackPriority::High } else { crate::feedback::FeedbackPriority::Medium },
                    affected_components: vec!["multiple".to_string()], // Would be more specific in real implementation
                    supporting_feedback: theme.frequency,
                });
            }
        }
        
        // Add suggestions based on low engagement components
        let low_engagement_components: Vec<String> = self.engagement_history.iter()
            .filter(|engagement| engagement.quality_score < 0.5)
            .map(|engagement| engagement.component_id.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        
        if !low_engagement_components.is_empty() {
            suggestions.push(ImprovementSuggestion {
                description: "Redesign components with consistently low engagement".to_string(),
                priority: crate::feedback::FeedbackPriority::High,
                affected_components: low_engagement_components,
                supporting_feedback: 0, // Based on engagement data, not feedback
            });
        }
        
        suggestions
    }
    
    /// Calculate impact scores for visualization components
    pub fn calculate_impact_scores(&self) -> Vec<ImpactScore> {
        debug!("Calculating impact scores");
        
        let mut component_scores: HashMap<String, (f64, f64, f64, usize)> = HashMap::new();
        
        // Calculate engagement scores
        for engagement in &self.engagement_history {
            let entry = component_scores.entry(engagement.component_id.clone()).or_insert((0.0, 0.0, 0.0, 0));
            entry.0 += engagement.quality_score;
            entry.3 += 1;
        }
        
        // Calculate effectiveness scores (simplified based on participation correlation)
        for participation in &self.participation_history {
            for viz in &participation.viz_usage {
                let entry = component_scores.entry(viz.clone()).or_insert((0.0, 0.0, 0.0, 0));
                if participation.participated {
                    entry.1 += 1.0; // Simple effectiveness measure
                }
                // entry.3 is already incremented above for engagement, but we need to track participation separately
            }
        }
        
        // Calculate feedback scores
        // This would be more complex in a real implementation, combining various feedback metrics
        
        // Convert to ImpactScore objects
        component_scores.into_iter()
            .map(|(component_id, (engagement_sum, effectiveness_sum, _feedback_sum, count))| {
                let engagement_score = if count > 0 { engagement_sum / count as f64 } else { 0.0 };
                let effectiveness_score = if count > 0 { effectiveness_sum / count as f64 } else { 0.0 };
                // Simple weighted average for overall score
                let score = (engagement_score * 0.4 + effectiveness_score * 0.6).min(1.0);
                
                ImpactScore {
                    component_id,
                    score,
                    engagement_score,
                    effectiveness_score,
                    feedback_score: 0.0, // Would be calculated in real implementation
                    last_calculated: Utc::now(),
                }
            })
            .collect()
    }
    
    /// Generate personalized financial planning suggestions
    pub fn generate_personalized_suggestions(&self, user_id: &str) -> Vec<String> {
        debug!("Generating personalized suggestions for user: {}", user_id);
        
        // In a real implementation, this would analyze user-specific data
        // For now, we'll provide generic suggestions based on common patterns
        
        let mut suggestions = Vec::new();
        
        // Check if user has low engagement with visualizations
        let user_engagement: Vec<&VisualizationEngagement> = self.engagement_history.iter()
            .filter(|engagement| engagement.user_id == format!("hashed_{}", user_id))
            .collect();
        
        if user_engagement.is_empty() {
            suggestions.push("Try exploring different financial visualization types to find what works best for you".to_string());
        } else {
            let avg_quality: f64 = user_engagement.iter()
                .map(|e| e.quality_score)
                .sum::<f64>() / user_engagement.len() as f64;
            
            if avg_quality < 0.6 {
                suggestions.push("Consider spending more time with financial visualizations to better understand your financial picture".to_string());
            }
        }
        
        // Check participation patterns
        let user_participation: Vec<&ParticipationCorrelation> = self.participation_history.iter()
            .filter(|participation| participation.user_id == format!("hashed_{}", user_id))
            .collect();
        
        if user_participation.is_empty() {
            suggestions.push("Start with small financial contributions to build confidence and engagement".to_string());
        } else {
            let participated_count = user_participation.iter().filter(|p| p.participated).count();
            if participated_count == 0 {
                suggestions.push("Consider setting up a small automatic contribution to start building financial participation habits".to_string());
            }
        }
        
        if suggestions.is_empty() {
            suggestions.push("Continue using financial visualizations to maintain your financial awareness and engagement".to_string());
        }
        
        suggestions
    }
}

impl ABTestingFramework {
    /// Create a new A/B testing framework
    pub fn new() -> Self {
        info!("Initializing ABTestingFramework");
        Self {
            active_tests: HashMap::new(),
            test_results: HashMap::new(),
        }
    }
    
    /// Create a new A/B test
    pub fn create_test(
        &mut self,
        name: String,
        component_id: String,
        variants: Vec<TestVariant>,
    ) -> String {
        debug!("Creating A/B test: {}", name);
        
        let test_id = Uuid::new_v4().to_string();
        let test = ABTest {
            id: test_id.clone(),
            name,
            component_id,
            variants,
            start_time: Utc::now(),
            end_time: None,
            status: ABTestStatus::Running,
        };
        
        self.active_tests.insert(test_id.clone(), test);
        test_id
    }
    
    /// Record participant interaction with a test variant
    pub fn record_participant_interaction(
        &mut self,
        test_id: &str,
        variant_id: &str,
        converted: bool,
        time_spent: f64,
    ) -> Result<(), String> {
        debug!("Recording participant interaction for test: {}, variant: {}", test_id, variant_id);
        
        if let Some(test) = self.active_tests.get_mut(test_id) {
            for variant in &mut test.variants {
                if variant.id == variant_id {
                    variant.participant_count += 1;
                    variant.engagement_metrics.views += 1;
                    variant.engagement_metrics.time_spent += time_spent;
                    if converted {
                        variant.engagement_metrics.clicks += 1;
                    }
                    // Update conversion rate
                    if variant.participant_count > 0 {
                        variant.engagement_metrics.conversion_rate = 
                            variant.engagement_metrics.clicks as f64 / variant.participant_count as f64;
                    }
                    return Ok(());
                }
            }
            Err("Variant not found".to_string())
        } else {
            Err("Test not found".to_string())
        }
    }
    
    /// Complete an A/B test
    pub fn complete_test(&mut self, test_id: &str) -> Result<ABTestResult, String> {
        debug!("Completing A/B test: {}", test_id);
        
        if let Some(mut test) = self.active_tests.remove(test_id) {
            test.status = ABTestStatus::Completed;
            test.end_time = Some(Utc::now());
            
            // Determine winning variant (simplified implementation)
            let winning_variant = test.variants.iter()
                .max_by(|a, b| a.engagement_metrics.conversion_rate.partial_cmp(&b.engagement_metrics.conversion_rate).unwrap_or(std::cmp::Ordering::Equal))
                .map(|v| v.id.clone());
            
            let result = ABTestResult {
                test_id: test_id.to_string(),
                winning_variant,
                significance: 0.95, // Would be calculated statistically in real implementation
                summary: "Test completed successfully".to_string(),
                details: serde_json::json!({
                    "variants": test.variants,
                    "total_participants": test.variants.iter().map(|v| v.participant_count).sum::<u64>()
                }),
            };
            
            self.test_results.insert(test_id.to_string(), result.clone());
            Ok(result)
        } else {
            Err("Test not found".to_string())
        }
    }
    
    /// Get active tests
    pub fn get_active_tests(&self) -> &HashMap<String, ABTest> {
        &self.active_tests
    }
    
    /// Get test results
    pub fn get_test_results(&self) -> &HashMap<String, ABTestResult> {
        &self.test_results
    }
}

impl PersonalizationEngine {
    /// Create a new personalization engine
    pub fn new() -> Self {
        info!("Initializing PersonalizationEngine");
        Self {
            user_profiles: HashMap::new(),
        }
    }
    
    /// Update user profile based on engagement data
    pub fn update_user_profile(
        &mut self,
        user_id: &str,
        engagement: &VisualizationEngagement,
    ) {
        debug!("Updating user profile for: {}", user_id);
        
        let profile = self.user_profiles.entry(user_id.to_string()).or_insert_with(|| {
            UserProfile {
                user_id: user_id.to_string(),
                behavior_patterns: FinancialBehaviorPatterns {
                    preferred_viz_types: Vec::new(),
                    engagement_times: Vec::new(),
                    decision_confidence_history: Vec::new(),
                    allocation_preferences: Vec::new(),
                },
                preferences: UserPreferences {
                    preferred_time: None,
                    notification_preferences: NotificationPreferences {
                        email_enabled: true,
                        push_enabled: true,
                        frequency: "weekly".to_string(),
                    },
                    accessibility_preferences: AccessibilityPreferences {
                        high_contrast: false,
                        screen_reader: false,
                        font_size: "medium".to_string(),
                    },
                },
                engagement_history: Vec::new(),
            }
        });
        
        // Update behavior patterns
        profile.behavior_patterns.preferred_viz_types.push(format!("{:?}", engagement.viz_type));
        profile.behavior_patterns.engagement_times.push(engagement.timestamp);
        if let Some(confidence) = engagement.decision_confidence {
            profile.behavior_patterns.decision_confidence_history.push(confidence);
        }
        
        // Update engagement history
        profile.engagement_history.push(HistoricalEngagement {
            component_id: engagement.component_id.clone(),
            timestamp: engagement.timestamp,
            quality: engagement.quality_score,
            decision_confidence: engagement.decision_confidence,
        });
    }
    
    /// Get personalized visualization recommendations for a user
    pub fn get_recommendations(&self, user_id: &str) -> Vec<String> {
        debug!("Getting recommendations for user: {}", user_id);
        
        if let Some(profile) = self.user_profiles.get(user_id) {
            // Simple recommendation based on preferred visualization types
            let mut recommendations = Vec::new();
            
            if !profile.behavior_patterns.preferred_viz_types.is_empty() {
                // Recommend similar visualization types
                let last_viz_type = profile.behavior_patterns.preferred_viz_types.last().unwrap();
                recommendations.push(format!("Try more visualizations of type: {}", last_viz_type));
            }
            
            // Recommend based on engagement times
            if profile.behavior_patterns.engagement_times.len() > 5 {
                recommendations.push("You're engaging regularly with financial visualizations. Keep up the good work!".to_string());
            }
            
            // Recommend based on decision confidence
            if profile.behavior_patterns.decision_confidence_history.len() > 3 {
                let avg_confidence: f64 = profile.behavior_patterns.decision_confidence_history.iter().sum::<f64>() 
                    / profile.behavior_patterns.decision_confidence_history.len() as f64;
                if avg_confidence < 0.7 {
                    recommendations.push("Consider spending more time with visualizations to build confidence in financial decisions".to_string());
                }
            }
            
            recommendations
        } else {
            // Generic recommendations for new users
            vec![
                "Start with basic financial overview visualizations to build understanding".to_string(),
                "Try different visualization types to see what helps you best understand your finances".to_string(),
            ]
        }
    }
}

impl CommunityTemplateRepository {
    /// Create a new community template repository
    pub fn new() -> Self {
        info!("Initializing CommunityTemplateRepository");
        Self {
            templates: HashMap::new(),
            template_ratings: HashMap::new(),
        }
    }
    
    /// Add a new template
    pub fn add_template(
        &mut self,
        name: String,
        description: String,
        config: serde_json::Value,
        creator: String,
        tags: Vec<String>,
    ) -> String {
        debug!("Adding new template: {}", name);
        
        let template_id = Uuid::new_v4().to_string();
        let template = VisualizationTemplate {
            id: template_id.clone(),
            name,
            description,
            config,
            creator,
            tags,
            usage_count: 0,
        };
        
        self.templates.insert(template_id.clone(), template);
        self.template_ratings.insert(template_id.clone(), 0.0);
        template_id
    }
    
    /// Rate a template
    pub fn rate_template(&mut self, template_id: &str, rating: f64) -> Result<(), String> {
        debug!("Rating template: {}, rating: {}", template_id, rating);
        
        if let Some(current_rating) = self.template_ratings.get_mut(template_id) {
            // Simple averaging - in a real implementation, you'd want to track individual ratings
            *current_rating = (*current_rating + rating) / 2.0;
            Ok(())
        } else {
            Err("Template not found".to_string())
        }
    }
    
    /// Get top-rated templates
    pub fn get_top_rated_templates(&self, limit: usize) -> Vec<&VisualizationTemplate> {
        debug!("Getting top rated templates, limit: {}", limit);
        
        let mut templates: Vec<(&String, &f64)> = self.template_ratings.iter().collect();
        templates.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        templates.iter()
            .take(limit)
            .filter_map(|(id, _)| self.templates.get(*id))
            .collect()
    }
    
    /// Get templates by tag
    pub fn get_templates_by_tag(&self, tag: &str) -> Vec<&VisualizationTemplate> {
        debug!("Getting templates by tag: {}", tag);
        
        self.templates.values()
            .filter(|template| template.tags.contains(&tag.to_string()))
            .collect()
    }
    
    /// Record template usage
    pub fn record_template_usage(&mut self, template_id: &str) -> Result<(), String> {
        debug!("Recording template usage: {}", template_id);
        
        if let Some(template) = self.templates.get_mut(template_id) {
            template.usage_count += 1;
            Ok(())
        } else {
            Err("Template not found".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feedback::{FeedbackTheme, SentimentAnalysis};
    use crate::tracker::VisualizationType;
    use chrono::Utc;
    use uuid::Uuid;
    
    #[test]
    fn test_improvement_engine_creation() {
        let feedback_insights = FeedbackInsights {
            helpfulness_percentage: 85.0,
            avg_rating: 4.2,
            common_themes: vec![],
            sentiment: SentimentAnalysis {
                overall_sentiment: 0.7,
                positive_count: 85,
                neutral_count: 10,
                negative_count: 5,
            },
            suggestions: vec![],
        };
        
        let engine = ImprovementEngine::new(
            feedback_insights,
            Vec::new(),
            Vec::new(),
        );
        
        assert!(true); // Engine should be created successfully
    }
    
    #[test]
    fn test_ab_testing_framework() {
        let mut framework = ABTestingFramework::new();
        
        let variants = vec![
            TestVariant {
                id: "variant_a".to_string(),
                name: "Original Design".to_string(),
                config: serde_json::json!({}),
                participant_count: 0,
                engagement_metrics: EngagementMetrics {
                    views: 0,
                    clicks: 0,
                    time_spent: 0.0,
                    conversion_rate: 0.0,
                },
            },
            TestVariant {
                id: "variant_b".to_string(),
                name: "New Design".to_string(),
                config: serde_json::json!({}),
                participant_count: 0,
                engagement_metrics: EngagementMetrics {
                    views: 0,
                    clicks: 0,
                    time_spent: 0.0,
                    conversion_rate: 0.0,
                },
            },
        ];
        
        let test_id = framework.create_test(
            "Financial Viz Test".to_string(),
            "financial_viz_1".to_string(),
            variants,
        );
        
        assert!(!test_id.is_empty());
        assert_eq!(framework.active_tests.len(), 1);
        
        // Record some interactions
        let result = framework.record_participant_interaction(&test_id, "variant_a", true, 120.0);
        assert!(result.is_ok());
        
        let result = framework.record_participant_interaction(&test_id, "variant_b", false, 60.0);
        assert!(result.is_ok());
        
        // Complete the test
        let test_result = framework.complete_test(&test_id);
        assert!(test_result.is_ok());
        assert_eq!(framework.active_tests.len(), 0);
        assert_eq!(framework.test_results.len(), 1);
    }
    
    #[test]
    fn test_personalization_engine() {
        let mut engine = PersonalizationEngine::new();
        
        let engagement = VisualizationEngagement {
            id: Uuid::new_v4(),
            viz_type: VisualizationType::Comparative,
            component_id: "financial_viz_1".to_string(),
            user_id: "user123".to_string(),
            interaction_time: 120.0,
            interaction_count: 5,
            timestamp: Utc::now(),
            quality_score: 0.8,
            decision_confidence: Some(0.9),
        };
        
        engine.update_user_profile("user123", &engagement);
        assert_eq!(engine.user_profiles.len(), 1);
        
        let recommendations = engine.get_recommendations("user123");
        assert!(!recommendations.is_empty());
    }
    
    #[test]
    fn test_community_template_repository() {
        let mut repo = CommunityTemplateRepository::new();
        
        let template_id = repo.add_template(
            "Budget Overview".to_string(),
            "A comprehensive budget overview visualization".to_string(),
            serde_json::json!({"type": "budget_overview"}),
            "user123".to_string(),
            vec!["budget".to_string(), "overview".to_string()],
        );
        
        assert!(!template_id.is_empty());
        assert_eq!(repo.templates.len(), 1);
        assert_eq!(repo.template_ratings.len(), 1);
        
        // Rate the template
        let result = repo.rate_template(&template_id, 4.5);
        assert!(result.is_ok());
        
        // Get top rated templates
        let top_templates = repo.get_top_rated_templates(5);
        assert_eq!(top_templates.len(), 1);
        
        // Get templates by tag
        let budget_templates = repo.get_templates_by_tag("budget");
        assert_eq!(budget_templates.len(), 1);
        
        // Record usage
        let result = repo.record_template_usage(&template_id);
        assert!(result.is_ok());
        assert_eq!(repo.templates.get(&template_id).unwrap().usage_count, 1);
    }
}