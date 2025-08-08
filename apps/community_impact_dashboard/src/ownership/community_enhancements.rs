//! Community-led enhancement processes for the Unified Community Impact Dashboard
//!
//! This module provides tools for community members to propose, discuss, and implement
//! enhancements to the dashboard, fostering continuous improvement through community engagement.

use tracing::info;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

/// Community enhancement system
pub struct CommunityEnhancementSystem {
    enhancement_proposals: HashMap<Uuid, EnhancementProposal>,
    enhancement_discussions: HashMap<Uuid, EnhancementDiscussion>,
    enhancement_implementations: HashMap<Uuid, EnhancementImplementation>,
    enhancement_contributions: HashMap<Uuid, Vec<Contribution>>, // Enhancement ID to contributions
    enhancement_reviews: HashMap<Uuid, Vec<EnhancementReview>>, // Enhancement ID to reviews
    enhancement_history: Vec<EnhancementRecord>,
    community_leaders: Vec<String>, // Role IDs of community leaders
    enhancement_categories: Vec<EnhancementCategory>,
}

impl CommunityEnhancementSystem {
    /// Create a new community enhancement system
    pub fn new() -> Self {
        Self {
            enhancement_proposals: HashMap::new(),
            enhancement_discussions: HashMap::new(),
            enhancement_implementations: HashMap::new(),
            enhancement_contributions: HashMap::new(),
            enhancement_reviews: HashMap::new(),
            enhancement_history: Vec::new(),
            community_leaders: Vec::new(),
            enhancement_categories: vec![
                EnhancementCategory::new("ui_improvements".to_string(), "UI Improvements".to_string()),
                EnhancementCategory::new("performance".to_string(), "Performance".to_string()),
                EnhancementCategory::new("features".to_string(), "New Features".to_string()),
                EnhancementCategory::new("accessibility".to_string(), "Accessibility".to_string()),
                EnhancementCategory::new("documentation".to_string(), "Documentation".to_string()),
                EnhancementCategory::new("community".to_string(), "Community Features".to_string()),
            ],
        }
    }

    /// Add a community leader
    pub fn add_community_leader(&mut self, role_id: String) {
        self.community_leaders.push(role_id);
        info!("Added community leader: {}", &self.community_leaders.last().unwrap());
    }

    /// Propose a new enhancement
    pub fn propose_enhancement(&mut self, proposal: EnhancementProposal) -> Uuid {
        let proposal_id = proposal.id;
        self.enhancement_proposals.insert(proposal_id, proposal);
        self.enhancement_contributions.insert(proposal_id, Vec::new());
        self.enhancement_reviews.insert(proposal_id, Vec::new());
        info!("Proposed enhancement: {}", proposal_id);
        proposal_id
    }

    /// Get an enhancement proposal by ID
    pub fn get_proposal(&self, proposal_id: Uuid) -> Option<&EnhancementProposal> {
        self.enhancement_proposals.get(&proposal_id)
    }

    /// Update enhancement proposal status
    pub fn update_proposal_status(&mut self, proposal_id: Uuid, status: EnhancementStatus) -> Result<(), EnhancementError> {
        let proposal = self.enhancement_proposals.get_mut(&proposal_id)
            .ok_or(EnhancementError::ProposalNotFound(proposal_id))?;
        
        proposal.status = status;
        proposal.updated_at = Utc::now();
        
        info!("Updated enhancement proposal {} status to {:?}", proposal_id, status);
        Ok(())
    }

    /// Submit proposal for community review
    pub fn submit_for_review(&mut self, proposal_id: Uuid) -> Result<(), EnhancementError> {
        self.update_proposal_status(proposal_id, EnhancementStatus::InReview)
    }

    /// Approve an enhancement proposal
    pub fn approve_proposal(&mut self, proposal_id: Uuid) -> Result<(), EnhancementError> {
        self.update_proposal_status(proposal_id, EnhancementStatus::Approved)
    }

    /// Reject an enhancement proposal
    pub fn reject_proposal(&mut self, proposal_id: Uuid) -> Result<(), EnhancementError> {
        self.update_proposal_status(proposal_id, EnhancementStatus::Rejected)
    }

    /// Create a discussion for an enhancement
    pub fn create_discussion(&mut self, discussion: EnhancementDiscussion) -> Uuid {
        let discussion_id = discussion.id;
        self.enhancement_discussions.insert(discussion_id, discussion);
        info!("Created enhancement discussion: {}", discussion_id);
        discussion_id
    }

    /// Add a comment to an enhancement discussion
    pub fn add_comment(&mut self, discussion_id: Uuid, comment: EnhancementComment) -> Result<(), EnhancementError> {
        let discussion = self.enhancement_discussions.get_mut(&discussion_id)
            .ok_or(EnhancementError::DiscussionNotFound(discussion_id))?;
        
        discussion.add_comment(comment);
        info!("Added comment to enhancement discussion: {}", discussion_id);
        Ok(())
    }

    /// Get comments for an enhancement discussion
    pub fn get_comments(&self, discussion_id: Uuid) -> Result<&Vec<EnhancementComment>, EnhancementError> {
        let discussion = self.enhancement_discussions.get(&discussion_id)
            .ok_or(EnhancementError::DiscussionNotFound(discussion_id))?;
        
        Ok(&discussion.comments)
    }

    /// Start implementation of an enhancement
    pub fn start_implementation(&mut self, implementation: EnhancementImplementation) -> Uuid {
        let implementation_id = implementation.id;
        self.enhancement_implementations.insert(implementation_id, implementation);
        info!("Started enhancement implementation: {}", implementation_id);
        implementation_id
    }

    /// Add contribution to an enhancement
    pub fn add_contribution(&mut self, enhancement_id: Uuid, contribution: Contribution) -> Result<(), EnhancementError> {
        if !self.enhancement_proposals.contains_key(&enhancement_id) {
            return Err(EnhancementError::ProposalNotFound(enhancement_id));
        }
        
        self.enhancement_contributions.get_mut(&enhancement_id)
            .ok_or(EnhancementError::ProposalNotFound(enhancement_id))?
            .push(contribution);
        
        info!("Added contribution to enhancement: {}", enhancement_id);
        Ok(())
    }

