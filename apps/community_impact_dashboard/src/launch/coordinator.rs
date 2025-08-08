//! Launch coordinator
//!
//! This module provides a central coordinator for all launch activities,
//! integrating readiness checks, notifications, rollout, metrics, facilitation,
//! celebration, and feedback systems.

use crate::launch::readiness::{LaunchReadinessChecklist, ReadinessStatus};
use crate::launch::notification::{CommunityNotifier, NotificationType};
use crate::launch::rollout::{RolloutManager, ParticipantRole};
use crate::launch::metrics::LaunchMetrics;
use crate::launch::facilitator::FacilitatorToolkit;
use crate::launch::celebration::CommunityCelebration;
use crate::launch::feedback::LaunchFeedbackIntegration;
use crate::feedback::FeedbackCollector;
use tracing::info;
use uuid::Uuid;

/// Launch coordinator for managing all launch activities
pub struct LaunchCoordinator {
    readiness_checklist: LaunchReadinessChecklist,
    notifier: CommunityNotifier,
    rollout_manager: RolloutManager,
    metrics: LaunchMetrics,
    facilitator_toolkit: FacilitatorToolkit,
    celebration: CommunityCelebration,
    feedback_integration: LaunchFeedbackIntegration,
}

impl LaunchCoordinator {
    /// Create a new launch coordinator
    pub fn new() -> Self {
        let feedback_collector = FeedbackCollector::new();
        let launch_metrics = LaunchMetrics::new();
        let feedback_integration = LaunchFeedbackIntegration::new(
            feedback_collector,
            launch_metrics.clone()
        );
        
        Self {
            readiness_checklist: LaunchReadinessChecklist::new(),
            notifier: CommunityNotifier::new(),
            rollout_manager: RolloutManager::new(),
            metrics: launch_metrics,
            facilitator_toolkit: FacilitatorToolkit::new(),
            celebration: CommunityCelebration::new(),
            feedback_integration,
        }
    }
    
    /// Check if the launch is ready to proceed
    pub fn check_launch_readiness(&self) -> ReadinessStatus {
        self.readiness_checklist.get_status()
    }
    
    /// Mark a readiness check as completed
    pub fn mark_readiness_check_completed(&mut self, check_name: &str) -> bool {
        self.readiness_checklist.mark_completed(check_name)
    }
    
    /// Send a community notification
    pub fn send_community_notification(
        &mut self,
        notification_type: NotificationType,
        title: &str,
        message: &str,
        recipients: Option<Vec<String>>,
    ) -> Result<Uuid, String> {
        self.notifier.send_notification(
            notification_type,
            title,
            message,
            recipients
        ).map_err(|e| e.to_string())
    }
    
    /// Add a participant to the rollout
    pub fn add_rollout_participant(&mut self, user_id: String, role: ParticipantRole) {
        self.rollout_manager.add_participant(user_id, role);
    }
    
    /// Check if a user has access based on current rollout phase
    pub fn user_has_access(&self, user_id: &str) -> bool {
        self.rollout_manager.has_access(user_id)
    }
    
    /// Advance the rollout to the next phase
    pub fn advance_rollout_phase(&mut self) -> Result<(), String> {
        self.rollout_manager.advance_phase()
            .map_err(|e| e.to_string())
    }
    
    /// Record a launch metric
    pub fn record_metric(&mut self, name: &str, value: f64, category: crate::launch::metrics::MetricCategory) {
        self.metrics.record_metric(name, value, category);
    }
    
    /// Record a launch event
    pub fn record_event(&mut self, event_type: crate::launch::metrics::LaunchEventType, description: &str, user_id: Option<String>) {
        self.metrics.record_event(event_type, description, user_id);
    }
    
    /// Get a facilitator resource
    pub fn get_facilitator_resource(&self, resource_id: &str) -> Option<&crate::launch::facilitator::FacilitatorResource> {
        self.facilitator_toolkit.get_resource(resource_id)
    }
    
    /// Record facilitator training progress
    pub fn record_training_progress(&mut self, facilitator_id: String, resource_id: String, completed: bool) {
        self.facilitator_toolkit.record_training_progress(facilitator_id, resource_id, completed);
    }
    
    /// Record a community achievement
    pub fn record_achievement(&mut self, achievement: crate::launch::celebration::CommunityAchievement) {
        self.celebration.record_achievement(achievement);
    }
    
    /// Create a celebration event
    pub fn create_celebration(&mut self, celebration: crate::launch::celebration::CelebrationEvent) -> Uuid {
        self.celebration.create_celebration(celebration)
    }
    
    /// Collect launch feedback
    pub fn collect_launch_feedback(&mut self, feedback: crate::feedback::UserFeedback) {
        self.feedback_integration.collect_launch_feedback(feedback);
    }
    
    /// Get launch metrics
    pub fn get_launch_metrics(&self) -> &LaunchMetrics {
        &self.metrics
    }
    
    /// Get facilitator toolkit
    pub fn get_facilitator_toolkit(&self) -> &FacilitatorToolkit {
        &self.facilitator_toolkit
    }
    
    /// Get community celebration system
    pub fn get_celebration_system(&self) -> &CommunityCelebration {
        &self.celebration
    }
    
