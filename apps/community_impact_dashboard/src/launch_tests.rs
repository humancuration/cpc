//! Tests for the launch module components
//!
//! This module contains integration tests for the launch preparation system.

#[cfg(test)]
mod tests {
    use crate::launch::readiness::LaunchReadinessChecklist;
    use crate::launch::notification::{CommunityNotifier, NotificationType};
    use crate::launch::rollout::{RolloutManager, ParticipantRole};
    use crate::launch::metrics::LaunchMetrics;
    use crate::launch::facilitator::FacilitatorToolkit;
    use crate::launch::celebration::CommunityCelebration;
    use crate::launch::coordinator::LaunchCoordinator;
    use crate::feedback::FeedbackCollector;
    
    #[test]
    fn test_launch_module_integration() {
        // Test that all launch components can be created and work together
        let readiness = LaunchReadinessChecklist::new();
        assert!(!readiness.is_ready());
        
        let notifier = CommunityNotifier::new();
        assert_eq!(notifier.get_notification_stats().total_sent, 0);
        
        let rollout = RolloutManager::new();
        assert_eq!(rollout.get_current_phase().name, "beta");
        
        let metrics = LaunchMetrics::new();
        assert_eq!(metrics.get_metrics_by_category(crate::launch::metrics::MetricCategory::Adoption).len(), 0);
        
        let facilitator_toolkit = FacilitatorToolkit::new();
        assert!(!facilitator_toolkit.get_preparation_resources().is_empty());
        
        let celebration = CommunityCelebration::new();
        assert_eq!(celebration.generate_celebration_report().total_achievements, 0);
    }
    
    #[test]
    fn test_launch_coordinator() {
        let coordinator = LaunchCoordinator::new();
        assert!(!coordinator.check_launch_readiness().ready);
        
        // Test adding a rollout participant
        coordinator.get_rollout_manager().add_participant(
            "test_user".to_string(), 
            ParticipantRole::CommunityMember
        );
        
        // Test recording a metric
        coordinator.get_launch_metrics().record_metric(
            "test_metric", 
            42.0, 
            crate::launch::metrics::MetricCategory::Adoption
        );
        
        assert!(coordinator.get_launch_metrics().get_metric("test_metric").is_some());
    }
    
    #[test]
    fn test_facilitator_toolkit() {
        let toolkit = FacilitatorToolkit::new();
        
        // Test getting resources by type
        let workshop_templates = toolkit.get_resources_by_type(
            crate::launch::facilitator::ResourceType::WorkshopTemplate
        );
        assert!(!workshop_templates.is_empty());
        
        // Test searching resources
        let search_results = toolkit.search_resources("workshop");
        assert!(!search_results.is_empty());
    }
    
    #[test]
    fn test_notification_system() {
        let mut notifier = CommunityNotifier::new();
        
        // Test sending a notification
        let result = notifier.send_notification(
            NotificationType::Launch,
            "Test Launch",
            "This is a test notification",
            None
        );
        assert!(result.is_ok());
        assert_eq!(notifier.get_notification_stats().total_sent, 1);
    }
    
    #[test]
    fn test_rollout_management() {
        let mut rollout = RolloutManager::new();
        
        // Test adding participants
        rollout.add_participant("admin_user".to_string(), ParticipantRole::Admin);
        rollout.add_participant("beta_user".to_string(), ParticipantRole::BetaTester);
        
        // Admins should always have access
        assert!(rollout.has_access("admin_user"));
        
        // Beta users should have access in beta phase
        assert!(rollout.has_access("beta_user"));
        
        // Regular users should not have access in beta phase
        rollout.add_participant("regular_user".to_string(), ParticipantRole::CommunityMember);
        assert!(!rollout.has_access("regular_user"));
    }
}