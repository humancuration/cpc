//! Transformation Milestone Recognition System
//!
//! This module provides tools for recognizing and celebrating transformation milestones
//! as communities deepen their understanding of interconnected impact.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::models::community_validation::CommunityInsight;

/// Transformation milestone recognition system
pub struct TransformationMilestoneRecognition {
    /// Stored milestones
    milestones: HashMap<Uuid, TransformationMilestone>,
    
    /// Milestone templates
    milestone_templates: Vec<MilestoneTemplate>,
    
    /// Recognition criteria
    recognition_criteria: Vec<RecognitionCriterion>,
    
    /// Celebration patterns
    celebration_patterns: Vec<CelebrationPattern>,
}

impl TransformationMilestoneRecognition {
    /// Create a new transformation milestone recognition system
    pub fn new() -> Self {
        Self {
            milestones: HashMap::new(),
            milestone_templates: Self::create_default_templates(),
            recognition_criteria: Self::create_default_criteria(),
            celebration_patterns: Vec::new(),
        }
    }

    /// Initialize the milestone recognition system
    pub fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Initialize with default templates and criteria
        Ok(())
    }

    /// Create a new transformation milestone
    pub fn create_milestone(
        &mut self,
        community_id: String,
        title: String,
        milestone_type: MilestoneType,
        description: String,
    ) -> Uuid {
        let milestone = TransformationMilestone {
            id: Uuid::new_v4(),
            community_id,
            title,
            milestone_type,
            description,
            achieved_at: Utc::now(),
            significance_level: SignificanceLevel::Medium,
            contributors: Vec::new(),
            evidence: Vec::new(),
            impact: MilestoneImpact::default(),
            celebration: None,
            recognition_status: RecognitionStatus::Pending,
            tags: Vec::new(),
            related_milestones: Vec::new(),
            insights_gained: Vec::new(),
        };
        
        let milestone_id = milestone.id;
        self.milestones.insert(milestone_id, milestone);
        milestone_id
    }

    /// Get a milestone by ID
    pub fn get_milestone(&self, milestone_id: Uuid) -> Option<&TransformationMilestone> {
        self.milestones.get(&milestone_id)
    }

    /// Get all milestones for a community
    pub fn get_community_milestones(&self, community_id: &str) -> Vec<&TransformationMilestone> {
        self.milestones
            .values()
            .filter(|milestone| milestone.milestone_id == community_id)
            .collect()
    }

    /// Add a contributor to a milestone
    pub fn add_contributor(
        &mut self,
        milestone_id: Uuid,
        contributor: MilestoneContributor,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(milestone) = self.milestones.get_mut(&milestone_id) {
            milestone.contributors.push(contributor);
            Ok(())
        } else {
            Err("Milestone not found".into())
        }
    }

    /// Add evidence to a milestone
    pub fn add_evidence(
        &mut self,
        milestone_id: Uuid,
        evidence: MilestoneEvidence,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(milestone) = self.milestones.get_mut(&milestone_id) {
            milestone.evidence.push(evidence);
            Ok(())
        } else {
            Err("Milestone not found".into())
        }
    }

    /// Add an insight gained from a milestone
    pub fn add_insight(
        &mut self,
        milestone_id: Uuid,
        insight: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(milestone) = self.milestones.get_mut(&milestone_id) {
            milestone.insights_gained.push(MilestoneInsight {
                insight,
                timestamp: Utc::now(),
            });
            Ok(())
        } else {
            Err("Milestone not found".into())
        }
    }

    /// Plan a celebration for a milestone
    pub fn plan_celebration(
        &mut self,
        milestone_id: Uuid,
        celebration: MilestoneCelebration,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(milestone) = self.milestones.get_mut(&milestone_id) {
            milestone.celebration = Some(celebration);
            Ok(())
        } else {
            Err("Milestone not found".into())
        }
    }

    /// Recognize a milestone
    pub fn recognize_milestone(&mut self, milestone_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(milestone) = self.milestones.get_mut(&milestone_id) {
            milestone.recognition_status = RecognitionStatus::Recognized;
            Ok(())
        } else {
            Err("Milestone not found".into())
        }
    }

    /// Feature a milestone
    pub fn feature_milestone(&mut self, milestone_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(milestone) = self.milestones.get_mut(&milestone_id) {
            milestone.recognition_status = RecognitionStatus::Featured;
            Ok(())
        } else {
            Err("Milestone not found".into())
        }
    }

    /// Check for and create automatic milestones based on criteria
    pub fn check_automatic_milestones(&mut self, community_id: &str) -> Vec<Uuid> {
        let mut created_milestones = Vec::new();
        
        for criterion in &self.recognition_criteria {
            if criterion.check_criteria(community_id) {
                let milestone_id = self.create_milestone(
                    community_id.to_string(),
                    criterion.title.clone(),
                    criterion.milestone_type.clone(),
                    criterion.description.clone(),
                );
                created_milestones.push(milestone_id);
            }
        }
        
        created_milestones
    }

    /// Get milestones by type
    pub fn get_milestones_by_type(&self, milestone_type: &MilestoneType) -> Vec<&TransformationMilestone> {
        self.milestones
            .values()
            .filter(|milestone| milestone.milestone_type == *milestone_type)
            .collect()
    }

    /// Get milestones by significance level
    pub fn get_milestones_by_significance(&self, significance: &SignificanceLevel) -> Vec<&TransformationMilestone> {
        self.milestones
            .values()
            .filter(|milestone| &milestone.significance_level == significance)
            .collect()
    }

    /// Get recent milestones (within last 30 days)
    pub fn get_recent_milestones(&self) -> Vec<&TransformationMilestone> {
        let thirty_days_ago = Utc::now() - chrono::Duration::days(30);
        
        self.milestones
            .values()
            .filter(|milestone| milestone.achieved_at > thirty_days_ago)
            .collect()
    }

    /// Create default milestone templates
    fn create_default_templates() -> Vec<MilestoneTemplate> {
        vec![
            MilestoneTemplate {
                id: Uuid::new_v4(),
                name: "First Connection Discovery".to_string(),
                description: "Community discovers first connection between impact domains".to_string(),
                milestone_type: MilestoneType::ConnectionDiscovery,
                suggested_evidence: vec![
                    "Dashboard visualization showing connection".to_string(),
                    "Community discussion notes".to_string(),
                    "Documentation of the connection".to_string(),
                ],
                celebration_ideas: vec![
                    "Community acknowledgment in meeting".to_string(),
                    "Visual celebration on dashboard".to_string(),
                    "Share story with other communities".to_string(),
                ],
            },
            MilestoneTemplate {
                id: Uuid::new_v4(),
                name: "Integrated Understanding".to_string(),
                description: "Community demonstrates integrated understanding of all four domains".to_string(),
                milestone_type: MilestoneType::UnderstandingShift,
                suggested_evidence: vec![
                    "Community validation outcomes".to_string(),
                    "Impact stories showing integration".to_string(),
                    "Meeting recordings or transcripts".to_string(),
                ],
                celebration_ideas: vec![
                    "Community celebration event".to_string(),
                    "Feature in community newsletter".to_string(),
                    "Create commemorative visualization".to_string(),
                ],
            },
            MilestoneTemplate {
                id: Uuid::new_v4(),
                name: "Process Integration".to_string(),
                description: "Community integrates interconnected impact understanding into regular processes".to_string(),
                milestone_type: MilestoneType::ProcessIntegration,
                suggested_evidence: vec![
                    "Updated process documentation".to_string(),
                    "Meeting agendas showing integration".to_string(),
                    "Community feedback on new processes".to_string(),
                ],
                celebration_ideas: vec![
                    "Process launch celebration".to_string(),
                    "Recognition of contributors".to_string(),
                    "Share success story widely".to_string(),
                ],
            },
            MilestoneTemplate {
                id: Uuid::new_v4(),
                name: "Transformative Action".to_string(),
                description: "Community takes transformative action based on interconnected understanding".to_string(),
                milestone_type: MilestoneType::TransformativeAction,
                suggested_evidence: vec![
                    "Documentation of the action".to_string(),
                    "Impact metrics showing change".to_string(),
                    "Community testimonials about the action".to_string(),
                ],
                celebration_ideas: vec![
                    "Major community celebration".to_string(),
                    "Media coverage or press release".to_string(),
                    "Create case study for other communities".to_string(),
                ],
            },
        ]
    }

    /// Create default recognition criteria
    fn create_default_criteria() -> Vec<RecognitionCriterion> {
        vec![
            RecognitionCriterion {
                id: Uuid::new_v4(),
                name: "First Dashboard Use".to_string(),
                title: "First Dashboard Engagement".to_string(),
                description: "Community actively engages with dashboard for first time".to_string(),
                milestone_type: MilestoneType::Engagement,
                criteria: Criterion::DashboardUsage { min_sessions: 5, min_users: 3 },
            },
            RecognitionCriterion {
                id: Uuid::new_v4(),
                name: "First Story Shared".to_string(),
                title: "First Community Impact Story".to_string(),
                description: "Community shares first impact story through dashboard".to_string(),
                milestone_type: MilestoneType::Storytelling,
                criteria: Criterion::StorySharing { min_stories: 1 },
            },
            RecognitionCriterion {
                id: Uuid::new_v4(),
                name: "First Validation Session".to_string(),
                title: "First Collaborative Validation".to_string(),
                description: "Community completes first collaborative validation session".to_string(),
                milestone_type: MilestoneType::Collaboration,
                criteria: Criterion::ValidationSessions { min_sessions: 1 },
            },
            RecognitionCriterion {
                id: Uuid::new_v4(),
                name: "Multiple Connections".to_string(),
                title: "Multiple Domain Connections".to_string(),
                description: "Community discovers and documents multiple domain connections".to_string(),
                milestone_type: MilestoneType::ConnectionDiscovery,
                criteria: Criterion::ConnectionDiscovery { min_connections: 3 },
            },
        ]
    }
}

