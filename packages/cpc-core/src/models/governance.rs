//! Governance models for cooperative decision-making and voting
//! 
//! This module implements a governance system that enables users to create, vote on, 
//! and implement proposals that affect content ranking and platform rules. The system
//! uses Rated Choice Voting (RCV) to ensure outcomes reflect majority consensus.
//!
//! Key features:
//! - Proposal creation and lifecycle management
//! - Rated choice voting with weighted votes based on cooperative scores
//! - Governance participation tracking
//! - Integration with cooperative scoring system

// DateTime handling follows the standard defined in [DATETIME_STANDARD.md](../../docs/DATETIME_STANDARD.md)
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Represents a governance proposal that can be voted on by cooperative members
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Proposal {
    pub id: Uuid,
    pub cooperative_id: Uuid,
    pub proposer_id: Uuid,
    pub title: String,
    pub description: String,
    pub status: ProposalStatus,
    pub proposal_type: ProposalType,
    pub options: Vec<String>, // Voting options for rated choice voting
    pub proposed_change: ProposedChange,
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "crate::utils::datetime")]
    pub voting_deadline: DateTime<Utc>,
    #[serde(with = "crate::utils::datetime")]
    pub updated_at: DateTime<Utc>,
    pub quorum_threshold: f64, // Percentage of eligible voters needed
    pub participation_count: i32, // Number of users who have voted
    pub eligible_voter_count: i32, // Total number of eligible voters
}

/// Status of a governance proposal
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProposalStatus {
    Draft,      // Proposal is being drafted, not yet open for voting
    Voting,     // Proposal is open for voting
    Passed,     // Voting period ended, quorum met, and proposal passed
    Failed,     // Voting period ended, and proposal failed (quorum not met or voted down)
    Executed,   // The passed proposal's change has been applied
    Expired,    // Voting period ended without reaching quorum
}

/// Type of governance proposal
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProposalType {
    Feature,    // New feature proposal
    Content,    // Content moderation or ranking changes
    Policy,     // Platform policy changes
    BugFix,     // Bug fix prioritization
    Technical,  // Technical architecture changes
    Community,  // Community guidelines or rules
}

/// Describes the specific change being proposed
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProposedChange {
    pub change_type: String, // Type of change (e.g., "algorithm_update", "policy_change")
    pub target_system: String, // System being changed (e.g., "content_ranking", "moderation")
    pub change_description: String, // Detailed description of the change
    pub implementation_notes: Option<String>, // Technical implementation details
    pub rollback_plan: Option<String>, // How to rollback if needed
    pub impact_assessment: Option<String>, // Expected impact on users/system
}

/// Represents a user's vote on a proposal using rated choice voting
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vote {
    pub id: Uuid,
    pub proposal_id: Uuid,
    pub voter_id: Uuid,
    pub choices: Vec<String>, // Ordered list of choices (rated choice voting)
    pub voting_weight: f64, // Weight based on cooperative score
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "crate::utils::datetime")]
    pub updated_at: DateTime<Utc>,
    pub is_anonymous: bool, // Whether the vote should be anonymous
}

/// Represents the tally results for a proposal using rated choice voting
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteTally {
    pub id: Uuid,
    pub proposal_id: Uuid,
    pub round_number: i32, // Round number in rated choice voting
    pub round_results: std::collections::HashMap<String, VoteCount>, // Option -> vote count
    pub eliminated_options: Vec<String>, // Options eliminated in this round
    pub total_votes: i32,
    pub total_weight: f64, // Total voting weight
    #[serde(with = "crate::utils::datetime")]
    pub calculated_at: DateTime<Utc>,
    pub is_final_round: bool,
    pub winner: Option<String>, // Winner if this is the final round
}

/// Vote count information for an option in a round
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteCount {
    pub vote_count: i32,
    pub weighted_count: f64,
    pub percentage: f64,
}

/// Final voting result for a proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VotingResult {
    pub id: Uuid,
    pub proposal_id: Uuid,
    pub rounds: Vec<VoteTally>,
    pub winner: Option<String>, // Winning option, if any
    pub total_participants: i32,
    pub quorum_met: bool,
    pub final_status: ProposalStatus,
    #[serde(with = "crate::utils::datetime")]
    pub finalized_at: DateTime<Utc>,
}

