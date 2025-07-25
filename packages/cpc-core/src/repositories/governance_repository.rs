use crate::models::governance::{
    Proposal, ProposalStatus, ProposalType, ProposedChange,
    Vote, VoteTally, VoteCount, VotingResult, GovernanceParticipation
};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Data structure for creating proposals
pub struct CreateProposalData {
    pub cooperative_id: Uuid,
    pub proposer_id: Uuid,
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub options: Vec<String>,
    pub proposed_change: ProposedChange,
    pub voting_deadline: DateTime<Utc>,
    pub quorum_threshold: f64,
    pub eligible_voter_count: i32,
}

/// Data structure for creating votes
pub struct CreateVoteData {
    pub proposal_id: Uuid,
    pub voter_id: Uuid,
    pub choices: Vec<String>,
    pub cooperative_score: f64,
    pub is_anonymous: bool,
}

#[async_trait]
pub trait GovernanceRepository: Send + Sync {
    // Proposal operations
    async fn create_proposal(&self, data: CreateProposalData) -> Result<Proposal>;
    async fn find_proposal_by_id(&self, id: Uuid) -> Result<Option<Proposal>>;
    async fn update_proposal(&self, proposal: &Proposal) -> Result<()>;
    async fn delete_proposal(&self, id: Uuid) -> Result<()>;
    async fn get_cooperative_proposals(&self, cooperative_id: Uuid, status: Option<ProposalStatus>, limit: i32, offset: i32) -> Result<Vec<Proposal>>;
    async fn get_user_proposals(&self, user_id: Uuid, limit: i32, offset: i32) -> Result<Vec<Proposal>>;
    async fn get_active_proposals(&self, cooperative_id: Option<Uuid>, limit: i32, offset: i32) -> Result<Vec<Proposal>>;
    async fn start_voting(&self, proposal_id: Uuid) -> Result<()>;
    async fn finalize_proposal(&self, proposal_id: Uuid, status: ProposalStatus) -> Result<()>;
    
    // Vote operations
    async fn create_vote(&self, data: CreateVoteData) -> Result<Vote>;
    async fn find_vote_by_id(&self, id: Uuid) -> Result<Option<Vote>>;
    async fn update_vote(&self, vote: &Vote) -> Result<()>;
    async fn delete_vote(&self, id: Uuid) -> Result<()>;
    async fn get_proposal_votes(&self, proposal_id: Uuid) -> Result<Vec<Vote>>;
    async fn get_user_vote(&self, proposal_id: Uuid, voter_id: Uuid) -> Result<Option<Vote>>;
    async fn get_user_votes(&self, user_id: Uuid, limit: i32, offset: i32) -> Result<Vec<Vote>>;
    
    // Vote tallying operations
    async fn create_vote_tally(&self, tally: &VoteTally) -> Result<()>;
    async fn get_proposal_tallies(&self, proposal_id: Uuid) -> Result<Vec<VoteTally>>;
    async fn get_latest_tally(&self, proposal_id: Uuid) -> Result<Option<VoteTally>>;
    
    // Voting result operations
    async fn create_voting_result(&self, result: &VotingResult) -> Result<()>;
    async fn get_voting_result(&self, proposal_id: Uuid) -> Result<Option<VotingResult>>;
    
    // Governance participation operations
    async fn create_participation(&self, participation: &GovernanceParticipation) -> Result<()>;
    async fn find_participation(&self, user_id: Uuid, cooperative_id: Uuid) -> Result<Option<GovernanceParticipation>>;
    async fn update_participation(&self, participation: &GovernanceParticipation) -> Result<()>;
    async fn get_user_participation(&self, user_id: Uuid) -> Result<Vec<GovernanceParticipation>>;
    async fn get_cooperative_participation(&self, cooperative_id: Uuid, limit: i32, offset: i32) -> Result<Vec<GovernanceParticipation>>;
    async fn record_proposal_created(&self, user_id: Uuid, cooperative_id: Uuid) -> Result<()>;
    async fn record_vote_cast(&self, user_id: Uuid, cooperative_id: Uuid, proposal_id: Uuid) -> Result<()>;
    
    // Analytics and reporting
    async fn get_proposal_statistics(&self, proposal_id: Uuid) -> Result<ProposalStatistics>;
    async fn get_cooperative_governance_stats(&self, cooperative_id: Uuid) -> Result<CooperativeGovernanceStats>;
    async fn get_user_governance_stats(&self, user_id: Uuid) -> Result<UserGovernanceStats>;
}

/// Statistics for a specific proposal
#[derive(Debug, Clone)]
pub struct ProposalStatistics {
    pub proposal_id: Uuid,
    pub total_votes: i32,
    pub total_weight: f64,
    pub participation_rate: f64,
    pub option_breakdown: HashMap<String, VoteCount>,
    pub voting_timeline: Vec<VotingTimelineEntry>,
}

