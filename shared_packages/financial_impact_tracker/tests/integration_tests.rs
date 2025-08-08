//! Integration tests for the financial impact tracker
//!
//! These tests verify that all components of the financial impact tracker work together correctly.

use financial_impact_tracker::{
    FinancialImpactTracker,
    ImpactAnalyticsDashboard,
    FeedbackCollector,
    ImprovementEngine,
    EcosystemIntegrator,
    tracker::{VisualizationEngagement, ParticipationCorrelation, VisualizationType, ValidationType},
    analytics::DashboardSummary,
    feedback::{FeedbackInsights, FeedbackProcessor},
    improvement::{ABTestingFramework, PersonalizationEngine, CommunityTemplateRepository},
    integration::{CrossSystemImpact, SystemConnection, ConnectionStatus, IntegrationType},
};
use consent_manager::domain::consent::DataSharingLevel;
use common_utils::financial::{MonetaryValue, RoundingStrategy};
use fixed::types::I64F64;
use uuid::Uuid;
use chrono::Utc;

/// Test the complete financial impact tracking workflow
#[test]
fn test_complete_financial_impact_tracking_workflow() {
    // Create a financial impact tracker
    let mut tracker = FinancialImpactTracker::new(DataSharingLevel::Standard);
    
    // Track visualization engagement
    let engagement_result = tracker.track_visualization_engagement(
        "user123",
        "budget_viz_1",
        VisualizationType::Comparative,
        150.0, // 2.5 minutes
        20,    // 20 interactions
        0.85,  // Quality score
        Some(0.9), // Decision confidence
    );
    assert!(engagement_result.is_ok());
    
    // Track participation correlation
    let amount = MonetaryValue::new(I64F64::from_num(150.75), "USD");
    let participation_result = tracker.track_participation_correlation(
        "user123",
        vec!["budget_viz_1".to_string(), "expense_viz_1".to_string()],
        true,   // Participated
        Some(8.5), // 8.5 months of participation
        Some(9),   // Satisfaction rating
        Some(amount),
    );
    assert!(participation_result.is_ok());
    
    // Record community validation
    let validation_result = tracker.record_community_validation(
        "user123",
        "budget_viz_1",
        ValidationType::Endorsement,
        "This visualization really helped me understand my spending patterns",
        Some("Personal budgeting".to_string()),
    );
    assert!(validation_result.is_ok());
    
    // Record feedback
    let feedback_result = tracker.record_feedback(
        "user123",
        "budget_viz_1",
        5, // 5-star rating
        Some("Excellent visualization that made budgeting easy to understand".to_string()),
        true, // Helpful
        Some("Helped me create my first budget".to_string()),
        Some(9), // Understanding improvement
        Some(8), // Confidence improvement
    );
    assert!(feedback_result.is_ok());
    
    // Verify metrics were collected
    let metrics = tracker.get_metrics();
    assert_eq!(metrics.visualization_engagement.len(), 1);
    assert_eq!(metrics.participation_correlation.len(), 1);
    assert_eq!(metrics.community_validation.len(), 1);
    assert_eq!(metrics.feedback_data.len(), 1);
    
    // Test analytics dashboard
    let dashboard = ImpactAnalyticsDashboard::new(metrics.clone());
    let community_data = skill_development::ml::CommunityData {
        member_count: 1000,
        skill_distribution: std::collections::HashMap::new(),
        activity_levels: std::collections::HashMap::new(),
        resource_availability: std::collections::HashMap::new(),
    };
    let summary = dashboard.generate_summary(&community_data);
    
    // Verify dashboard summary contains expected data
    assert!(summary.engagement.total_views > 0);
    assert!(summary.financial_effectiveness.participation_rate_with_viz > 0.0);
    assert!(summary.feedback.helpful_percentage > 0.0);
    
    // Test feedback processing
    let feedback_processor = FeedbackProcessor::new(metrics.feedback_data.clone());
    let feedback_insights = feedback_processor.process_feedback();
    
    // Verify feedback insights
    assert_eq!(feedback_insights.helpfulness_percentage, 100.0);
    assert_eq!(feedback_insights.avg_rating, 5.0);
    
    // Test improvement engine
    let improvement_engine = ImprovementEngine::new(
        feedback_insights,
        metrics.visualization_engagement.values().cloned().collect(),
        metrics.participation_correlation.clone(),
    );
    
    // Generate improvement suggestions
    let suggestions = improvement_engine.generate_improvement_suggestions();
    // At minimum, we should have the default suggestion from the feedback insights
    assert!(!suggestions.is_empty());
    
    // Calculate impact scores
    let impact_scores = improvement_engine.calculate_impact_scores();
    assert!(!impact_scores.is_empty());
    
    // Generate personalized suggestions
    let personalized_suggestions = improvement_engine.generate_personalized_suggestions("user123");
    assert!(!personalized_suggestions.is_empty());
    
    // Test A/B testing framework
    let mut ab_framework = ABTestingFramework::new();
    let variants = vec![
        financial_impact_tracker::improvement::TestVariant {
            id: "variant_a".to_string(),
            name: "Original Design".to_string(),
            config: serde_json::json!({}),
            participant_count: 0,
            engagement_metrics: financial_impact_tracker::improvement::EngagementMetrics {
                views: 0,
                clicks: 0,
                time_spent: 0.0,
                conversion_rate: 0.0,
            },
        },
        financial_impact_tracker::improvement::TestVariant {
            id: "variant_b".to_string(),
            name: "New Design".to_string(),
            config: serde_json::json!({}),
            participant_count: 0,
            engagement_metrics: financial_impact_tracker::improvement::EngagementMetrics {
                views: 0,
                clicks: 0,
                time_spent: 0.0,
                conversion_rate: 0.0,
            },
        },
    ];
    
    let test_id = ab_framework.create_test(
        "Budget Visualization Test".to_string(),
        "budget_viz_1".to_string(),
        variants,
    );
    assert!(!test_id.is_empty());
    
    // Record participant interactions
    let interaction_result = ab_framework.record_participant_interaction(&test_id, "variant_a", true, 180.0);
    assert!(interaction_result.is_ok());
    
    let interaction_result = ab_framework.record_participant_interaction(&test_id, "variant_b", true, 200.0);
    assert!(interaction_result.is_ok());
    
    // Complete the test
    let test_result = ab_framework.complete_test(&test_id);
    assert!(test_result.is_ok());
    
    // Test personalization engine
    let mut personalization_engine = PersonalizationEngine::new();
    if let Some(engagement) = metrics.visualization_engagement.values().next() {
        personalization_engine.update_user_profile("user123", engagement);
        assert_eq!(personalization_engine.get_recommendations("user123").len(), 3);
    }
    
    // Test community template repository
    let mut template_repo = CommunityTemplateRepository::new();
    let template_id = template_repo.add_template(
        "Monthly Budget Tracker".to_string(),
        "A comprehensive monthly budget tracking visualization".to_string(),
        serde_json::json!({"type": "monthly_budget"}),
        "user123".to_string(),
        vec!["budget".to_string(), "monthly".to_string()],
    );
    assert!(!template_id.is_empty());
    
    // Rate the template
    let rating_result = template_repo.rate_template(&template_id, 4.5);
    assert!(rating_result.is_ok());
    
    // Record template usage
    let usage_result = template_repo.record_template_usage(&template_id);
    assert!(usage_result.is_ok());
    
    // Test ecosystem integration
    let mut ecosystem_integrator = EcosystemIntegrator::new();
    
    // Connect to learning system
    let connection_result = ecosystem_integrator.connect_system(
        "learning_system_1".to_string(),
        "Learning Platform".to_string(),
        vec![IntegrationType::Learning, IntegrationType::Feedback],
    );
    assert!(connection_result.is_ok());
    
    // Connect to volunteer system
    let connection_result = ecosystem_integrator.connect_system(
        "volunteer_system_1".to_string(),
        "Volunteer Coordination".to_string(),
        vec![IntegrationType::Volunteer, IntegrationType::Feedback],
    );
    assert!(connection_result.is_ok());
    
    // Verify connections
    assert!(matches!(
        ecosystem_integrator.get_connection_status("learning_system_1"),
        Some(ConnectionStatus::Connected)
    ));
    
    assert!(matches!(
        ecosystem_integrator.get_connection_status("volunteer_system_1"),
        Some(ConnectionStatus::Connected)
    ));
    
    // Create sample data for cross-system impact calculation
    let learning_data = vec![learning_impact_tracker::tracker::CourseCompletionCorrelation {
        id: Uuid::new_v4(),
        course_id: "financial_literacy_101".to_string(),
        user_id: "user123".to_string(),
        viz_usage: vec!["budget_viz_1".to_string()],
        completed: true,
        time_to_completion: Some(180.0),
        satisfaction: Some(8),
        timestamp: Utc::now(),
    }];
    
    let volunteer_data = vec![volunteer_impact_tracker::tracker::TaskCompletion {
        id: Uuid::new_v4(),
        user_id: "user123".to_string(),
        task_id: "community_fundraiser".to_string(),
        influencing_viz: Some("budget_viz_1".to_string()),
        completed: true,
        quality: Some(9),
        time_to_completion: Some(300.0),
        timestamp: Utc::now(),
    }];
    
    let cause_data = vec![cause_management::domain::cause::Cause {
        id: Uuid::new_v4(),
        name: "Community Development".to_string(),
        description: "Supporting local community development initiatives".to_string(),
        status: cause_management::domain::cause::CauseStatus::Active,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        impact_metrics: None,
    }];
    
    // Calculate cross-system impact
    let cross_impact = ecosystem_integrator.calculate_cross_system_impact(
        &metrics.participation_correlation,
        &learning_data,
        &volunteer_data,
        &cause_data,
    );
    
    // Verify cross-system impact metrics
    assert_eq!(cross_impact.financial_metrics.participation_rate, 1.0);
    assert!(cross_impact.financial_metrics.avg_contribution.is_some());
    assert_eq!(cross_impact.learning_metrics.completion_rate, 1.0);
    assert_eq!(cross_impact.volunteer_metrics.task_completion_rate, 1.0);
    assert!(cross_impact.correlations.learning_financial_correlation > 0.0);
    assert!(cross_impact.correlations.volunteer_financial_correlation > 0.0);
    assert!(cross_impact.correlations.cause_financial_correlation > 0.0);
    
    // Verify connected systems
    let connected_systems = ecosystem_integrator.get_connected_systems();
    assert_eq!(connected_systems.len(), 2);
    
    // Test system disconnection
    let disconnect_result = ecosystem_integrator.disconnect_system("learning_system_1");
    assert!(disconnect_result.is_ok());
    
    assert!(matches!(
        ecosystem_integrator.get_connection_status("learning_system_1"),
        Some(ConnectionStatus::Disconnected)
    ));
}