/// Tracks a user's participation in governance activities
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GovernanceParticipation {
    pub id: Uuid,
    pub user_id: Uuid,
    pub cooperative_id: Uuid,
    pub proposals_created: i32,
    pub votes_cast: i32,
    pub proposals_participated: Vec<Uuid>, // Proposal IDs user has voted on
    pub participation_score: f64, // Score based on governance activity
    #[serde(with = "crate::utils::datetime")]
    pub last_activity: DateTime<Utc>,
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "crate::utils::datetime")]
    pub updated_at: DateTime<Utc>,
}

impl Proposal {
    /// Creates a new proposal in draft status
    pub fn new(
        cooperative_id: Uuid,
        proposer_id: Uuid,
        title: String,
        description: String,
        proposal_type: ProposalType,
        options: Vec<String>,
        proposed_change: ProposedChange,
        voting_deadline: DateTime<Utc>,
        quorum_threshold: f64,
        eligible_voter_count: i32,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            cooperative_id,
            proposer_id,
            title,
            description,
            status: ProposalStatus::Draft,
            proposal_type,
            options,
            proposed_change,
            created_at: now,
            voting_deadline,
            updated_at: now,
            quorum_threshold,
            participation_count: 0,
            eligible_voter_count,
        }
    }

    /// Starts the voting period for the proposal
    pub fn start_voting(&mut self) {
        self.status = ProposalStatus::Voting;
        self.updated_at = Utc::now();
    }

    /// Checks if the proposal has reached quorum
    pub fn has_quorum(&self) -> bool {
        if self.eligible_voter_count == 0 {
            return false;
        }
        let participation_rate = self.participation_count as f64 / self.eligible_voter_count as f64;
        participation_rate >= self.quorum_threshold
    }

    /// Checks if the voting period has expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.voting_deadline
    }

    /// Updates the participation count
    pub fn increment_participation(&mut self) {
        self.participation_count += 1;
        self.updated_at = Utc::now();
    }

    /// Finalizes the proposal with the given status
    pub fn finalize(&mut self, status: ProposalStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }
}

impl Vote {
    /// Creates a new vote with calculated weight based on cooperative score
    pub fn new(
        proposal_id: Uuid,
        voter_id: Uuid,
        choices: Vec<String>,
        cooperative_score: f64,
        is_anonymous: bool,
    ) -> Self {
        let now = Utc::now();
        let voting_weight = Self::calculate_voting_weight(cooperative_score);
        
        Self {
            id: Uuid::new_v4(),
            proposal_id,
            voter_id,
            choices,
            voting_weight,
            created_at: now,
            updated_at: now,
            is_anonymous,
        }
    }

    /// Calculates voting weight based on cooperative score
    /// Base weight of 1.0, with bonus based on cooperative score (capped at 0.5)
    pub fn calculate_voting_weight(cooperative_score: f64) -> f64 {
        1.0 + (cooperative_score * 0.1).min(0.5)
    }

    /// Creates a new vote using a CooperativeScore struct
    pub fn new_with_cooperative_score(
        proposal_id: Uuid,
        voter_id: Uuid,
        choices: Vec<String>,
        cooperative_score: &crate::models::CooperativeScore,
        is_anonymous: bool,
    ) -> Self {
        Self::new(proposal_id, voter_id, choices, cooperative_score.value, is_anonymous)
    }

    /// Updates the vote choices
    pub fn update_choices(&mut self, choices: Vec<String>) {
        self.choices = choices;
        self.updated_at = Utc::now();
    }
}

impl VoteTally {
    /// Creates a new vote tally for a round
    pub fn new(
        proposal_id: Uuid,
        round_number: i32,
        round_results: std::collections::HashMap<String, VoteCount>,
        eliminated_options: Vec<String>,
        total_votes: i32,
        total_weight: f64,
        is_final_round: bool,
        winner: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            proposal_id,
            round_number,
            round_results,
            eliminated_options,
            total_votes,
            total_weight,
            calculated_at: Utc::now(),
            is_final_round,
            winner,
        }
    }
}

impl VotingResult {
    /// Creates a new voting result
    pub fn new(
        proposal_id: Uuid,
        rounds: Vec<VoteTally>,
        winner: Option<String>,
        total_participants: i32,
        quorum_met: bool,
        final_status: ProposalStatus,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            proposal_id,
            rounds,
            winner,
            total_participants,
            quorum_met,
            final_status,
            finalized_at: Utc::now(),
        }
    }
}