    /// Add review to an enhancement
    pub fn add_review(&mut self, enhancement_id: Uuid, review: EnhancementReview) -> Result<(), EnhancementError> {
        if !self.enhancement_proposals.contains_key(&enhancement_id) {
            return Err(EnhancementError::ProposalNotFound(enhancement_id));
        }
        
        self.enhancement_reviews.get_mut(&enhancement_id)
            .ok_or(EnhancementError::ProposalNotFound(enhancement_id))?
            .push(review);
        
        info!("Added review to enhancement: {}", enhancement_id);
        Ok(())
    }

    /// Get contributions for an enhancement
    pub fn get_contributions(&self, enhancement_id: Uuid) -> Result<&Vec<Contribution>, EnhancementError> {
        self.enhancement_contributions.get(&enhancement_id)
            .ok_or(EnhancementError::ProposalNotFound(enhancement_id))
    }

    /// Get reviews for an enhancement
    pub fn get_reviews(&self, enhancement_id: Uuid) -> Result<&Vec<EnhancementReview>, EnhancementError> {
        self.enhancement_reviews.get(&enhancement_id)
            .ok_or(EnhancementError::ProposalNotFound(enhancement_id))
    }

    /// Complete enhancement implementation
    pub fn complete_implementation(&mut self, implementation_id: Uuid) -> Result<(), EnhancementError> {
        let implementation = self.enhancement_implementations.get_mut(&implementation_id)
            .ok_or(EnhancementError::ImplementationNotFound(implementation_id))?;
        
        implementation.complete();
        
        // Update the related proposal status
        self.update_proposal_status(implementation.enhancement_id, EnhancementStatus::Implemented)?;
        
        // Record the enhancement
        let record = EnhancementRecord::new(
            format!("Completed enhancement implementation: {}", implementation_id),
            implementation.enhancement_id,
            EnhancementRecordType::ImplementationCompleted,
        );
        self.enhancement_history.push(record);
        
        info!("Completed enhancement implementation: {}", implementation_id);
        Ok(())
    }

    /// Record an enhancement activity
    pub fn record_enhancement(&mut self, record: EnhancementRecord) {
        self.enhancement_history.push(record);
        info!("Recorded enhancement activity");
    }

    /// Get recent enhancement activities
    pub fn get_recent_activities(&self, days: i64) -> Vec<&EnhancementRecord> {
        let cutoff = Utc::now() - chrono::Duration::days(days);
        self.enhancement_history.iter()
            .filter(|record| record.timestamp > cutoff)
            .collect()
    }

    /// Get enhancement activities by type
    pub fn get_activities_by_type(&self, record_type: EnhancementRecordType) -> Vec<&EnhancementRecord> {
        self.enhancement_history.iter()
            .filter(|record| record.record_type == record_type)
            .collect()
    }

    /// Get enhancement statistics
    pub fn get_statistics(&self) -> EnhancementStatistics {
        let total_proposals = self.enhancement_proposals.len();
        let total_discussions = self.enhancement_discussions.len();
        let total_implementations = self.enhancement_implementations.len();
        let total_contributions: usize = self.enhancement_contributions.values().map(|v| v.len()).sum();
        let total_reviews: usize = self.enhancement_reviews.values().map(|v| v.len()).sum();
        
        // Count proposals by status
        let proposed_enhancements = self.enhancement_proposals.values()
            .filter(|p| p.status == EnhancementStatus::Proposed)
            .count();
        
        let approved_enhancements = self.enhancement_proposals.values()
            .filter(|p| p.status == EnhancementStatus::Approved)
            .count();
        
        let implemented_enhancements = self.enhancement_proposals.values()
            .filter(|p| p.status == EnhancementStatus::Implemented)
            .count();
        
        EnhancementStatistics {
            total_proposals,
            total_discussions,
            total_implementations,
            total_contributions,
            total_reviews,
            proposed_enhancements,
            approved_enhancements,
            implemented_enhancements,
        }
    }

    /// Generate enhancement report
    pub fn generate_enhancement_report(&self) -> EnhancementReport {
        let stats = self.get_statistics();
        let recent_activities = self.get_recent_activities(30); // Last 30 days
        
        EnhancementReport {
            generated_at: Utc::now(),
            statistics: stats,
            recent_activities: recent_activities.len(),
        }
    }

    /// Search enhancement proposals by keyword
    pub fn search_proposals(&self, keyword: &str) -> Vec<&EnhancementProposal> {
        self.enhancement_proposals.values()
            .filter(|proposal| {
                proposal.title.to_lowercase().contains(&keyword.to_lowercase()) ||
                proposal.description.to_lowercase().contains(&keyword.to_lowercase())
            })
            .collect()
    }

    /// Get enhancement proposals by status
    pub fn get_proposals_by_status(&self, status: EnhancementStatus) -> Vec<&EnhancementProposal> {
        self.enhancement_proposals.values()
            .filter(|proposal| proposal.status == status)
            .collect()
    }

    /// Get enhancement proposals by category
    pub fn get_proposals_by_category(&self, category_id: &str) -> Vec<&EnhancementProposal> {
        self.enhancement_proposals.values()
            .filter(|proposal| proposal.category.id == category_id)
            .collect()
    }

    /// Get enhancement categories
    pub fn get_categories(&self) -> &Vec<EnhancementCategory> {
        &self.enhancement_categories
    }

    /// Add a new enhancement category
    pub fn add_category(&mut self, category: EnhancementCategory) {
        self.enhancement_categories.push(category);
        info!("Added enhancement category: {}", &self.enhancement_categories.last().unwrap().id);
    }

