//! Community-Specific Adaptation Tracking System
//!
//! This module provides tools for tracking how communities adapt the dashboard
//! and interconnected impact concepts to their specific contexts and needs.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::models::impact_data::ImpactDomain;

/// Community adaptation tracking system
pub struct CommunityAdaptationTracking {
    /// Stored adaptation records
    adaptations: HashMap<Uuid, CommunityAdaptation>,
    
    /// Adaptation patterns across communities
    adaptation_patterns: Vec<AdaptationPattern>,
    
    /// Best practices from adaptations
    best_practices: Vec<AdaptationBestPractice>,
    
    /// Adaptation categories
    categories: Vec<AdaptationCategory>,
}

impl CommunityAdaptationTracking {
    /// Create a new community adaptation tracking system
    pub fn new() -> Self {
        Self {
            adaptations: HashMap::new(),
            adaptation_patterns: Vec::new(),
            best_practices: Vec::new(),
            categories: Self::create_default_categories(),
        }
    }

    /// Initialize the adaptation tracking system
    pub fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Initialize with default categories
        Ok(())
    }

    /// Create a new community adaptation record
    pub fn create_adaptation(
        &mut self,
        community_id: String,
        name: String,
        description: String,
        adaptation_type: AdaptationType,
    ) -> Uuid {
        let adaptation = CommunityAdaptation {
            id: Uuid::new_v4(),
            community_id,
            name,
            description,
            adaptation_type,
            context: CommunityContext::default(),
            modifications: Vec::new(),
            outcomes: Vec::new(),
            challenges: Vec::new(),
            solutions: Vec::new(),
            lessons_learned: Vec::new(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
            status: AdaptationStatus::InProgress,
            effectiveness_metrics: AdaptationEffectiveness::default(),
            sharing_status: SharingStatus::Internal,
        };
        
        let adaptation_id = adaptation.id;
        self.adaptations.insert(adaptation_id, adaptation);
        adaptation_id
    }

    /// Get an adaptation by ID
    pub fn get_adaptation(&self, adaptation_id: Uuid) -> Option<&CommunityAdaptation> {
        self.adaptations.get(&adaptation_id)
    }

    /// Get all adaptations for a community
    pub fn get_community_adaptations(&self, community_id: &str) -> Vec<&CommunityAdaptation> {
        self.adaptations
            .values()
            .filter(|adaptation| adaptation.community_id == community_id)
            .collect()
    }

    /// Add a modification to an adaptation
    pub fn add_modification(
        &mut self,
        adaptation_id: Uuid,
        modification: AdaptationModification,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(adaptation) = self.adaptations.get_mut(&adaptation_id) {
            adaptation.modifications.push(modification);
            adaptation.last_updated = Utc::now();
            Ok(())
        } else {
            Err("Adaptation not found".into())
        }
    }

    /// Add an outcome to an adaptation
    pub fn add_outcome(
        &mut self,
        adaptation_id: Uuid,
        outcome: AdaptationOutcome,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(adaptation) = self.adaptations.get_mut(&adaptation_id) {
            adaptation.outcomes.push(outcome);
            adaptation.last_updated = Utc::now();
            Ok(())
        } else {
            Err("Adaptation not found".into())
        }
    }

    /// Add a challenge to an adaptation
    pub fn add_challenge(
        &mut self,
        adaptation_id: Uuid,
        challenge: AdaptationChallenge,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(adaptation) = self.adaptations.get_mut(&adaptation_id) {
            adaptation.challenges.push(challenge);
            adaptation.last_updated = Utc::now();
            Ok(())
        } else {
            Err("Adaptation not found".into())
        }
    }

    /// Add a solution to an adaptation
    pub fn add_solution(
        &mut self,
        adaptation_id: Uuid,
        solution: AdaptationSolution,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(adaptation) = self.adaptations.get_mut(&adaptation_id) {
            adaptation.solutions.push(solution);
            adaptation.last_updated = Utc::now();
            Ok(())
        } else {
            Err("Adaptation not found".into())
        }
    }

    /// Add a lesson learned to an adaptation
    pub fn add_lesson_learned(
        &mut self,
        adaptation_id: Uuid,
        lesson: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(adaptation) = self.adaptations.get_mut(&adaptation_id) {
            adaptation.lessons_learned.push(LessonLearned {
                lesson,
                timestamp: Utc::now(),
            });
            adaptation.last_updated = Utc::now();
            Ok(())
        } else {
            Err("Adaptation not found".into())
        }
    }

    /// Complete an adaptation
    pub fn complete_adaptation(&mut self, adaptation_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(adaptation) = self.adaptations.get_mut(&adaptation_id) {
            adaptation.status = AdaptationStatus::Completed;
            adaptation.last_updated = Utc::now();
            Ok(())
        } else {
            Err("Adaptation not found".into())
        }
    }

    /// Share an adaptation with other communities
    pub fn share_adaptation(&mut self, adaptation_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(adaptation) = self.adaptations.get_mut(&adaptation_id) {
            adaptation.sharing_status = SharingStatus::Shared;
            adaptation.last_updated = Utc::now();
            Ok(())
        } else {
            Err("Adaptation not found".into())
        }
    }

    /// Analyze adaptation patterns across communities
    pub fn analyze_patterns(&mut self) -> Vec<AdaptationPattern> {
        let mut patterns = Vec::new();
        
        // Analyze common modifications
        let mut modification_counts: HashMap<String, usize> = HashMap::new();
        
        for adaptation in self.adaptations.values() {
            for modification in &adaptation.modifications {
                let key = modification.component.clone();
                *modification_counts.entry(key).or_insert(0) += 1;
            }
        }
        
        // Identify common patterns
        for (component, count) in modification_counts {
            if count >= 2 { // Threshold for pattern recognition
                patterns.push(AdaptationPattern {
                    id: Uuid::new_v4(),
                    pattern_type: PatternType::ComponentModification,
                    component,
                    frequency: count,
                    description: format!("Common adaptation pattern for component: {}", component),
                    communities_using: self.get_communities_with_component_modification(&component),
                    effectiveness_rating: self.calculate_pattern_effectiveness(&component),
                });
            }
        }
        
        self.adaptation_patterns = patterns.clone();
        patterns
    }

    /// Get communities that modified a specific component
    fn get_communities_with_component_modification(&self, component: &str) -> Vec<String> {
        let mut communities = std::collections::HashSet::new();
        
        for adaptation in self.adaptations.values() {
            for modification in &adaptation.modifications {
                if modification.component == component {
                    communities.insert(adaptation.community_id.clone());
                }
            }
        }
        
        communities.into_iter().collect()
    }

    /// Calculate effectiveness rating for a pattern
    fn calculate_pattern_effectiveness(&self, component: &str) -> f64 {
        let mut total_effectiveness = 0.0;
        let mut count = 0;
        
        for adaptation in self.adaptations.values() {
            for modification in &adaptation.modifications {
                if modification.component == component {
                    total_effectiveness += adaptation.effectiveness_metrics.overall_effectiveness;
                    count += 1;
                }
            }
        }
        
        if count > 0 {
            total_effectiveness / count as f64
        } else {
            0.0
        }
    }

    /// Extract best practices from adaptations
    pub fn extract_best_practices(&mut self) -> Vec<AdaptationBestPractice> {
        let mut best_practices = Vec::new();
        
        // Look for highly effective adaptations
        for adaptation in self.adaptations.values() {
            if adaptation.effectiveness_metrics.overall_effectiveness >= 0.8 {
                for lesson in &adaptation.lessons_learned {
                    best_practices.push(AdaptationBestPractice {
                        id: Uuid::new_v4(),
                        title: adaptation.name.clone(),
                        description: lesson.lesson.clone(),
                        source_community: adaptation.community_id.clone(),
                        adaptation_type: adaptation.adaptation_type.clone(),
                        effectiveness_rating: adaptation.effectiveness_metrics.overall_effectiveness,
                        applicable_contexts: adaptation.context.clone(),
                        implementation_steps: adaptation.modifications.iter()
                            .map(|m| m.description.clone())
                            .collect(),
                        created_at: lesson.timestamp,
                    });
                }
            }
        }
        
        self.best_practices = best_practices.clone();
        best_practices
    }

    /// Create default adaptation categories
    fn create_default_categories() -> Vec<AdaptationCategory> {
        vec![
            AdaptationCategory {
                id: Uuid::new_v4(),
                name: "Visual Customization".to_string(),
                description: "Changes to dashboard appearance and visualization".to_string(),
                examples: vec![
                    "Color scheme modifications".to_string(),
                    "Layout adjustments".to_string(),
                    "Custom visualization types".to_string(),
                ],
            },
            AdaptationCategory {
                id: Uuid::new_v4(),
                name: "Feature Adaptation".to_string(),
                description: "Modifications to dashboard functionality".to_string(),
                examples: vec![
                    "Custom metrics definitions".to_string(),
                    "New domain additions".to_string(),
                    "Workflow changes".to_string(),
                ],
            },
            AdaptationCategory {
                id: Uuid::new_v4(),
                name: "Process Integration".to_string(),
                description: "Integration with community processes and practices".to_string(),
                examples: vec![
                    "Meeting integration".to_string(),
                    "Decision-making processes".to_string(),
                    "Community rituals".to_string(),
                ],
            },
            AdaptationCategory {
                id: Uuid::new_v4(),
                name: "Cultural Adaptation".to_string(),
                description: "Adaptations to fit cultural context and values".to_string(),
                examples: vec![
                    "Language localization".to_string(),
                    "Cultural relevance adjustments".to_string(),
                    "Value alignment modifications".to_string(),
                ],
            },
        ]
    }
}

