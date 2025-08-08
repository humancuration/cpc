//! Community Impact Dashboard Library
//!
//! This crate provides the core functionality for the Unified Community Impact Dashboard,
//! including launch orchestration, community ownership frameworks, and collaborative governance tools.

// Public modules
pub mod launch;
pub mod ownership;

// Re-export key components for easier access
pub use launch::{
    LaunchSystem,
    LaunchPhase,
    ConsentStatus,
    initialize_launch_system,
};

pub use ownership::{
    OwnershipFramework,
    initialize_ownership_framework,
};

pub use launch::execution::{
    LaunchExecutionSystem,
    LaunchProgress,
    LaunchStatus,
    CommunityConsent,
};

pub use launch::experience::{
    CommunityLaunchExperience,
    WelcomeExperience,
    LaunchAnnouncement,
    CommunityStory,
    CelebrationEvent,
    OwnershipTransfer,
};

pub use launch::support::{
    LaunchSupportSystem,
    HelpDesk,
    IssueTracker,
    KnowledgeBase,
    FeedbackTriage,
    TranslationSupport,
};

pub use ownership::governance::{
    GovernanceFramework,
    GovernancePrinciple,
    GovernanceRole,
    Responsibility,
    DecisionMakingProcess,
    GovernanceDocument,
};

pub use ownership::decision_making::{
    DecisionMakingSystem,
    Proposal,
    Discussion,
    ConsensusProcess,
    Participant,
    Position,
};

pub use ownership::feature_voting::{
    FeatureVotingSystem,
    Feature,
    VotingPeriod,
    Vote,
    RoadmapItem,
};

pub use ownership::community_enhancements::{
    CommunityEnhancementSystem,
    EnhancementProposal,
    EnhancementDiscussion,
    EnhancementImplementation,
};

pub use ownership::transfer::{
    OwnershipTransferSystem,
    TransferRequest,
    TransferProcess,
    TransferRecord,
    CommunitySteward,
};

/// Main application struct
pub struct CommunityImpactDashboard {
    launch_system: LaunchSystem,
    ownership_framework: OwnershipFramework,
}

impl CommunityImpactDashboard {
    /// Create a new Community Impact Dashboard
    pub fn new() -> Self {
        Self {
            launch_system: initialize_launch_system(),
            ownership_framework: initialize_ownership_framework(),
        }
    }

    /// Get reference to launch system
    pub fn launch_system(&self) -> &LaunchSystem {
        &self.launch_system
    }

    /// Get mutable reference to launch system
    pub fn launch_system_mut(&mut self) -> &mut LaunchSystem {
        &mut self.launch_system
    }

    /// Get reference to ownership framework
    pub fn ownership_framework(&self) -> &OwnershipFramework {
        &self.ownership_framework
    }

    /// Get mutable reference to ownership framework
    pub fn ownership_framework_mut(&mut self) -> &mut OwnershipFramework {
        &mut self.ownership_framework
    }

    /// Initialize the dashboard systems
    pub fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Initialize systems
        Ok(())
    }

    /// Start the community launch process
    pub fn start_community_launch(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.launch_system.start_community_launch()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    /// Advance to the next launch phase
    pub fn advance_launch_phase(&mut self) -> Result<launch::LaunchPhase, Box<dyn std::error::Error>> {
        self.launch_system.advance_phase()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}

/// Run the Community Impact Dashboard
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut dashboard = CommunityImpactDashboard::new();
    dashboard.initialize()?;
    dashboard.start_community_launch()?;
    Ok(())
}