    /// Get top contributors
    pub fn get_top_contributors(&self, count: usize) -> Vec<(String, usize)> {
        let mut contributor_counts: HashMap<String, usize> = HashMap::new();
        
        // Count contributions
        for contributions in self.enhancement_contributions.values() {
            for contribution in contributions {
                *contributor_counts.entry(contribution.contributor_id.clone()).or_insert(0) += 1;
            }
        }
        
        // Count reviews
        for reviews in self.enhancement_reviews.values() {
            for review in reviews {
                *contributor_counts.entry(review.reviewer_id.clone()).or_insert(0) += 1;
            }
        }
        
        // Convert to vector and sort
        let mut contributors: Vec<(String, usize)> = contributor_counts.into_iter().collect();
        contributors.sort_by(|a, b| b.1.cmp(&a.1));
        contributors.truncate(count);
        
        contributors
    }

    /// Get enhancement implementation by enhancement ID
    pub fn get_implementation_by_enhancement(&self, enhancement_id: Uuid) -> Option<&EnhancementImplementation> {
        self.enhancement_implementations.values()
            .find(|impl_| impl_.enhancement_id == enhancement_id)
    }

    /// Get enhancement discussion by enhancement ID
    pub fn get_discussion_by_enhancement(&self, enhancement_id: Uuid) -> Option<&EnhancementDiscussion> {
        self.enhancement_discussions.values()
            .find(|disc| disc.enhancement_id == enhancement_id)
    }
}

/// Enhancement proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancementProposal {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub author: String, // User ID
    pub category: EnhancementCategory,
    pub status: EnhancementStatus,
    pub priority: Option<PriorityLevel>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub discussion_thread: Option<Uuid>, // Discussion ID
    pub estimated_effort: Option<u32>, // In hours
    pub dependencies: Vec<Uuid>, // Enhancement IDs this enhancement depends on
    pub tags: Vec<String>,
    pub implementation_plan: Option<String>,
}

impl EnhancementProposal {
    /// Create a new enhancement proposal
    pub fn new(
        title: String,
        description: String,
        author: String,
        category: EnhancementCategory,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            author,
            category,
            status: EnhancementStatus::Proposed,
            priority: None,
            created_at: now,
            updated_at: now,
            discussion_thread: None,
            estimated_effort: None,
            dependencies: Vec::new(),
            tags: Vec::new(),
            implementation_plan: None,
        }
    }

    /// Set priority level
    pub fn set_priority(&mut self, priority: PriorityLevel) {
        self.priority = Some(priority);
        self.updated_at = Utc::now();
    }

    /// Set estimated effort
    pub fn set_estimated_effort(&mut self, effort: u32) {
        self.estimated_effort = Some(effort);
        self.updated_at = Utc::now();
    }

    /// Add dependencies
    pub fn add_dependencies(&mut self, dependencies: Vec<Uuid>) {
        self.dependencies.extend(dependencies);
        self.updated_at = Utc::now();
    }

    /// Add tags
    pub fn add_tags(&mut self, tags: Vec<String>) {
        self.tags.extend(tags);
        self.updated_at = Utc::now();
    }

    /// Set discussion thread
    pub fn set_discussion_thread(&mut self, discussion_id: Uuid) {
        self.discussion_thread = Some(discussion_id);
        self.updated_at = Utc::now();
    }

    /// Set implementation plan
    pub fn set_implementation_plan(&mut self, plan: String) {
        self.implementation_plan = Some(plan);
        self.updated_at = Utc::now();
    }

    /// Submit for review
    pub fn submit(&mut self) {
        self.status = EnhancementStatus::InReview;
        self.updated_at = Utc::now();
    }

    /// Approve the enhancement
    pub fn approve(&mut self) {
        self.status = EnhancementStatus::Approved;
        self.updated_at = Utc::now();
    }

    /// Reject the enhancement
    pub fn reject(&mut self) {
        self.status = EnhancementStatus::Rejected;
        self.updated_at = Utc::now();
    }

    /// Mark as implemented
    pub fn implement(&mut self) {
        self.status = EnhancementStatus::Implemented;
        self.updated_at = Utc::now();
    }

    /// Mark as in development
    pub fn start_development(&mut self) {
        self.status = EnhancementStatus::InProgress;
        self.updated_at = Utc::now();
    }
}

/// Enhancement category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancementCategory {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>, // Hex color code
}

impl EnhancementCategory {
    /// Create a new enhancement category
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            description: None,
            color: None,
        }
    }

    /// Set category description
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    /// Set category color
    pub fn set_color(&mut self, color: String) {
        self.color = Some(color);
    }
}

/// Enhancement discussion
#[derive(Debug, Clone)]
pub struct EnhancementDiscussion {
    pub id: Uuid,
    pub enhancement_id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub comments: Vec<EnhancementComment>,
    pub participants: Vec<String>, // User IDs
    pub is_locked: bool,
}

impl EnhancementDiscussion {
    /// Create a new enhancement discussion
    pub fn new(enhancement_id: Uuid, title: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            enhancement_id,
            title,
            created_at: Utc::now(),
            comments: Vec::new(),
            participants: Vec::new(),
            is_locked: false,
        }
    }

    /// Add a comment to the discussion
    pub fn add_comment(&mut self, comment: EnhancementComment) {
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
    pub fn get_comments_by_author(&self, author: &str) -> Vec<&EnhancementComment> {
        self.comments.iter()
            .filter(|comment| comment.author == author)
            .collect()
    }

    /// Get recent comments
    pub fn get_recent_comments(&self, count: usize) -> Vec<&EnhancementComment> {
        self.comments.iter()
            .rev()
            .take(count)
            .collect()
    }
}

/// Comment in an enhancement discussion
#[derive(Debug, Clone)]
pub struct EnhancementComment {
    pub id: Uuid,
    pub author: String, // User ID
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub edited_at: Option<DateTime<Utc>>,
    pub reactions: HashMap<String, usize>, // Reaction type and count
    pub replies: Vec<Uuid>, // Comment IDs of replies
}

