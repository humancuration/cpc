//! Feature voting and prioritization for the Unified Community Impact Dashboard
//!
//! This module provides tools for community-driven feature prioritization,
//! including voting systems, impact assessment, and roadmap planning.

use tracing::info;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

/// Feature voting and prioritization system
pub struct FeatureVotingSystem {
    features: HashMap<Uuid, Feature>,
    votes: HashMap<Uuid, Vec<Vote>>, // Feature ID to votes
    voting_periods: HashMap<Uuid, VotingPeriod>, // Feature ID to voting period
    voting_history: Vec<VotingRecord>,
    impact_assessments: HashMap<Uuid, ImpactAssessment>, // Feature ID to assessment
    roadmap: Vec<RoadmapItem>,
    facilitators: Vec<String>, // Role IDs of voting facilitators
}

impl FeatureVotingSystem {
    /// Create a new feature voting system
    pub fn new() -> Self {
        Self {
            features: HashMap::new(),
            votes: HashMap::new(),
            voting_periods: HashMap::new(),
            voting_history: Vec::new(),
            impact_assessments: HashMap::new(),
            roadmap: Vec::new(),
            facilitators: Vec::new(),
        }
    }

    /// Add a facilitator to the system
    pub fn add_facilitator(&mut self, role_id: String) {
        self.facilitators.push(role_id);
        info!("Added facilitator: {}", &self.facilitators.last().unwrap());
    }

    /// Propose a new feature
    pub fn propose_feature(&mut self, feature: Feature) -> Uuid {
        let feature_id = feature.id;
        self.features.insert(feature_id, feature);
        self.votes.insert(feature_id, Vec::new());
        info!("Proposed feature: {}", feature_id);
        feature_id
    }

    /// Get a feature by ID
    pub fn get_feature(&self, feature_id: Uuid) -> Option<&Feature> {
        self.features.get(&feature_id)
    }

    /// Update feature status
    pub fn update_feature_status(&mut self, feature_id: Uuid, status: FeatureStatus) -> Result<(), VotingError> {
        let feature = self.features.get_mut(&feature_id)
            .ok_or(VotingError::FeatureNotFound(feature_id))?;
        
        feature.status = status;
        feature.updated_at = Utc::now();
        
        info!("Updated feature {} status to {:?}", feature_id, status);
        Ok(())
    }

    /// Start voting period for a feature
    pub fn start_voting_period(&mut self, feature_id: Uuid, period: VotingPeriod) -> Result<(), VotingError> {
        if !self.features.contains_key(&feature_id) {
            return Err(VotingError::FeatureNotFound(feature_id));
        }
        
        self.voting_periods.insert(feature_id, period);
        info!("Started voting period for feature: {}", feature_id);
        Ok(())
    }

    /// Cast a vote for a feature
    pub fn cast_vote(&mut self, feature_id: Uuid, vote: Vote) -> Result<(), VotingError> {
        // Check if feature exists
        if !self.features.contains_key(&feature_id) {
            return Err(VotingError::FeatureNotFound(feature_id));
        }
        
        // Check if voting period is active
        if let Some(period) = self.voting_periods.get(&feature_id) {
            let now = Utc::now();
            if now < period.start_time || now > period.end_time {
                return Err(VotingError::VotingPeriodInactive);
            }
        } else {
            return Err(VotingError::VotingPeriodNotStarted);
        }
        
        // Check if user has already voted
        if let Some(votes) = self.votes.get(&feature_id) {
            if votes.iter().any(|v| v.voter_id == vote.voter_id) {
                return Err(VotingError::DuplicateVote);
            }
        }
        
        // Record the vote
        self.votes.get_mut(&feature_id)
            .ok_or(VotingError::FeatureNotFound(feature_id))?
            .push(vote);
        
        info!("Recorded vote for feature: {}", feature_id);
        Ok(())
    }

    /// Get votes for a feature
    pub fn get_votes(&self, feature_id: Uuid) -> Result<&Vec<Vote>, VotingError> {
        self.votes.get(&feature_id)
            .ok_or(VotingError::FeatureNotFound(feature_id))
    }

