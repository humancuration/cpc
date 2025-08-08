//! Transformation Story Collection System
//!
//! This module provides tools for collecting and managing community transformation stories
//! that document how understanding interconnected impact transforms community engagement.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::models::impact_story::{ImpactStory, StoryAuthor};
use shared_packages::social_interactions::domain::{Reaction, Comment, Share};

/// Transformation story that captures community evolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationStory {
    /// Unique identifier
    pub id: Uuid,
    
    /// Community identifier
    pub community_id: String,
    
    /// Title of the transformation story
    pub title: String,
    
    /// Core transformation narrative
    pub narrative: TransformationNarrative,
    
    /// Author information
    pub author: StoryAuthor,
    
    /// Timeline of transformation events
    pub timeline: Vec<TransformationEvent>,
    
    /// Impact domain connections discovered
    pub domain_connections: Vec<DomainConnection>,
    
    /// Emotional journey markers
    pub emotional_journey: Vec<EmotionalMarker>,
    
    /// Key insights and learnings
    pub insights: Vec<TransformationInsight>,
    
    /// Related impact stories
    pub related_stories: Vec<Uuid>,
    
    /// Community validation status
    pub validation_status: ValidationStatus,
    
    /// Tags and categories
    pub tags: Vec<String>,
    
    /// When this story was created
    pub created_at: DateTime<Utc>,
    
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
}

/// Transformation narrative structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationNarrative {
    /// Before state - how community understood impact before
    pub before_state: CommunityState,
    
    /// Catalyst event - what triggered the transformation
    pub catalyst: CatalystEvent,
    
    /// Transformation process - how understanding evolved
    pub process: TransformationProcess,
    
    /// After state - how community understands impact now
    pub after_state: CommunityState,
    
    /// Key learnings from the transformation
    pub key_learnings: Vec<String>,
}

/// Community state at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityState {
    /// Description of community understanding
    pub understanding_description: String,
    
    /// Impact awareness level
    pub awareness_level: AwarenessLevel,
    
    /// Engagement patterns
    pub engagement_patterns: Vec<EngagementPattern>,
    
    /// Community challenges
    pub challenges: Vec<String>,
    
    /// Community strengths
    pub strengths: Vec<String>,
}

/// Levels of impact awareness
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AwarenessLevel {
    /// Siloed understanding - domains seen as separate
    Siloed,
    /// Emerging awareness - starting to see connections
    Emerging,
    /// Integrated understanding - seeing interconnected impact
    Integrated,
    /// Transformative understanding - using connections to drive change
    Transformative,
}

/// Catalyst event that triggered transformation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalystEvent {
    /// Description of the catalyst
    pub description: String,
    
    /// Type of catalyst
    pub catalyst_type: CatalystType,
    
    /// When the catalyst occurred
    pub timestamp: DateTime<Utc>,
    
    /// Who was involved
    pub participants: Vec<String>,
    
    /// Immediate impact of the catalyst
    pub immediate_impact: String,
}

/// Types of catalyst events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CatalystType {
    /// Dashboard visualization that revealed connections
    DashboardInsight,
    /// Community workshop or discussion
    CommunityWorkshop,
    /// External event or crisis
    ExternalEvent,
    /// Leadership change or initiative
    LeadershipInitiative,
    /// Data-driven discovery
    DataDiscovery,
    /// Other type of catalyst
    Other(String),
}

/// Transformation process description
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationProcess {
    /// Key phases of the transformation
    pub phases: Vec<TransformationPhase>,
    
    /// Methods used to facilitate transformation
    pub methods: Vec<TransformationMethod>,
    
    /// Challenges encountered during transformation
    pub challenges: Vec<String>,
    
    /// How challenges were overcome
    pub solutions: Vec<String>,
    
    /// Duration of transformation
    pub duration: std::time::Duration,
}

/// Individual phase of transformation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationPhase {
    /// Phase name or title
    pub title: String,
    
    /// Phase description
    pub description: String,
    
    /// Start of phase
    pub start_time: DateTime<Utc>,
    
    /// End of phase (if completed)
    pub end_time: Option<DateTime<Utc>>,
    
    /// Key activities in this phase
    pub activities: Vec<String>,
    
    /// Outcomes of this phase
    pub outcomes: Vec<String>,
}

/// Methods used to facilitate transformation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransformationMethod {
    /// Collaborative interpretation sessions
    CollaborativeInterpretation,
    /// Data visualization and exploration
    DataVisualization,
    /// Community storytelling circles
    StorytellingCircles,
    /// Peer learning exchanges
    PeerLearning,
    /// Facilitated workshops
    FacilitatedWorkshop,
    /// Reflective practice
    ReflectivePractice,
    /// Other method
    Other(String),
}

