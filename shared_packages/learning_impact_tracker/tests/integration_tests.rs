//! Integration tests for the learning impact tracker system

use learning_impact_tracker::{
    LearningImpactTracker, 
    ImpactAnalyticsDashboard,
    FeedbackCollector,
    ImprovementEngine,
    EcosystemIntegrator
};
use consent_manager::domain::consent::DataSharingLevel;
use impact_viz::core::VisualizationType;

#[test]
fn test_full_system_integration() {
    // Initialize all components
    let mut tracker = LearningImpactTracker::new(DataSharingLevel::Standard);
    let dashboard = ImpactAnalyticsDashboard::new(tracker.get_metrics().clone());
    let feedback_collector = FeedbackCollector::new(DataSharingLevel::Standard);
    let improvement_engine = ImprovementEngine::new();
    let ecosystem_integrator = EcosystemIntegrator::new();
    
    // Test tracking functionality
    let track_result = tracker.track_visualization_engagement(
        "user123",
        "skill_viz_1",
        VisualizationType::Comparative,
        120.5,
        15,
        0.85,
    );
    assert!(track_result.is_ok());
    
    // Test feedback collection
    let feedback_result = feedback_collector.record_feedback(
        "user123",
        "skill_viz_1",
        5,
        Some("Very helpful visualization".to_string()),
        true,
    );
    assert!(feedback_result.is_ok());
    
    // Verify system components work together
    assert!(!tracker.get_metrics().visualization_engagement.is_empty());
    assert!(!feedback_collector.get_feedback_data().is_empty());
    
    println!("Full system integration test passed!");
}