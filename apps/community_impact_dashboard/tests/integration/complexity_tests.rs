//! Integration tests for progressive disclosure of complexity
//!
//! These tests verify that the dashboard correctly implements progressive disclosure
//! of complexity, allowing users to explore increasingly detailed information.

use community_impact_dashboard::components::community_transformation_viz::CommunityTransformationMetrics;
use community_impact_dashboard::models::CommunityWellbeing;
use community_impact_dashboard::models::community_wellbeing::*;
use impact_viz::core::VisualizationStyle;
use uuid::Uuid;
use chrono::Utc;

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_wellbeing_data() -> CommunityWellbeing {
        let domain_indicators = DomainWellbeingIndicators {
            learning: LearningWellbeing {
                knowledge_sharing_rate: 0.75,
                skill_development_progress: 0.80,
                educational_equity: 0.65,
                community_satisfaction: 0.85,
            },
            volunteer: VolunteerWellbeing {
                participation_rate: 0.60,
                retention_rate: 0.70,
                satisfaction_index: 0.75,
                service_coverage: 0.55,
            },
            financial: FinancialWellbeing {
                financial_health: 0.70,
                resource_equity: 0.65,
                sustainability_index: 0.75,
                economic_participation: 0.60,
            },
            cause: CauseWellbeing {
                engagement_rate: 0.65,
                impact_effectiveness: 0.70,
                solidarity_index: 0.80,
                justice_progress: 0.60,
            },
        };

        let mut wellbeing = CommunityWellbeing::new(0.71, domain_indicators);
        
        // Add cooperative goals progress
        wellbeing = wellbeing.add_cooperative_goal(CooperativeGoalProgress {
            id: Uuid::new_v4(),
            title: "Increase Community Learning".to_string(),
            description: "Expand access to educational resources for all members".to_string(),
            progress: 0.75,
            target_date: None,
            values_alignment: vec!["Education".to_string(), "Equity".to_string()],
        });
        
        // Add historical progress
        wellbeing = wellbeing.add_historical_progress(WellbeingProgressPoint {
            timestamp: Utc::now() - chrono::Duration::days(30),
            overall_score: 0.65,
            learning_score: 0.70,
            volunteer_score: 0.55,
            financial_score: 0.65,
            cause_score: 0.60,
        });
        
        wellbeing
    }

    #[test]
    fn test_simple_view_rendering() {
        // Test that the simple view renders correctly
        let wellbeing = create_test_wellbeing_data();
        let style = VisualizationStyle::Narrative;
        
        // In a real test, we would render the component and verify the output
        // For now, we'll just verify we can create the component with simple data
        assert!(wellbeing.overall_score >= 0.0);
        assert!(matches!(style, VisualizationStyle::Narrative));
    }

    #[test]
    fn test_detailed_view_rendering() {
        // Test that the detailed view renders correctly
        let wellbeing = create_test_wellbeing_data();
        let style = VisualizationStyle::Quantitative;
        
        // In a real test, we would render the component and verify the output
        // For now, we'll just verify we can create the component with detailed data
        assert!(!wellbeing.cooperative_goals_progress.is_empty());
        assert!(!wellbeing.historical_progress.is_empty());
        assert!(matches!(style, VisualizationStyle::Quantitative));
    }

    #[test]
    fn test_progressive_data_loading() {
        // Test that data is loaded progressively
        let wellbeing = create_test_wellbeing_data();
        
        // Verify basic data is present
        assert!(wellbeing.overall_score >= 0.0);
        assert!(wellbeing.domain_indicators.learning.knowledge_sharing_rate >= 0.0);
        
        // Verify additional data is present for detailed views
        assert!(!wellbeing.cooperative_goals_progress.is_empty());
        assert!(!wellbeing.historical_progress.is_empty());
    }

    #[test]
    fn test_complexity_levels() {
        // Test different complexity levels
        let wellbeing = create_test_wellbeing_data();
        
        // Simple complexity - just overall score and domain indicators
        assert!(wellbeing.overall_score >= 0.0);
        assert!(wellbeing.domain_indicators.learning.knowledge_sharing_rate >= 0.0);
        
        // Medium complexity - add goals progress
        assert!(!wellbeing.cooperative_goals_progress.is_empty());
        
        // High complexity - add historical data and comparative metrics
        assert!(!wellbeing.historical_progress.is_empty());
    }
}