/// Community adaptation record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityAdaptation {
    /// Adaptation identifier
    pub id: Uuid,
    
    /// Community identifier
    pub community_id: String,
    
    /// Adaptation name
    pub name: String,
    
    /// Adaptation description
    pub description: String,
    
    /// Type of adaptation
    pub adaptation_type: AdaptationType,
    
    /// Community context where adaptation was made
    pub context: CommunityContext,
    
    /// Modifications made
    pub modifications: Vec<AdaptationModification>,
    
    /// Outcomes of the adaptation
    pub outcomes: Vec<AdaptationOutcome>,
    
    /// Challenges encountered
    pub challenges: Vec<AdaptationChallenge>,
    
    /// Solutions implemented
    pub solutions: Vec<AdaptationSolution>,
    
    /// Lessons learned from the adaptation
    pub lessons_learned: Vec<LessonLearned>,
    
    /// When this adaptation was created
    pub created_at: DateTime<Utc>,
    
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
    
    /// Current status of the adaptation
    pub status: AdaptationStatus,
    
    /// Effectiveness metrics
    pub effectiveness_metrics: AdaptationEffectiveness,
    
    /// Sharing status with other communities
    pub sharing_status: SharingStatus,
}

/// Types of adaptations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdaptationType {
    /// Visual adaptation (appearance, layout)
    Visual,
    /// Functional adaptation (features, workflows)
    Functional,
    /// Process adaptation (integration with community processes)
    Process,
    /// Cultural adaptation (language, values, context)
    Cultural,
    /// Technical adaptation (integration, data sources)
    Technical,
    /// Other type of adaptation
    Other(String),
}