/// Transformation milestone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationMilestone {
    /// Milestone identifier
    pub id: Uuid,
    
    /// Community identifier
    pub community_id: String,
    
    /// Milestone title
    pub title: String,
    
    /// Type of milestone
    pub milestone_type: MilestoneType,
    
    /// Milestone description
    pub description: String,
    
    /// When this milestone was achieved
    pub achieved_at: DateTime<Utc>,
    
    /// Significance level of this milestone
    pub significance_level: SignificanceLevel,
    
    /// Contributors to this milestone
    pub contributors: Vec<MilestoneContributor>,
    
    /// Evidence supporting this milestone
    pub evidence: Vec<MilestoneEvidence>,
    
    /// Impact of this milestone
    pub impact: MilestoneImpact,
    
    /// Celebration details (if planned or completed)
    pub celebration: Option<MilestoneCelebration>,
    
    /// Recognition status
    pub recognition_status: RecognitionStatus,
    
    /// Tags for categorization
    pub tags: Vec<String>,
    
    /// Related milestones
    pub related_milestones: Vec<Uuid>,
    
    /// Insights gained from this milestone
    pub insights_gained: Vec<MilestoneInsight>,
}

/// Types of transformation milestones
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MilestoneType {
    /// Discovery of connections between domains
    ConnectionDiscovery,
    /// Shift in community understanding
    UnderstandingShift,
    /// Integration into community processes
    ProcessIntegration,
    /// Transformative action taken
    TransformativeAction,
    /// Community engagement milestone
    Engagement,
    /// Storytelling milestone
    Storytelling,
    /// Collaboration milestone
    Collaboration,
    /// Other type of milestone
    Other(String),
}