/// Test privacy-preserving data collection with different consent levels
#[test]
fn test_privacy_preserving_data_collection() {
    // Test with None consent level
    let mut tracker_none = FinancialImpactTracker::new(DataSharingLevel::None);
    
    let result = tracker_none.track_visualization_engagement(
        "user123",
        "budget_viz_1",
        VisualizationType::Comparative,
        150.0,
        20,
        0.85,
        Some(0.9),
    );
    assert!(result.is_ok());
    
    // No data should be collected with None consent
    assert_eq!(tracker_none.get_metrics().visualization_engagement.len(), 0);
    
    // Test with Standard consent level
    let mut tracker_standard = FinancialImpactTracker::new(DataSharingLevel::Standard);
    
    let result = tracker_standard.track_visualization_engagement(
        "user123",
        "budget_viz_1",
        VisualizationType::Comparative,
        150.0,
        20,
        0.85,
        Some(0.9),
    );
    assert!(result.is_ok());
    
    // Data should be collected with Standard consent
    assert_eq!(tracker_standard.get_metrics().visualization_engagement.len(), 1);
    
    // Test feedback collector with different consent levels
    let mut feedback_none = FeedbackCollector::new(DataSharingLevel::None);
    let result = feedback_none.collect_quick_feedback("user123", "budget_viz_1", true);
    assert!(result.is_ok());
    assert_eq!(feedback_none.get_feedback().len(), 0);
    
    let mut feedback_standard = FeedbackCollector::new(DataSharingLevel::Standard);
    let result = feedback_standard.collect_quick_feedback("user123", "budget_viz_1", true);
    assert!(result.is_ok());
    assert_eq!(feedback_standard.get_feedback().len(), 1);
}

