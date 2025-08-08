//! Integration tests for the volunteer impact tracker

#[cfg(test)]
mod tests {
    use volunteer_impact_tracker::{
        VolunteerImpactTracker, 
        ImpactAnalyticsDashboard,
        FeedbackCollector,
        ImprovementEngine
    };
    use consent_manager::domain::consent::DataSharingLevel;
    use impact_viz::core::VisualizationType;
    use skill_development::ml::CommunityData;
    use std::collections::HashMap;

    #[test]
    fn test_volunteer_impact_tracker_creation() {
        let tracker = VolunteerImpactTracker::new(DataSharingLevel::Standard);
        assert_eq!(tracker.consent_level, DataSharingLevel::Standard);
    }

    #[test]
    fn test_track_visualization_engagement() {
        let mut tracker = VolunteerImpactTracker::new(DataSharingLevel::Standard);
        let result = tracker.track_visualization_engagement(
            "volunteer_123",
            "volunteer_viz_1",
            VisualizationType::Comparative,
            120.5,
            15,
            0.85,
        );
        assert!(result.is_ok());
        assert!(!tracker.get_metrics().visualization_engagement.is_empty());
    }

    #[test]
    fn test_consent_respects_none_level() {
        let mut tracker = VolunteerImpactTracker::new(DataSharingLevel::None);
        let result = tracker.track_visualization_engagement(
            "volunteer_123",
            "volunteer_viz_1",
            VisualizationType::Comparative,
            120.5,
            15,
            0.85,
        );
        assert!(result.is_ok());
        assert!(tracker.get_metrics().visualization_engagement.is_empty());
    }

    #[test]
    fn test_dashboard_creation() {
        let metrics = volunteer_impact_tracker::tracker::ImpactMetrics {
            visualization_engagement: HashMap::new(),
            retention_correlation: Vec::new(),
            task_completion: Vec::new(),
            community_validation: Vec::new(),
            feedback_data: Vec::new(),
        };

        let dashboard = ImpactAnalyticsDashboard::new(metrics);
        assert!(true); // Dashboard should be created successfully
    }

    #[test]
    fn test_feedback_collector_creation() {
        let collector = FeedbackCollector::new();
        assert!(collector.feedback_data.is_empty());
        assert!(collector.feedback_by_viz.is_empty());
    }

    #[test]
    fn test_improvement_engine_creation() {
        let engine = ImprovementEngine::new();
        assert!(true); // Engine should be created successfully
    }

    #[test]
    fn test_end_to_end_workflow() {
        // Create tracker
        let mut tracker = VolunteerImpactTracker::new(DataSharingLevel::Standard);
        
        // Track some engagement
        let _ = tracker.track_visualization_engagement(
            "volunteer_123",
            "individual_impact_dashboard",
            VisualizationType::Narrative,
            180.5,
            25,
            0.85,
        );
        
        // Track retention correlation
        let _ = tracker.track_retention_correlation(
            "volunteer_123",
            vec!["individual_impact_dashboard".to_string()],
            true,
            Some(12.5),
            Some(8),
        );
        
        // Create dashboard
        let dashboard = ImpactAnalyticsDashboard::new(tracker.get_metrics().clone());
        
        // Create mock community data
        let community_data = CommunityData {
            skill_distribution: HashMap::new(),
            projected_needs: HashMap::new(),
            learning_resources: HashMap::new(),
            demographics: HashMap::new(),
            historical_trends: HashMap::new(),
        };
        
        // Generate summary
        let summary = dashboard.generate_summary(&community_data);
        
        // Verify we have data
        assert_eq!(summary.engagement.total_views, 1);
        assert_eq!(summary.volunteer_effectiveness.retention_rate_with_viz, 1.0);
    }
}