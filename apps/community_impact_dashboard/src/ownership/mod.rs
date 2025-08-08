//! Community ownership framework for the Unified Community Impact Dashboard
//!
//! This module provides tools and processes for transferring and maintaining
//! community ownership of dashboard components, features, and governance.

// Governance documentation and principles
pub mod governance;

// Community decision-making workflows
pub mod decision_making;

// Feature voting and prioritization
pub mod feature_voting;

// Community-led enhancement processes
pub mod community_enhancements;

// Ownership transfer processes
pub mod transfer;

// Re-export key components for easier access
pub use governance::{
    GovernanceFramework,
    GovernancePrinciple,
    GovernanceRole,
    Responsibility,
    DecisionMakingProcess,
    GovernanceDocument,
    GovernanceDocumentCategory,
    DocumentStatus,
    PrinciplePriority,
    Permission,
    ResponsibilityCategory,
    ProcessType,
    VotingMethod,
    DecisionTimeline,
    ApprovalStatus,
    GovernanceChange,
    GovernanceChangeType,
    GovernanceStatistics,
    GovernanceReport,
    DocumentUpdate,
    GovernanceError,
};

pub use decision_making::{
    DecisionMakingSystem,
    Proposal,
    ProposalCategory,
    ProposalStatus,
    Discussion,
    Comment,
    ConsensusProcess,
    ConsensusType,
    ConsensusStatus,
    Participant,
    Position,
    Stance,
    ConsensusOutcome,
    ConsensusTimeline,
    DecisionRecord,
    DecisionType,
    DecisionStatus,
    DecisionStatistics,
    DecisionReport,
    DecisionError,
};

pub use feature_voting::{
    FeatureVotingSystem,
    Feature,
    FeatureCategory,
    FeatureStatus,
    VotingPeriod,
    Vote,
    VoteType,
    VotingMethod as FeatureVotingMethod,
    VotingResults,
    VotingRecord,
    VotingOutcome,
    ImpactAssessment,
    ImpactLevel,
    RiskLevel,
    ImpactDependency,
    DependencyImpactType,
    RoadmapItem,
    RoadmapStatus,
    PriorityLevel as FeaturePriorityLevel,
    VotingStatistics,
    VotingReport,
    VotingError,
};

pub use community_enhancements::{
    CommunityEnhancementSystem,
    EnhancementProposal,
    EnhancementCategory,
    EnhancementStatus,
    EnhancementDiscussion,
    EnhancementComment,
    EnhancementImplementation,
    ImplementationStatus,
    ImplementationMilestone,
    MilestoneStatus,
    Contribution,
    ContributionType,
    EnhancementReview,
    ReviewRating,
    EnhancementRecord,
    EnhancementRecordType,
    EnhancementStatistics,
    EnhancementReport,
    PriorityLevel as EnhancementPriorityLevel,
    EnhancementError,
};

pub use transfer::{
    OwnershipTransferSystem,
    TransferRequest,
    TransferRequestStatus,
    TransferProcess,
    TransferProcessStatus,
    TransferRequirement,
    RequirementStatus,
    TransferRequirementStatus,
    TransferType,
    ProgressNote,
    NoteVisibility,
    TransferRecord,
    TransferPolicy,
    CommunitySteward,
    StewardExpertise,
    TransferHistoryEntry,
    TransferHistoryType,
    PriorityLevel as TransferPriorityLevel,
    TransferStatistics,
    TransferReport,
    TransferError,
};

/// Initialize the community ownership framework
pub fn initialize_ownership_framework() -> OwnershipFramework {
    OwnershipFramework::new()
}

/// Main ownership framework struct that combines all ownership components
pub struct OwnershipFramework {
    pub governance: GovernanceFramework,
    pub decision_making: DecisionMakingSystem,
    pub feature_voting: FeatureVotingSystem,
    pub enhancements: CommunityEnhancementSystem,
    pub transfer_system: OwnershipTransferSystem,
}

impl OwnershipFramework {
    /// Create a new ownership framework
    pub fn new() -> Self {
        Self {
            governance: GovernanceFramework::new(),
            decision_making: DecisionMakingSystem::new(),
            feature_voting: FeatureVotingSystem::new(),
            enhancements: CommunityEnhancementSystem::new(),
            transfer_system: OwnershipTransferSystem::new(),
        }
    }

    /// Get governance framework
    pub fn get_governance(&self) -> &GovernanceFramework {
        &self.governance
    }

    /// Get mutable governance framework
    pub fn get_governance_mut(&mut self) -> &mut GovernanceFramework {
        &mut self.governance
    }

    /// Get decision making system
    pub fn get_decision_making(&self) -> &DecisionMakingSystem {
        &self.decision_making
    }

    /// Get mutable decision making system
    pub fn get_decision_making_mut(&mut self) -> &mut DecisionMakingSystem {
        &mut self.decision_making
    }

    /// Get feature voting system
    pub fn get_feature_voting(&self) -> &FeatureVotingSystem {
        &self.feature_voting
    }

    /// Get mutable feature voting system
    pub fn get_feature_voting_mut(&mut self) -> &mut FeatureVotingSystem {
        &mut self.feature_voting
    }

    /// Get community enhancements system
    pub fn get_enhancements(&self) -> &CommunityEnhancementSystem {
        &self.enhancements
    }

