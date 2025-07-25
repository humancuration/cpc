//! Governance Service for cooperative decision-making and voting
//! 
//! This service implements a comprehensive governance system that enables users to create, vote on, 
//! and implement proposals that affect content ranking and platform rules. The system uses Rated 
//! Choice Voting (RCV) to ensure outcomes reflect majority consensus and includes sophisticated
//! voting weight calculations based on cooperative scores.
//!
//! Key features:
//! - Proposal lifecycle management (creation, voting, finalization)
//! - Rated choice voting with weighted votes based on cooperative scores
//! - Governance participation tracking and incentives
//! - Fraud detection and security measures
//! - Integration with cooperative scoring system

use crate::{
    models::{
        governance::{
            Proposal, ProposalStatus, ProposalType, ProposedChange, Vote, VoteTally, VoteCount,
            VotingResult, GovernanceParticipation,
        },
        user::CooperativeScore,
    },
    repositories::governance_repository::{
        GovernanceRepository, CreateProposalData, CreateVoteData, ProposalStatistics,
        CooperativeGovernanceStats, UserGovernanceStats,
    },
    utils::datetime::now_utc,
};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;
use uuid::Uuid;

/// Configuration for governance system behavior
#[derive(Debug, Clone)]
pub struct GovernanceConfig {
    /// Minimum cooperative score required to create proposals
    pub min_proposal_score: f64,
    /// Minimum cooperative score required to vote
    pub min_voting_score: f64,
    /// Default quorum threshold for proposals (0.0 to 1.0)
    pub default_quorum_threshold: f64,
    /// Maximum voting period in days
    pub max_voting_period_days: i64,
    /// Minimum voting period in days
    pub min_voting_period_days: i64,
    /// Maximum number of options per proposal
    pub max_proposal_options: usize,
    /// Participation incentive multiplier
    pub participation_incentive_multiplier: f64,
}

impl Default for GovernanceConfig {
    fn default() -> Self {
        Self {
            min_proposal_score: 1.0,
            min_voting_score: 0.5,
            default_quorum_threshold: 0.3, // 30% participation required
            max_voting_period_days: 30,
            min_voting_period_days: 3,
            max_proposal_options: 10,
            participation_incentive_multiplier: 1.2,
        }
    }
}

/// Governance Service for managing proposals, voting, and participation
/// Ports functionality from the legacy Android codebase's feature_governance module
pub struct GovernanceService {
    governance_repo: Box<dyn GovernanceRepository>,
    config: GovernanceConfig,
    // In-memory fraud detection state (in production, this would be in Redis or similar)
    user_voting_timestamps: std::sync::Mutex<HashMap<Uuid, Vec<DateTime<Utc>>>>,
}

impl GovernanceService {
    /// Creates a new GovernanceService instance
    pub fn new(governance_repo: Box<dyn GovernanceRepository>, config: Option<GovernanceConfig>) -> Self {
        Self {
            governance_repo,
            config: config.unwrap_or_default(),
            user_voting_timestamps: std::sync::Mutex::new(HashMap::new()),
        }
    }    ///
 Creates a new governance proposal with validation and security checks
    pub async fn create_proposal(
        &self,
        cooperative_id: Uuid,
        proposer_id: Uuid,
        title: String,
        description: String,
        proposal_type: ProposalType,
        options: Vec<String>,
        proposed_change: ProposedChange,
        voting_deadline: DateTime<Utc>,
        quorum_threshold: Option<f64>,
        proposer_cooperative_score: &CooperativeScore,
    ) -> Result<Proposal> {
        // Validate proposer eligibility
        self.validate_proposer_eligibility(proposer_cooperative_score)?;

        // Validate proposal content
        self.validate_proposal_content(&title, &description, &options, &voting_deadline)?;

        // Calculate eligible voter count (simplified - in practice would query user repository)
        let eligible_voter_count = 100; // TODO: Implement actual eligible voter calculation

        let data = CreateProposalData {
            cooperative_id,
            proposer_id,
            title,
            description,
            proposal_type,
            options,
            proposed_change,
            voting_deadline,
            quorum_threshold: quorum_threshold.unwrap_or(self.config.default_quorum_threshold),
            eligible_voter_count,
        };

        let proposal = self.governance_repo.create_proposal(data).await?;

        // Record proposal creation for participation tracking
        self.governance_repo
            .record_proposal_created(proposer_id, cooperative_id)
            .await?;

        Ok(proposal)
    }

    /// Starts the voting period for a proposal
    pub async fn start_voting(&self, proposal_id: Uuid, user_id: Uuid) -> Result<()> {
        let proposal = self
            .governance_repo
            .find_proposal_by_id(proposal_id)
            .await?
            .ok_or_else(|| anyhow!("Proposal not found"))?;

        // Verify user has permission to start voting (proposal creator or admin)
        if proposal.proposer_id != user_id {
            return Err(anyhow!("Only the proposal creator can start voting"));
        }

        // Verify proposal is in draft status
        if proposal.status != ProposalStatus::Draft {
            return Err(anyhow!("Proposal is not in draft status"));
        }

        self.governance_repo.start_voting(proposal_id).await?;

        Ok(())
    }

