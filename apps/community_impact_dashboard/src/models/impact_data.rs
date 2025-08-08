//! Unified Impact Data Model
//!
//! This module defines the core data structure that combines impact data
//! from all four measurement systems.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use learning_impact_tracker::tracker::ImpactMetrics as LearningMetrics;
use volunteer_impact_tracker::tracker::ImpactMetrics as VolunteerMetrics;
use financial_impact_tracker::tracker::FinancialImpactRecord;
use cause_impact_tracker::tracker::ImpactMetrics as CauseMetrics;
use impact_viz::core::CommunityStory;

use super::{ImpactInterconnection, CommunityWellbeing};

/// Unified Impact Data Structure
/// 
/// This structure combines data from all four impact measurement systems
/// to provide a holistic view of community impact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedImpactData {
    /// Unique identifier for this data snapshot
    pub id: Uuid,
    
    /// Timestamp when this data was collected
    pub timestamp: DateTime<Utc>,
    
    /// Learning impact metrics
    pub learning_metrics: LearningMetrics,
    
    /// Volunteer impact metrics
    pub volunteer_metrics: VolunteerMetrics,
    
    /// Financial impact records
    pub financial_metrics: Vec<FinancialImpactRecord>,
    
    /// Cause impact metrics
    pub cause_metrics: CauseMetrics,
    
    /// Interconnections between impact domains
    pub interconnections: Vec<ImpactInterconnection>,
    
    /// Community wellbeing indicators
    pub community_wellbeing: CommunityWellbeing,
    
    /// Community impact stories
    pub community_stories: Vec<CommunityStory>,
    
    /// Member-specific impact data (if viewing individual view)
    pub member_data: Option<MemberImpactData>,
}

/// Member-specific Impact Data
/// 
/// This structure contains impact data specific to an individual member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberImpactData {
    /// Member ID
    pub member_id: String,
    
    /// Member's position within the community impact ecosystem
    pub ecosystem_position: EcosystemPosition,
    
    /// How member's actions contribute to the larger picture
    pub contribution_impact: ContributionImpact,
    
    /// Personalized suggestions for optimizing community impact
    pub impact_suggestions: Vec<ImpactSuggestion>,
    
    /// Impact evolution over time with milestone celebrations
    pub impact_evolution: ImpactEvolution,
}

/// Ecosystem Position
/// 
/// Represents a member's position within the community impact ecosystem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemPosition {
    /// Learning engagement level (0.0 to 1.0)
    pub learning_engagement: f64,
    
    /// Volunteer participation level (0.0 to 1.0)
    pub volunteer_participation: f64,
    
    /// Financial participation level (0.0 to 1.0)
    pub financial_participation: f64,
    
    /// Cause engagement level (0.0 to 1.0)
    pub cause_engagement: f64,
    
    /// Overall community connectivity score (0.0 to 1.0)
    pub community_connectivity: f64,
}

/// Contribution Impact
/// 
/// Shows how a member's specific actions contribute to the larger picture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributionImpact {
    /// Learning contribution to community knowledge (0.0 to 1.0)
    pub learning_contribution: f64,
    
    /// Volunteer contribution to community service (0.0 to 1.0)
    pub volunteer_contribution: f64,
    
    /// Financial contribution to community resources (0.0 to 1.0)
    pub financial_contribution: f64,
    
    /// Cause contribution to community transformation (0.0 to 1.0)
    pub cause_contribution: f64,
    
    /// Overall impact multiplier effect (0.0 to 1.0)
    pub multiplier_effect: f64,
}

/// Impact Suggestion
/// 
/// Personalized suggestion for optimizing community impact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactSuggestion {
    /// Suggestion ID
    pub id: Uuid,
    
    /// Domain this suggestion applies to
    pub domain: ImpactDomain,
    
    /// Title of the suggestion
    pub title: String,
    
    /// Detailed description of the suggestion
    pub description: String,
    
    /// Expected impact of following this suggestion (0.0 to 1.0)
    pub expected_impact: f64,
    
    /// Difficulty level of implementing this suggestion
    pub difficulty: DifficultyLevel,
    
    /// Priority of this suggestion
    pub priority: PriorityLevel,
}

/// Impact Domain
/// 
/// The four domains of community impact
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImpactDomain {
    Learning,
    Volunteer,
    Financial,
    Cause,
}

/// Difficulty Level
/// 
/// Difficulty level of implementing an impact suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DifficultyLevel {
    Easy,
    Medium,
    Hard,
}

/// Priority Level
/// 
/// Priority level of an impact suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PriorityLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Impact Evolution
/// 
/// Shows impact evolution over time with milestone celebrations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactEvolution {
    /// Timeline of impact milestones
    pub milestones: Vec<ImpactMilestone>,
    
    /// Current impact level in each domain
    pub current_levels: DomainLevels,
    
    /// Historical progress data
    pub historical_progress: Vec<HistoricalProgressPoint>,
}

/// Impact Milestone
/// 
/// A significant achievement in community impact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactMilestone {
    /// Milestone ID
    pub id: Uuid,
    
    /// Timestamp when milestone was achieved
    pub timestamp: DateTime<Utc>,
    
    /// Domain this milestone relates to
    pub domain: ImpactDomain,
    
    /// Title of the milestone
    pub title: String,
    
    /// Description of the milestone
    pub description: String,
    
    /// Celebration message
    pub celebration_message: String,
}

/// Domain Levels
/// 
/// Current impact level in each domain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainLevels {
    /// Learning impact level (0.0 to 1.0)
    pub learning: f64,
    
    /// Volunteer impact level (0.0 to 1.0)
    pub volunteer: f64,
    
    /// Financial impact level (0.0 to 1.0)
    pub financial: f64,
    
    /// Cause impact level (0.0 to 1.0)
    pub cause: f64,
}

/// Historical Progress Point
/// 
/// A point in time showing historical progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalProgressPoint {
    /// Timestamp of this progress point
    pub timestamp: DateTime<Utc>,
    
    /// Learning progress at this point (0.0 to 1.0)
    pub learning_progress: f64,
    
    /// Volunteer progress at this point (0.0 to 1.0)
    pub volunteer_progress: f64,
    
    /// Financial progress at this point (0.0 to 1.0)
    pub financial_progress: f64,
    
    /// Cause progress at this point (0.0 to 1.0)
    pub cause_progress: f64,
    
    /// Overall community progress at this point (0.0 to 1.0)
    pub community_progress: f64,
}

impl UnifiedImpactData {
    /// Create a new UnifiedImpactData instance
    pub fn new(
        learning_metrics: LearningMetrics,
        volunteer_metrics: VolunteerMetrics,
        financial_metrics: Vec<FinancialImpactRecord>,
        cause_metrics: CauseMetrics,
        interconnections: Vec<ImpactInterconnection>,
        community_wellbeing: CommunityWellbeing,
        community_stories: Vec<CommunityStory>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            learning_metrics,
            volunteer_metrics,
            financial_metrics,
            cause_metrics,
            interconnections,
            community_wellbeing,
            community_stories,
            member_data: None,
        }
    }
    
    /// Set member-specific data
    pub fn with_member_data(mut self, member_data: MemberImpactData) -> Self {
        self.member_data = Some(member_data);
        self
    }
}