    /// Calculate voting results for a feature
    pub fn calculate_results(&self, feature_id: Uuid) -> Result<VotingResults, VotingError> {
        let votes = self.get_votes(feature_id)?;
        
        let total_votes = votes.len();
        let mut support_votes = 0;
        let mut oppose_votes = 0;
        let mut neutral_votes = 0;
        
        let mut weighted_support = 0.0;
        let mut weighted_oppose = 0.0;
        let mut weighted_neutral = 0.0;
        
        for vote in votes {
            match vote.vote_type {
                VoteType::Support => {
                    support_votes += 1;
                    weighted_support += vote.weight;
                },
                VoteType::Oppose => {
                    oppose_votes += 1;
                    weighted_oppose += vote.weight;
                },
                VoteType::Neutral => {
                    neutral_votes += 1;
                    weighted_neutral += vote.weight;
                },
            }
        }
        
        let support_percentage = if total_votes > 0 {
            (support_votes as f64 / total_votes as f64) * 100.0
        } else {
            0.0
        };
        
        let oppose_percentage = if total_votes > 0 {
            (oppose_votes as f64 / total_votes as f64) * 100.0
        } else {
            0.0
        };
        
        let neutral_percentage = if total_votes > 0 {
            (neutral_votes as f64 / total_votes as f64) * 100.0
        } else {
            0.0
        };
        
        let total_weighted = weighted_support + weighted_oppose + weighted_neutral;
        let weighted_support_percentage = if total_weighted > 0.0 {
            (weighted_support / total_weighted) * 100.0
        } else {
            0.0
        };
        
        let weighted_oppose_percentage = if total_weighted > 0.0 {
            (weighted_oppose / total_weighted) * 100.0
        } else {
            0.0
        };
        
        let weighted_neutral_percentage = if total_weighted > 0.0 {
            (weighted_neutral / total_weighted) * 100.0
        } else {
            0.0
        };
        
        Ok(VotingResults {
            feature_id,
            total_votes,
            support_votes,
            oppose_votes,
            neutral_votes,
            support_percentage,
            oppose_percentage,
            neutral_percentage,
            weighted_support,
            weighted_oppose,
            weighted_neutral,
            weighted_support_percentage,
            weighted_oppose_percentage,
            weighted_neutral_percentage,
            calculated_at: Utc::now(),
        })
    }

    /// Finalize voting for a feature
    pub fn finalize_voting(&mut self, feature_id: Uuid) -> Result<VotingOutcome, VotingError> {
        let results = self.calculate_results(feature_id)?;
        let votes = self.get_votes(feature_id)?.clone();
        
        // Determine outcome based on results
        let outcome = if results.support_percentage > 50.0 {
            VotingOutcome::Approved
        } else if results.oppose_percentage > 50.0 {
            VotingOutcome::Rejected
        } else {
            VotingOutcome::Inconclusive
        };
        
        // Record the voting
        let record = VotingRecord::new(
            format!("Voting finalized for feature: {}", feature_id),
            feature_id,
            results.clone(),
            outcome,
        );
        self.voting_history.push(record);
        
        // Update feature status based on outcome
        let new_status = match outcome {
            VotingOutcome::Approved => FeatureStatus::Approved,
            VotingOutcome::Rejected => FeatureStatus::Rejected,
            VotingOutcome::Inconclusive => FeatureStatus::NeedsReview,
        };
        
        self.update_feature_status(feature_id, new_status)?;
        
        info!("Finalized voting for feature: {} with outcome {:?}", feature_id, outcome);
        
        Ok(outcome)
    }

    /// Conduct impact assessment for a feature
    pub fn conduct_impact_assessment(&mut self, feature_id: Uuid, assessment: ImpactAssessment) -> Result<(), VotingError> {
        if !self.features.contains_key(&feature_id) {
            return Err(VotingError::FeatureNotFound(feature_id));
        }
        
        self.impact_assessments.insert(feature_id, assessment);
        info!("Conducted impact assessment for feature: {}", feature_id);
        Ok(())
    }

    /// Get impact assessment for a feature
    pub fn get_impact_assessment(&self, feature_id: Uuid) -> Option<&ImpactAssessment> {
        self.impact_assessments.get(&feature_id)
    }

    /// Add feature to roadmap
    pub fn add_to_roadmap(&mut self, item: RoadmapItem) -> Uuid {
        let item_id = item.id;
        self.roadmap.push(item);
        info!("Added item to roadmap: {}", item_id);
        item_id
    }

    /// Get roadmap items by status
    pub fn get_roadmap_by_status(&self, status: RoadmapStatus) -> Vec<&RoadmapItem> {
        self.roadmap.iter()
            .filter(|item| item.status == status)
            .collect()
    }

