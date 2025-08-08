//! Community decision-making workflows for the Unified Community Impact Dashboard
//!
//! This module provides tools and workflows for community-driven decision making,
//! including proposal systems, discussion forums, and consensus building mechanisms.

use tracing::info;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

/// Community decision-making system
pub struct DecisionMakingSystem {
    proposals: HashMap<Uuid, Proposal>,
    discussions: HashMap<Uuid, Discussion>,
    consensus_processes: HashMap<Uuid, ConsensusProcess>,
    decision_history: Vec<DecisionRecord>,
    facilitators: Vec<String>, // Role IDs of decision facilitators
}

impl DecisionMakingSystem {
    /// Create a new decision-making system
    pub fn new() -> Self {
        Self {
            proposals: HashMap::new(),
            discussions: HashMap::new(),
            consensus_processes: HashMap::new(),
            decision_history: Vec::new(),
            facilitators: Vec::new(),
        }
    }

    /// Add a facilitator to the system
    pub fn add_facilitator(&mut self, role_id: String) {
        self.facilitators.push(role_id);
        info!("Added facilitator: {}", &self.facilitators.last().unwrap());
    }

    /// Create a new proposal
    pub fn create_proposal(&mut self, proposal: Proposal) -> Uuid {
        let proposal_id = proposal.id;
        self.proposals.insert(proposal_id, proposal);
        info!("Created proposal: {}", proposal_id);
        proposal_id
    }

    /// Get a proposal by ID
    pub fn get_proposal(&self, proposal_id: Uuid) -> Option<&Proposal> {
        self.proposals.get(&proposal_id)
    }

    /// Update proposal status
    pub fn update_proposal_status(&mut self, proposal_id: Uuid, status: ProposalStatus) -> Result<(), DecisionError> {
        let proposal = self.proposals.get_mut(&proposal_id)
            .ok_or(DecisionError::ProposalNotFound(proposal_id))?;
        
        proposal.status = status;
        proposal.updated_at = Utc::now();
        
        info!("Updated proposal {} status to {:?}", proposal_id, status);
        Ok(())
    }

    /// Submit a proposal for review
    pub fn submit_proposal(&mut self, proposal_id: Uuid) -> Result<(), DecisionError> {
        self.update_proposal_status(proposal_id, ProposalStatus::InReview)
    }

    /// Approve a proposal
    pub fn approve_proposal(&mut self, proposal_id: Uuid) -> Result<(), DecisionError> {
        self.update_proposal_status(proposal_id, ProposalStatus::Approved)
    }

    /// Reject a proposal
    pub fn reject_proposal(&mut self, proposal_id: Uuid) -> Result<(), DecisionError> {
        self.update_proposal_status(proposal_id, ProposalStatus::Rejected)
    }

    /// Create a discussion thread for a proposal
    pub fn create_discussion(&mut self, discussion: Discussion) -> Uuid {
        let discussion_id = discussion.id;
        self.discussions.insert(discussion_id, discussion);
        info!("Created discussion: {}", discussion_id);
        discussion_id
    }

    /// Add a comment to a discussion
    pub fn add_comment(&mut self, discussion_id: Uuid, comment: Comment) -> Result<(), DecisionError> {
        let discussion = self.discussions.get_mut(&discussion_id)
            .ok_or(DecisionError::DiscussionNotFound(discussion_id))?;
        
        discussion.add_comment(comment);
        info!("Added comment to discussion: {}", discussion_id);
        Ok(())
    }

    /// Get comments for a discussion
    pub fn get_comments(&self, discussion_id: Uuid) -> Result<&Vec<Comment>, DecisionError> {
        let discussion = self.discussions.get(&discussion_id)
            .ok_or(DecisionError::DiscussionNotFound(discussion_id))?;
        
        Ok(&discussion.comments)
    }