/// Test edge cases and error conditions
#[test]
fn test_edge_cases_and_error_conditions() {
    // Test with empty data sets
    let empty_metrics = financial_impact_tracker::tracker::ImpactMetrics {
        visualization_engagement: std::collections::HashMap::new(),
        participation_correlation: Vec::new(),
        allocation_effectiveness: Vec::new(),
        community_validation: Vec::new(),
        feedback_data: Vec::new(),
    };
    
    let dashboard = ImpactAnalyticsDashboard::new(empty_metrics);
    let community_data = skill_development::ml::CommunityData {
        member_count: 0,
        skill_distribution: std::collections::HashMap::new(),
        activity_levels: std::collections::HashMap::new(),
        resource_availability: std::collections::HashMap::new(),
    };
    let summary = dashboard.generate_summary(&community_data);
    
    // With empty data, all metrics should be 0 or have sensible defaults
    assert_eq!(summary.engagement.total_views, 0);
    assert_eq!(summary.financial_effectiveness.participation_rate_with_viz, 0.0);
    assert_eq!(summary.feedback.helpful_percentage, 0.0);
    
    // Test A/B testing error conditions
    let mut ab_framework = ABTestingFramework::new();
    
    // Try to record interaction for non-existent test
    let result = ab_framework.record_participant_interaction("nonexistent_test", "variant_a", true, 100.0);
    assert!(result.is_err());
    
    // Try to complete non-existent test
    let result = ab_framework.complete_test("nonexistent_test");
    assert!(result.is_err());
    
    // Test community template repository error conditions
    let mut template_repo = CommunityTemplateRepository::new();
    
    // Try to rate non-existent template
    let result = template_repo.rate_template("nonexistent_template", 4.5);
    assert!(result.is_err());
    
    // Try to record usage for non-existent template
    let result = template_repo.record_template_usage("nonexistent_template");
    assert!(result.is_err());
    
    // Test ecosystem integrator error conditions
    let mut ecosystem_integrator = EcosystemIntegrator::new();
    
    // Try to disconnect from non-existent system
    let result = ecosystem_integrator.disconnect_system("nonexistent_system");
    assert!(result.is_err());
    
    // Try to get connection status for non-existent system
    let status = ecosystem_integrator.get_connection_status("nonexistent_system");
    assert!(status.is_none());
}