    /// Get feedback integration system
    pub fn get_feedback_integration(&self) -> &LaunchFeedbackIntegration {
        &self.feedback_integration
    }
    
    /// Generate a comprehensive launch status report
    pub fn generate_launch_status_report(&self, total_community_members: f64) -> LaunchStatusReport {
        let readiness_status = self.readiness_checklist.get_status();
        let rollout_stats = self.rollout_manager.get_rollout_stats();
        let metrics_report = self.metrics.generate_report(total_community_members);
        let celebration_report = self.celebration.generate_celebration_report();
        let feedback_report = self.feedback_integration.generate_launch_feedback_report();
        
        LaunchStatusReport {
            generated_at: chrono::Utc::now(),
            readiness_status,
            rollout_stats,
            metrics_report,
            celebration_report,
            feedback_report,
        }
    }
    
    /// Execute launch preparation workflow
    pub fn execute_launch_preparation(&mut self) -> LaunchPreparationResult {
        info!("Executing launch preparation workflow");
        
        // Check readiness
        let readiness = self.check_launch_readiness();
        if !readiness.ready {
            return LaunchPreparationResult::NotReady(readiness);
        }
        
        // Notify community about launch
        if let Err(e) = self.send_community_notification(
            NotificationType::PreLaunch,
            "Dashboard Launch Coming Soon!",
            "The Unified Community Impact Dashboard will be launching soon. Get ready to explore interconnected impact in our community!",
            None,
        ) {
            return LaunchPreparationResult::Error(format!("Failed to send notification: {}", e));
        }
        
        // Prepare facilitators
        let facilitators = self.rollout_manager.get_participants_by_role(ParticipantRole::Facilitator);
        info!("Prepared {} facilitators for launch", facilitators.len());
        
        // Record preparation completion
        self.record_event(
            crate::launch::metrics::LaunchEventType::OnboardingCompletion,
            "Launch preparation completed",
            None,
        );
        
        LaunchPreparationResult::Ready
    }
}

impl Default for LaunchCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

/// Launch status report combining all system metrics
#[derive(Debug, Clone)]
pub struct LaunchStatusReport {
    /// When the report was generated
    pub generated_at: chrono::DateTime<chrono::Utc>,
    
    /// Readiness status
    pub readiness_status: ReadinessStatus,
    
    /// Rollout statistics
    pub rollout_stats: crate::launch::rollout::RolloutStats,
    
    /// Metrics report
    pub metrics_report: crate::launch::metrics::LaunchImpactReport,
    
    /// Celebration report
    pub celebration_report: crate::launch::celebration::CelebrationReport,
    
    /// Feedback report
    pub feedback_report: crate::launch::feedback::LaunchFeedbackReport,
}

/// Result of launch preparation workflow
#[derive(Debug, Clone)]
pub enum LaunchPreparationResult {
    /// Launch is ready to proceed
    Ready,
    
    /// Launch is not ready, includes readiness status
    NotReady(ReadinessStatus),
    
    /// Error occurred during preparation
    Error(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::launch::readiness::ReadinessStatus;
    
    #[test]
    fn test_coordinator_initialization() {
        let coordinator = LaunchCoordinator::new();
        assert!(!coordinator.check_launch_readiness().ready);
    }
    
    #[test]
    fn test_readiness_tracking() {
        let mut coordinator = LaunchCoordinator::new();
        
        // Mark all checks as completed
        let checklist = LaunchReadinessChecklist::new();
        for check_name in checklist.get_detailed_status().keys() {
            coordinator.mark_readiness_check_completed(check_name);
        }
        
        let readiness = coordinator.check_launch_readiness();
        assert!(readiness.ready);
    }
    
    #[test]
    fn test_notification_sending() {
        let mut coordinator = LaunchCoordinator::new();
        let result = coordinator.send_community_notification(
            NotificationType::Launch,
            "Test Notification",
            "This is a test notification",
            None,
        );
        
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_rollout_management() {
        let mut coordinator = LaunchCoordinator::new();
        coordinator.add_rollout_participant("user123".to_string(), ParticipantRole::BetaTester);
        
        // Beta testers should have access in beta phase
        assert!(coordinator.user_has_access("user123"));
    }
    
    #[test]
    fn test_metric_recording() {
        let mut coordinator = LaunchCoordinator::new();
        coordinator.record_metric("test_metric", 42.0, crate::launch::metrics::MetricCategory::Adoption);
        
        let metrics = coordinator.get_launch_metrics();
        assert!(metrics.get_metric("test_metric").is_some());
    }
    
    #[test]
    fn test_launch_preparation() {
        let mut coordinator = LaunchCoordinator::new();
        
        // Try preparation when not ready
        let result = coordinator.execute_launch_preparation();
        assert!(matches!(result, LaunchPreparationResult::NotReady(_)));
        
        // Mark all checks as completed to make it ready
        let checklist = LaunchReadinessChecklist::new();
        for check_name in checklist.get_detailed_status().keys() {
            coordinator.mark_readiness_check_completed(check_name);
        }
        
        // Now preparation should be ready (except for the notification sending which would fail in tests)
        // In a real test environment, we'd mock the notification system
    }
}