impl GovernanceParticipation {
    /// Creates a new governance participation record
    pub fn new(user_id: Uuid, cooperative_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            cooperative_id,
            proposals_created: 0,
            votes_cast: 0,
            proposals_participated: Vec::new(),
            participation_score: 0.0,
            last_activity: now,
            created_at: now,
            updated_at: now,
        }
    }

    /// Records a new proposal creation
    pub fn record_proposal_created(&mut self) {
        self.proposals_created += 1;
        self.last_activity = Utc::now();
        self.updated_at = Utc::now();
        self.recalculate_score();
    }

    /// Records a new vote cast
    pub fn record_vote_cast(&mut self, proposal_id: Uuid) {
        self.votes_cast += 1;
        if !self.proposals_participated.contains(&proposal_id) {
            self.proposals_participated.push(proposal_id);
        }
        self.last_activity = Utc::now();
        self.updated_at = Utc::now();
        self.recalculate_score();
    }

    /// Gets the participation rate (0.0 to 1.0) based on total proposals in the cooperative
    pub fn get_participation_rate(&self, total_proposals: i32) -> f64 {
        if total_proposals == 0 {
            return 0.0;
        }
        self.proposals_participated.len() as f64 / total_proposals as f64
    }

    /// Checks if the user is an active participant (voted in last 30 days)
    pub fn is_active_participant(&self) -> bool {
        let thirty_days_ago = Utc::now() - chrono::Duration::days(30);
        self.last_activity > thirty_days_ago
    }

    /// Gets a contribution factor for cooperative score calculation
    pub fn get_governance_contribution_factor(&self) -> crate::models::ContributionFactor {
        crate::models::ContributionFactor {
            name: "governance_participation".to_string(),
            weight: 0.3, // 30% weight for governance participation
            value: self.participation_score,
            description: Some("Participation in governance proposals and voting".to_string()),
        }
    }

    /// Recalculates the participation score based on activity
    fn recalculate_score(&mut self) {
        // Weighted scoring algorithm:
        // - Proposals created: 10 points each (encourages proposal creation)
        // - Votes cast: 1 point each (encourages participation)
        // - Participation rate bonus: up to 5 points for high participation
        let base_score = (self.proposals_created as f64 * 10.0) + (self.votes_cast as f64);
        
        // Add bonus for consistent participation (if they've participated in many proposals)
        let participation_bonus = if self.proposals_participated.len() > 10 {
            5.0
        } else if self.proposals_participated.len() > 5 {
            2.5
        } else {
            0.0
        };
        
        self.participation_score = base_score + participation_bonus;
    }
}

impl Default for ProposedChange {
    fn default() -> Self {
        Self {
            change_type: String::new(),
            target_system: String::new(),
            change_description: String::new(),
            implementation_notes: None,
            rollback_plan: None,
            impact_assessment: None,
        }
    }
}

