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
    }    //
/ Gets governance participation for a user with incentive calculations
    pub async fn get_user_participation(
        &self,
        user_id: Uuid,
        cooperative_id: Uuid,
    ) -> Result<GovernanceParticipation> {
        match self
            .governance_repo
            .find_participation(user_id, cooperative_id)
            .await?
        {
            Some(participation) => Ok(participation),
            None => {
                // Create new participation record
                let participation = GovernanceParticipation::new(user_id, cooperative_id);
                self.governance_repo.create_participation(&participation).await?;
                Ok(participation)
            }
        }
    }

    /// Calculates governance participation incentives for cooperative score
    pub async fn calculate_participation_incentives(
        &self,
        user_id: Uuid,
        cooperative_id: Uuid,
    ) -> Result<f64> {
        let participation = self.get_user_participation(user_id, cooperative_id).await?;
        
        // Base incentive calculation
        let base_score = participation.participation_score;
        
        // Apply participation multiplier
        let incentive_score = base_score * self.config.participation_incentive_multiplier;
        
        // Add bonus for recent activity
        let recent_activity_bonus = if participation.is_active_participant() {
            incentive_score * 0.1 // 10% bonus for recent activity
        } else {
            0.0
        };
        
        // Add consistency bonus
        let consistency_bonus = if participation.votes_cast > 10 && participation.proposals_created > 0 {
            incentive_score * 0.05 // 5% bonus for consistent participation
        } else {
            0.0
        };

        Ok(incentive_score + recent_activity_bonus + consistency_bonus)
    }

    /// Gets proposal statistics with detailed analytics
    pub async fn get_proposal_statistics(&self, proposal_id: Uuid) -> Result<ProposalStatistics> {
        self.governance_repo.get_proposal_statistics(proposal_id).await
    }

    /// Gets cooperative governance statistics
    pub async fn get_cooperative_governance_stats(
        &self,
        cooperative_id: Uuid,
    ) -> Result<CooperativeGovernanceStats> {
        self.governance_repo
            .get_cooperative_governance_stats(cooperative_id)
            .await
    }

    /// Gets user governance statistics
    pub async fn get_user_governance_stats(&self, user_id: Uuid) -> Result<UserGovernanceStats> {
        self.governance_repo.get_user_governance_stats(user_id).await
    }

    /// Finalizes expired proposals automatically
    pub async fn finalize_expired_proposals(&self, cooperative_id: Option<Uuid>) -> Result<Vec<Uuid>> {
        let active_proposals = self
            .governance_repo
            .get_active_proposals(cooperative_id, 100, 0)
            .await?;

        let mut finalized_proposals = Vec::new();

        for proposal in active_proposals {
            if proposal.is_expired() {
                let final_status = if proposal.has_quorum() {
                    // Calculate final tally to determine winner
                    match self.calculate_vote_tally(proposal.id).await {
                        Ok(result) => result.final_status,
                        Err(_) => ProposalStatus::Failed,
                    }
                } else {
                    ProposalStatus::Expired
                };

                self.governance_repo
                    .finalize_proposal(proposal.id, final_status)
                    .await?;
                
                finalized_proposals.push(proposal.id);
            }
        }

        Ok(finalized_proposals)
    }    //
 Private helper methods

    /// Validates proposer eligibility based on cooperative score
    fn validate_proposer_eligibility(&self, cooperative_score: &CooperativeScore) -> Result<()> {
        if cooperative_score.value < self.config.min_proposal_score {
            return Err(anyhow!(
                "Insufficient cooperative score to create proposals. Required: {}, Current: {}",
                self.config.min_proposal_score,
                cooperative_score.value
            ));
        }
        Ok(())
    }

    /// Validates proposal content and parameters
    fn validate_proposal_content(
        &self,
        title: &str,
        description: &str,
        options: &[String],
        voting_deadline: &DateTime<Utc>,
    ) -> Result<()> {
        // Title validation
        if title.trim().is_empty() {
            return Err(anyhow!("Proposal title cannot be empty"));
        }
        if title.len() > 200 {
            return Err(anyhow!("Proposal title must be 200 characters or less"));
        }

        // Description validation
        if description.trim().is_empty() {
            return Err(anyhow!("Proposal description cannot be empty"));
        }
        if description.len() > 5000 {
            return Err(anyhow!("Proposal description must be 5000 characters or less"));
        }

        // Options validation
        if options.len() < 2 {
            return Err(anyhow!("Proposal must have at least 2 options"));
        }
        if options.len() > self.config.max_proposal_options {
            return Err(anyhow!(
                "Proposal cannot have more than {} options",
                self.config.max_proposal_options
            ));
        }

        for option in options {
            if option.trim().is_empty() {
                return Err(anyhow!("Proposal options cannot be empty"));
            }
            if option.len() > 500 {
                return Err(anyhow!("Proposal options must be 500 characters or less"));
            }
        }

        // Check for duplicate options
        let mut unique_options = std::collections::HashSet::new();
        for option in options {
            if !unique_options.insert(option.trim().to_lowercase()) {
                return Err(anyhow!("Duplicate proposal options are not allowed"));
            }
        }

        // Voting deadline validation
        let now = now_utc();
        let min_deadline = now + Duration::days(self.config.min_voting_period_days);
        let max_deadline = now + Duration::days(self.config.max_voting_period_days);

        if *voting_deadline < min_deadline {
            return Err(anyhow!(
                "Voting deadline must be at least {} days from now",
                self.config.min_voting_period_days
            ));
        }
        if *voting_deadline > max_deadline {
            return Err(anyhow!(
                "Voting deadline cannot be more than {} days from now",
                self.config.max_voting_period_days
            ));
        }

        Ok(())
    }

    /// Validates voting eligibility
    async fn validate_voting_eligibility(
        &self,
        proposal: &Proposal,
        voter_id: Uuid,
        cooperative_score: &CooperativeScore,
    ) -> Result<()> {
        // Check proposal status
        if proposal.status != ProposalStatus::Voting {
            return Err(anyhow!("Proposal is not open for voting"));
        }

        // Check voting deadline
        if proposal.is_expired() {
            return Err(anyhow!("Voting deadline has passed"));
        }

        // Check cooperative score requirement
        if cooperative_score.value < self.config.min_voting_score {
            return Err(anyhow!(
                "Insufficient cooperative score to vote. Required: {}, Current: {}",
                self.config.min_voting_score,
                cooperative_score.value
            ));
        }

        // Prevent self-voting on own proposals (optional rule)
        if proposal.proposer_id == voter_id {
            return Err(anyhow!("Cannot vote on your own proposal"));
        }

        Ok(())
    }    /// De