    /// Update roadmap item status
    pub fn update_roadmap_status(&mut self, item_id: Uuid, status: RoadmapStatus) -> Result<(), VotingError> {
        let item = self.roadmap.iter_mut()
            .find(|item| item.id == item_id)
            .ok_or(VotingError::RoadmapItemNotFound(item_id))?;
        
        item.status = status;
        item.updated_at = Utc::now();
        
        info!("Updated roadmap item {} status to {:?}", item_id, status);
        Ok(())
    }

    /// Get voting statistics
    pub fn get_statistics(&self) -> VotingStatistics {
        let total_features = self.features.len();
        let total_votes: usize = self.votes.values().map(|v| v.len()).sum();
        let total_roadmap_items = self.roadmap.len();
        let active_voting_periods = self.voting_periods.values()
            .filter(|period| {
                let now = Utc::now();
                now >= period.start_time && now <= period.end_time
            })
            .count();
        
        // Count features by status
        let proposed_features = self.features.values()
            .filter(|f| f.status == FeatureStatus::Proposed)
            .count();
        
        let approved_features = self.features.values()
            .filter(|f| f.status == FeatureStatus::Approved)
            .count();
        
        let implemented_features = self.features.values()
            .filter(|f| f.status == FeatureStatus::Implemented)
            .count();
        
        VotingStatistics {
            total_features,
            total_votes,
            total_roadmap_items,
            active_voting_periods,
            proposed_features,
            approved_features,
            implemented_features,
        }
    }

    /// Generate voting report
    pub fn generate_voting_report(&self) -> VotingReport {
        let stats = self.get_statistics();
        let recent_votes = self.voting_history.iter()
            .filter(|record| {
                let cutoff = Utc::now() - chrono::Duration::days(30);
                record.timestamp > cutoff
            })
            .count();
        
        VotingReport {
            generated_at: Utc::now(),
            statistics: stats,
            recent_votes,
        }
    }

    /// Search features by keyword
    pub fn search_features(&self, keyword: &str) -> Vec<&Feature> {
        self.features.values()
            .filter(|feature| {
                feature.title.to_lowercase().contains(&keyword.to_lowercase()) ||
                feature.description.to_lowercase().contains(&keyword.to_lowercase())
            })
            .collect()
    }

    /// Get features by status
    pub fn get_features_by_status(&self, status: FeatureStatus) -> Vec<&Feature> {
        self.features.values()
            .filter(|feature| feature.status == status)
            .collect()
    }

    /// Get active voting periods
    pub fn get_active_voting_periods(&self) -> Vec<(&Uuid, &VotingPeriod)> {
        let now = Utc::now();
        self.voting_periods.iter()
            .filter(|(_, period)| now >= period.start_time && now <= period.end_time)
            .collect()
    }

    /// Get voting history for a feature
    pub fn get_voting_history(&self, feature_id: Uuid) -> Vec<&VotingRecord> {
        self.voting_history.iter()
            .filter(|record| record.feature_id == feature_id)
            .collect()
    }

    /// Get top voted features
    pub fn get_top_voted_features(&self, count: usize) -> Vec<(Uuid, usize)> {
        let mut vote_counts: Vec<(Uuid, usize)> = self.votes.iter()
            .map(|(feature_id, votes)| (*feature_id, votes.len()))
            .collect();
        
        vote_counts.sort_by(|a, b| b.1.cmp(&a.1));
        vote_counts.truncate(count);
        
        vote_counts
    }

    /// Get roadmap sorted by priority
    pub fn get_prioritized_roadmap(&self) -> Vec<&RoadmapItem> {
        let mut roadmap_items: Vec<&RoadmapItem> = self.roadmap.iter().collect();
        roadmap_items.sort_by(|a, b| a.priority.cmp(&b.priority));
        roadmap_items
    }
}

/// Feature proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub author: String, // User ID
    pub category: FeatureCategory,
    pub status: FeatureStatus,
    pub priority: Option<PriorityLevel>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub estimated_effort: Option<u32>, // In hours
    pub dependencies: Vec<Uuid>, // Feature IDs this feature depends on
    pub tags: Vec<String>,
}