    /// Start a consensus process
    pub fn start_consensus_process(&mut self, process: ConsensusProcess) -> Uuid {
        let process_id = process.id;
        self.consensus_processes.insert(process_id, process);
        info!("Started consensus process: {}", process_id);
        process_id
    }

    /// Add participant to consensus process
    pub fn add_participant(&mut self, process_id: Uuid, participant: Participant) -> Result<(), DecisionError> {
        let process = self.consensus_processes.get_mut(&process_id)
            .ok_or(DecisionError::ProcessNotFound(process_id))?;
        
        process.add_participant(participant);
        info!("Added participant to consensus process: {}", process_id);
        Ok(())
    }

    /// Record a participant's position
    pub fn record_position(&mut self, process_id: Uuid, participant_id: Uuid, position: Position) -> Result<(), DecisionError> {
        let process = self.consensus_processes.get_mut(&process_id)
            .ok_or(DecisionError::ProcessNotFound(process_id))?;
        
        process.record_position(participant_id, position);
        info!("Recorded position for participant {} in process {}", participant_id, process_id);
        Ok(())
    }

    /// Check if consensus has been reached
    pub fn check_consensus(&self, process_id: Uuid) -> Result<bool, DecisionError> {
        let process = self.consensus_processes.get(&process_id)
            .ok_or(DecisionError::ProcessNotFound(process_id))?;
        
        Ok(process.check_consensus())
    }

    /// Finalize a consensus process
    pub fn finalize_process(&mut self, process_id: Uuid, outcome: ConsensusOutcome) -> Result<(), DecisionError> {
        let process = self.consensus_processes.get_mut(&process_id)
            .ok_or(DecisionError::ProcessNotFound(process_id))?;
        
        process.finalize(outcome.clone());
        
        // Record the decision
        let decision = DecisionRecord::new(
            format!("Consensus decision for {}", process.topic),
            DecisionType::Consensus,
            DecisionStatus::Finalized,
            Some(process_id),
            Some(outcome.clone()),
        );
        
        self.decision_history.push(decision);
        
        info!("Finalized consensus process: {}", process_id);
        Ok(())
    }

    /// Record a decision
    pub fn record_decision(&mut self, decision: DecisionRecord) {
        self.decision_history.push(decision);
        info!("Recorded decision");
    }

    /// Get recent decisions
    pub fn get_recent_decisions(&self, days: i64) -> Vec<&DecisionRecord> {
        let cutoff = Utc::now() - chrono::Duration::days(days);
        self.decision_history.iter()
            .filter(|decision| decision.timestamp > cutoff)
            .collect()
    }

    /// Get decisions by type
    pub fn get_decisions_by_type(&self, decision_type: DecisionType) -> Vec<&DecisionRecord> {
        self.decision_history.iter()
            .filter(|decision| decision.decision_type == decision_type)
            .collect()
    }

    /// Get decision statistics
    pub fn get_statistics(&self) -> DecisionStatistics {
        let total_proposals = self.proposals.len();
        let total_discussions = self.discussions.len();
        let total_processes = self.consensus_processes.len();
        let total_decisions = self.decision_history.len();
        
        // Count proposals by status
        let approved_proposals = self.proposals.values()
            .filter(|p| p.status == ProposalStatus::Approved)
            .count();
        
        let rejected_proposals = self.proposals.values()
            .filter(|p| p.status == ProposalStatus::Rejected)
            .count();
        
        let active_processes = self.consensus_processes.values()
            .filter(|p| p.status == ConsensusStatus::Active)
            .count();
        
        DecisionStatistics {
            total_proposals,
            total_discussions,
            total_processes,
            total_decisions,
            approved_proposals,
            rejected_proposals,
            active_processes,
        }
    }

    /// Generate decision report
    pub fn generate_decision_report(&self) -> DecisionReport {
        let stats = self.get_statistics();
        let recent_decisions = self.get_recent_decisions(30); // Last 30 days
        
        DecisionReport {
            generated_at: Utc::now(),
            statistics: stats,
            recent_decisions: recent_decisions.len(),
        }
    }