tects voting fraud patterns
    async fn detect_voting_fraud(&self, voter_id: Uuid, choices: &[String]) -> Result<()> {
        // Rate limiting: max 10 votes per hour
        let now = now_utc();
        let mut timestamps = self.user_voting_timestamps.lock().unwrap();
        let user_votes = timestamps.entry(voter_id).or_insert_with(Vec::new);
        
        // Remove old timestamps (older than 1 hour)
        user_votes.retain(|&timestamp| now - timestamp < Duration::hours(1));
        
        let recent_count = user_votes.len();
        if recent_count >= 10 {
            return Err(anyhow!("Too many votes in a short time. Please try again later."));
        }
        
        user_votes.push(now);

        // Pattern analysis for fraud detection
        let fraud_patterns = vec!["spam", "fake", "test", "bot"];
        for choice in choices {
            let lower_choice = choice.to_lowercase();
            if fraud_patterns.iter().any(|pattern| lower_choice.contains(pattern)) {
                return Err(anyhow!("Vote contains suspicious patterns"));
            }
        }

        // Artificial delay to prevent rapid automated voting
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

        Ok(())
    }

    /// Validates vote choices against proposal options
    fn validate_vote_choices(&self, proposal: &Proposal, choices: &[String]) -> Result<()> {
        if choices.is_empty() {
            return Err(anyhow!("Vote must include at least one choice"));
        }

        if choices.len() > proposal.options.len() {
            return Err(anyhow!("Cannot rank more options than available"));
        }

        // Check that all choices are valid proposal options
        for choice in choices {
            if !proposal.options.contains(choice) {
                return Err(anyhow!("Invalid vote choice: {}", choice));
            }
        }

        // Check for duplicate choices
        let mut unique_choices = std::collections::HashSet::new();
        for choice in choices {
            if !unique_choices.insert(choice) {
                return Err(anyhow!("Duplicate choices are not allowed in ranked voting"));
            }
        }

        Ok(())
    }

    /// Calculates round results for RCV
    fn calculate_round_results(
        &self,
        votes: &[Vote],
        active_options: &[String],
    ) -> HashMap<String, VoteCount> {
        let mut results = HashMap::new();
        let total_votes = votes.len() as i32;

        // Initialize all active options with zero counts
        for option in active_options {
            results.insert(
                option.clone(),
                VoteCount::new(0, 0.0, total_votes),
            );
        }

        // Count first-choice votes for each active option
        for vote in votes {
            // Find the first choice that's still active
            for choice in &vote.choices {
                if active_options.contains(choice) {
                    let count = results.get_mut(choice).unwrap();
                    count.vote_count += 1;
                    count.weighted_count += vote.voting_weight;
                    break; // Only count the first valid choice
                }
            }
        }

        // Recalculate percentages
        for count in results.values_mut() {
            count.percentage = if total_votes > 0 {
                (count.vote_count as f64 / total_votes as f64) * 100.0
            } else {
                0.0
            };
        }

        results
    }
}