/// Community context information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityContext {
    /// Community size
    pub size: CommunitySize,
    
    /// Community type
    pub community_type: CommunityType,
    
    /// Primary language(s)
    pub languages: Vec<String>,
    
    /// Cultural context factors
    pub cultural_factors: Vec<String>,
    
    /// Technical capacity
    pub technical_capacity: TechnicalCapacity,
    
    /// Organizational structure
    pub organizational_structure: OrganizationalStructure,
}

impl Default for CommunityContext {
    fn default() -> Self {
        Self {
            size: CommunitySize::Medium,
            community_type: CommunityType::Cooperative,
            languages: vec!["English".to_string()],
            cultural_factors: Vec::new(),
            technical_capacity: TechnicalCapacity::Moderate,
            organizational_structure: OrganizationalStructure::Flat,
        }
    }
}

/// Community size categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommunitySize {
    Small,
    Medium,
    Large,
    VeryLarge,
}

/// Community types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommunityType {
    Cooperative,
    Nonprofit,
    CommunityOrganization,
    SocialEnterprise,
    EducationalInstitution,
    ReligiousOrganization,
    Other(String),
}

/// Technical capacity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TechnicalCapacity {
    Limited,
    Moderate,
    Advanced,
    Expert,
}

/// Organizational structure types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrganizationalStructure {
    Flat,
    Hierarchical,
    Networked,
    Hybrid,
}

/// Adaptation modification details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationModification {
    /// Component that was modified
    pub component: String,
    
    /// Description of the modification
    pub description: String,
    
    /// Original state before modification
    pub original_state: String,
    
    /// Modified state after change
    pub modified_state: String,
    
    /// Reason for the modification
    pub reason: String,
    
    /// When this modification was made
    pub timestamp: DateTime<Utc>,
    
    /// Who made this modification
    pub made_by: Vec<String>,
    
    /// Complexity of the modification
    pub complexity: ComplexityLevel,
}

/// Complexity levels of modifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplexityLevel {
    Simple,
    Moderate,
    Complex,
    VeryComplex,
}