impl Feature {
    /// Create a new feature proposal
    pub fn new(
        title: String,
        description: String,
        author: String,
        category: FeatureCategory,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            author,
            category,
            status: FeatureStatus::Proposed,
            priority: None,
            created_at: now,
            updated_at: now,
            estimated_effort: None,
            dependencies: Vec::new(),
            tags: Vec::new(),
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

    /// Approve the feature
    pub fn approve(&mut self) {
        self.status = FeatureStatus::Approved;
        self.updated_at = Utc::now();
    }

    /// Reject the feature
    pub fn reject(&mut self) {
        self.status = FeatureStatus::Rejected;
        self.updated_at = Utc::now();
    }

    /// Mark as implemented
    pub fn implement(&mut self) {
        self.status = FeatureStatus::Implemented;
        self.updated_at = Utc::now();
    }

    /// Mark as in development
    pub fn start_development(&mut self) {
        self.status = FeatureStatus::InProgress;
        self.updated_at = Utc::now();
    }
}

/// Voting period for a feature
#[derive(Debug, Clone)]
pub struct VotingPeriod {
    pub id: Uuid,
    pub feature_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub voting_method: VotingMethod,
    pub quorum_required: Option<usize>,
    pub created_at: DateTime<Utc>,
}

impl VotingPeriod {
    /// Create a new voting period
    pub fn new(
        feature_id: Uuid,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        voting_method: VotingMethod,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            feature_id,
            start_time,
            end_time,
            voting_method,
            quorum_required: None,
            created_at: Utc::now(),
        }
    }

    /// Set quorum requirement
    pub fn set_quorum(&mut self, quorum: usize) {
        self.quorum_required = Some(quorum);
    }

    /// Check if voting period is active
    pub fn is_active(&self) -> bool {
        let now = Utc::now();
        now >= self.start_time && now <= self.end_time
    }

    /// Check if voting period has ended
    pub fn has_ended(&self) -> bool {
        Utc::now() > self.end_time
    }

    /// Get duration of voting period in hours
    pub fn duration_hours(&self) -> i64 {
        (self.end_time - self.start_time).num_hours()
    }
}

/// Individual vote
#[derive(Debug, Clone)]
pub struct Vote {
    pub id: Uuid,
    pub voter_id: String, // User ID
    pub feature_id: Uuid,
    pub vote_type: VoteType,
    pub weight: f64, // Voting weight based on community role/engagement
    pub comment: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl Vote {
    /// Create a new vote
    pub fn new(
        voter_id: String,
        feature_id: Uuid,
        vote_type: VoteType,
        weight: f64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            voter_id,
            feature_id,
            vote_type,
            weight,
            comment: None,
            timestamp: Utc::now(),
        }
    }

    /// Add comment to vote
    pub fn add_comment(&mut self, comment: String) {
        self.comment = Some(comment);
    }
}

/// Voting results
#[derive(Debug, Clone)]
pub struct VotingResults {
    pub feature_id: Uuid,
    pub total_votes: usize,
    pub support_votes: usize,
    pub oppose_votes: usize,
    pub neutral_votes: usize,
    pub support_percentage: f64,
    pub oppose_percentage: f64,
    pub neutral_percentage: f64,
    pub weighted_support: f64,
    pub weighted_oppose: f64,
    pub weighted_neutral: f64,
    pub weighted_support_percentage: f64,
    pub weighted_oppose_percentage: f64,
    pub weighted_neutral_percentage: f64,
    pub calculated_at: DateTime<Utc>,
}

/// Voting record for historical tracking
#[derive(Debug, Clone)]
pub struct VotingRecord {
    pub id: Uuid,
    pub description: String,
    pub feature_id: Uuid,
    pub results: VotingResults,
    pub outcome: VotingOutcome,
    pub timestamp: DateTime<Utc>,
}

impl VotingRecord {
    /// Create a new voting record
    pub fn new(
        description: String,
        feature_id: Uuid,
        results: VotingResults,
        outcome: VotingOutcome,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            description,
            feature_id,
            results,
            outcome,
            timestamp: Utc::now(),
        }
    }
}

/// Impact assessment for a feature
#[derive(Debug, Clone)]
pub struct ImpactAssessment {
    pub id: Uuid,
    pub feature_id: Uuid,
    pub community_impact: ImpactLevel,
    pub technical_impact: ImpactLevel,
    pub resource_impact: ImpactLevel,
    pub timeline_impact: ImpactLevel,
    pub risk_level: RiskLevel,
    pub dependencies_impact: Vec<ImpactDependency>,
    pub assessment_by: String, // User ID
    pub notes: String,
    pub created_at: DateTime<Utc>,
}