/// Test financial impact tracker with various monetary values and currencies
#[test]
fn test_financial_impact_tracker_with_various_currencies() {
    let mut tracker = FinancialImpactTracker::new(DataSharingLevel::Standard);
    
    // Test with USD
    let usd_amount = MonetaryValue::new(I64F64::from_num(100.50), "USD");
    let result = tracker.track_participation_correlation(
        "user123",
        vec!["budget_viz_1".to_string()],
        true,
        Some(12.0),
        Some(8),
        Some(usd_amount),
    );
    assert!(result.is_ok());
    
    // Test with EUR
    let eur_amount = MonetaryValue::new(I64F64::from_num(85.75), "EUR");
    let result = tracker.track_participation_correlation(
        "user124",
        vec!["expense_viz_1".to_string()],
        true,
        Some(6.0),
        Some(7),
        Some(eur_amount),
    );
    assert!(result.is_ok());
    
    // Test with Dabloons (fictional currency)
    let dabloons_amount = MonetaryValue::new(I64F64::from_num(500.0), "DABLOONS");
    let result = tracker.track_participation_correlation(
        "user125",
        vec!["savings_viz_1".to_string()],
        false, // Not participating
        None,
        Some(5),
        Some(dabloons_amount),
    );
    assert!(result.is_ok());
    
    // Verify all data was collected
    let metrics = tracker.get_metrics();
    assert_eq!(metrics.participation_correlation.len(), 3);
    
    // Verify currencies are preserved
    let currencies: std::collections::HashSet<&str> = metrics.participation_correlation
        .iter()
        .filter_map(|c| c.contribution_amount.as_ref())
        .map(|amount| amount.currency())
        .collect();
    
    assert!(currencies.contains("USD"));
    assert!(currencies.contains("EUR"));
    assert!(currencies.contains("DABLOONS"));
}