    /// Search proposals by keyword
    pub fn search_proposals(&self, keyword: &str) -> Vec<&Proposal> {
        self.proposals.values()
            .filter(|proposal| {
                proposal.title.to_lowercase().contains(&keyword.to_lowercase()) ||
                proposal.description.to_lowercase().contains(&keyword.to_lowercase())
            })
            .collect()
    }

    /// Get proposals by status
    pub fn get_proposals_by_status(&self, status: ProposalStatus) -> Vec<&Proposal> {
        self.proposals.values()
            .filter(|proposal| proposal.status == status)
            .collect()
    }

    /// Get active consensus processes
    pub fn get_active_processes(&self) -> Vec<&ConsensusProcess> {
        self.consensus_processes.values()
            .filter(|process| process.status == ConsensusStatus::Active)
            .collect()
    }
}

/// Proposal for community consideration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub author: String, // User ID
    pub category: ProposalCategory,
    pub status: ProposalStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub discussion_thread: Option<Uuid>, // Discussion ID
    pub required_consensus: Option<ConsensusType>,
    pub voting_deadline: Option<DateTime<Utc>>,
    pub supporting_documents: Vec<Uuid>, // Document IDs
}

impl Proposal {
    /// Create a new proposal
    pub fn new(
        title: String,
        description: String,
        author: String,
        category: ProposalCategory,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            author,
            category,
            status: ProposalStatus::Draft,
            created_at: now,
            updated_at: now,
            discussion_thread: None,
            required_consensus: None,
            voting_deadline: None,
            supporting_documents: Vec::new(),
        }
    }

    /// Set discussion thread
    pub fn set_discussion_thread(&mut self, discussion_id: Uuid) {
        self.discussion_thread = Some(discussion_id);
        self.updated_at = Utc::now();
    }

    /// Set required consensus type
    pub fn set_required_consensus(&mut self, consensus_type: ConsensusType) {
        self.required_consensus = Some(consensus_type);
        self.updated_at = Utc::now();
    }

    /// Set voting deadline
    pub fn set_voting_deadline(&mut self, deadline: DateTime<Utc>) {
        self.voting_deadline = Some(deadline);
        self.updated_at = Utc::now();
    }

    /// Add supporting documents
    pub fn add_supporting_documents(&mut self, documents: Vec<Uuid>) {
        self.supporting_documents.extend(documents);
        self.updated_at = Utc::now();
    }

    /// Submit for review
    pub fn submit(&mut self) {
        self.status = ProposalStatus::InReview;
        self.updated_at = Utc::now();
    }

    /// Approve the proposal
    pub fn approve(&mut self) {
        self.status = ProposalStatus::Approved;
        self.updated_at = Utc::now();
    }

    /// Reject the proposal
    pub fn reject(&mut self) {
        self.status = ProposalStatus::Rejected;
        self.updated_at = Utc::now();
    }
}

/// Discussion thread for proposals
#[derive(Debug, Clone)]
pub struct Discussion {
    pub id: Uuid,
    pub proposal_id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub comments: Vec<Comment>,
    pub participants: Vec<String>, // User IDs
    pub is_locked: bool,
}

impl Discussion {
    /// Create a new discussion
    pub fn new(proposal_id: Uuid, title: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            proposal_id,
            title,
            created_at: Utc::now(),
            comments: Vec::new(),
            participants: Vec::new(),
            is_locked: false,
        }
    }

    /// Add a comment to the discussion
    pub fn add_comment(&mut self, comment: Comment) {
        self.comments.push(comment);
        
        // Add participant if not already in list
        if !self.participants.contains(&comment.author) {
            self.participants.push(comment.author.clone());
        }
    }

    /// Lock the discussion
    pub fn lock(&mut self) {
        self.is_locked = true;
    }

    /// Unlock the discussion
    pub fn unlock(&mut self) {
        self.is_locked = false;
    }

    /// Get comments by author
    pub fn get_comments_by_author(&self, author: &str) -> Vec<&Comment> {
        self.comments.iter()
            .filter(|comment| comment.author == author)
            .collect()
    }

    /// Get recent comments
    pub fn get_recent_comments(&self, count: usize) -> Vec<&Comment> {
        self.comments.iter()
            .rev()
            .take(count)
            .collect()
    }
}