/// Outcome of an adaptation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationOutcome {
    /// Outcome description
    pub description: String,
    
    /// Type of outcome
    pub outcome_type: OutcomeType,
    
    /// When this outcome was observed
    pub timestamp: DateTime<Utc>,
    
    /// Evidence supporting this outcome
    pub evidence: Vec<String>,
    
    /// Impact level of this outcome
    pub impact_level: ImpactLevel,
}

/// Types of outcomes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutcomeType {
    Positive,
    Negative,
    Neutral,
    Mixed,
}

/// Impact levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactLevel {
    Low,
    Medium,
    High,
    Transformative,
}

/// Challenge encountered during adaptation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationChallenge {
    /// Challenge description
    pub description: String,
    
    /// Type of challenge
    pub challenge_type: ChallengeType,
    
    /// When this challenge was encountered
    pub timestamp: DateTime<Utc>,
    
    /// Severity of the challenge
    pub severity: SeverityLevel,
    
    /// Who was affected by this challenge
    pub affected_parties: Vec<String>,
}

/// Types of challenges
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChallengeType {
    Technical,
    Cultural,
    Resource,
    Adoption,
    Process,
    Other(String),
}

/// Severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SeverityLevel {
    Minor,
    Moderate,
    Major,
    Critical,
}

/// Solution implemented for a challenge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationSolution {
    /// Solution description
    pub description: String,
    
    /// Challenge this solution addresses
    pub challenge_id: Uuid,
    
    /// When this solution was implemented
    pub timestamp: DateTime<Utc>,
    
    /// Who implemented this solution
    pub implemented_by: Vec<String>,
    
    /// Effectiveness of this solution
    pub effectiveness: f64,
    
    /// Resources used for this solution
    pub resources_used: Vec<String>,
}

/// Lesson learned from adaptation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonLearned {
    /// The lesson learned
    pub lesson: String,
    
    /// When this lesson was documented
    pub timestamp: DateTime<Utc>,
}

/// Adaptation status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdaptationStatus {
    Draft,
    InProgress,
    Completed,
    Archived,
}

/// Effectiveness metrics for adaptations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationEffectiveness {
    /// Overall effectiveness rating (0.0 to 1.0)
    pub overall_effectiveness: f64,
    
    /// User satisfaction rating (0.0 to 1.0)
    pub user_satisfaction: f64,
    
    /// Goal achievement rating (0.0 to 1.0)
    pub goal_achievement: f64,
    
    /// Sustainability rating (0.0 to 1.0)
    pub sustainability: f64,
    
    /// Replicability rating (0.0 to 1.0)
    pub replicability: f64,
    
    /// Comments on effectiveness
    pub effectiveness_comments: Vec<String>,
}

impl Default for AdaptationEffectiveness {
    fn default() -> Self {
        Self {
            overall_effectiveness: 0.0,
            user_satisfaction: 0.0,
            goal_achievement: 0.0,
            sustainability: 0.0,
            replicability: 0.0,
            effectiveness_comments: Vec::new(),
        }
    }
}

/// Sharing status with other communities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SharingStatus {
    Internal,
    Shared,
    Featured,
}

/// Adaptation pattern observed across communities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationPattern {
    /// Pattern identifier
    pub id: Uuid,
    
    /// Type of pattern
    pub pattern_type: PatternType,
    
    /// Component that was adapted
    pub component: String,
    
    /// How many communities use this pattern
    pub frequency: usize,
    
    /// Pattern description
    pub description: String,
    
    /// Communities using this pattern
    pub communities_using: Vec<String>,
    
    /// Average effectiveness rating
    pub effectiveness_rating: f64,
}

/// Types of patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    ComponentModification,
    ProcessChange,
    CulturalAdaptation,
    TechnicalIntegration,
    Other(String),
}

/// Best practice extracted from adaptations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationBestPractice {
    /// Best practice identifier
    pub id: Uuid,
    
    /// Best practice title
    pub title: String,
    
    /// Best practice description
    pub description: String,
    
    /// Community that originated this practice
    pub source_community: String,
    
    /// Type of adaptation this practice relates to
    pub adaptation_type: AdaptationType,
    
    /// Effectiveness rating of this practice
    pub effectiveness_rating: f64,
    
    /// Contexts where this practice is applicable
    pub applicable_contexts: CommunityContext,
    
    /// Steps to implement this practice
    pub implementation_steps: Vec<String>,
    
    /// When this practice was documented
    pub created_at: DateTime<Utc>,
}

/// Adaptation category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationCategory {
    /// Category identifier
    pub id: Uuid,
    
    /// Category name
    pub name: String,
    
    /// Category description
    pub description: String,
    
    /// Examples in this category
    pub examples: Vec<String>,
}