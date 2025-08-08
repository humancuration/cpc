//! Integration tests for the Cause Impact Tracker
//!
//! These tests verify that the Cause Impact Tracker integrates properly
//! with other impact tracking systems in the CPC ecosystem.

use cause_impact_tracker::{
    CauseImpactTracker,
    ImpactAnalyticsDashboard,
    EcosystemIntegrator,
    tracker::VisualizationType,
};
use consent_manager::domain::consent::DataSharingLevel;
use std::collections::HashMap;
use uuid::Uuid;

#[test]
fn test_cause_impact_tracker_creation() {
    let tracker = CauseImpactTracker::new(DataSharingLevel::Standard);
    assert_eq!(tracker.get_metrics().visualization_engagement.len(), 0);
    assert_eq!(tracker.get_metrics().engagement_correlation.len(), 0);
    assert_eq!(tracker.get_metrics().contribution_effectiveness.len(), 0);
}

#[test]
fn test_track_visualization_engagement() {
    let mut tracker = CauseImpactTracker::new(DataSharingLevel::Standard);
    
    let result = tracker.track_visualization_engagement(
        "user_123",
        "cause_viz_1",
        VisualizationType::Narrative,
        120.5,
        15,
        0.85,
        Some(0.9),
    );
    
    assert!(result.is_ok());
    assert_eq!(tracker.get_metrics().visualization_engagement.len(), 1);
    
    let engagement = tracker.get_metrics().visualization_engagement.get("cause_viz_1").unwrap();
    assert_eq!(engagement.component_id, "cause_viz_1");
    assert_eq!(engagement.interaction_time, 120.5);
    assert_eq!(engagement.quality_score, 0.85);
    assert_eq!(engagement.decision_confidence, Some(0.9));
}

#[test]
fn test_consent_respects_none_level() {
    let mut tracker = CauseImpactTracker::new(DataSharingLevel::None);
    
    let result = tracker.track_visualization_engagement(
        "user_123",
        "cause_viz_1",
        VisualizationType::Narrative,
        120.5,
        15,
        0.85,
        Some(0.9),
    );
    
    assert!(result.is_ok());
    assert_eq!(tracker.get_metrics().visualization_engagement.len(), 0);
}

#[test]
fn test_track_engagement_correlation() {
    let mut tracker = CauseImpactTracker::new(DataSharingLevel::Standard);
    
    let result = tracker.track_engagement_correlation(
        "user_123",
        vec!["viz1".to_string(), "viz2".to_string()],
        true,
        Some(6.5),
        Some(8),
        Some(100.50),
    );
    
    assert!(result.is_ok());
    assert_eq!(tracker.get_metrics().engagement_correlation.len(), 1);
    
    let correlation = &tracker.get_metrics().engagement_correlation[0];
    assert_eq!(correlation.user_id, "hashed_user_123");
    assert_eq!(correlation.engaged, true);
    assert_eq!(correlation.engagement_months, Some(6.5));
    assert_eq!(correlation.satisfaction, Some(8));
    assert_eq!(correlation.contribution_amount, Some(100.50));
}

#[test]
fn test_ecosystem_integration() {
    let mut integrator = EcosystemIntegrator::new();
    
    // Create mock metrics for other systems
    let learning_metrics = learning_impact_tracker::tracker::ImpactMetrics {
        visualization_engagement: HashMap::new(),
        course_completion_correlation: Vec::new(),
        learning_effectiveness: Vec::new(),
        community_validation: Vec::new(),
        feedback_data: Vec::new(),
    };
    
    let volunteer_metrics = volunteer_impact_tracker::tracker::ImpactMetrics {
        visualization_engagement: HashMap::new(),
        task_completion_correlation: Vec::new(),
        volunteer_effectiveness: Vec::new(),
        community_validation: Vec::new(),
        feedback_data: Vec::new(),
    };
    
    let financial_metrics = financial_impact_tracker::tracker::ImpactMetrics {
        visualization_engagement: HashMap::new(),
        participation_correlation: Vec::new(),
        allocation_effectiveness: Vec::new(),
        community_validation: Vec::new(),
        feedback_data: Vec::new(),
    };
    
    // Set metrics in integrator
    integrator.set_learning_metrics(learning_metrics);
    integrator.set_volunteer_metrics(volunteer_metrics);
    integrator.set_financial_metrics(financial_metrics);
    
    // Create cause metrics
    let cause_metrics = cause_impact_tracker::tracker::ImpactMetrics {
        visualization_engagement: HashMap::new(),
        engagement_correlation: Vec::new(),
        contribution_effectiveness: Vec::new(),
        community_validation: Vec::new(),
        feedback_data: Vec::new(),
    };
    
    // Perform integrated analysis
    let analysis = integrator.analyze_integrated_impact(&cause_metrics);
    
    assert!(analysis.community_engagement_score >= 0.0);
    assert!(analysis.community_engagement_score <= 1.0);
    assert!(!analysis.integrated_recommendations.is_empty());
}

#[test]
fn test_dashboard_generation() {
    let mut tracker = CauseImpactTracker::new(DataSharingLevel::Standard);
    
    // Add some mock data
    let _ = tracker.track_visualization_engagement(
        "user_123",
        "cause_viz_1",
        VisualizationType::Narrative,
        120.5,
        15,
        0.85,
        Some(0.9),
    );
    
    let _ = tracker.track_engagement_correlation(
        "user_123",
        vec!["cause_viz_1".to_string()],
        true,
        Some(6.5),
        Some(8),
        Some(100.50),
    );
    
    // Generate dashboard
    let metrics = tracker.get_metrics();
    let dashboard = ImpactAnalyticsDashboard::new(metrics.clone());
    
    // Create mock community data
    let community_data = skill_development::ml::CommunityData {
        member_count: 1000,
        skill_distribution: HashMap::new(),
        activity_levels: HashMap::new(),
        resource_availability: HashMap::new(),
    };
    
    let summary = dashboard.generate_summary(&community_data);
    
    assert!(summary.engagement.total_views > 0);
    assert!(summary.cause_effectiveness.engagement_rate_with_viz >= 0.0);
    assert!(summary.feedback.avg_rating >= 0.0);
}