/// Comment in a discussion
#[derive(Debug, Clone)]
pub struct Comment {
    pub id: Uuid,
    pub author: String, // User ID
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub edited_at: Option<DateTime<Utc>>,
    pub reactions: HashMap<String, usize>, // Reaction type and count
    pub replies: Vec<Uuid>, // Comment IDs of replies
}

impl Comment {
    /// Create a new comment
    pub fn new(author: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            author,
            content,
            created_at: Utc::now(),
            edited_at: None,
            reactions: HashMap::new(),
            replies: Vec::new(),
        }
    }

    /// Edit the comment
    pub fn edit(&mut self, new_content: String) {
        self.content = new_content;
        self.edited_at = Some(Utc::now());
    }

    /// Add a reaction to the comment
    pub fn add_reaction(&mut self, reaction_type: String) {
        *self.reactions.entry(reaction_type).or_insert(0) += 1;
    }

    /// Add a reply to the comment
    pub fn add_reply(&mut self, reply_id: Uuid) {
        self.replies.push(reply_id);
    }
}

/// Consensus process for decision making
#[derive(Debug, Clone)]
pub struct ConsensusProcess {
    pub id: Uuid,
    pub topic: String,
    pub description: String,
    pub facilitator: String, // Role ID
    pub participants: Vec<Participant>,
    pub positions: HashMap<Uuid, Position>, // Participant ID to Position
    pub status: ConsensusStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub outcome: Option<ConsensusOutcome>,
    pub process_type: ConsensusType,
    pub timeline: Option<ConsensusTimeline>,
}