impl ImpactAssessment {
    /// Create a new impact assessment
    pub fn new(
        feature_id: Uuid,
        community_impact: ImpactLevel,
        technical_impact: ImpactLevel,
        resource_impact: ImpactLevel,
        timeline_impact: ImpactLevel,
        risk_level: RiskLevel,
        assessment_by: String,
        notes: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            feature_id,
            community_impact,
            technical_impact,
            resource_impact,
            timeline_impact,
            risk_level,
            dependencies_impact: Vec::new(),
            assessment_by,
            notes,
            created_at: Utc::now(),
        }
    }

    /// Add dependency impacts
    pub fn add_dependency_impacts(&mut self, impacts: Vec<ImpactDependency>) {
        self.dependencies_impact.extend(impacts);
    }

    /// Get overall impact score
    pub fn overall_impact_score(&self) -> f64 {
        let community_score = self.community_impact as i32;
        let technical_score = self.technical_impact as i32;
        let resource_score = self.resource_impact as i32;
        let timeline_score = self.timeline_impact as i32;
        
        (community_score + technical_score + resource_score + timeline_score) as f64 / 4.0
    }
}

/// Dependency impact assessment
#[derive(Debug, Clone)]
pub struct ImpactDependency {
    pub dependency_feature_id: Uuid,
    pub impact_type: DependencyImpactType,
    pub impact_description: String,
}

impl ImpactDependency {
    /// Create a new impact dependency
    pub fn new(
        dependency_feature_id: Uuid,
        impact_type: DependencyImpactType,
        impact_description: String,
    ) -> Self {
        Self {
            dependency_feature_id,
            impact_type,
            impact_description,
        }
    }
}

/// Roadmap item
#[derive(Debug, Clone)]
pub struct RoadmapItem {
    pub id: Uuid,
    pub feature_id: Uuid,
    pub title: String,
    pub description: String,
    pub priority: PriorityLevel,
    pub status: RoadmapStatus,
    pub estimated_timeline: Option<DateTime<Utc>>,
    pub assigned_team: Option<String>, // Team ID
    pub dependencies: Vec<Uuid>, // Feature IDs
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub progress: f64, // 0.0 to 1.0
}

impl RoadmapItem {
    /// Create a new roadmap item
    pub fn new(
        feature_id: Uuid,
        title: String,
        description: String,
        priority: PriorityLevel,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            feature_id,
            title,
            description,
            priority,
            status: RoadmapStatus::Planned,
            estimated_timeline: None,
            assigned_team: None,
            dependencies: Vec::new(),
            created_at: now,
            updated_at: now,
            progress: 0.0,
        }
    }

    /// Set estimated timeline
    pub fn set_timeline(&mut self, timeline: DateTime<Utc>) {
        self.estimated_timeline = Some(timeline);
        self.updated_at = Utc::now();
    }

    /// Assign to team
    pub fn assign_team(&mut self, team_id: String) {
        self.assigned_team = Some(team_id);
        self.updated_at = Utc::now();
    }

    /// Add dependencies
    pub fn add_dependencies(&mut self, dependencies: Vec<Uuid>) {
        self.dependencies.extend(dependencies);
        self.updated_at = Utc::now();
    }

    /// Update progress
    pub fn update_progress(&mut self, progress: f64) -> Result<(), VotingError> {
        if progress < 0.0 || progress > 1.0 {
            return Err(VotingError::InvalidProgress);
        }
        
        self.progress = progress;
        self.updated_at = Utc::now();
        
        // Update status based on progress
        self.status = if progress == 0.0 {
            RoadmapStatus::Planned
        } else if progress == 1.0 {
            RoadmapStatus::Completed
        } else {
            RoadmapStatus::InProgress
        };
        
        Ok(())
    }

    /// Mark as completed
    pub fn complete(&mut self) {
        self.progress = 1.0;
        self.status = RoadmapStatus::Completed;
        self.updated_at = Utc::now();
    }

    /// Mark as blocked
    pub fn block(&mut self, reason: String) {
        self.status = RoadmapStatus::Blocked { reason };
        self.updated_at = Utc::now();
    }
}