impl EnhancementComment {
    /// Create a new enhancement comment
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

/// Enhancement implementation
#[derive(Debug, Clone)]
pub struct EnhancementImplementation {
    pub id: Uuid,
    pub enhancement_id: Uuid,
    pub title: String,
    pub description: String,
    pub implementers: Vec<String>, // User IDs
    pub status: ImplementationStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub estimated_completion: Option<DateTime<Utc>>,
    pub progress: f64, // 0.0 to 1.0
    pub milestones: Vec<ImplementationMilestone>,
}

impl EnhancementImplementation {
    /// Create a new enhancement implementation
    pub fn new(
        enhancement_id: Uuid,
        title: String,
        description: String,
        implementers: Vec<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            enhancement_id,
            title,
            description,
            implementers,
            status: ImplementationStatus::Planned,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            estimated_completion: None,
            progress: 0.0,
            milestones: Vec::new(),
        }
    }

    /// Start the implementation
    pub fn start(&mut self) {
        self.status = ImplementationStatus::InProgress;
        self.started_at = Some(Utc::now());
        self.progress = 0.1; // Started
    }

    /// Update progress
    pub fn update_progress(&mut self, progress: f64) -> Result<(), EnhancementError> {
        if progress < 0.0 || progress > 1.0 {
            return Err(EnhancementError::InvalidProgress);
        }
        
        self.progress = progress;
        self.updated_at();
        Ok(())
    }

    /// Add a milestone
    pub fn add_milestone(&mut self, milestone: ImplementationMilestone) {
        self.milestones.push(milestone);
        self.updated_at();
    }

    /// Complete the implementation
    pub fn complete(&mut self) {
        self.status = ImplementationStatus::Completed;
        self.completed_at = Some(Utc::now());
        self.progress = 1.0;
        self.updated_at();
    }

    /// Set estimated completion date
    pub fn set_estimated_completion(&mut self, date: DateTime<Utc>) {
        self.estimated_completion = Some(date);
        self.updated_at();
    }

    /// Add implementer
    pub fn add_implementer(&mut self, implementer_id: String) {
        self.implementers.push(implementer_id);
        self.updated_at();
    }

    /// Update timestamp
    fn updated_at(&mut self) {
        // This would typically update a last_modified field if we had one
    }
}

/// Implementation milestone
#[derive(Debug, Clone)]
pub struct ImplementationMilestone {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub due_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub status: MilestoneStatus,
}

impl ImplementationMilestone {
    /// Create a new implementation milestone
    pub fn new(title: String, description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            due_date: None,
            completed_at: None,
            status: MilestoneStatus::Pending,
        }
    }

    /// Set due date
    pub fn set_due_date(&mut self, date: DateTime<Utc>) {
        self.due_date = Some(date);
    }

    /// Complete the milestone
    pub fn complete(&mut self) {
        self.status = MilestoneStatus::Completed;
        self.completed_at = Some(Utc::now());
    }

    /// Mark as in progress
    pub fn start(&mut self) {
        self.status = MilestoneStatus::InProgress;
    }
}

/// Community contribution to an enhancement
#[derive(Debug, Clone)]
pub struct Contribution {
    pub id: Uuid,
    pub enhancement_id: Uuid,
    pub contributor_id: String, // User ID
    pub contribution_type: ContributionType,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub recognition_points: u32, // Points for community recognition
}

impl Contribution {
    /// Create a new contribution
    pub fn new(
        enhancement_id: Uuid,
        contributor_id: String,
        contribution_type: ContributionType,
        description: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            enhancement_id,
            contributor_id,
            contribution_type,
            description,
            timestamp: Utc::now(),
            recognition_points: 10, // Default points
        }
    }

    /// Set recognition points
    pub fn set_recognition_points(&mut self, points: u32) {
        self.recognition_points = points;
    }
}

/// Enhancement review
#[derive(Debug, Clone)]
pub struct EnhancementReview {
    pub id: Uuid,
    pub enhancement_id: Uuid,
    pub reviewer_id: String, // User ID
    pub rating: ReviewRating,
    pub feedback: String,
    pub suggestions: Vec<String>,
    pub timestamp: DateTime<Utc>,
    pub is_approved: Option<bool>,
}

impl EnhancementReview {
    /// Create a new enhancement review
    pub fn new(
        enhancement_id: Uuid,
        reviewer_id: String,
        rating: ReviewRating,
        feedback: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            enhancement_id,
            reviewer_id,
            rating,
            feedback,
            suggestions: Vec::new(),
            timestamp: Utc::now(),
            is_approved: None,
        }
    }

    /// Add suggestions
    pub fn add_suggestions(&mut self, suggestions: Vec<String>) {
        self.suggestions.extend(suggestions);
    }

    /// Approve the enhancement
    pub fn approve(&mut self) {
        self.is_approved = Some(true);
    }

    /// Reject the enhancement
    pub fn reject(&mut self) {
        self.is_approved = Some(false);
    }
}

/// Enhancement record for historical tracking
#[derive(Debug, Clone)]
pub struct EnhancementRecord {
    pub id: Uuid,
    pub description: String,
    pub enhancement_id: Uuid,
    pub record_type: EnhancementRecordType,
    pub timestamp: DateTime<Utc>,
    pub recorded_by: Option<String>, // User ID
}

impl EnhancementRecord {
    /// Create a new enhancement record
    pub fn new(
        description: String,
        enhancement_id: Uuid,
        record_type: EnhancementRecordType,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            description,
            enhancement_id,
            record_type,
            timestamp: Utc::now(),
            recorded_by: None,
        }
    }

    /// Set who recorded the enhancement
    pub fn set_recorded_by(&mut self, recorded_by: String) {
        self.recorded_by = Some(recorded_by);
    }
}

/// Statistics about community enhancements
#[derive(Debug, Clone)]
pub struct EnhancementStatistics {
    pub total_proposals: usize,
    pub total_discussions: usize,
    pub total_implementations: usize,
    pub total_contributions: usize,
    pub total_reviews: usize,
    pub proposed_enhancements: usize,
    pub approved_enhancements: usize,
    pub implemented_enhancements: usize,
}