impl ConsensusProcess {
    /// Create a new consensus process
    pub fn new(
        topic: String,
        description: String,
        facilitator: String,
        process_type: ConsensusType,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            topic,
            description,
            facilitator,
            participants: Vec::new(),
            positions: HashMap::new(),
            status: ConsensusStatus::Created,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            outcome: None,
            process_type,
            timeline: None,
        }
    }

    /// Add a participant to the process
    pub fn add_participant(&mut self, participant: Participant) {
        self.participants.push(participant);
    }

    /// Record a participant's position
    pub fn record_position(&mut self, participant_id: Uuid, position: Position) {
        self.positions.insert(participant_id, position);
    }

    /// Check if consensus has been reached
    pub fn check_consensus(&self) -> bool {
        match self.process_type {
            ConsensusType::Unanimous => {
                // All participants must agree
                self.positions.values().all(|p| matches!(p.stance, Stance::Agree))
            },
            ConsensusType::RoughConsensus => {
                // No strong objections and majority agreement
                let objections = self.positions.values()
                    .filter(|p| matches!(p.stance, Stance::Block | Stance::StronglyDisagree))
                    .count();
                
                if objections > 0 {
                    return false;
                }
                
                let agreements = self.positions.values()
                    .filter(|p| matches!(p.stance, Stance::Agree | Stance::StronglyAgree))
                    .count();
                
                let total_positions = self.positions.len();
                agreements > (total_positions * 2 / 3) // 2/3 majority
            },
            ConsensusType::ModifiedConsensus => {
                // No blocks and supermajority agreement
                let blocks = self.positions.values()
                    .filter(|p| matches!(p.stance, Stance::Block))
                    .count();
                
                if blocks > 0 {
                    return false;
                }
                
                let agreements = self.positions.values()
                    .filter(|p| matches!(p.stance, Stance::Agree | Stance::StronglyAgree))
                    .count();
                
                let total_positions = self.positions.len();
                agreements > (total_positions * 3 / 4) // 3/4 supermajority
            }
        }
    }

    /// Start the consensus process
    pub fn start(&mut self) {
        self.status = ConsensusStatus::Active;
        self.started_at = Some(Utc::now());
    }

    /// Finalize the consensus process
    pub fn finalize(&mut self, outcome: ConsensusOutcome) {
        self.status = ConsensusStatus::Completed;
        self.completed_at = Some(Utc::now());
        self.outcome = Some(outcome);
    }

    /// Set timeline for the process
    pub fn set_timeline(&mut self, timeline: ConsensusTimeline) {
        self.timeline = Some(timeline);
    }

    /// Get participant positions
    pub fn get_positions(&self) -> &HashMap<Uuid, Position> {
        &self.positions
    }

    /// Get participants who agreed
    pub fn get_agreements(&self) -> Vec<&Participant> {
        self.participants.iter()
            .filter(|p| {
                self.positions.get(&p.id)
                    .map(|pos| matches!(pos.stance, Stance::Agree | Stance::StronglyAgree))
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Get participants with objections
    pub fn get_objections(&self) -> Vec<(&Participant, &Position)> {
        self.participants.iter()
            .filter_map(|p| {
                self.positions.get(&p.id)
                    .and_then(|pos| {
                        if matches!(pos.stance, Stance::Disagree | Stance::StronglyDisagree | Stance::Block) {
                            Some((p, pos))
                        } else {
                            None
                        }
                    })
            })
            .collect()
    }
}

/// Participant in a consensus process
#[derive(Debug, Clone)]
pub struct Participant {
    pub id: Uuid,
    pub user_id: String,
    pub name: String,
    pub role: String, // Role in the community
    pub joined_at: DateTime<Utc>,
}

impl Participant {
    /// Create a new participant
    pub fn new(user_id: String, name: String, role: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            name,
            role,
            joined_at: Utc::now(),
        }
    }
}

/// Position statement from a participant
#[derive(Debug, Clone)]
pub struct Position {
    pub participant_id: Uuid,
    pub stance: Stance,
    pub explanation: String,
    pub alternatives_suggested: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

impl Position {
    /// Create a new position statement
    pub fn new(participant_id: Uuid, stance: Stance, explanation: String) -> Self {
        Self {
            participant_id,
            stance,
            explanation,
            alternatives_suggested: Vec::new(),
            timestamp: Utc::now(),
        }
    }

    /// Add alternative suggestions
    pub fn add_alternatives(&mut self, alternatives: Vec<String>) {
        self.alternatives_suggested.extend(alternatives);
    }
}

/// Decision record for historical tracking
#[derive(Debug, Clone)]
pub struct DecisionRecord {
    pub id: Uuid,
    pub description: String,
    pub decision_type: DecisionType,
    pub status: DecisionStatus,
    pub related_entity: Option<Uuid>, // Proposal ID, Process ID, etc.
    pub outcome: Option<ConsensusOutcome>,
    pub timestamp: DateTime<Utc>,
    pub made_by: Option<String>, // Role ID or user ID
}

impl DecisionRecord {
    /// Create a new decision record
    pub fn new(
        description: String,
        decision_type: DecisionType,
        status: DecisionStatus,
        related_entity: Option<Uuid>,
        outcome: Option<ConsensusOutcome>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            description,
            decision_type,
            status,
            related_entity,
            outcome,
            timestamp: Utc::now(),
            made_by: None,
        }
    }

    /// Set who made the decision
    pub fn set_made_by(&mut self, made_by: String) {
        self.made_by = Some(made_by);
    }
}

/// Statistics about decision-making activities
#[derive(Debug, Clone)]
pub struct DecisionStatistics {
    pub total_proposals: usize,
    pub total_discussions: usize,
    pub total_processes: usize,
    pub total_decisions: usize,
    pub approved_proposals: usize,
    pub rejected_proposals: usize,
    pub active_processes: usize,
}

/// Decision report
#[derive(Debug, Clone)]
pub struct DecisionReport {
    pub generated_at: DateTime<Utc>,
    pub statistics: DecisionStatistics,
    pub recent_decisions: usize,
}

/// Types of proposals
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProposalCategory {
    FeatureRequest,
    PolicyChange,
    ProcessImprovement,
    ResourceAllocation,
    CommunityEvent,
    TechnicalChange,
    Governance,
}

/// Status of proposals
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProposalStatus {
    Draft,
    InReview,
    Approved,
    Rejected,
    Implemented,
    Withdrawn,
}

/// Types of consensus processes
#[derive(Debug, Clone, PartialEq)]
pub enum ConsensusType {
    Unanimous,
    RoughConsensus,
    ModifiedConsensus,
}

/// Status of consensus processes
#[derive(Debug, Clone, PartialEq)]
pub enum ConsensusStatus {
    Created,
    Active,
    Completed,
    Cancelled,
}

/// Stance in a position statement
#[derive(Debug, Clone, PartialEq)]
pub enum Stance {
    StronglyAgree,
    Agree,
    Neutral,
    Disagree,
    StronglyDisagree,
    Block, // Veto power
}

/// Consensus outcome
#[derive(Debug, Clone)]
pub struct ConsensusOutcome {
    pub decision: String,
    pub next_steps: Vec<String>,
    pub responsibilities: HashMap<String, String>, // Role ID to responsibility
    pub timeline: Option<DateTime<Utc>>,
}

impl ConsensusOutcome {
    /// Create a new consensus outcome
    pub fn new(decision: String) -> Self {
        Self {
            decision,
            next_steps: Vec::new(),
            responsibilities: HashMap::new(),
            timeline: None,
        }
    }

    /// Add next steps
    pub fn add_next_steps(&mut self, steps: Vec<String>) {
        self.next_steps.extend(steps);
    }

    /// Assign responsibilities
    pub fn assign_responsibilities(&mut self, assignments: HashMap<String, String>) {
        self.responsibilities.extend(assignments);
    }

    /// Set timeline
    pub fn set_timeline(&mut self, deadline: DateTime<Utc>) {
        self.timeline = Some(deadline);
    }
}

/// Timeline for consensus process
#[derive(Debug, Clone)]
pub struct ConsensusTimeline {
    pub proposal_period: Option<i64>, // Hours
    pub discussion_period: Option<i64>, // Hours
    pub position_statement_period: Option<i64>, // Hours
    pub consensus_check_period: Option<i64>, // Hours
}

/// Types of decisions
#[derive(Debug, Clone, PartialEq)]
pub enum DecisionType {
    Consensus,
    Vote,
    Proposal,
    Administrative,
}

/// Status of decisions
#[derive(Debug, Clone, PartialEq)]
pub enum DecisionStatus {
    Proposed,
    InProgress,
    Finalized,
    Implemented,
    Rejected,
}

/// Error types for decision-making system
#[derive(Debug)]
pub enum DecisionError {
    ProposalNotFound(Uuid),
    DiscussionNotFound(Uuid),
    ProcessNotFound(Uuid),
    ParticipantNotFound(Uuid),
    ConsensusNotReached,
    ProcessNotActive,
    UpdateError(String),
}

impl std::fmt::Display for DecisionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecisionError::ProposalNotFound(id) => write!(f, "Proposal not found: {}", id),
            DecisionError::DiscussionNotFound(id) => write!(f, "Discussion not found: {}", id),
            DecisionError::ProcessNotFound(id) => write!(f, "Process not found: {}", id),
            DecisionError::ParticipantNotFound(id) => write!(f, "Participant not found: {}", id),
            DecisionError::ConsensusNotReached => write!(f, "Consensus not reached"),
            DecisionError::ProcessNotActive => write!(f, "Process not active"),
            DecisionError::UpdateError(msg) => write!(f, "Update error: {}", msg),
        }
    }
}

