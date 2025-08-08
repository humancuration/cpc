//! Launch system for the Unified Community Impact Dashboard
//!
//! This module orchestrates the community launch of the dashboard,
//! including execution phases, community experience, and support systems.

// Launch execution system
pub mod execution;

// Community launch experience
pub mod experience;

// Launch support system
pub mod support;

// Re-export key components for easier access
pub use execution::{
    LaunchExecutionSystem,
    LaunchPhase,
    CommunityConsent,
    LaunchStatus,
    ConsentStatus,
    LaunchProgress,
    LaunchError,
};

pub use experience::{
    CommunityLaunchExperience,
    WelcomeExperience,
    LaunchAnnouncement,
    CommunityStory,
    CelebrationEvent,
    OwnershipTransfer,
    ExperienceError,
};

pub use support::{
    LaunchSupportSystem,
    HelpDesk,
    IssueTracker,
    KnowledgeBase,
    FeedbackTriage,
    TranslationSupport,
    SupportError,
};

/// Initialize the launch system
pub fn initialize_launch_system() -> LaunchSystem {
    LaunchSystem::new()
}

/// Main launch system struct that combines all launch components
pub struct LaunchSystem {
    pub execution: LaunchExecutionSystem,
    pub experience: CommunityLaunchExperience,
    pub support: LaunchSupportSystem,
}

impl LaunchSystem {
    /// Create a new launch system
    pub fn new() -> Self {
        Self {
            execution: LaunchExecutionSystem::new(),
            experience: CommunityLaunchExperience::new(),
            support: LaunchSupportSystem::new(),
        }
    }

    /// Get launch execution system
    pub fn get_execution(&self) -> &LaunchExecutionSystem {
        &self.execution
    }

    /// Get mutable launch execution system
    pub fn get_execution_mut(&mut self) -> &mut LaunchExecutionSystem {
        &mut self.execution
    }

    /// Get community launch experience
    pub fn get_experience(&self) -> &CommunityLaunchExperience {
        &self.experience
    }

    /// Get mutable community launch experience
    pub fn get_experience_mut(&mut self) -> &mut CommunityLaunchExperience {
        &mut self.experience
    }

    /// Get launch support system
    pub fn get_support(&self) -> &LaunchSupportSystem {
        &self.support
    }

    /// Get mutable launch support system
    pub fn get_support_mut(&mut self) -> &mut LaunchSupportSystem {
        &mut self.support
    }

    /// Start the community launch process
    pub fn start_community_launch(&mut self) -> Result<(), LaunchError> {
        self.execution.start_launch()
    }

    /// Advance to the next launch phase
    pub fn advance_phase(&mut self) -> Result<LaunchPhase, LaunchError> {
        self.execution.advance_phase()
    }

    /// Get current launch phase
    pub fn get_current_phase(&self) -> LaunchPhase {
        self.execution.get_current_phase()
    }

    /// Check if community consent has been granted for current phase
    pub fn check_community_consent(&self) -> ConsentStatus {
        self.execution.check_community_consent()
    }

    /// Get launch progress
    pub fn get_launch_progress(&self) -> LaunchProgress {
        self.execution.get_progress()
    }

    /// Generate launch status report
    pub fn generate_launch_report(&self) -> LaunchReport {
        let execution_status = self.execution.get_status();
        let experience_metrics = self.experience.get_experience_metrics();
        let support_metrics = self.support.get_support_metrics();
        
        LaunchReport {
            generated_at: chrono::Utc::now(),
            execution_status,
            experience_metrics,
            support_metrics,
        }
    }
}

/// Launch status report
#[derive(Debug, Clone)]
pub struct LaunchReport {
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub execution_status: LaunchStatus,
    pub experience_metrics: ExperienceMetrics,
    pub support_metrics: SupportMetrics,
}

/// Experience metrics
#[derive(Debug, Clone)]
pub struct ExperienceMetrics {
    pub total_welcome_experiences: usize,
    pub total_announcements: usize,
    pub total_stories_shared: usize,
    pub total_celebrations: usize,
    pub total_ownership_transfers: usize,
}

/// Support metrics
#[derive(Debug, Clone)]
pub struct SupportMetrics {
    pub total_support_requests: usize,
    pub resolved_issues: usize,
    pub pending_issues: usize,
    pub knowledge_base_articles: usize,
    pub feedback_items: usize,
    pub translation_requests: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_launch_system_initialization() {
        let system = LaunchSystem::new();
        assert_eq!(system.get_current_phase(), LaunchPhase::PreLaunch);
        assert_eq!(system.check_community_consent(), ConsentStatus::Pending);
    }

    #[test]
    fn test_launch_system_getters() {
        let mut system = LaunchSystem::new();
        
        // Test execution getters
        let execution = system.get_execution();
        assert_eq!(execution.get_current_phase(), LaunchPhase::PreLaunch);
        
        let execution_mut = system.get_execution_mut();
        assert!(execution_mut.start_launch().is_ok());
        
        // Test experience getters
        let experience = system.get_experience();
        assert_eq!(experience.get_experience_metrics().total_welcome_experiences, 0);
        
        // Test support getters
        let support = system.get_support();
        assert_eq!(support.get_support_metrics().total_support_requests, 0);
    }

    #[test]
    fn test_launch_system_operations() {
        let mut system = LaunchSystem::new();
        
        // Test starting launch
        assert!(system.start_community_launch().is_ok());
        assert_eq!(system.get_current_phase(), LaunchPhase::Beta);
        
        // Test advancing phase
        let next_phase = system.advance_phase();
        assert!(next_phase.is_ok());
        assert_eq!(next_phase.unwrap(), LaunchPhase::EarlyAdopters);
        
        // Test launch progress
        let progress = system.get_launch_progress();
        assert!(progress.current_phase >= 1);
        assert!(progress.total_phases >= 4);
    }

    #[test]
    fn test_launch_report_generation() {
        let system = LaunchSystem::new();
        let report = system.generate_launch_report();
        
        assert!(report.generated_at <= chrono::Utc::now());
        assert_eq!(report.execution_status.current_phase, LaunchPhase::PreLaunch);
        assert_eq!(report.experience_metrics.total_welcome_experiences, 0);
        assert_eq!(report.support_metrics.total_support_requests, 0);
    }
}