impl VoteCount {
    /// Creates a new vote count
    pub fn new(vote_count: i32, weighted_count: f64, total_votes: i32) -> Self {
        let percentage = if total_votes > 0 {
            (vote_count as f64 / total_votes as f64) * 100.0
        } else {
            0.0
        };
        
        Self {
            vote_count,
            weighted_count,
            percentage,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    #[test]
    fn test_proposal_creation() {
        let cooperative_id = Uuid::new_v4();
        let proposer_id = Uuid::new_v4();
        let voting_deadline = Utc::now() + Duration::days(7);
        let proposed_change = ProposedChange::default();
        
        let proposal = Proposal::new(
            cooperative_id,
            proposer_id,
            "Test Proposal".to_string(),
            "A test proposal".to_string(),
            ProposalType::Feature,
            vec!["Option A".to_string(), "Option B".to_string()],
            proposed_change,
            voting_deadline,
            0.5, // 50% quorum
            100, // 100 eligible voters
        );
        
        assert_eq!(proposal.status, ProposalStatus::Draft);
        assert_eq!(proposal.title, "Test Proposal");
        assert_eq!(proposal.participation_count, 0);
        assert!(!proposal.has_quorum());
    }

    #[test]
    fn test_proposal_quorum() {
        let mut proposal = create_test_proposal();
        proposal.participation_count = 50; // 50 out of 100 voters
        proposal.eligible_voter_count = 100;
        proposal.quorum_threshold = 0.5; // 50% required
        
        assert!(proposal.has_quorum());
        
        proposal.participation_count = 40; // 40 out of 100 voters
        assert!(!proposal.has_quorum());
    }

    #[test]
    fn test_vote_weight_calculation() {
        let proposal_id = Uuid::new_v4();
        let voter_id = Uuid::new_v4();
        let choices = vec!["Option A".to_string()];
        
        // Test with high cooperative score
        let vote_high_score = Vote::new(proposal_id, voter_id, choices.clone(), 10.0, false);
        assert_eq!(vote_high_score.voting_weight, 1.5); // 1.0 + 0.5 (capped)
        
        // Test with low cooperative score
        let vote_low_score = Vote::new(proposal_id, voter_id, choices.clone(), 1.0, false);
        assert_eq!(vote_low_score.voting_weight, 1.1); // 1.0 + 0.1
        
        // Test with zero cooperative score
        let vote_zero_score = Vote::new(proposal_id, voter_id, choices, 0.0, false);
        assert_eq!(vote_zero_score.voting_weight, 1.0); // 1.0 + 0.0
        
        // Test direct weight calculation
        assert_eq!(Vote::calculate_voting_weight(5.0), 1.5); // 1.0 + 0.5 (capped)
        assert_eq!(Vote::calculate_voting_weight(2.0), 1.2); // 1.0 + 0.2
        assert_eq!(Vote::calculate_voting_weight(0.5), 1.05); // 1.0 + 0.05
    }

    #[test]
    fn test_vote_with_cooperative_score_struct() {
        let proposal_id = Uuid::new_v4();
        let voter_id = Uuid::new_v4();
        let choices = vec!["Option A".to_string()];
        
        let cooperative_score = crate::models::CooperativeScore {
            value: 3.0,
            last_updated: Utc::now(),
            contribution_factors: vec![],
        };
        
        let vote = Vote::new_with_cooperative_score(
            proposal_id, 
            voter_id, 
            choices, 
            &cooperative_score, 
            false
        );
        
        assert_eq!(vote.voting_weight, 1.3); // 1.0 + 0.3
    }

    #[test]
    fn test_governance_participation() {
        let user_id = Uuid::new_v4();
        let cooperative_id = Uuid::new_v4();
        let proposal_id = Uuid::new_v4();
        
        let mut participation = GovernanceParticipation::new(user_id, cooperative_id);
        assert_eq!(participation.participation_score, 0.0);
        
        participation.record_proposal_created();
        assert_eq!(participation.participation_score, 10.0);
        assert_eq!(participation.proposals_created, 1);
        
        participation.record_vote_cast(proposal_id);
        assert_eq!(participation.participation_score, 11.0);
        assert_eq!(participation.votes_cast, 1);
        assert!(participation.proposals_participated.contains(&proposal_id));
        
        // Test participation rate
        assert_eq!(participation.get_participation_rate(10), 0.1); // 1 out of 10 proposals
        
        // Test contribution factor
        let factor = participation.get_governance_contribution_factor();
        assert_eq!(factor.name, "governance_participation");
        assert_eq!(factor.weight, 0.3);
        assert_eq!(factor.value, 11.0);
    }

    #[test]
    fn test_vote_count_percentage() {
        let vote_count = VoteCount::new(25, 30.0, 100);
        assert_eq!(vote_count.vote_count, 25);
        assert_eq!(vote_count.weighted_count, 30.0);
        assert_eq!(vote_count.percentage, 25.0);
    }

    fn create_test_proposal() -> Proposal {
        let cooperative_id = Uuid::new_v4();
        let proposer_id = Uuid::new_v4();
        let voting_deadline = Utc::now() + Duration::days(7);
        let proposed_change = ProposedChange::default();
        
        Proposal::new(
            cooperative_id,
            proposer_id,
            "Test Proposal".to_string(),
            "A test proposal".to_string(),
            ProposalType::Feature,
            vec!["Option A".to_string(), "Option B".to_string()],
            proposed_change,
            voting_deadline,
            0.5,
            100,
        )
    }
}