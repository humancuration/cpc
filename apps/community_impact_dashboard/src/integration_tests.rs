//! Integration tests for the Unified Community Impact Dashboard
//!
//! This module contains integration tests that verify the correct interaction
//! between different components of the dashboard system.

#[cfg(test)]
mod integration_tests {
    use crate::launch::coordinator::LaunchCoordinator;
    use crate::launch::readiness::LaunchReadinessChecklist;
    use crate::launch::notification::CommunityNotifier;
    use crate::launch::rollout::RolloutManager;
    use crate::launch::metrics::LaunchMetrics;
    use crate::launch::facilitator::FacilitatorToolkit;
    use crate::launch::celebration::CommunityCelebration;
    use crate::launch::feedback::LaunchFeedbackIntegration;
    use crate::feedback::FeedbackCollector;
    
    /// Test that all launch components can be instantiated and work together
    #[test]
    fn test_launch_components_integration() {
        // Create all launch components
        let mut coordinator = LaunchCoordinator::new();
        let readiness = LaunchReadinessChecklist::new();
        let notifier = CommunityNotifier::new();
        let mut rollout = RolloutManager::new();
        let metrics = LaunchMetrics::new();
        let facilitator_toolkit = FacilitatorToolkit::new();
        let celebration = CommunityCelebration::new();
        let feedback_integration = LaunchFeedbackIntegration::new(FeedbackCollector::new());
        
        // Verify all components were created successfully
        assert!(coordinator.is_ready());
        assert_eq!(readiness.get_status(), crate::launch::readiness::ReadinessStatus::NotStarted);
        assert!(notifier.is_configured());
        assert_eq!(rollout.get_current_phase(), crate::launch::rollout::RolloutPhase::Beta);
        assert_eq!(metrics.get_all_metrics().len(), 0);
        assert!(facilitator_toolkit.get_all_resources().len() > 0);
        assert_eq!(celebration.get_all_achievements().len(), 0);
        assert_eq!(feedback_integration.get_launch_feedback().len(), 0);
        
        // Test coordinator integration
        coordinator.record_metric("test_integration", 1.0, crate::launch::metrics::MetricCategory::Adoption);
        assert_eq!(coordinator.get_report().metrics_report.metrics.len(), 1);
        
        println!("✅ All launch components integrate successfully");
    }
    
    /// Test that the launch coordinator can manage the full launch lifecycle
    #[test]
    fn test_launch_coordinator_lifecycle() {
        let mut coordinator = LaunchCoordinator::new();
        
        // Simulate launch preparation
        let preparation_result = coordinator.prepare_for_launch();
        assert!(preparation_result.success);
        
        // Simulate launch execution
        coordinator.execute_launch();
        assert!(coordinator.is_launched());
        
        // Simulate post-launch activities
        coordinator.record_metric("post_launch_engagement", 85.5, crate::launch::metrics::MetricCategory::Adoption);
        coordinator.record_event(
            crate::launch::metrics::LaunchEventType::CommunityValidation,
            "First community validation session completed",
            Some("user_123".to_string())
        );
        
        let report = coordinator.get_report();
        assert!(report.metrics_report.metrics.len() > 0);
        assert!(report.rollout_stats.participants.len() > 0);
        
        println!("✅ Launch coordinator manages full lifecycle successfully");
    }
    
    /// Test that facilitator resources are properly integrated
    #[test]
    fn test_facilitator_resources_integration() {
        let toolkit = FacilitatorToolkit::new();
        let resources = toolkit.get_all_resources();
        
        // Verify we have the expected resource types
        let workshop_templates = toolkit.get_resources_by_type(crate::launch::facilitator::ResourceType::WorkshopTemplate);
        let guides = toolkit.get_resources_by_type(crate::launch::facilitator::ResourceType::Guide);
        let troubleshooting = toolkit.get_resources_by_type(crate::launch::facilitator::ResourceType::Troubleshooting);
        
        assert!(workshop_templates.len() > 0);
        assert!(guides.len() > 0);
        assert!(troubleshooting.len() > 0);
        
        // Verify specific resources exist
        assert!(resources.iter().any(|r| r.id == "introduction_workshop"));
        assert!(resources.iter().any(|r| r.id == "validation_workshop"));
        assert!(resources.iter().any(|r| r.id == "facilitation_basics"));
        assert!(resources.iter().any(|r| r.id == "troubleshooting_guide"));
        
        println!("✅ Facilitator resources are properly integrated");
    }
}