impl std::error::Error for DecisionError {}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_decision_making_system_initialization() {
        let system = DecisionMakingSystem::new();
        assert!(system.proposals.is_empty());
        assert!(system.discussions.is_empty());
        assert!(system.consensus_processes.is_empty());
    }

    #[test]
    fn test_create_proposal() {
        let mut system = DecisionMakingSystem::new();
        let proposal = Proposal::new(
            "New Feature".to_string(),
            "Implement a new feature".to_string(),
            "user123".to_string(),
            ProposalCategory::FeatureRequest,
        );
        
        let proposal_id = system.create_proposal(proposal);
        assert!(!proposal_id.is_nil());
        assert_eq!(system.proposals.len(), 1);
    }

    #[test]
    fn test_update_proposal_status() {
        let mut system = DecisionMakingSystem::new();
        let proposal = Proposal::new(
            "New Feature".to_string(),
            "Implement a new feature".to_string(),
            "user123".to_string(),
            ProposalCategory::FeatureRequest,
        );
        
        let proposal_id = system.create_proposal(proposal);
        let result = system.update_proposal_status(proposal_id, ProposalStatus::InReview);
        assert!(result.is_ok());
        
        let updated_proposal = system.get_proposal(proposal_id).unwrap();
        assert_eq!(updated_proposal.status, ProposalStatus::InReview);
    }

    #[test]
    fn test_create_discussion() {
        let mut system = DecisionMakingSystem::new();
        let proposal = Proposal::new(
            "New Feature".to_string(),
            "Implement a new feature".to_string(),
            "user123".to_string(),
            ProposalCategory::FeatureRequest,
        );
        
        let proposal_id = system.create_proposal(proposal);
        let discussion = Discussion::new(proposal_id, "Discussion on New Feature".to_string());
        
        let discussion_id = system.create_discussion(discussion);
        assert!(!discussion_id.is_nil());
        assert_eq!(system.discussions.len(), 1);
    }

    #[test]
    fn test_add_comment() {
        let mut system = DecisionMakingSystem::new();
        let proposal = Proposal::new(
            "New Feature".to_string(),
            "Implement a new feature".to_string(),
            "user123".to_string(),
            ProposalCategory::FeatureRequest,
        );
        
        let proposal_id = system.create_proposal(proposal);
        let discussion = Discussion::new(proposal_id, "Discussion on New Feature".to_string());
        let discussion_id = system.create_discussion(discussion);
        
        let comment = Comment::new("user123".to_string(), "Great idea!".to_string());
        let result = system.add_comment(discussion_id, comment);
        assert!(result.is_ok());
        
        let comments = system.get_comments(discussion_id).unwrap();
        assert_eq!(comments.len(), 1);
    }

    #[test]
    fn test_start_consensus_process() {
        let mut system = DecisionMakingSystem::new();
        let process = ConsensusProcess::new(
            "Feature Implementation".to_string(),
            "Decide on implementing the new feature".to_string(),
            "facilitator123".to_string(),
            ConsensusType::RoughConsensus,
        );
        
        let process_id = system.start_consensus_process(process);
        assert!(!process_id.is_nil());
        assert_eq!(system.consensus_processes.len(), 1);
    }

    #[test]
    fn test_add_participant() {
        let mut system = DecisionMakingSystem::new();
        let process = ConsensusProcess::new(
            "Feature Implementation".to_string(),
            "Decide on implementing the new feature".to_string(),
            "facilitator123".to_string(),
            ConsensusType::RoughConsensus,
        );
        
        let process_id = system.start_consensus_process(process);
        let participant = Participant::new(
            "user123".to_string(),
            "Alice".to_string(),
            "Developer".to_string(),
        );
        
        let result = system.add_participant(process_id, participant);
        assert!(result.is_ok());
        
        let process = system.consensus_processes.get(&process_id).unwrap();
        assert_eq!(process.participants.len(), 1);
    }

    #[test]
    fn test_record_position() {
        let mut system = DecisionMakingSystem::new();
        let process = ConsensusProcess::new(
            "Feature Implementation".to_string(),
            "Decide on implementing the new feature".to_string(),
            "facilitator123".to_string(),
            ConsensusType::RoughConsensus,
        );
        
        let process_id = system.start_consensus_process(process);
        let participant = Participant::new(
            "user123".to_string(),
            "Alice".to_string(),
            "Developer".to_string(),
        );
        
        system.add_participant(process_id, participant.clone());
        
        let position = Position::new(
            participant.id,
            Stance::Agree,
            "I agree with this proposal".to_string(),
        );
        
        let result = system.record_position(process_id, participant.id, position);
        assert!(result.is_ok());
        
        let process = system.consensus_processes.get(&process_id).unwrap();
        assert!(process.positions.contains_key(&participant.id));
    }

    #[test]
    fn test_check_consensus_rough_consensus() {
        let mut system = DecisionMakingSystem::new();
        let process = ConsensusProcess::new(
            "Feature Implementation".to_string(),
            "Decide on implementing the new feature".to_string(),
            "facilitator123".to_string(),
            ConsensusType::RoughConsensus,
        );
        
        let process_id = system.start_consensus_process(process);
        
        // Add participants
        let participant1 = Participant::new(
            "user1".to_string(),
            "Alice".to_string(),
            "Developer".to_string(),
        );
        let participant2 = Participant::new(
            "user2".to_string(),
            "Bob".to_string(),
            "Designer".to_string(),
        );
        let participant3 = Participant::new(
            "user3".to_string(),
            "Charlie".to_string(),
            "User".to_string(),
        );
        
        system.add_participant(process_id, participant1.clone());
        system.add_participant(process_id, participant2.clone());
        system.add_participant(process_id, participant3.clone());
        
        // Record positions - 2 agree, 1 neutral (should reach rough consensus)
        let position1 = Position::new(
            participant1.id,
            Stance::Agree,
            "I agree".to_string(),
        );
        let position2 = Position::new(
            participant2.id,
            Stance::Agree,
            "I also agree".to_string(),
        );
        let position3 = Position::new(
            participant3.id,
            Stance::Neutral,
            "No strong opinion".to_string(),
        );
        
        system.record_position(process_id, participant1.id, position1);
        system.record_position(process_id, participant2.id, position2);
        system.record_position(process_id, participant3.id, position3);
        
        let consensus_reached = system.check_consensus(process_id).unwrap();
        assert!(consensus_reached);
    }

    #[test]
    fn test_record_decision() {
        let mut system = DecisionMakingSystem::new();
        let decision = DecisionRecord::new(
            "Approved feature implementation".to_string(),
            DecisionType::Consensus,
            DecisionStatus::Finalized,
            None,
            None,
        );
        
        system.record_decision(decision);
        assert_eq!(system.decision_history.len(), 1);
    }

    #[test]
    fn test_get_statistics() {
        let mut system = DecisionMakingSystem::new();
        
        // Add some data
        let proposal = Proposal::new(
            "New Feature".to_string(),
            "Implement a new feature".to_string(),
            "user123".to_string(),
            ProposalCategory::FeatureRequest,
        );
        system.create_proposal(proposal);
        
        let stats = system.get_statistics();
        assert_eq!(stats.total_proposals, 1);
        assert_eq!(stats.total_discussions, 0);
        assert_eq!(stats.total_processes, 0);
    }

    #[test]
    fn test_search_proposals() {
        let mut system = DecisionMakingSystem::new();
        let proposal = Proposal::new(
            "New Feature Implementation".to_string(),
            "Implement a new feature for the dashboard".to_string(),
            "user123".to_string(),
            ProposalCategory::FeatureRequest,
        );
        system.create_proposal(proposal);
        
        let results = system.search_proposals("feature");
        assert_eq!(results.len(), 1);
    }
}