/// Statistics about voting activities
#[derive(Debug, Clone)]
pub struct VotingStatistics {
    pub total_features: usize,
    pub total_votes: usize,
    pub total_roadmap_items: usize,
    pub active_voting_periods: usize,
    pub proposed_features: usize,
    pub approved_features: usize,
    pub implemented_features: usize,
}

/// Voting report
#[derive(Debug, Clone)]
pub struct VotingReport {
    pub generated_at: DateTime<Utc>,
    pub statistics: VotingStatistics,
    pub recent_votes: usize,
}

/// Types of features
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FeatureCategory {
    UserInterface,
    DataVisualization,
    CommunityFeatures,
    Performance,
    Security,
    Accessibility,
    Integration,
    Reporting,
    Analytics,
    Customization,
}

/// Status of features
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FeatureStatus {
    Proposed,
    Approved,
    Rejected,
    InProgress,
    Implemented,
    NeedsReview,
    Deferred,
}

/// Priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum PriorityLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Types of votes
#[derive(Debug, Clone, PartialEq)]
pub enum VoteType {
    Support,
    Oppose,
    Neutral,
}

/// Voting methods
#[derive(Debug, Clone, PartialEq)]
pub enum VotingMethod {
    SimpleMajority,
    WeightedMajority,
    RankedChoice,
    Quadratic,
}

/// Voting outcomes
#[derive(Debug, Clone, PartialEq)]
pub enum VotingOutcome {
    Approved,
    Rejected,
    Inconclusive,
}

/// Impact levels
#[derive(Debug, Clone, PartialEq)]
pub enum ImpactLevel {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

/// Risk levels
#[derive(Debug, Clone, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Types of dependency impacts
#[derive(Debug, Clone, PartialEq)]
pub enum DependencyImpactType {
    Blocking,
    Delaying,
    Enhancing,
    Neutral,
}

/// Status of roadmap items
#[derive(Debug, Clone, PartialEq)]
pub enum RoadmapStatus {
    Planned,
    InProgress,
    Completed,
    Blocked { reason: String },
    Deferred,
}

/// Error types for voting system
#[derive(Debug)]
pub enum VotingError {
    FeatureNotFound(Uuid),
    RoadmapItemNotFound(Uuid),
    VotingPeriodNotStarted,
    VotingPeriodInactive,
    DuplicateVote,
    InvalidProgress,
    AssessmentError(String),
    UpdateError(String),
}

impl std::fmt::Display for VotingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VotingError::FeatureNotFound(id) => write!(f, "Feature not found: {}", id),
            VotingError::RoadmapItemNotFound(id) => write!(f, "Roadmap item not found: {}", id),
            VotingError::VotingPeriodNotStarted => write!(f, "Voting period not started"),
            VotingError::VotingPeriodInactive => write!(f, "Voting period is not active"),
            VotingError::DuplicateVote => write!(f, "User has already voted"),
            VotingError::InvalidProgress => write!(f, "Invalid progress value"),
            VotingError::AssessmentError(msg) => write!(f, "Assessment error: {}", msg),
            VotingError::UpdateError(msg) => write!(f, "Update error: {}", msg),
        }
    }
}