    /// Casts a vote on a proposal with fraud detection and weight calculation
    pub async fn cast_vote(
        &self,
        proposal_id: Uuid,
        voter_id: Uuid,
        choices: Vec<String>,
        voter_cooperative_score: &CooperativeScore,
        is_anonymous: bool,
    ) -> Result<Vote> {
        // Get proposal and validate voting eligibility
        let proposal = self
            .governance_repo
            .find_proposal_by_id(proposal_id)
            .await?
            .ok_or_else(|| anyhow!("Proposal not found"))?;

        self.validate_voting_eligibility(&proposal, voter_id, voter_cooperative_score).await?;

        // Fraud detection
        self.detect_voting_fraud(voter_id, &choices).await?;

        // Check if user has already voted
        if let Some(_existing_vote) = self
            .governance_repo
            .get_user_vote(proposal_id, voter_id)
            .await?
        {
            return Err(anyhow!("User has already voted on this proposal"));
        }

        // Validate choices against proposal options
        self.validate_vote_choices(&proposal, &choices)?;

        let data = CreateVoteData {
            proposal_id,
            voter_id,
            choices,
            cooperative_score: voter_cooperative_score.value,
            is_anonymous,
        };

        let vote = self.governance_repo.create_vote(data).await?;

        // Record vote for participation tracking
        self.governance_repo
            .record_vote_cast(voter_id, proposal.cooperative_id, proposal_id)
            .await?;

        Ok(vote)
    }    
/// Updates an existing vote (if allowed)
    pub async fn update_vote(
        &self,
        vote_id: Uuid,
        voter_id: Uuid,
        new_choices: Vec<String>,
    ) -> Result<Vote> {
        let mut vote = self
            .governance_repo
            .find_vote_by_id(vote_id)
            .await?
            .ok_or_else(|| anyhow!("Vote not found"))?;

        // Verify ownership
        if vote.voter_id != voter_id {
            return Err(anyhow!("Cannot update another user's vote"));
        }

        // Get proposal to validate choices and check if voting is still open
        let proposal = self
            .governance_repo
            .find_proposal_by_id(vote.proposal_id)
            .await?
            .ok_or_else(|| anyhow!("Proposal not found"))?;

        if proposal.status != ProposalStatus::Voting {
            return Err(anyhow!("Voting period has ended"));
        }

        if proposal.is_expired() {
            return Err(anyhow!("Voting deadline has passed"));
        }

        // Validate new choices
        self.validate_vote_choices(&proposal, &new_choices)?;

        vote.update_choices(new_choices);
        self.governance_repo.update_vote(&vote).await?;

        Ok(vote)
    }

    /// Calculates vote tallies using Rated Choice Voting (RCV)
    pub async fn calculate_vote_tally(&self, proposal_id: Uuid) -> Result<VotingResult> {
        let proposal = self
            .governance_repo
            .find_proposal_by_id(proposal_id)
            .await?
            .ok_or_else(|| anyhow!("Proposal not found"))?;

        let votes = self.governance_repo.get_proposal_votes(proposal_id).await?;

        if votes.is_empty() {
            return Err(anyhow!("No votes to tally"));
        }

        // Implement Rated Choice Voting algorithm
        let mut rounds = Vec::new();
        let mut active_options = proposal.options.clone();
        let mut round_number = 1;

        loop {
            let round_results = self.calculate_round_results(&votes, &active_options);
            let total_votes = votes.len() as i32;
            let total_weight: f64 = votes.iter().map(|v| v.voting_weight).sum();

            // Check if any option has majority (>50% of weighted votes)
            let majority_threshold = total_weight * 0.5;
            let winner = round_results
                .iter()
                .find(|(_, count)| count.weighted_count > majority_threshold)
                .map(|(option, _)| option.clone());

            let is_final_round = winner.is_some() || active_options.len() <= 2;

            // Find option with lowest vote count to eliminate (if not final round)
            let eliminated_options = if !is_final_round {
                let min_count = round_results
                    .values()
                    .map(|c| c.weighted_count)
                    .fold(f64::INFINITY, f64::min);
                
                round_results
                    .iter()
                    .filter(|(_, count)| count.weighted_count == min_count)
                    .map(|(option, _)| option.clone())
                    .collect()
            } else {
                Vec::new()
            };

            let tally = VoteTally::new(
                proposal_id,
                round_number,
                round_results,
                eliminated_options.clone(),
                total_votes,
                total_weight,
                is_final_round,
                winner.clone(),
            );

            self.governance_repo.create_vote_tally(&tally).await?;
            rounds.push(tally);

            if is_final_round {
                break;
            }

            // Remove eliminated options for next round
            active_options.retain(|option| !eliminated_options.contains(option));
            round_number += 1;
        }

        // Determine final status
        let quorum_met = proposal.has_quorum();
        let final_winner = rounds.last().and_then(|r| r.winner.clone());
        
        let final_status = if quorum_met {
            if final_winner.is_some() {
                ProposalStatus::Passed
            } else {
                ProposalStatus::Failed
            }
        } else {
            ProposalStatus::Expired
        };

        let voting_result = VotingResult::new(
            proposal_id,
            rounds,
            final_winner,
            votes.len() as i32,
            quorum_met,
            final_status.clone(),
        );

        self.governance_repo.create_voting_result(&voting_result).await?;

        // Finalize the proposal
        self.governance_repo
            .finalize_proposal(proposal_id, final_status)
            .await?;

        Ok(voting_result)
    }