/// Timeline entry for voting activity
#[derive(Debug, Clone)]
pub struct VotingTimelineEntry {
    pub timestamp: DateTime<Utc>,
    pub votes_cast: i32,
    pub cumulative_votes: i32,
}

/// Governance statistics for a cooperative
#[derive(Debug, Clone)]
pub struct CooperativeGovernanceStats {
    pub cooperative_id: Uuid,
    pub total_proposals: i32,
    pub active_proposals: i32,
    pub passed_proposals: i32,
    pub failed_proposals: i32,
    pub average_participation_rate: f64,
    pub most_active_proposers: Vec<(Uuid, i32)>, // (user_id, proposal_count)
    pub most_active_voters: Vec<(Uuid, i32)>, // (user_id, vote_count)
}

/// Governance statistics for a user
#[derive(Debug, Clone)]
pub struct UserGovernanceStats {
    pub user_id: Uuid,
    pub proposals_created: i32,
    pub votes_cast: i32,
    pub participation_rate: f64,
    pub cooperatives_participated: Vec<Uuid>,
    pub recent_activity: Vec<GovernanceActivity>,
}

/// Recent governance activity entry
#[derive(Debug, Clone)]
pub struct GovernanceActivity {
    pub activity_type: String, // "proposal_created", "vote_cast", etc.
    pub proposal_id: Uuid,
    pub cooperative_id: Uuid,
    pub timestamp: DateTime<Utc>,
}

/// SQLite implementation of GovernanceRepository
pub struct SqliteGovernanceRepository {
    pool: SqlitePool,
}

impl SqliteGovernanceRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl GovernanceRepository for SqliteGovernanceRepository {
    async fn create_proposal(&self, data: CreateProposalData) -> Result<Proposal> {
        let mut tx = self.pool.begin().await?;
        
        let proposal_id = Uuid::new_v4();
        let now = Utc::now();
        
        // Insert the proposal
        sqlx::query!(
            "INSERT INTO proposals (
                id, cooperative_id, proposer_id, title, description, status,
                proposal_type, voting_deadline, quorum_threshold, participation_count,
                eligible_voter_count, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            proposal_id,
            data.cooperative_id,
            data.proposer_id,
            data.title,
            data.description,
            ProposalStatus::Draft,
            data.proposal_type,
            data.voting_deadline,
            data.quorum_threshold,
            0,
            data.eligible_voter_count,
            now,
            now
        )
        .execute(&mut *tx)
        .await?;
        
        // Insert proposal options
        for (index, option) in data.options.iter().enumerate() {
            sqlx::query!(
                "INSERT INTO proposal_options (proposal_id, option_text, order_index) VALUES (?, ?, ?)",
                proposal_id,
                option,
                index as i32
            )
            .execute(&mut *tx)
            .await?;
        }
        
        // Insert proposed change
        sqlx::query!(
            "INSERT INTO proposed_changes (
                proposal_id, change_type, target_system, change_description,
                implementation_notes, rollback_plan, impact_assessment
            ) VALUES (?, ?, ?, ?, ?, ?, ?)",
            proposal_id,
            data.proposed_change.change_type,
            data.proposed_change.target_system,
            data.proposed_change.change_description,
            data.proposed_change.implementation_notes,
            data.proposed_change.rollback_plan,
            data.proposed_change.impact_assessment
        )
        .execute(&mut *tx)
        .await?;
        
        tx.commit().await?;
        