    /// Get mutable community enhancements system
    pub fn get_enhancements_mut(&mut self) -> &mut CommunityEnhancementSystem {
        &mut self.enhancements
    }

    /// Get ownership transfer system
    pub fn get_transfer_system(&self) -> &OwnershipTransferSystem {
        &self.transfer_system
    }

    /// Get mutable ownership transfer system
    pub fn get_transfer_system_mut(&mut self) -> &mut OwnershipTransferSystem {
        &mut self.transfer_system
    }

    /// Generate a comprehensive ownership report
    pub fn generate_ownership_report(&self) -> OwnershipReport {
        let governance_report = self.governance.generate_governance_report();
        let decision_report = self.decision_making.generate_decision_report();
        let voting_report = self.feature_voting.generate_voting_report();
        let enhancement_report = self.enhancements.generate_enhancement_report();
        let transfer_report = self.transfer_system.generate_transfer_report();
        
        OwnershipReport {
            generated_at: chrono::Utc::now(),
            governance_report,
            decision_report,
            voting_report,
            enhancement_report,
            transfer_report,
        }
    }

    /// Get overall ownership statistics
    pub fn get_ownership_statistics(&self) -> OwnershipStatistics {
        let governance_stats = self.governance.get_statistics();
        let decision_stats = self.decision_making.get_statistics();
        let voting_stats = self.feature_voting.get_statistics();
        let enhancement_stats = self.enhancements.get_statistics();
        let transfer_stats = self.transfer_system.get_statistics();
        
        OwnershipStatistics {
            governance_stats,
            decision_stats,
            voting_stats,
            enhancement_stats,
            transfer_stats,
        }
    }
}

/// Comprehensive ownership report
#[derive(Debug, Clone)]
pub struct OwnershipReport {
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub governance_report: GovernanceReport,
    pub decision_report: DecisionReport,
    pub voting_report: VotingReport,
    pub enhancement_report: EnhancementReport,
    pub transfer_report: TransferReport,
}

/// Overall ownership statistics
#[derive(Debug, Clone)]
pub struct OwnershipStatistics {
    pub governance_stats: GovernanceStatistics,
    pub decision_stats: DecisionStatistics,
    pub voting_stats: VotingStatistics,
    pub enhancement_stats: EnhancementStatistics,
    pub transfer_stats: TransferStatistics,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ownership_framework_initialization() {
        let framework = OwnershipFramework::new();
        assert!(framework.governance.get_principles().is_empty());
        assert!(framework.decision_making.get_proposals_by_status(ProposalStatus::Proposed).is_empty());
        assert!(framework.feature_voting.get_features_by_status(FeatureStatus::Proposed).is_empty());
        assert!(framework.enhancements.get_proposals_by_status(EnhancementStatus::Proposed).is_empty());
        assert!(framework.transfer_system.get_transfer_requests(&TransferRequestStatus::Pending).is_empty());
    }

    #[test]
    fn test_ownership_framework_getters() {
        let mut framework = OwnershipFramework::new();
        
        // Test governance getters
        let governance = framework.get_governance();
        assert_eq!(governance.get_principles().len(), 0);
        
        let governance_mut = framework.get_governance_mut();
        let principle = GovernancePrinciple::new(
            "test_principle".to_string(),
            "Test Principle".to_string(),
            "A test principle".to_string(),
            PrinciplePriority::Fundamental,
        );
        governance_mut.add_principle(principle);
        assert_eq!(framework.governance.get_principles().len(), 1);
        
        // Test decision making getters
        let decision_making = framework.get_decision_making();
        assert_eq!(decision_making.get_proposals_by_status(ProposalStatus::Proposed).len(), 0);
        
        // Test feature voting getters
        let feature_voting = framework.get_feature_voting();
        assert_eq!(feature_voting.get_features_by_status(FeatureStatus::Proposed).len(), 0);
        
        // Test enhancements getters
        let enhancements = framework.get_enhancements();
        assert_eq!(enhancements.get_proposals_by_status(EnhancementStatus::Proposed).len(), 0);
        
        // Test transfer system getters
        let transfer_system = framework.get_transfer_system();
        assert_eq!(transfer_system.get_requests_by_status(TransferRequestStatus::Pending).len(), 0);
    }

    #[test]
    fn test_ownership_report_generation() {
        let framework = OwnershipFramework::new();
        let report = framework.generate_ownership_report();
        
        assert!(report.generated_at <= chrono::Utc::now());
        assert_eq!(report.governance_report.statistics.total_principles, 0);
        assert_eq!(report.decision_report.statistics.total_proposals, 0);
        assert_eq!(report.voting_report.statistics.total_features, 0);
        assert_eq!(report.enhancement_report.statistics.total_proposals, 0);
        assert_eq!(report.transfer_report.statistics.total_requests, 0);
    }

    #[test]
    fn test_ownership_statistics() {
        let framework = OwnershipFramework::new();
        let stats = framework.get_ownership_statistics();
        
        assert_eq!(stats.governance_stats.total_principles, 0);
        assert_eq!(stats.decision_stats.total_proposals, 0);
        assert_eq!(stats.voting_stats.total_features, 0);
        assert_eq!(stats.enhancement_stats.total_proposals, 0);
        assert_eq!(stats.transfer_stats.total_requests, 0);
    }
}