/// Individual transformation event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationEvent {
    /// Event identifier
    pub id: Uuid,
    
    /// Event title
    pub title: String,
    
    /// Event description
    pub description: String,
    
    /// When the event occurred
    pub timestamp: DateTime<Utc>,
    
    /// Significance of this event
    pub significance: EventSignificance,
    
    /// People involved
    pub participants: Vec<String>,
    
    /// Outcomes or results
    pub outcomes: Vec<String>,
}

/// Significance level of transformation events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventSignificance {
    /// Minor event with limited impact
    Minor,
    /// Notable event with some impact
    Notable,
    /// Major event with significant impact
    Major,
    /// Pivotal event that changed direction
    Pivotal,
}

/// Connection between impact domains discovered during transformation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainConnection {
    /// Source domain
    pub source_domain: String,
    
    /// Target domain
    pub target_domain: String,
    
    /// Description of the connection
    pub connection_description: String,
    
    /// How this connection was discovered
    pub discovery_method: String,
    
    /// Impact of understanding this connection
    pub impact: String,
    
    /// Evidence supporting the connection
    pub evidence: Vec<String>,
    
    /// When this connection was discovered
    pub discovered_at: DateTime<Utc>,
}

/// Emotional marker in the transformation journey
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalMarker {
    /// Type of emotion experienced
    pub emotion_type: TransformationEmotion,
    
    /// When this emotion was prominent
    pub timestamp: DateTime<Utc>,
    
    /// Trigger for this emotion
    pub trigger: String,
    
    /// How the community processed this emotion
    pub processing: String,
    
    /// Outcome of processing this emotion
    pub outcome: String,
}

/// Emotions experienced during transformation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransformationEmotion {
    /// Aha moment of insight
    Insight,
    /// Confusion or uncertainty
    Confusion,
    /// Excitement about possibilities
    Excitement,
    /// Resistance to change
    Resistance,
    /// Acceptance of new understanding
    Acceptance,
    /// Joy in transformation
    Joy,
    /// Anxiety about the unknown
    Anxiety,
    /// Hope for the future
    Hope,
    /// Other emotion
    Other(String),
}

/// Key insight from the transformation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationInsight {
    /// Insight description
    pub description: String,
    
    /// How this insight was discovered
    pub discovery_method: String,
    
    /// Impact of this insight
    pub impact: String,
    
    /// Related domain connections
    pub related_connections: Vec<Uuid>,
    
    /// When this insight emerged
    pub timestamp: DateTime<Utc>,
}

/// Community engagement pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngagementPattern {
    /// Pattern description
    pub description: String,
    
    /// Frequency of this pattern
    pub frequency: PatternFrequency,
    
    /// Impact domains involved
    pub domains: Vec<String>,
    
    /// Effectiveness of this pattern
    pub effectiveness: f64,
}

/// Frequency of engagement patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternFrequency {
    /// Rarely occurs
    Rare,
    /// Occasional occurrence
    Occasional,
    /// Regular occurrence
    Regular,
    /// Frequent occurrence
    Frequent,
    /// Constant presence
    Constant,
}

/// Validation status of transformation story
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationStatus {
    /// Story is a draft
    Draft,
    /// Under community review
    UnderReview,
    /// Community validated
    Validated,
    /// Featured transformation story
    Featured,
}

/// Story collection system
pub struct StoryCollectionSystem {
    /// Storage for transformation stories
    stories: HashMap<Uuid, TransformationStory>,
    
    /// Templates for story creation
    templates: Vec<StoryTemplate>,
    
    /// Validation workflows
    validation_workflows: Vec<ValidationWorkflow>,
}

impl StoryCollectionSystem {
    /// Create a new story collection system
    pub fn new() -> Self {
        Self {
            stories: HashMap::new(),
            templates: Self::create_default_templates(),
            validation_workflows: Self::create_default_workflows(),
        }
    }