/// Significance levels of milestones
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SignificanceLevel {
    Low,
    Medium,
    High,
    Transformative,
}

/// Contributor to a milestone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilestoneContributor {
    /// Contributor name or identifier
    pub name: String,
    
    /// Role in the milestone
    pub role: String,
    
    /// Contribution description
    pub contribution: String,
    
    /// When this contribution was made
    pub timestamp: DateTime<Utc>,
}

/// Evidence supporting a milestone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilestoneEvidence {
    /// Evidence description
    pub description: String,
    
    /// Type of evidence
    pub evidence_type: EvidenceType,
    
    /// Source of the evidence
    pub source: String,
    
    /// Link to evidence (if applicable)
    pub link: Option<String>,
    
    /// When this evidence was collected
    pub collected_at: DateTime<Utc>,
}

/// Types of evidence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceType {
    DashboardVisualization,
    CommunityDiscussion,
    Documentation,
    Testimonial,
    DataMetric,
    ExternalReference,
    Other(String),
}

/// Impact of a milestone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilestoneImpact {
    /// Description of the impact
    pub description: String,
    
    /// Changes in community understanding
    pub understanding_changes: Vec<String>,
    
    /// Changes in community behavior
    pub behavior_changes: Vec<String>,
    
    /// New initiatives or actions
    pub new_initiatives: Vec<String>,
    
    /// Quantitative metrics showing impact
    pub quantitative_impact: Vec<String>,
    
    /// Qualitative impact descriptions
    pub qualitative_impact: Vec<String>,
}