/// Enhancement report
#[derive(Debug, Clone)]
pub struct EnhancementReport {
    pub generated_at: DateTime<Utc>,
    pub statistics: EnhancementStatistics,
    pub recent_activities: usize,
}

/// Status of enhancement proposals
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EnhancementStatus {
    Proposed,
    InReview,
    Approved,
    Rejected,
    InProgress,
    Implemented,
    Deferred,
}

/// Status of implementation
#[derive(Debug, Clone, PartialEq)]
pub enum ImplementationStatus {
    Planned,
    InProgress,
    Completed,
    Blocked,
    Cancelled,
}

/// Status of milestones
#[derive(Debug, Clone, PartialEq)]
pub enum MilestoneStatus {
    Pending,
    InProgress,
    Completed,
    Blocked,
}

/// Types of contributions
#[derive(Debug, Clone, PartialEq)]
pub enum ContributionType {
    Code,
    Documentation,
    Testing,
    Design,
    Review,
    Translation,
    CommunitySupport,
    Other(String),
}

/// Review ratings
#[derive(Debug, Clone, PartialEq)]
pub enum ReviewRating {
    Excellent,
    Good,
    Average,
    Poor,
    VeryPoor,
}

/// Types of enhancement records
#[derive(Debug, Clone, PartialEq)]
pub enum EnhancementRecordType {
    ProposalCreated,
    ProposalApproved,
    ProposalRejected,
    ImplementationStarted,
    ImplementationCompleted,
    EnhancementImplemented,
    ContributionAdded,
    ReviewAdded,
}

/// Priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum PriorityLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Error types for enhancement system
#[derive(Debug)]
pub enum EnhancementError {
    ProposalNotFound(Uuid),
    DiscussionNotFound(Uuid),
    ImplementationNotFound(Uuid),
    ContributionError(String),
    ReviewError(String),
    InvalidProgress,
    UpdateError(String),
}

impl std::fmt::Display for EnhancementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnhancementError::ProposalNotFound(id) => write!(f, "Enhancement proposal not found: {}", id),
            EnhancementError::DiscussionNotFound(id) => write!(f, "Enhancement discussion not found: {}", id),
            EnhancementError::ImplementationNotFound(id) => write!(f, "Enhancement implementation not found: {}", id),
            EnhancementError::ContributionError(msg) => write!(f, "Contribution error: {}", msg),
            EnhancementError::ReviewError(msg) => write!(f, "Review error: {}", msg),
            EnhancementError::InvalidProgress => write!(f, "Invalid progress value"),
            EnhancementError::UpdateError(msg) => write!(f, "Update error: {}", msg),
        }
    }
}

