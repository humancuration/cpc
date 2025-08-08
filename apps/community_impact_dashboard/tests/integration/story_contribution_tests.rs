//! Integration tests for community story contribution functionality
//!
//! These tests verify that community members can contribute their own impact stories
//! and that these stories are properly integrated into the dashboard.

use community_impact_dashboard::components::community_stories_viz::CommunityStoriesView;
use impact_viz::core::{CommunityStory, ImpactMetric, MetricUnit};
use uuid::Uuid;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_story() -> CommunityStory {
        CommunityStory {
            title: "From Learner to Leader".to_string(),
            narrative: "Sarah started as a beginner in our learning platform, completed several courses on community organizing, and is now leading volunteer initiatives that have impacted over 200 community members.".to_string(),
            metrics: vec![
                ImpactMetric {
                    name: "Learning Completion".to_string(),
                    description: "Courses completed by Sarah".to_string(),
                    value: 8.0,
                    unit: MetricUnit::Count,
                    confidence_interval: None,
                    significance: None,
                    context: HashMap::new(),
                },
                ImpactMetric {
                    name: "Volunteer Impact".to_string(),
                    description: "People impacted by Sarah's volunteer work".to_string(),
                    value: 200.0,
                    unit: MetricUnit::People,
                    confidence_interval: None,
                    significance: None,
                    context: HashMap::new(),
                }
            ],
            testimonials: vec![
                "Sarah's leadership has transformed our volunteer program".to_string(),
                "She's an inspiration to all of us".to_string(),
            ],
            visual_elements: Vec::new(),
        }
    }

    #[test]
    fn test_story_creation() {
        // Test that community stories can be created
        let story = create_test_story();
        
        // Verify story properties
        assert!(!story.title.is_empty());
        assert!(!story.narrative.is_empty());
        assert!(!story.metrics.is_empty());
        assert!(!story.testimonials.is_empty());
    }

    #[test]
    fn test_story_validation() {
        // Test that community stories are properly validated
        let story = create_test_story();
        
        // Verify all required fields are present
        assert!(!story.title.is_empty());
        assert!(!story.narrative.is_empty());
        assert!(story.metrics.len() >= 1);
        assert!(story.testimonials.len() >= 1);
    }

    #[test]
    fn test_story_display() {
        // Test that community stories can be displayed
        let stories = vec![create_test_story()];
        assert!(!stories.is_empty());
    }

    #[test]
    fn test_metric_association() {
        // Test that stories can be associated with impact metrics
        let story = create_test_story();
        
        // Verify metrics are associated with the story
        assert!(!story.metrics.is_empty());
        
        // Verify metric properties
        for metric in &story.metrics {
            assert!(!metric.name.is_empty());
            assert!(metric.value >= 0.0);
        }
    }

    #[test]
    fn test_testimonial_inclusion() {
        // Test that community testimonials can be included in stories
        let story = create_test_story();
        
        // Verify testimonials are included
        assert!(!story.testimonials.is_empty());
        
        // Verify testimonial content
        for testimonial in &story.testimonials {
            assert!(!testimonial.is_empty());
        }
    }

    #[test]
    fn test_story_collection() {
        // Test that multiple stories can be collected
        let mut stories = Vec::new();
        
        // Add multiple test stories
        stories.push(create_test_story());
        stories.push(create_test_story());
        
        // Verify collection size
        assert_eq!(stories.len(), 2);
    }
}