impl Default for MilestoneImpact {
    fn default() -> Self {
        Self {
            description: String::new(),
            understanding_changes: Vec::new(),
            behavior_changes: Vec::new(),
            new_initiatives: Vec::new(),
            quantitative_impact: Vec::new(),
            qualitative_impact: Vec::new(),
        }
    }
}

/// Milestone celebration details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilestoneCelebration {
    /// Celebration title
    pub title: String,
    
    /// Celebration description
    pub description: String,
    
    /// Type of celebration
    pub celebration_type: CelebrationType,
    
    /// When the celebration occurred or will occur
    pub celebration_date: DateTime<Utc>,
    
    /// Participants in the celebration
    pub participants: Vec<String>,
    
    /// Celebration activities
    pub activities: Vec<String>,
    
    /// Celebration outcomes
    pub outcomes: Vec<String>,
    
    /// Celebration status
    pub status: CelebrationStatus,
}

/// Types of celebrations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CelebrationType {
    CommunityMeeting,
    SpecialEvent,
    VirtualCelebration,
    Documentation,
    SocialMedia,
    PressRelease,
    Other(String),
}

/// Celebration status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CelebrationStatus {
    Planned,
    InProgress,
    Completed,
    Cancelled,
}

/// Recognition status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecognitionStatus {
    Pending,
    Recognized,
    Featured,
    Archived,
}

/// Insight gained from a milestone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilestoneInsight {
    /// The insight gained
    pub insight: String,
    
    /// When this insight was documented
    pub timestamp: DateTime<Utc>,
}

/// Milestone template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilestoneTemplate {
    /// Template identifier
    pub id: Uuid,
    
    /// Template name
    pub name: String,
    
    /// Template description
    pub description: String,
    
    /// Type of milestone this template creates
    pub milestone_type: MilestoneType,
    
    /// Suggested evidence for this milestone
    pub suggested_evidence: Vec<String>,
    
    /// Celebration ideas for this milestone
    pub celebration_ideas: Vec<String>,
}

/// Recognition criterion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecognitionCriterion {
    /// Criterion identifier
    pub id: Uuid,
    
    /// Criterion name
    pub name: String,
    
    /// Title for milestone created by this criterion
    pub title: String,
    
    /// Description for milestone created by this criterion
    pub description: String,
    
    /// Type of milestone created by this criterion
    pub milestone_type: MilestoneType,
    
    /// The actual criteria logic
    pub criteria: Criterion,
}

impl RecognitionCriterion {
    /// Check if the criteria are met for a community
    fn check_criteria(&self, _community_id: &str) -> bool {
        // In a real implementation, this would check actual data
        // For now, return false as placeholder
        false
    }
}

/// Criteria types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Criterion {
    DashboardUsage {
        min_sessions: u32,
        min_users: u32,
    },
    StorySharing {
        min_stories: u32,
    },
    ValidationSessions {
        min_sessions: u32,
    },
    ConnectionDiscovery {
        min_connections: u32,
    },
    TimePeriod {
        days_active: u32,
    },
    EngagementLevel {
        min_engagement_score: f64,
    },
    Other(String),
}

/// Celebration pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CelebrationPattern {
    /// Pattern identifier
    pub id: Uuid,
    
    /// Pattern name
    pub name: String,
    
    /// Pattern description
    pub description: String,
    
    /// Types of milestones this pattern works for
    pub applicable_milestone_types: Vec<MilestoneType>,
    
    /// Celebration activities in this pattern
    pub activities: Vec<CelebrationActivity>,
    
    /// Communities that have used this pattern
    pub communities_used: Vec<String>,
    
    /// Effectiveness rating of this pattern
    pub effectiveness_rating: f64,
}

/// Celebration activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CelebrationActivity {
    /// Activity name
    pub name: String,
    
    /// Activity description
    pub description: String,
    
    /// Resources needed for this activity
    pub resources_needed: Vec<String>,
    
    /// Time required for this activity
    pub time_required: String,
    
    /// Participant requirements
    pub participant_requirements: Vec<String>,
}