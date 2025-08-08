//! Unit tests for the Cause Impact Tracker

use cause_impact_tracker::{
    CauseImpactTracker,
    tracker::{VisualizationType, ValidationType},
};
use consent_manager::domain::consent::DataSharingLevel;

#[test]
fn test_tracker_creation() {
    let tracker = CauseImpactTracker::new(DataSharingLevel::Standard);
    assert_eq!(tracker.get_metrics().visualization_engagement.len(), 0);
    assert_eq!(tracker.get_metrics().engagement_correlation.len(), 0);
    assert_eq!(tracker.get_metrics().contribution_effectiveness.len(), 0);
    assert_eq!(tracker.get_metrics().community_validation.len(), 0);
    assert_eq!(tracker.get_metrics().feedback_data.len(), 0);
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
    assert_eq!(engagement.interaction_count, 15);
    assert_eq!(engagement.quality_score, 0.85);
    assert_eq!(engagement.decision_confidence, Some(0.9));
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
fn test_track_contribution_effectiveness() {
    let mut tracker = CauseImpactTracker::new(DataSharingLevel::Standard);
    
    let result = tracker.track_contribution_effectiveness(
        "user_123",
        Some("cause_viz_1".to_string()),
        "Increased monthly contribution",
        Some(9),
        Some(0.85),
    );
    
    assert!(result.is_ok());
    assert_eq!(tracker.get_metrics().contribution_effectiveness.len(), 1);
    
    let effectiveness = &tracker.get_metrics().contribution_effectiveness[0];
    assert_eq!(effectiveness.user_id, "hashed_user_123");
    assert_eq!(effectiveness.influencing_viz, Some("cause_viz_1".to_string()));
    assert_eq!(effectiveness.contribution_decision, "Increased monthly contribution");
    assert_eq!(effectiveness.quality, Some(9));
    assert_eq!(effectiveness.impact, Some(0.85));
}

#[test]
fn test_record_community_validation() {
    let mut tracker = CauseImpactTracker::new(DataSharingLevel::Standard);
    
    let result = tracker.record_community_validation(
        "user_123",
        "cause_viz_1",
        ValidationType::Endorsement,
        "Great visualization!",
        Some("Education cause".to_string()),
    );
    
    assert!(result.is_ok());
    assert_eq!(tracker.get_metrics().community_validation.len(), 1);
    
    let validation = &tracker.get_metrics().community_validation[0];
    assert_eq!(validation.user_id, "hashed_user_123");
    assert_eq!(validation.viz_id, "cause_viz_1");
    assert_eq!(validation.validation_type, ValidationType::Endorsement);
    assert_eq!(validation.content, "Great visualization!");
    assert_eq!(validation.cause_context, Some("Education cause".to_string()));
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
fn test_feedback_collection() {
    use cause_impact_tracker::FeedbackCollector;
    
    let mut collector = FeedbackCollector::new(DataSharingLevel::Standard);
    
    let result = collector.collect_quick_feedback(
        "user_123",
        "cause_viz_1",
        true,
    );
    
    assert!(result.is_ok());
    assert_eq!(collector.get_feedback().len(), 1);
    
    let feedback = &collector.get_feedback()[0];
    assert_eq!(feedback.user_id, "hashed_user_123");
    assert_eq!(feedback.viz_id, "cause_viz_1");
    assert_eq!(feedback.helpful, true);
    assert_eq!(feedback.rating, 5);
}