impl std::error::Error for EnhancementError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhancement_system_initialization() {
        let system = CommunityEnhancementSystem::new();
        assert!(system.enhancement_proposals.is_empty());
        assert!(system.enhancement_discussions.is_empty());
        assert!(system.enhancement_implementations.is_empty());
        assert!(!system.enhancement_categories.is_empty());
    }

    #[test]
    fn test_propose_enhancement() {
        let mut system = CommunityEnhancementSystem::new();
        let category = EnhancementCategory::new("ui".to_string(), "User Interface".to_string());
        let proposal = EnhancementProposal::new(
            "Improve Dashboard Layout".to_string(),
            "Enhance the dashboard layout for better usability".to_string(),
            "user123".to_string(),
            category,
        );
        
        let proposal_id = system.propose_enhancement(proposal);
        assert!(!proposal_id.is_nil());
        assert_eq!(system.enhancement_proposals.len(), 1);
        assert!(system.enhancement_contributions.contains_key(&proposal_id));
        assert!(system.enhancement_reviews.contains_key(&proposal_id));
    }

    #[test]
    fn test_update_proposal_status() {
        let mut system = CommunityEnhancementSystem::new();
        let category = EnhancementCategory::new("ui".to_string(), "User Interface".to_string());
        let proposal = EnhancementProposal::new(
            "Improve Dashboard Layout".to_string(),
            "Enhance the dashboard layout for better usability".to_string(),
            "user123".to_string(),
            category,
        );
        
        let proposal_id = system.propose_enhancement(proposal);
        let result = system.update_proposal_status(proposal_id, EnhancementStatus::InReview);
        assert!(result.is_ok());
        
        let updated_proposal = system.get_proposal(proposal_id).unwrap();
        assert_eq!(updated_proposal.status, EnhancementStatus::InReview);
    }

    #[test]
    fn test_create_discussion() {
        let mut system = CommunityEnhancementSystem::new();
        let category = EnhancementCategory::new("ui".to_string(), "User Interface".to_string());
        let proposal = EnhancementProposal::new(
            "Improve Dashboard Layout".to_string(),
            "Enhance the dashboard layout for better usability".to_string(),
            "user123".to_string(),
            category,
        );
        
        let proposal_id = system.propose_enhancement(proposal);
        let discussion = EnhancementDiscussion::new(proposal_id, "Discussion on Dashboard Layout".to_string());
        
        let discussion_id = system.create_discussion(discussion);
        assert!(!discussion_id.is_nil());
        assert_eq!(system.enhancement_discussions.len(), 1);
    }

    #[test]
    fn test_add_comment() {
        let mut system = CommunityEnhancementSystem::new();
        let category = EnhancementCategory::new("ui".to_string(), "User Interface".to_string());
        let proposal = EnhancementProposal::new(
            "Improve Dashboard Layout".to_string(),
            "Enhance the dashboard layout for better usability".to_string(),
            "user123".to_string(),
            category,
        );
        
        let proposal_id = system.propose_enhancement(proposal);
        let discussion = EnhancementDiscussion::new(proposal_id, "Discussion on Dashboard Layout".to_string());
        let discussion_id = system.create_discussion(discussion);
        
        let comment = EnhancementComment::new("user123".to_string(), "Great idea!".to_string());
        let result = system.add_comment(discussion_id, comment);
        assert!(result.is_ok());
        
        let comments = system.get_comments(discussion_id).unwrap();
        assert_eq!(comments.len(), 1);
    }

    #[test]
    fn test_start_implementation() {
        let mut system = CommunityEnhancementSystem::new();
        let category = EnhancementCategory::new("ui".to_string(), "User Interface".to_string());
        let proposal = EnhancementProposal::new(
            "Improve Dashboard Layout".to_string(),
            "Enhance the dashboard layout for better usability".to_string(),
            "user123".to_string(),
            category,
        );
        
        let proposal_id = system.propose_enhancement(proposal);
        let implementation = EnhancementImplementation::new(
            proposal_id,
            "Implement Dashboard Layout Improvements".to_string(),
            "Implementation of dashboard layout enhancements".to_string(),
            vec!["developer123".to_string()],
        );
        
        let implementation_id = system.start_implementation(implementation);
        assert!(!implementation_id.is_nil());
        assert_eq!(system.enhancement_implementations.len(), 1);
    }

    #[test]
    fn test_add_contribution() {
        let mut system = CommunityEnhancementSystem::new();
        let category = EnhancementCategory::new("ui".to_string(), "User Interface".to_string());
        let proposal = EnhancementProposal::new(
            "Improve Dashboard Layout".to_string(),
            "Enhance the dashboard layout for better usability".to_string(),
            "user123".to_string(),
            category,
        );
        
        let proposal_id = system.propose_enhancement(proposal);
        let contribution = Contribution::new(
            proposal_id,
            "contributor123".to_string(),
            ContributionType::Code,
            "Implemented the core layout changes".to_string(),
        );
        
        let result = system.add_contribution(proposal_id, contribution);
        assert!(result.is_ok());
        
        let contributions = system.get_contributions(proposal_id).unwrap();
        assert_eq!(contributions.len(), 1);
    }

    #[test]
    fn test_add_review() {
        let mut system = CommunityEnhancementSystem::new();
        let category = EnhancementCategory::new("ui".to_string(), "User Interface".to_string());
        let proposal = EnhancementProposal::new(
            "Improve Dashboard Layout".to_string(),
            "Enhance the dashboard layout for better usability".to_string(),
            "user123".to_string(),
            category,
        );
        
        let proposal_id = system.propose_enhancement(proposal);
        let review = EnhancementReview::new(
            proposal_id,
            "reviewer123".to_string(),
            ReviewRating::Excellent,
            "Excellent enhancement proposal".to_string(),
        );
        
        let result = system.add_review(proposal_id, review);
        assert!(result.is_ok());
        
        let reviews = system.get_reviews(proposal_id).unwrap();
        assert_eq!(reviews.len(), 1);
    }

    #[test]
    fn test_complete_implementation() {
        let mut system = CommunityEnhancementSystem::new();
        let category = EnhancementCategory::new("ui".to_string(), "User Interface".to_string());
        let proposal = EnhancementProposal::new(
            "Improve Dashboard Layout".to_string(),
            "Enhance the dashboard layout for better usability".to_string(),
            "user123".to_string(),
            category,
        );
        
        let proposal_id = system.propose_enhancement(proposal);
        let implementation = EnhancementImplementation::new(
            proposal_id,
            "Implement Dashboard Layout Improvements".to_string(),
            "Implementation of dashboard layout enhancements".to_string(),
            vec!["developer123".to_string()],
        );
        
        let implementation_id = system.start_implementation(implementation);
        let result = system.complete_implementation(implementation_id);
        assert!(result.is_ok());
        
        let updated_proposal = system.get_proposal(proposal_id).unwrap();
        assert_eq!(updated_proposal.status, EnhancementStatus::Implemented);
    }

    #[test]
    fn test_get_statistics() {
        let mut system = CommunityEnhancementSystem::new();
        let category = EnhancementCategory::new("ui".to_string(), "User Interface".to_string());
        let proposal = EnhancementProposal::new(
            "Improve Dashboard Layout".to_string(),
            "Enhance the dashboard layout for better usability".to_string(),
            "user123".to_string(),
            category,
        );
        
        system.propose_enhancement(proposal);
        
        let stats = system.get_statistics();
        assert_eq!(stats.total_proposals, 1);
        assert_eq!(stats.total_discussions, 0);
        assert_eq!(stats.total_implementations, 0);
    }

    #[test]
    fn test_search_proposals() {
        let mut system = CommunityEnhancementSystem::new();
        let category = EnhancementCategory::new("ui".to_string(), "User Interface".to_string());
        let proposal = EnhancementProposal::new(
            "Improve Dashboard Layout".to_string(),
            "Enhance the dashboard layout for better usability and user experience".to_string(),
            "user123".to_string(),
            category,
        );
        
        system.propose_enhancement(proposal);
        
        let results = system.search_proposals("dashboard");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_enhancement_proposal_methods() {
        let mut proposal = EnhancementProposal::new(
            "Test Enhancement".to_string(),
            "Test description".to_string(),
            "user123".to_string(),
            EnhancementCategory::new("test".to_string(), "Test Category".to_string()),
        );
        
        // Test setting priority
        proposal.set_priority(PriorityLevel::High);
        assert_eq!(proposal.priority, Some(PriorityLevel::High));
        
        // Test setting estimated effort
        proposal.set_estimated_effort(10);
        assert_eq!(proposal.estimated_effort, Some(10));
        
        // Test adding dependencies
        let dep_id = Uuid::new_v4();
        proposal.add_dependencies(vec![dep_id]);
        assert!(proposal.dependencies.contains(&dep_id));
        
        // Test adding tags
        proposal.add_tags(vec!["tag1".to_string(), "tag2".to_string()]);
        assert!(proposal.tags.contains(&"tag1".to_string()));
        assert!(proposal.tags.contains(&"tag2".to_string()));
        
        // Test setting discussion thread
        let disc_id = Uuid::new_v4();
        proposal.set_discussion_thread(disc_id);
        assert_eq!(proposal.discussion_thread, Some(disc_id));
        
        // Test setting implementation plan
        proposal.set_implementation_plan("Implementation plan".to_string());
        assert_eq!(proposal.implementation_plan, Some("Implementation plan".to_string()));
        
        // Test status changes
        proposal.submit();
        assert_eq!(proposal.status, EnhancementStatus::InReview);
        
        proposal.approve();
        assert_eq!(proposal.status, EnhancementStatus::Approved);
        
        proposal.start_development();
        assert_eq!(proposal.status, EnhancementStatus::InProgress);
        
        proposal.implement();
        assert_eq!(proposal.status, EnhancementStatus::Implemented);
    }

    #[test]
    fn test_enhancement_implementation_progress() {
        let enhancement_id = Uuid::new_v4();
        let mut implementation = EnhancementImplementation::new(
            enhancement_id,
            "Test Implementation".to_string(),
            "Test description".to_string(),
            vec!["developer123".to_string()],
        );
        
        // Test valid progress update
        let result = implementation.update_progress(0.5);
        assert!(result.is_ok());
        assert_eq!(implementation.progress, 0.5);
        
        // Test completion
        implementation.complete();
        assert_eq!(implementation.progress, 1.0);
        assert_eq!(implementation.status, ImplementationStatus::Completed);
        
        // Test invalid progress
        let result = implementation.update_progress(1.5);
        assert!(result.is_err());
    }

    #[test]
    fn test_enhancement_review_approval() {
        let enhancement_id = Uuid::new_v4();
        let mut review = EnhancementReview::new(
            enhancement_id,
            "reviewer123".to_string(),
            ReviewRating::Good,
            "Good enhancement".to_string(),
        );
        
        // Test approval
        review.approve();
        assert_eq!(review.is_approved, Some(true));
        
        // Test rejection
        review.reject();
        assert_eq!(review.is_approved, Some(false));
        
        // Test adding suggestions
        review.add_suggestions(vec!["Suggestion 1".to_string(), "Suggestion 2".to_string()]);
        assert_eq!(review.suggestions.len(), 2);
    }

    #[test]
    fn test_enhancement_discussion_commenting() {
        let enhancement_id = Uuid::new_v4();
        let mut discussion = EnhancementDiscussion::new(enhancement_id, "Test Discussion".to_string());
        
        // Test adding comments
        let comment1 = EnhancementComment::new("user1".to_string(), "First comment".to_string());
        let comment2 = EnhancementComment::new("user2".to_string(), "Second comment".to_string());
        
        discussion.add_comment(comment1);
        discussion.add_comment(comment2);
        
        assert_eq!(discussion.comments.len(), 2);
        assert_eq!(discussion.participants.len(), 2);
        assert!(discussion.participants.contains(&"user1".to_string()));
        assert!(discussion.participants.contains(&"user2".to_string()));
        
        // Test getting comments by author
        let user1_comments = discussion.get_comments_by_author("user1");
        assert_eq!(user1_comments.len(), 1);
        
        // Test getting recent comments
        let recent_comments = discussion.get_recent_comments(1);
        assert_eq!(recent_comments.len(), 1);
        assert_eq!(recent_comments[0].author, "user2");
    }

    #[test]
    fn test_enhancement_comment_reactions() {
        let mut comment = EnhancementComment::new("user1".to_string(), "Test comment".to_string());
        
        // Test adding reactions
        comment.add_reaction("like".to_string());
        comment.add_reaction("like".to_string());
        comment.add_reaction("love".to_string());
        
        assert_eq!(comment.reactions.get("like"), Some(&2));
        assert_eq!(comment.reactions.get("love"), Some(&1));
        
        // Test editing comment
        let original_timestamp = comment.edited_at;
        comment.edit("Updated comment".to_string());
        assert_eq!(comment.content, "Updated comment");
        assert!(comment.edited_at.is_some());
        assert_ne!(comment.edited_at, original_timestamp);
        
        // Test adding replies
        let reply_id = Uuid::new_v4();
        comment.add_reply(reply_id);
        assert!(comment.replies.contains(&reply_id));
    }

    #[test]
    fn test_implementation_milestones() {
        let mut milestone = ImplementationMilestone::new(
            "Test Milestone".to_string(),
            "Test milestone description".to_string(),
        );
        
        // Test setting due date
        let due_date = Utc::now() + chrono::Duration::days(7);
        milestone.set_due_date(due_date);
        assert_eq!(milestone.due_date, Some(due_date));
        
        // Test starting milestone
        milestone.start();
        assert_eq!(milestone.status, MilestoneStatus::InProgress);
        
        // Test completing milestone
        milestone.complete();
        assert_eq!(milestone.status, MilestoneStatus::Completed);
        assert!(milestone.completed_at.is_some());
    }

    #[test]
    fn test_contribution_recognition_points() {
        let enhancement_id = Uuid::new_v4();
        let mut contribution = Contribution::new(
            enhancement_id,
            "contributor123".to_string(),
            ContributionType::Code,
            "Code contribution".to_string(),
        );
        
        // Test default recognition points
        assert_eq!(contribution.recognition_points, 10);
        
        // Test setting recognition points
        contribution.set_recognition_points(25);
        assert_eq!(contribution.recognition_points, 25);
    }

    #[test]
    fn test_enhancement_category_creation() {
        let mut category = EnhancementCategory::new("ui".to_string(), "User Interface".to_string());
        
        // Test setting description
        category.set_description("UI enhancements".to_string());
        assert_eq!(category.description, Some("UI enhancements".to_string()));
        
        // Test setting color
        category.set_color("#FF0000".to_string());
        assert_eq!(category.color, Some("#FF0000".to_string()));
    }

    #[test]
    fn test_enhancement_system_categories() {
        let mut system = CommunityEnhancementSystem::new();
        
        // Test getting default categories
        let categories = system.get_categories();
        assert!(!categories.is_empty());
        assert!(categories.iter().any(|c| c.id == "ui_improvements"));
        assert!(categories.iter().any(|c| c.id == "performance"));
        assert!(categories.iter().any(|c| c.id == "features"));
        
        // Test adding new category
        let new_category = EnhancementCategory::new("security".to_string(), "Security".to_string());
        system.add_category(new_category);
        
        let categories = system.get_categories();
        assert!(categories.iter().any(|c| c.id == "security"));
    }

    #[test]
    fn test_enhancement_system_top_contributors() {
        let mut system = CommunityEnhancementSystem::new();
        let category = EnhancementCategory::new("ui".to_string(), "User Interface".to_string());
        let proposal = EnhancementProposal::new(
            "Test Enhancement".to_string(),
            "Test description".to_string(),
            "user123".to_string(),
            category,
        );
        
        let proposal_id = system.propose_enhancement(proposal);
        
        // Add contributions
        let contribution1 = Contribution::new(
            proposal_id,
            "contributor1".to_string(),
            ContributionType::Code,
            "Code contribution".to_string(),
        );
        let contribution2 = Contribution::new(
            proposal_id,
            "contributor1".to_string(),
            ContributionType::Documentation,
            "Documentation contribution".to_string(),
        );
        let contribution3 = Contribution::new(
            proposal_id,
            "contributor2".to_string(),
            ContributionType::Review,
            "Review contribution".to_string(),
        );
        
        system.add_contribution(proposal_id, contribution1).unwrap();
        system.add_contribution(proposal_id, contribution2).unwrap();
        system.add_contribution(proposal_id, contribution3).unwrap();
        
        // Add reviews
        let review1 = EnhancementReview::new(
            proposal_id,
            "contributor1".to_string(),
            ReviewRating::Good,
            "Good review".to_string(),
        );
        let review2 = EnhancementReview::new(
            proposal_id,
            "contributor3".to_string(),
            ReviewRating::Excellent,
            "Excellent review".to_string(),
        );
        
        system.add_review(proposal_id, review1).unwrap();
        system.add_review(proposal_id, review2).unwrap();
        
        // Test getting top contributors
        let top_contributors = system.get_top_contributors(5);
        assert!(!top_contributors.is_empty());
        
        // contributor1 should have 3 contributions/reviews (2 contributions + 1 review)
        let contributor1_count = top_contributors.iter().find(|(id, _)| id == "contributor1");
        assert!(contributor1_count.is_some());
        assert_eq!(contributor1_count.unwrap().1, 3);
    }

    #[test]
    fn test_enhancement_system_proposal_filtering() {
        let mut system = CommunityEnhancementSystem::new();
        
        // Create proposals with different statuses and categories
        let ui_category = EnhancementCategory::new("ui_improvements".to_string(), "UI Improvements".to_string());
        let perf_category = EnhancementCategory::new("performance".to_string(), "Performance".to_string());
        
        let mut proposal1 = EnhancementProposal::new(
            "UI Enhancement".to_string(),
            "UI enhancement description".to_string(),
            "user123".to_string(),
            ui_category.clone(),
        );
        proposal1.approve();
        
        let mut proposal2 = EnhancementProposal::new(
            "Performance Enhancement".to_string(),
            "Performance enhancement description".to_string(),
            "user123".to_string(),
            perf_category.clone(),
        );
        proposal2.implement();
        
        let proposal3 = EnhancementProposal::new(
            "Another UI Enhancement".to_string(),
            "Another UI enhancement description".to_string(),
            "user123".to_string(),
            ui_category.clone(),
        );
        
        let proposal1_id = system.propose_enhancement(proposal1);
        let proposal2_id = system.propose_enhancement(proposal2);
        let proposal3_id = system.propose_enhancement(proposal3);
        
        // Test filtering by status
        let approved_proposals = system.get_proposals_by_status(EnhancementStatus::Approved);
        assert_eq!(approved_proposals.len(), 1);
        assert_eq!(approved_proposals[0].id, proposal1_id);
        
        let implemented_proposals = system.get_proposals_by_status(EnhancementStatus::Implemented);
        assert_eq!(implemented_proposals.len(), 1);
        assert_eq!(implemented_proposals[0].id, proposal2_id);
        
        let proposed_proposals = system.get_proposals_by_status(EnhancementStatus::Proposed);
        assert_eq!(proposed_proposals.len(), 1);
        assert_eq!(proposed_proposals[0].id, proposal3_id);
        
        // Test filtering by category
        let ui_proposals = system.get_proposals_by_category("ui_improvements");
        assert_eq!(ui_proposals.len(), 2);
        assert!(ui_proposals.iter().any(|p| p.id == proposal1_id));
        assert!(ui_proposals.iter().any(|p| p.id == proposal3_id));
        
        let perf_proposals = system.get_proposals_by_category("performance");
        assert_eq!(perf_proposals.len(), 1);
        assert_eq!(perf_proposals[0].id, proposal2_id);
    }

    #[test]
    fn test_enhancement_system_cross_references() {
        let mut system = CommunityEnhancementSystem::new();
        let category = EnhancementCategory::new("ui".to_string(), "User Interface".to_string());
        let proposal = EnhancementProposal::new(
            "Test Enhancement".to_string(),
            "Test description".to_string(),
            "user123".to_string(),
            category,
        );
        
        let proposal_id = system.propose_enhancement(proposal);
        
        // Create discussion
        let discussion = EnhancementDiscussion::new(proposal_id, "Test Discussion".to_string());
        let discussion_id = system.create_discussion(discussion);
        
        // Create implementation
        let implementation = EnhancementImplementation::new(
            proposal_id,
            "Test Implementation".to_string(),
            "Test implementation description".to_string(),
            vec!["developer123".to_string()],
        );
        let implementation_id = system.start_implementation(implementation);
        
        // Test cross-references
        let discussion_ref = system.get_discussion_by_enhancement(proposal_id);
        assert!(discussion_ref.is_some());
        assert_eq!(discussion_ref.unwrap().id, discussion_id);
        
        let implementation_ref = system.get_implementation_by_enhancement(proposal_id);
        assert!(implementation_ref.is_some());
        assert_eq!(implementation_ref.unwrap().id, implementation_id);
    }
}