impl std::error::Error for VotingError {}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_feature_voting_system_initialization() {
        let system = FeatureVotingSystem::new();
        assert!(system.features.is_empty());
        assert!(system.votes.is_empty());
        assert!(system.voting_periods.is_empty());
    }

    #[test]
    fn test_propose_feature() {
        let mut system = FeatureVotingSystem::new();
        let feature = Feature::new(
            "New Dashboard View".to_string(),
            "Add a new view for dashboard metrics".to_string(),
            "user123".to_string(),
            FeatureCategory::DataVisualization,
        );
        
        let feature_id = system.propose_feature(feature);
        assert!(!feature_id.is_nil());
        assert_eq!(system.features.len(), 1);
        assert!(system.votes.contains_key(&feature_id));
    }

    #[test]
    fn test_start_voting_period() {
        let mut system = FeatureVotingSystem::new();
        let feature = Feature::new(
            "New Dashboard View".to_string(),
            "Add a new view for dashboard metrics".to_string(),
            "user123".to_string(),
            FeatureCategory::DataVisualization,
        );
        
        let feature_id = system.propose_feature(feature);
        let start_time = Utc::now();
        let end_time = start_time + Duration::days(7);
        let period = VotingPeriod::new(
            feature_id,
            start_time,
            end_time,
            VotingMethod::SimpleMajority,
        );
        
        let result = system.start_voting_period(feature_id, period);
        assert!(result.is_ok());
        assert!(system.voting_periods.contains_key(&feature_id));
    }

    #[test]
    fn test_cast_vote() {
        let mut system = FeatureVotingSystem::new();
        let feature = Feature::new(
            "New Dashboard View".to_string(),
            "Add a new view for dashboard metrics".to_string(),
            "user123".to_string(),
            FeatureCategory::DataVisualization,
        );
        
        let feature_id = system.propose_feature(feature);
        let start_time = Utc::now() - Duration::hours(1);
        let end_time = Utc::now() + Duration::days(7);
        let period = VotingPeriod::new(
            feature_id,
            start_time,
            end_time,
            VotingMethod::SimpleMajority,
        );
        
        system.start_voting_period(feature_id, period).unwrap();
        
        let vote = Vote::new(
            "user456".to_string(),
            feature_id,
            VoteType::Support,
            1.0,
        );
        
        let result = system.cast_vote(feature_id, vote);
        assert!(result.is_ok());
        assert_eq!(system.votes.get(&feature_id).unwrap().len(), 1);
    }

    #[test]
    fn test_calculate_results() {
        let mut system = FeatureVotingSystem::new();
        let feature = Feature::new(
            "New Dashboard View".to_string(),
            "Add a new view for dashboard metrics".to_string(),
            "user123".to_string(),
            FeatureCategory::DataVisualization,
        );
        
        let feature_id = system.propose_feature(feature);
        let start_time = Utc::now() - Duration::hours(1);
        let end_time = Utc::now() + Duration::days(7);
        let period = VotingPeriod::new(
            feature_id,
            start_time,
            end_time,
            VotingMethod::SimpleMajority,
        );
        
        system.start_voting_period(feature_id, period).unwrap();
        
        // Cast some votes
        let vote1 = Vote::new("user1".to_string(), feature_id, VoteType::Support, 1.0);
        let vote2 = Vote::new("user2".to_string(), feature_id, VoteType::Support, 1.0);
        let vote3 = Vote::new("user3".to_string(), feature_id, VoteType::Oppose, 1.0);
        
        system.cast_vote(feature_id, vote1).unwrap();
        system.cast_vote(feature_id, vote2).unwrap();
        system.cast_vote(feature_id, vote3).unwrap();
        
        let results = system.calculate_results(feature_id).unwrap();
        assert_eq!(results.total_votes, 3);
        assert_eq!(results.support_votes, 2);
        assert_eq!(results.oppose_votes, 1);
        assert_eq!(results.support_percentage, 66.66666666666666);
        assert_eq!(results.oppose_percentage, 33.33333333333333);
    }

    #[test]
    fn test_finalize_voting() {
        let mut system = FeatureVotingSystem::new();
        let feature = Feature::new(
            "New Dashboard View".to_string(),
            "Add a new view for dashboard metrics".to_string(),
            "user123".to_string(),
            FeatureCategory::DataVisualization,
        );
        
        let feature_id = system.propose_feature(feature);
        let start_time = Utc::now() - Duration::hours(1);
        let end_time = Utc::now() + Duration::days(7);
        let period = VotingPeriod::new(
            feature_id,
            start_time,
            end_time,
            VotingMethod::SimpleMajority,
        );
        
        system.start_voting_period(feature_id, period).unwrap();
        
        // Cast supporting votes
        let vote1 = Vote::new("user1".to_string(), feature_id, VoteType::Support, 1.0);
        let vote2 = Vote::new("user2".to_string(), feature_id, VoteType::Support, 1.0);
        let vote3 = Vote::new("user3".to_string(), feature_id, VoteType::Support, 1.0);
        
        system.cast_vote(feature_id, vote1).unwrap();
        system.cast_vote(feature_id, vote2).unwrap();
        system.cast_vote(feature_id, vote3).unwrap();
        
        let outcome = system.finalize_voting(feature_id).unwrap();
        assert_eq!(outcome, VotingOutcome::Approved);
        
        let feature = system.get_feature(feature_id).unwrap();
        assert_eq!(feature.status, FeatureStatus::Approved);
    }

    #[test]
    fn test_conduct_impact_assessment() {
        let mut system = FeatureVotingSystem::new();
        let feature = Feature::new(
            "New Dashboard View".to_string(),
            "Add a new view for dashboard metrics".to_string(),
            "user123".to_string(),
            FeatureCategory::DataVisualization,
        );
        
        let feature_id = system.propose_feature(feature);
        let assessment = ImpactAssessment::new(
            feature_id,
            ImpactLevel::High,
            ImpactLevel::Medium,
            ImpactLevel::Low,
            ImpactLevel::Medium,
            RiskLevel::Medium,
            "assessor123".to_string(),
            "This feature will significantly improve user experience".to_string(),
        );
        
        let result = system.conduct_impact_assessment(feature_id, assessment);
        assert!(result.is_ok());
        assert!(system.impact_assessments.contains_key(&feature_id));
    }

    #[test]
    fn test_add_to_roadmap() {
        let mut system = FeatureVotingSystem::new();
        let feature = Feature::new(
            "New Dashboard View".to_string(),
            "Add a new view for dashboard metrics".to_string(),
            "user123".to_string(),
            FeatureCategory::DataVisualization,
        );
        
        let feature_id = system.propose_feature(feature);
        let roadmap_item = RoadmapItem::new(
            feature_id,
            "New Dashboard View".to_string(),
            "Add a new view for dashboard metrics".to_string(),
            PriorityLevel::High,
        );
        
        let item_id = system.add_to_roadmap(roadmap_item);
        assert!(!item_id.is_nil());
        assert_eq!(system.roadmap.len(), 1);
    }

    #[test]
    fn test_get_statistics() {
        let mut system = FeatureVotingSystem::new();
        
        // Add some data
        let feature = Feature::new(
            "New Feature".to_string(),
            "Implement a new feature".to_string(),
            "user123".to_string(),
            FeatureCategory::UserInterface,
        );
        system.propose_feature(feature);
        
        let stats = system.get_statistics();
        assert_eq!(stats.total_features, 1);
        assert_eq!(stats.total_votes, 0);
        assert_eq!(stats.total_roadmap_items, 0);
    }

    #[test]
    fn test_search_features() {
        let mut system = FeatureVotingSystem::new();
        let feature = Feature::new(
            "New Dashboard View".to_string(),
            "Add a new view for dashboard metrics and visualization".to_string(),
            "user123".to_string(),
            FeatureCategory::DataVisualization,
        );
        system.propose_feature(feature);
        
        let results = system.search_features("dashboard");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_update_roadmap_status() {
        let mut system = FeatureVotingSystem::new();
        let feature = Feature::new(
            "New Dashboard View".to_string(),
            "Add a new view for dashboard metrics".to_string(),
            "user123".to_string(),
            FeatureCategory::DataVisualization,
        );
        
        let feature_id = system.propose_feature(feature);
        let roadmap_item = RoadmapItem::new(
            feature_id,
            "New Dashboard View".to_string(),
            "Add a new view for dashboard metrics".to_string(),
            PriorityLevel::High,
        );
        
        let item_id = system.add_to_roadmap(roadmap_item);
        let result = system.update_roadmap_status(item_id, RoadmapStatus::InProgress);
        assert!(result.is_ok());
        
        let item = system.roadmap.iter().find(|i| i.id == item_id).unwrap();
        assert_eq!(item.status, RoadmapStatus::InProgress);
    }

    #[test]
    fn test_roadmap_item_update_progress() {
        let mut system = FeatureVotingSystem::new();
        let feature = Feature::new(
            "New Dashboard View".to_string(),
            "Add a new view for dashboard metrics".to_string(),
            "user123".to_string(),
            FeatureCategory::DataVisualization,
        );
        
        let feature_id = system.propose_feature(feature);
        let mut roadmap_item = RoadmapItem::new(
            feature_id,
            "New Dashboard View".to_string(),
            "Add a new view for dashboard metrics".to_string(),
            PriorityLevel::High,
        );
        
        // Test valid progress update
        let result = roadmap_item.update_progress(0.5);
        assert!(result.is_ok());
        assert_eq!(roadmap_item.progress, 0.5);
        assert_eq!(roadmap_item.status, RoadmapStatus::InProgress);
        
        // Test completion
        roadmap_item.update_progress(1.0).unwrap();
        assert_eq!(roadmap_item.progress, 1.0);
        assert_eq!(roadmap_item.status, RoadmapStatus::Completed);
        
        // Test invalid progress
        let result = roadmap_item.update_progress(1.5);
        assert!(result.is_err());
    }
}