        // Return the created proposal
        self.find_proposal_by_id(proposal_id).await?.ok_or_else(|| anyhow::anyhow!("Failed to retrieve created proposal"))
    }
    
    async fn find_proposal_by_id(&self, id: Uuid) -> Result<Option<Proposal>> {
        let row = sqlx::query!(
            "SELECT * FROM proposals WHERE id = ?",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                // Load options
                let options = sqlx::query!(
                    "SELECT option_text FROM proposal_options WHERE proposal_id = ? ORDER BY order_index",
                    id
                )
                .fetch_all(&self.pool)
                .await?;
                let options: Vec<String> = options.into_iter().map(|o| o.option_text).collect();
                
                // Load proposed change
                let change_row = sqlx::query!(
                    "SELECT * FROM proposed_changes WHERE proposal_id = ?",
                    id
                )
                .fetch_one(&self.pool)
                .await?;
                
                let proposed_change = ProposedChange {
                    change_type: change_row.change_type,
                    target_system: change_row.target_system,
                    change_description: change_row.change_description,
                    implementation_notes: change_row.implementation_notes,
                    rollback_plan: change_row.rollback_plan,
                    impact_assessment: change_row.impact_assessment,
                };
                
                let proposal = Proposal {
                    id: row.id,
                    cooperative_id: row.cooperative_id,
                    proposer_id: row.proposer_id,
                    title: row.title,
                    description: row.description,
                    status: row.status,
                    proposal_type: row.proposal_type,
                    options,
                    proposed_change,
                    created_at: row.created_at,
                    voting_deadline: row.voting_deadline,
                    updated_at: row.updated_at,
                    quorum_threshold: row.quorum_threshold,
                    participation_count: row.participation_count,
                    eligible_voter_count: row.eligible_voter_count,
                };
                
                Ok(Some(proposal))
            }
            None => Ok(None),
        }
    }
    
    async fn update_proposal(&self, proposal: &Proposal) -> Result<()> {
        sqlx::query!(
            "UPDATE proposals SET 
                title = ?, description = ?, status = ?, voting_deadline = ?,
                quorum_threshold = ?, participation_count = ?, eligible_voter_count = ?,
                updated_at = ?
            WHERE id = ?",
            proposal.title,
            proposal.description,
            proposal.status,
            proposal.voting_deadline,
            proposal.quorum_threshold,
            proposal.participation_count,
            proposal.eligible_voter_count,
            proposal.updated_at,
            proposal.id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn delete_proposal(&self, id: Uuid) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        
        // Delete related data first
        sqlx::query!("DELETE FROM proposal_options WHERE proposal_id = ?", id)
            .execute(&mut *tx)
            .await?;
        sqlx::query!("DELETE FROM proposed_changes WHERE proposal_id = ?", id)
            .execute(&mut *tx)
            .await?;
        sqlx::query!("DELETE FROM votes WHERE proposal_id = ?", id)
            .execute(&mut *tx)
            .await?;
        sqlx::query!("DELETE FROM vote_tallies WHERE proposal_id = ?", id)
            .execute(&mut *tx)
            .await?;
        sqlx::query!("DELETE FROM voting_results WHERE proposal_id = ?", id)
            .execute(&mut *tx)
            .await?;
        
        // Delete the proposal
        sqlx::query!("DELETE FROM proposals WHERE id = ?", id)
            .execute(&mut *tx)
            .await?;
        
        tx.commit().await?;
        Ok(())
    }
    
    async fn get_cooperative_proposals(&self, cooperative_id: Uuid, status: Option<ProposalStatus>, limit: i32, offset: i32) -> Result<Vec<Proposal>> {
        let rows = if let Some(status) = status {
            sqlx::query!(
                "SELECT id FROM proposals WHERE cooperative_id = ? AND status = ? 
                 ORDER BY created_at DESC LIMIT ? OFFSET ?",
                cooperative_id,
                status,
                limit,
                offset
            )
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query!(
                "SELECT id FROM proposals WHERE cooperative_id = ? 
                 ORDER BY created_at DESC LIMIT ? OFFSET ?",
                cooperative_id,
                limit,
                offset
            )
            .fetch_all(&self.pool)
            .await?
        };
        
        let mut proposals = Vec::new();
        for row in rows {
            if let Some(proposal) = self.find_proposal_by_id(row.id).await? {
                proposals.push(proposal);
            }
        }
        
        Ok(proposals)
    }
    
    async fn get_user_proposals(&self, user_id: Uuid, limit: i32, offset: i32) -> Result<Vec<Proposal>> {
        let rows = sqlx::query!(
            "SELECT id FROM proposals WHERE proposer_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
            user_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut proposals = Vec::new();
        for row in rows {
            if let Some(proposal) = self.find_proposal_by_id(row.id).await? {
                proposals.push(proposal);
            }
        }
        
        Ok(proposals)
    }
    
    async fn get_active_proposals(&self, cooperative_id: Option<Uuid>, limit: i32, offset: i32) -> Result<Vec<Proposal>> {
        let rows = if let Some(cooperative_id) = cooperative_id {
            sqlx::query!(
                "SELECT id FROM proposals WHERE cooperative_id = ? AND status = 'VOTING' 
                 ORDER BY voting_deadline ASC LIMIT ? OFFSET ?",
                cooperative_id,
                limit,
                offset
            )
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query!(
                "SELECT id FROM proposals WHERE status = 'VOTING' 
                 ORDER BY voting_deadline ASC LIMIT ? OFFSET ?",
                limit,
                offset
            )
            .fetch_all(&self.pool)
            .await?
        };
        
        let mut proposals = Vec::new();
        for row in rows {
            if let Some(proposal) = self.find_proposal_by_id(row.id).await? {
                proposals.push(proposal);
            }
        }
        
        Ok(proposals)
    }
    
    async fn start_voting(&self, proposal_id: Uuid) -> Result<()> {
        sqlx::query!(
            "UPDATE proposals SET status = ?, updated_at = ? WHERE id = ?",
            ProposalStatus::Voting,
            Utc::now(),
            proposal_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn finalize_proposal(&self, proposal_id: Uuid, status: ProposalStatus) -> Result<()> {
        sqlx::query!(
            "UPDATE proposals SET status = ?, updated_at = ? WHERE id = ?",
            status,
            Utc::now(),
            proposal_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }    
    a
sync fn create_vote(&self, data: CreateVoteData) -> Result<Vote> {
        let mut tx = self.pool.begin().await?;
        
        let vote_id = Uuid::new_v4();
        let now = Utc::now();
        let voting_weight = Vote::calculate_voting_weight(data.cooperative_score);
        
        // Insert the vote
        sqlx::query!(
            "INSERT INTO votes (
                id, proposal_id, voter_id, voting_weight, is_anonymous, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?)",
            vote_id,
            data.proposal_id,
            data.voter_id,
            voting_weight,
            data.is_anonymous,
            now,
            now
        )
        .execute(&mut *tx)
        .await?;
        
        // Insert vote choices
        for (index, choice) in data.choices.iter().enumerate() {
            sqlx::query!(
                "INSERT INTO vote_choices (vote_id, choice, order_index) VALUES (?, ?, ?)",
                vote_id,
                choice,
                index as i32
            )
            .execute(&mut *tx)
            .await?;
        }
        
        // Update proposal participation count
        sqlx::query!(
            "UPDATE proposals SET participation_count = participation_count + 1, updated_at = ? WHERE id = ?",
            now,
            data.proposal_id
        )
        .execute(&mut *tx)
        .await?;
        
        tx.commit().await?;
        
        let vote = Vote {
            id: vote_id,
            proposal_id: data.proposal_id,
            voter_id: data.voter_id,
            choices: data.choices,
            voting_weight,
            created_at: now,
            updated_at: now,
            is_anonymous: data.is_anonymous,
        };
        
        Ok(vote)
    }
    
    async fn find_vote_by_id(&self, id: Uuid) -> Result<Option<Vote>> {
        let row = sqlx::query!(
            "SELECT * FROM votes WHERE id = ?",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                // Load choices
                let choices = sqlx::query!(
                    "SELECT choice FROM vote_choices WHERE vote_id = ? ORDER BY order_index",
                    id
                )
                .fetch_all(&self.pool)
                .await?;
                let choices: Vec<String> = choices.into_iter().map(|c| c.choice).collect();
                
                let vote = Vote {
                    id: row.id,
                    proposal_id: row.proposal_id,
                    voter_id: row.voter_id,
                    choices,
                    voting_weight: row.voting_weight,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                    is_anonymous: row.is_anonymous,
                };
                
                Ok(Some(vote))
            }
            None => Ok(None),
        }
    }
    
    async fn update_vote(&self, vote: &Vote) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        
        // Update vote
        sqlx::query!(
            "UPDATE votes SET voting_weight = ?, is_anonymous = ?, updated_at = ? WHERE id = ?",
            vote.voting_weight,
            vote.is_anonymous,
            vote.updated_at,
            vote.id
        )
        .execute(&mut *tx)
        .await?;
        
        // Update choices - delete and re-insert
        sqlx::query!("DELETE FROM vote_choices WHERE vote_id = ?", vote.id)
            .execute(&mut *tx)
            .await?;
        
        for (index, choice) in vote.choices.iter().enumerate() {
            sqlx::query!(
                "INSERT INTO vote_choices (vote_id, choice, order_index) VALUES (?, ?, ?)",
                vote.id,
                choice,
                index as i32
            )
            .execute(&mut *tx)
            .await?;
        }
        
        tx.commit().await?;
        Ok(())
    }
    
    async fn delete_vote(&self, id: Uuid) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        
        // Get proposal_id for updating participation count
        let proposal_id = sqlx::query!(
            "SELECT proposal_id FROM votes WHERE id = ?",
            id
        )
        .fetch_one(&mut *tx)
        .await?
        .proposal_id;
        
        // Delete related data first
        sqlx::query!("DELETE FROM vote_choices WHERE vote_id = ?", id)
            .execute(&mut *tx)
            .await?;
        
        // Delete the vote
        sqlx::query!("DELETE FROM votes WHERE id = ?", id)
            .execute(&mut *tx)
            .await?;
        
        // Update proposal participation count
        sqlx::query!(
            "UPDATE proposals SET participation_count = participation_count - 1, updated_at = ? WHERE id = ?",
            Utc::now(),
            proposal_id
        )
        .execute(&mut *tx)
        .await?;
        
        tx.commit().await?;
        Ok(())
    }
    
    async fn get_proposal_votes(&self, proposal_id: Uuid) -> Result<Vec<Vote>> {
        let rows = sqlx::query!(
            "SELECT id FROM votes WHERE proposal_id = ? ORDER BY created_at ASC",
            proposal_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut votes = Vec::new();
        for row in rows {
            if let Some(vote) = self.find_vote_by_id(row.id).await? {
                votes.push(vote);
            }
        }
        
        Ok(votes)
    }
    
    async fn get_user_vote(&self, proposal_id: Uuid, voter_id: Uuid) -> Result<Option<Vote>> {
        let row = sqlx::query!(
            "SELECT id FROM votes WHERE proposal_id = ? AND voter_id = ?",
            proposal_id,
            voter_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => self.find_vote_by_id(row.id).await,
            None => Ok(None),
        }
    }
    
    async fn get_user_votes(&self, user_id: Uuid, limit: i32, offset: i32) -> Result<Vec<Vote>> {
        let rows = sqlx::query!(
            "SELECT id FROM votes WHERE voter_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
            user_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut votes = Vec::new();
        for row in rows {
            if let Some(vote) = self.find_vote_by_id(row.id).await? {
                votes.push(vote);
            }
        }
        
        Ok(votes)
    }
    
    async fn create_vote_tally(&self, tally: &VoteTally) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        
        // Insert the tally
        sqlx::query!(
            "INSERT INTO vote_tallies (
                id, proposal_id, round_number, total_votes, total_weight,
                is_final_round, winner, calculated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            tally.id,
            tally.proposal_id,
            tally.round_number,
            tally.total_votes,
            tally.total_weight,
            tally.is_final_round,
            tally.winner,
            tally.calculated_at
        )
        .execute(&mut *tx)
        .await?;
        
        // Insert round results
        for (option, vote_count) in &tally.round_results {
            sqlx::query!(
                "INSERT INTO tally_results (
                    tally_id, option_text, vote_count, weighted_count, percentage
                ) VALUES (?, ?, ?, ?, ?)",
                tally.id,
                option,
                vote_count.vote_count,
                vote_count.weighted_count,
                vote_count.percentage
            )
            .execute(&mut *tx)
            .await?;
        }
        
        // Insert eliminated options
        for option in &tally.eliminated_options {
            sqlx::query!(
                "INSERT INTO eliminated_options (tally_id, option_text) VALUES (?, ?)",
                tally.id,
                option
            )
            .execute(&mut *tx)
            .await?;
        }
        
        tx.commit().await?;
        Ok(())
    }
    
    async fn get_proposal_tallies(&self, proposal_id: Uuid) -> Result<Vec<VoteTally>> {
        let rows = sqlx::query!(
            "SELECT * FROM vote_tallies WHERE proposal_id = ? ORDER BY round_number ASC",
            proposal_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut tallies = Vec::new();
        for row in rows {
            // Load round results
            let results = sqlx::query!(
                "SELECT * FROM tally_results WHERE tally_id = ?",
                row.id
            )
            .fetch_all(&self.pool)
            .await?;
            
            let mut round_results = HashMap::new();
            for result in results {
                let vote_count = VoteCount {
                    vote_count: result.vote_count,
                    weighted_count: result.weighted_count,
                    percentage: result.percentage,
                };
                round_results.insert(result.option_text, vote_count);
            }
            
            // Load eliminated options
            let eliminated = sqlx::query!(
                "SELECT option_text FROM eliminated_options WHERE tally_id = ?",
                row.id
            )
            .fetch_all(&self.pool)
            .await?;
            let eliminated_options: Vec<String> = eliminated.into_iter().map(|e| e.option_text).collect();
            
            let tally = VoteTally {
                id: row.id,
                proposal_id: row.proposal_id,
                round_number: row.round_number,
                round_results,
                eliminated_options,
                total_votes: row.total_votes,
                total_weight: row.total_weight,
                calculated_at: row.calculated_at,
                is_final_round: row.is_final_round,
                winner: row.winner,
            };
            
            tallies.push(tally);
        }
        
        Ok(tallies)
    }
    
    async fn get_latest_tally(&self, proposal_id: Uuid) -> Result<Option<VoteTally>> {
        let row = sqlx::query!(
            "SELECT * FROM vote_tallies WHERE proposal_id = ? ORDER BY round_number DESC LIMIT 1",
            proposal_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                // Load round results
                let results = sqlx::query!(
                    "SELECT * FROM tally_results WHERE tally_id = ?",
                    row.id
                )
                .fetch_all(&self.pool)
                .await?;
                
                let mut round_results = HashMap::new();
                for result in results {
                    let vote_count = VoteCount {
                        vote_count: result.vote_count,
                        weighted_count: result.weighted_count,
                        percentage: result.percentage,
                    };
                    round_results.insert(result.option_text, vote_count);
                }
                
                // Load eliminated options
                let eliminated = sqlx::query!(
                    "SELECT option_text FROM eliminated_options WHERE tally_id = ?",
                    row.id
                )
                .fetch_all(&self.pool)
                .await?;
                let eliminated_options: Vec<String> = eliminated.into_iter().map(|e| e.option_text).collect();
                
                let tally = VoteTally {
                    id: row.id,
                    proposal_id: row.proposal_id,
                    round_number: row.round_number,
                    round_results,
                    eliminated_options,
                    total_votes: row.total_votes,
                    total_weight: row.total_weight,
                    calculated_at: row.calculated_at,
                    is_final_round: row.is_final_round,
                    winner: row.winner,
                };
                
                Ok(Some(tally))
            }
            None => Ok(None),
        }
    }
    
    async fn create_voting_result(&self, result: &VotingResult) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        
        // Insert the voting result
        sqlx::query!(
            "INSERT INTO voting_results (
                id, proposal_id, winner, total_participants, quorum_met,
                final_status, finalized_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?)",
            result.id,
            result.proposal_id,
            result.winner,
            result.total_participants,
            result.quorum_met,
            result.final_status,
            result.finalized_at
        )
        .execute(&mut *tx)
        .await?;
        
        // Link to tallies (tallies should already exist)
        for tally in &result.rounds {
            sqlx::query!(
                "UPDATE vote_tallies SET voting_result_id = ? WHERE id = ?",
                result.id,
                tally.id
            )
            .execute(&mut *tx)
            .await?;
        }
        
        tx.commit().await?;
        Ok(())
    }
    
    async fn get_voting_result(&self, proposal_id: Uuid) -> Result<Option<VotingResult>> {
        let row = sqlx::query!(
            "SELECT * FROM voting_results WHERE proposal_id = ?",
            proposal_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                // Load associated tallies
                let tallies = self.get_proposal_tallies(proposal_id).await?;
                
                let result = VotingResult {
                    id: row.id,
                    proposal_id: row.proposal_id,
                    rounds: tallies,
                    winner: row.winner,
                    total_participants: row.total_participants,
                    quorum_met: row.quorum_met,
                    final_status: row.final_status,
                    finalized_at: row.finalized_at,
                };
                
                Ok(Some(result))
            }
            None => Ok(None),
        }
    }    
  
  async fn create_participation(&self, participation: &GovernanceParticipation) -> Result<()> {
        sqlx::query!(
            "INSERT INTO governance_participation (
                id, user_id, cooperative_id, proposals_created, votes_cast,
                participation_score, last_activity, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            participation.id,
            participation.user_id,
            participation.cooperative_id,
            participation.proposals_created,
            participation.votes_cast,
            participation.participation_score,
            participation.last_activity,
            participation.created_at,
            participation.updated_at
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn find_participation(&self, user_id: Uuid, cooperative_id: Uuid) -> Result<Option<GovernanceParticipation>> {
        let row = sqlx::query!(
            "SELECT * FROM governance_participation WHERE user_id = ? AND cooperative_id = ?",
            user_id,
            cooperative_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                // Load proposals participated
                let proposals = sqlx::query!(
                    "SELECT DISTINCT proposal_id FROM votes WHERE voter_id = ? 
                     AND proposal_id IN (SELECT id FROM proposals WHERE cooperative_id = ?)",
                    user_id,
                    cooperative_id
                )
                .fetch_all(&self.pool)
                .await?;
                let proposals_participated: Vec<Uuid> = proposals.into_iter().map(|p| p.proposal_id).collect();
                
                let participation = GovernanceParticipation {
                    id: row.id,
                    user_id: row.user_id,
                    cooperative_id: row.cooperative_id,
                    proposals_created: row.proposals_created,
                    votes_cast: row.votes_cast,
                    proposals_participated,
                    participation_score: row.participation_score,
                    last_activity: row.last_activity,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                };
                
                Ok(Some(participation))
            }
            None => Ok(None),
        }
    }
    
    async fn update_participation(&self, participation: &GovernanceParticipation) -> Result<()> {
        sqlx::query!(
            "UPDATE governance_participation SET 
                proposals_created = ?, votes_cast = ?, participation_score = ?,
                last_activity = ?, updated_at = ?
            WHERE id = ?",
            participation.proposals_created,
            participation.votes_cast,
            participation.participation_score,
            participation.last_activity,
            participation.updated_at,
            participation.id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_user_participation(&self, user_id: Uuid) -> Result<Vec<GovernanceParticipation>> {
        let rows = sqlx::query!(
            "SELECT cooperative_id FROM governance_participation WHERE user_id = ?",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut participations = Vec::new();
        for row in rows {
            if let Some(participation) = self.find_participation(user_id, row.cooperative_id).await? {
                participations.push(participation);
            }
        }
        
        Ok(participations)
    }
    
    async fn get_cooperative_participation(&self, cooperative_id: Uuid, limit: i32, offset: i32) -> Result<Vec<GovernanceParticipation>> {
        let rows = sqlx::query!(
            "SELECT user_id FROM governance_participation WHERE cooperative_id = ? 
             ORDER BY participation_score DESC LIMIT ? OFFSET ?",
            cooperative_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut participations = Vec::new();
        for row in rows {
            if let Some(participation) = self.find_participation(row.user_id, cooperative_id).await? {
                participations.push(participation);
            }
        }
        
        Ok(participations)
    }
    
    async fn record_proposal_created(&self, user_id: Uuid, cooperative_id: Uuid) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        
        // Check if participation record exists
        let existing = sqlx::query!(
            "SELECT id FROM governance_participation WHERE user_id = ? AND cooperative_id = ?",
            user_id,
            cooperative_id
        )
        .fetch_optional(&mut *tx)
        .await?;
        
        let now = Utc::now();
        
        if let Some(existing) = existing {
            // Update existing record
            sqlx::query!(
                "UPDATE governance_participation SET 
                    proposals_created = proposals_created + 1,
                    last_activity = ?, updated_at = ?
                 WHERE id = ?",
                now,
                now,
                existing.id
            )
            .execute(&mut *tx)
            .await?;
        } else {
            // Create new record
            let participation = GovernanceParticipation::new(user_id, cooperative_id);
            sqlx::query!(
                "INSERT INTO governance_participation (
                    id, user_id, cooperative_id, proposals_created, votes_cast,
                    participation_score, last_activity, created_at, updated_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
                participation.id,
                participation.user_id,
                participation.cooperative_id,
                1, // First proposal
                participation.votes_cast,
                participation.participation_score,
                now,
                participation.created_at,
                participation.updated_at
            )
            .execute(&mut *tx)
            .await?;
        }
        
        tx.commit().await?;
        Ok(())
    }
    
    async fn record_vote_cast(&self, user_id: Uuid, cooperative_id: Uuid, proposal_id: Uuid) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        
        // Check if participation record exists
        let existing = sqlx::query!(
            "SELECT id FROM governance_participation WHERE user_id = ? AND cooperative_id = ?",
            user_id,
            cooperative_id
        )
        .fetch_optional(&mut *tx)
        .await?;
        
        let now = Utc::now();
        
        if let Some(existing) = existing {
            // Update existing record
            sqlx::query!(
                "UPDATE governance_participation SET 
                    votes_cast = votes_cast + 1,
                    last_activity = ?, updated_at = ?
                 WHERE id = ?",
                now,
                now,
                existing.id
            )
            .execute(&mut *tx)
            .await?;
        } else {
            // Create new record
            let participation = GovernanceParticipation::new(user_id, cooperative_id);
            sqlx::query!(
                "INSERT INTO governance_participation (
                    id, user_id, cooperative_id, proposals_created, votes_cast,
                    participation_score, last_activity, created_at, updated_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
                participation.id,
                participation.user_id,
                participation.cooperative_id,
                participation.proposals_created,
                1, // First vote
                participation.participation_score,
                now,
                participation.created_at,
                participation.updated_at
            )
            .execute(&mut *tx)
            .await?;
        }
        
        tx.commit().await?;
        Ok(())
    }
    
    async fn get_proposal_statistics(&self, proposal_id: Uuid) -> Result<ProposalStatistics> {
        // Get basic vote counts
        let vote_stats = sqlx::query!(
            "SELECT COUNT(*) as total_votes, SUM(voting_weight) as total_weight FROM votes WHERE proposal_id = ?",
            proposal_id
        )
        .fetch_one(&self.pool)
        .await?;
        
        // Get proposal info for participation rate
        let proposal = sqlx::query!(
            "SELECT eligible_voter_count FROM proposals WHERE id = ?",
            proposal_id
        )
        .fetch_one(&self.pool)
        .await?;
        
        let total_votes = vote_stats.total_votes;
        let total_weight = vote_stats.total_weight.unwrap_or(0.0);
        let participation_rate = if proposal.eligible_voter_count > 0 {
            total_votes as f64 / proposal.eligible_voter_count as f64
        } else {
            0.0
        };
        
        // Get option breakdown from latest tally
        let mut option_breakdown = HashMap::new();
        if let Some(tally) = self.get_latest_tally(proposal_id).await? {
            option_breakdown = tally.round_results;
        }
        
        // Get voting timeline (simplified - votes per day)
        let timeline_data = sqlx::query!(
            "SELECT DATE(created_at) as vote_date, COUNT(*) as votes_count 
             FROM votes WHERE proposal_id = ? 
             GROUP BY DATE(created_at) 
             ORDER BY vote_date ASC",
            proposal_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut voting_timeline = Vec::new();
        let mut cumulative_votes = 0;
        for entry in timeline_data {
            cumulative_votes += entry.votes_count;
            voting_timeline.push(VotingTimelineEntry {
                timestamp: entry.vote_date.and_hms_opt(0, 0, 0).unwrap().and_utc(),
                votes_cast: entry.votes_count,
                cumulative_votes,
            });
        }
        
        Ok(ProposalStatistics {
            proposal_id,
            total_votes,
            total_weight,
            participation_rate,
            option_breakdown,
            voting_timeline,
        })
    }
    
    async fn get_cooperative_governance_stats(&self, cooperative_id: Uuid) -> Result<CooperativeGovernanceStats> {
        // Get proposal counts by status
        let proposal_stats = sqlx::query!(
            "SELECT 
                COUNT(*) as total_proposals,
                SUM(CASE WHEN status = 'VOTING' THEN 1 ELSE 0 END) as active_proposals,
                SUM(CASE WHEN status = 'PASSED' THEN 1 ELSE 0 END) as passed_proposals,
                SUM(CASE WHEN status = 'FAILED' THEN 1 ELSE 0 END) as failed_proposals
             FROM proposals WHERE cooperative_id = ?",
            cooperative_id
        )
        .fetch_one(&self.pool)
        .await?;
        
        // Calculate average participation rate
        let participation_data = sqlx::query!(
            "SELECT AVG(CAST(participation_count AS REAL) / CAST(eligible_voter_count AS REAL)) as avg_participation
             FROM proposals WHERE cooperative_id = ? AND eligible_voter_count > 0",
            cooperative_id
        )
        .fetch_one(&self.pool)
        .await?;
        
        let average_participation_rate = participation_data.avg_participation.unwrap_or(0.0);
        
        // Get most active proposers
        let active_proposers = sqlx::query!(
            "SELECT proposer_id, COUNT(*) as proposal_count 
             FROM proposals WHERE cooperative_id = ? 
             GROUP BY proposer_id 
             ORDER BY proposal_count DESC 
             LIMIT 10",
            cooperative_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let most_active_proposers: Vec<(Uuid, i32)> = active_proposers
            .into_iter()
            .map(|p| (p.proposer_id, p.proposal_count))
            .collect();
        
        // Get most active voters
        let active_voters = sqlx::query!(
            "SELECT v.voter_id, COUNT(*) as vote_count 
             FROM votes v 
             INNER JOIN proposals p ON v.proposal_id = p.id 
             WHERE p.cooperative_id = ? 
             GROUP BY v.voter_id 
             ORDER BY vote_count DESC 
             LIMIT 10",
            cooperative_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let most_active_voters: Vec<(Uuid, i32)> = active_voters
            .into_iter()
            .map(|v| (v.voter_id, v.vote_count))
            .collect();
        
        Ok(CooperativeGovernanceStats {
            cooperative_id,
            total_proposals: proposal_stats.total_proposals,
            active_proposals: proposal_stats.active_proposals,
            passed_proposals: proposal_stats.passed_proposals,
            failed_proposals: proposal_stats.failed_proposals,
            average_participation_rate,
            most_active_proposers,
            most_active_voters,
        })
    }
    
    async fn get_user_governance_stats(&self, user_id: Uuid) -> Result<UserGovernanceStats> {
        // Get user's proposal and vote counts
        let user_stats = sqlx::query!(
            "SELECT 
                (SELECT COUNT(*) FROM proposals WHERE proposer_id = ?) as proposals_created,
                (SELECT COUNT(*) FROM votes WHERE voter_id = ?) as votes_cast",
            user_id,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;
        
        // Calculate participation rate
        let total_eligible = sqlx::query!(
            "SELECT SUM(eligible_voter_count) as total_eligible 
             FROM proposals p 
             INNER JOIN votes v ON p.id = v.proposal_id 
             WHERE v.voter_id = ?",
            user_id
        )
        .fetch_one(&self.pool)
        .await?;
        
        let participation_rate = if let Some(total_eligible) = total_eligible.total_eligible {
            if total_eligible > 0 {
                user_stats.votes_cast as f64 / total_eligible as f64
            } else {
                0.0
            }
        } else {
            0.0
        };
        
        // Get cooperatives participated in
        let cooperatives = sqlx::query!(
            "SELECT DISTINCT cooperative_id FROM governance_participation WHERE user_id = ?",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let cooperatives_participated: Vec<Uuid> = cooperatives
            .into_iter()
            .map(|c| c.cooperative_id)
            .collect();
        
        // Get recent activity (last 10 activities)
        let recent_activities = sqlx::query!(
            "SELECT 'proposal_created' as activity_type, id as proposal_id, cooperative_id, created_at as timestamp
             FROM proposals WHERE proposer_id = ?
             UNION ALL
             SELECT 'vote_cast' as activity_type, proposal_id, 
                    (SELECT cooperative_id FROM proposals WHERE id = votes.proposal_id) as cooperative_id,
                    created_at as timestamp
             FROM votes WHERE voter_id = ?
             ORDER BY timestamp DESC
             LIMIT 10",
            user_id,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let recent_activity: Vec<GovernanceActivity> = recent_activities
            .into_iter()
            .map(|a| GovernanceActivity {
                activity_type: a.activity_type,
                proposal_id: a.proposal_id,
                cooperative_id: a.cooperative_id,
                timestamp: a.timestamp,
            })
            .collect();
        
        Ok(UserGovernanceStats {
            user_id,
            proposals_created: user_stats.proposals_created,
            votes_cast: user_stats.votes_cast,
            participation_rate,
            cooperatives_participated,
            recent_activity,
        })
    }
}