    /// Initialize the story collection system
    pub fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Initialize with default templates and workflows
        Ok(())
    }

    /// Create a new transformation story
    pub fn create_story(
        &mut self,
        community_id: String,
        title: String,
        narrative: TransformationNarrative,
        author: StoryAuthor,
    ) -> Uuid {
        let story = TransformationStory {
            id: Uuid::new_v4(),
            community_id,
            title,
            narrative,
            author,
            timeline: Vec::new(),
            domain_connections: Vec::new(),
            emotional_journey: Vec::new(),
            insights: Vec::new(),
            related_stories: Vec::new(),
            validation_status: ValidationStatus::Draft,
            tags: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let story_id = story.id;
        self.stories.insert(story_id, story);
        story_id
    }

    /// Get a transformation story by ID
    pub fn get_story(&self, story_id: Uuid) -> Option<&TransformationStory> {
        self.stories.get(&story_id)
    }

    /// Get all stories for a community
    pub fn get_community_stories(&self, community_id: &str) -> Vec<&TransformationStory> {
        self.stories
            .values()
            .filter(|story| story.community_id == community_id)
            .collect()
    }

    /// Add a transformation event to a story
    pub fn add_transformation_event(
        &mut self,
        story_id: Uuid,
        event: TransformationEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(story) = self.stories.get_mut(&story_id) {
            story.timeline.push(event);
            story.updated_at = Utc::now();
            Ok(())
        } else {
            Err("Story not found".into())
        }
    }

    /// Add a domain connection to a story
    pub fn add_domain_connection(
        &mut self,
        story_id: Uuid,
        connection: DomainConnection,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(story) = self.stories.get_mut(&story_id) {
            story.domain_connections.push(connection);
            story.updated_at = Utc::now();
            Ok(())
        } else {
            Err("Story not found".into())
        }
    }

    /// Submit story for validation
    pub fn submit_for_validation(&mut self, story_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(story) = self.stories.get_mut(&story_id) {
            story.validation_status = ValidationStatus::UnderReview;
            story.updated_at = Utc::now();
            Ok(())
        } else {
            Err("Story not found".into())
        }
    }

    /// Validate a transformation story
    pub fn validate_story(&mut self, story_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(story) = self.stories.get_mut(&story_id) {
            story.validation_status = ValidationStatus::Validated;
            story.updated_at = Utc::now();
            Ok(())
        } else {
            Err("Story not found".into())
        }
    }

    /// Feature a transformation story
    pub fn feature_story(&mut self, story_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(story) = self.stories.get_mut(&story_id) {
            story.validation_status = ValidationStatus::Featured;
            story.updated_at = Utc::now();
            Ok(())
        } else {
            Err("Story not found".into())
        }
    }

    /// Create default story templates
    fn create_default_templates() -> Vec<StoryTemplate> {
        vec![
            StoryTemplate {
                id: Uuid::new_v4(),
                name: "Basic Transformation Story".to_string(),
                description: "Template for documenting basic community transformation".to_string(),
                sections: vec![
                    "Before State".to_string(),
                    "Catalyst Event".to_string(),
                    "Transformation Process".to_string(),
                    "After State".to_string(),
                    "Key Learnings".to_string(),
                ],
            },
            StoryTemplate {
                id: Uuid::new_v4(),
                name: "Deep Dive Transformation".to_string(),
                description: "Comprehensive template for detailed transformation analysis".to_string(),
                sections: vec![
                    "Community Context".to_string(),
                    "Initial Understanding".to_string(),
                    "Catalyst and Trigger".to_string(),
                    "Transformation Journey".to_string(),
                    "Domain Connections Discovered".to_string(),
                    "Emotional Journey".to_string(),
                    "Challenges and Solutions".to_string(),
                    "Key Insights".to_string(),
                    "Current State".to_string(),
                    "Future Implications".to_string(),
                ],
            },
        ]
    }

    /// Create default validation workflows
    fn create_default_workflows() -> Vec<ValidationWorkflow> {
        vec![
            ValidationWorkflow {
                id: Uuid::new_v4(),
                name: "Community Review".to_string(),
                description: "Basic community review process".to_string(),
                steps: vec![
                    ValidationStep {
                        name: "Initial Review".to_string(),
                        description: "Initial review by community moderators".to_string(),
                        required: true,
                    },
                    ValidationStep {
                        name: "Community Feedback".to_string(),
                        description: "Open feedback period for all community members".to_string(),
                        required: true,
                    },
                    ValidationStep {
                        name: "Final Approval".to_string(),
                        description: "Final approval by community leadership".to_string(),
                        required: true,
                    },
                ],
            },
        ]
    }
}

/// Story template for guiding story creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryTemplate {
    /// Template identifier
    pub id: Uuid,
    
    /// Template name
    pub name: String,
    
    /// Template description
    pub description: String,
    
    /// Required sections in the story
    pub sections: Vec<String>,
}

/// Validation workflow for transformation stories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationWorkflow {
    /// Workflow identifier
    pub id: Uuid,
    
    /// Workflow name
    pub name: String,
    
    /// Workflow description
    pub description: String,
    
    /// Steps in the validation process
    pub steps: Vec<ValidationStep>,
}

/// Individual step in validation workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationStep {
    /// Step name
    pub name: String,
    
    /// Step description
    pub description: String,
    
    /// Whether this step is required
    pub required: bool,
}