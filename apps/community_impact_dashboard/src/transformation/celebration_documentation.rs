//! Celebration Documentation System
//!
//! This module provides tools for documenting community celebration moments,
//! highlighting transformation milestones, and sharing celebration practices.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::transformation::milestone_recognition::{TransformationMilestone, CelebrationType};

/// Celebration documentation system
pub struct CelebrationDocumentation {
    /// Stored celebration records
    celebrations: HashMap<Uuid, CelebrationRecord>,
    
    /// Celebration practices
    celebration_practices: Vec<CelebrationPractice>,
    
    /// Cultural celebration traditions
    cultural_traditions: Vec<CulturalTradition>,
    
    /// Celebration templates
    celebration_templates: Vec<CelebrationTemplate>,
}

impl CelebrationDocumentation {
    /// Create a new celebration documentation system
    pub fn new() -> Self {
        Self {
            celebrations: HashMap::new(),
            celebration_practices: Vec::new(),
            cultural_traditions: Vec::new(),
            celebration_templates: Self::create_default_templates(),
        }
    }

    /// Initialize the celebration documentation system
    pub fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Initialize with default templates
        Ok(())
    }

    /// Create a new celebration record
    pub fn create_celebration(
        &mut self,
        community_id: String,
        title: String,
        celebration_type: CelebrationType,
        description: String,
    ) -> Uuid {
        let celebration = CelebrationRecord {
            id: Uuid::new_v4(),
            community_id,
            title,
            celebration_type,
            description,
            celebration_date: Utc::now(),
            participants: CelebrationParticipants::default(),
            activities: Vec::new(),
            emotions: Vec::new(),
            outcomes: Vec::new(),
            documentation: CelebrationDocumentationContent::default(),
            related_milestones: Vec::new(),
            cultural_significance: None,
            shared_elements: Vec::new(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
            sharing_status: SharingStatus::Internal,
        };
        
        let celebration_id = celebration.id;
        self.celebrations.insert(celebration_id, celebration);
        celebration_id
    }

    /// Get a celebration record by ID
    pub fn get_celebration(&self, celebration_id: Uuid) -> Option<&CelebrationRecord> {
        self.celebrations.get(&celebration_id)
    }

    /// Get all celebrations for a community
    pub fn get_community_celebrations(&self, community_id: &str) -> Vec<&CelebrationRecord> {
        self.celebrations
            .values()
            .filter(|celebration| celebration.community_id == community_id)
            .collect()
    }

    /// Add an activity to a celebration
    pub fn add_activity(
        &mut self,
        celebration_id: Uuid,
        activity: CelebrationActivity,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(celebration) = self.celebrations.get_mut(&celebration_id) {
            celebration.activities.push(activity);
            celebration.last_updated = Utc::now();
            Ok(())
        } else {
            Err("Celebration not found".into())
        }
    }

    /// Add an emotion experienced during celebration
    pub fn add_emotion(
        &mut self,
        celebration_id: Uuid,
        emotion: CelebrationEmotion,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(celebration) = self.celebrations.get_mut(&celebration_id) {
            celebration.emotions.push(emotion);
            celebration.last_updated = Utc::now();
            Ok(())
        } else {
            Err("Celebration not found".into())
        }
    }

    /// Add an outcome from the celebration
    pub fn add_outcome(
        &mut self,
        celebration_id: Uuid,
        outcome: CelebrationOutcome,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(celebration) = self.celebrations.get_mut(&celebration_id) {
            celebration.outcomes.push(outcome);
            celebration.last_updated = Utc::now();
            Ok(())
        } else {
            Err("Celebration not found".into())
        }
    }

    /// Link a celebration to a milestone
    pub fn link_to_milestone(
        &mut self,
        celebration_id: Uuid,
        milestone_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(celebration) = self.celebrations.get_mut(&celebration_id) {
            celebration.related_milestones.push(milestone_id);
            celebration.last_updated = Utc::now();
            Ok(())
        } else {
            Err("Celebration not found".into())
        }
    }

    /// Add cultural significance to a celebration
    pub fn add_cultural_significance(
        &mut self,
        celebration_id: Uuid,
        significance: CulturalSignificance,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(celebration) = self.celebrations.get_mut(&celebration_id) {
            celebration.cultural_significance = Some(significance);
            celebration.last_updated = Utc::now();
            Ok(())
        } else {
            Err("Celebration not found".into())
        }
    }

    /// Add shared element from celebration
    pub fn add_shared_element(
        &mut self,
        celebration_id: Uuid,
        element: SharedElement,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(celebration) = self.celebrations.get_mut(&celebration_id) {
            celebration.shared_elements.push(element);
            celebration.last_updated = Utc::now();
            Ok(())
        } else {
            Err("Celebration not found".into())
        }
    }

    /// Share a celebration with other communities
    pub fn share_celebration(&mut self, celebration_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(celebration) = self.celebrations.get_mut(&celebration_id) {
            celebration.sharing_status = SharingStatus::Shared;
            celebration.last_updated = Utc::now();
            Ok(())
        } else {
            Err("Celebration not found".into())
        }
    }

    /// Feature a celebration
    pub fn feature_celebration(&mut self, celebration_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(celebration) = self.celebrations.get_mut(&celebration_id) {
            celebration.sharing_status = SharingStatus::Featured;
            celebration.last_updated = Utc::now();
            Ok(())
        } else {
            Err("Celebration not found".into())
        }
    }

    /// Create a celebration practice from a celebration
    pub fn create_practice(&mut self, celebration_id: Uuid) -> Result<Uuid, Box<dyn std::error::Error>> {
        if let Some(celebration) = self.celebrations.get(&celebration_id) {
            let practice = CelebrationPractice {
                id: Uuid::new_v4(),
                name: celebration.title.clone(),
                description: celebration.description.clone(),
                celebration_type: celebration.celebration_type.clone(),
                source_community: celebration.community_id.clone(),
                activities: celebration.activities.clone(),
                required_resources: celebration.activities.iter()
                    .flat_map(|a| a.resources_needed.clone())
                    .collect(),
                time_requirements: celebration.activities.iter()
                    .map(|a| a.time_required.clone())
                    .collect(),
                participant_guidelines: celebration.participants.get_guidelines(),
                cultural_considerations: celebration.cultural_significance.clone()
                    .map(|s| s.considerations)
                    .unwrap_or_default(),
                effectiveness_rating: 0.0, // To be updated based on feedback
                communities_using: Vec::new(),
                created_from: celebration_id,
                created_at: Utc::now(),
            };
            
            let practice_id = practice.id;
            self.celebration_practices.push(practice);
            Ok(practice_id)
        } else {
            Err("Celebration not found".into())
        }
    }

    /// Get celebrations by type
    pub fn get_celebrations_by_type(&self, celebration_type: &CelebrationType) -> Vec<&CelebrationRecord> {
        self.celebrations
            .values()
            .filter(|celebration| &celebration.celebration_type == celebration_type)
            .collect()
    }

    /// Get recent celebrations (within last 30 days)
    pub fn get_recent_celebrations(&self) -> Vec<&CelebrationRecord> {
        let thirty_days_ago = Utc::now() - chrono::Duration::days(30);
        
        self.celebrations
            .values()
            .filter(|celebration| celebration.celebration_date > thirty_days_ago)
            .collect()
    }

    /// Get featured celebrations
    pub fn get_featured_celebrations(&self) -> Vec<&CelebrationRecord> {
        self.celebrations
            .values()
            .filter(|celebration| celebration.sharing_status == SharingStatus::Featured)
            .collect()
    }

    /// Get shared celebration practices
    pub fn get_shared_practices(&self) -> Vec<&CelebrationPractice> {
        self.celebration_practices
            .iter()
            .filter(|practice| !practice.communities_using.is_empty())
            .collect()
    }

    /// Create default celebration templates
    fn create_default_templates() -> Vec<CelebrationTemplate> {
        vec![
            CelebrationTemplate {
                id: Uuid::new_v4(),
                name: "Milestone Recognition Ceremony".to_string(),
                description: "Formal ceremony to recognize transformation milestones".to_string(),
                celebration_type: CelebrationType::CommunityMeeting,
                suggested_activities: vec![
                    "Opening remarks and context".to_string(),
                    "Milestone story sharing".to_string(),
                    "Recognition of contributors".to_string(),
                    "Community reflection".to_string(),
                    "Looking forward together".to_string(),
                ],
                suggested_structure: CelebrationStructure {
                    opening: "Welcome and purpose setting".to_string(),
                    main_activities: vec![
                        "Storytelling about the milestone".to_string(),
                        "Recognition of contributors".to_string(),
                        "Group reflection on impact".to_string(),
                    ],
                    closing: "Commitment to continued journey".to_string(),
                },
                estimated_duration: "60-90 minutes".to_string(),
                participant_range: "5-50 people".to_string(),
            },
            CelebrationTemplate {
                id: Uuid::new_v4(),
                name: "Virtual Celebration Gathering".to_string(),
                description: "Online celebration for distributed communities".to_string(),
                celebration_type: CelebrationType::VirtualCelebration,
                suggested_activities: vec![
                    "Virtual welcome and icebreaker".to_string(),
                    "Shared digital milestone visualization".to_string(),
                    "Breakout room discussions".to_string(),
                    "Digital toast or ritual".to_string(),
                    "Photo sharing and memories".to_string(),
                ],
                suggested_structure: CelebrationStructure {
                    opening: "Virtual welcome and technical check".to_string(),
                    main_activities: vec![
                        "Screen share milestone achievements".to_string(),
                        "Small group discussions in breakout rooms".to_string(),
                        "Return to main room for sharing".to_string(),
                    ],
                    closing: "Group photo and closing remarks".to_string(),
                },
                estimated_duration: "45-60 minutes".to_string(),
                participant_range: "10-100 people".to_string(),
            },
            CelebrationTemplate {
                id: Uuid::new_v4(),
                name: "Documentation and Story Sharing".to_string(),
                description: "Create lasting documentation of the celebration".to_string(),
                celebration_type: CelebrationType::Documentation,
                suggested_activities: vec![
                    "Collect stories and testimonials".to_string(),
                    "Create visual documentation".to_string(),
                    "Write reflection pieces".to_string(),
                    "Share with wider community".to_string(),
                    "Archive for future reference".to_string(),
                ],
                suggested_structure: CelebrationStructure {
                    opening: "Introduction to documentation process".to_string(),
                    main_activities: vec![
                        "Story collection and recording".to_string(),
                        "Visual documentation creation".to_string(),
                        "Review and refinement together".to_string(),
                    ],
                    closing: "Plan for sharing and archiving".to_string(),
                },
                estimated_duration: "2-4 hours (can be split)".to_string(),
                participant_range: "3-20 people".to_string(),
            },
        ]
    }
}

/// Celebration record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CelebrationRecord {
    /// Celebration identifier
    pub id: Uuid,
    
    /// Community identifier
    pub community_id: String,
    
    /// Celebration title
    pub title: String,
    
    /// Type of celebration
    pub celebration_type: CelebrationType,
    
    /// Celebration description
    pub description: String,
    
    /// When the celebration occurred
    pub celebration_date: DateTime<Utc>,
    
    /// Participants in the celebration
    pub participants: CelebrationParticipants,
    
    /// Activities during the celebration
    pub activities: Vec<CelebrationActivity>,
    
    /// Emotions experienced during celebration
    pub emotions: Vec<CelebrationEmotion>,
    
    /// Outcomes from the celebration
    pub outcomes: Vec<CelebrationOutcome>,
    
    /// Documentation content
    pub documentation: CelebrationDocumentationContent,
    
    /// Related transformation milestones
    pub related_milestones: Vec<Uuid>,
    
    /// Cultural significance of this celebration
    pub cultural_significance: Option<CulturalSignificance>,
    
    /// Elements that can be shared with other communities
    pub shared_elements: Vec<SharedElement>,
    
    /// When this record was created
    pub created_at: DateTime<Utc>,
    
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
    
    /// Sharing status with other communities
    pub sharing_status: SharingStatus,
}

/// Celebration participants information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CelebrationParticipants {
    /// Total number of participants
    pub total_count: usize,
    
    /// Participant roles
    pub roles: Vec<ParticipantRole>,
    
    /// Demographic information (optional)
    pub demographics: Option<ParticipantDemographics>,
    
    /// Engagement levels
    pub engagement_levels: Vec<EngagementLevel>,
}

impl Default for CelebrationParticipants {
    fn default() -> Self {
        Self {
            total_count: 0,
            roles: Vec::new(),
            demographics: None,
            engagement_levels: Vec::new(),
        }
    }
}

impl CelebrationParticipants {
    /// Get participant guidelines based on roles and engagement
    fn get_guidelines(&self) -> Vec<String> {
        let mut guidelines = Vec::new();
        
        for role in &self.roles {
            guidelines.push(format!("Include participants in {} roles", role.role_type));
        }
        
        for engagement in &self.engagement_levels {
            match engagement.level {
                ParticipationLevel::High => {
                    guidelines.push("Provide opportunities for active participation".to_string());
                },
                ParticipationLevel::Medium => {
                    guidelines.push("Balance passive and active participation".to_string());
                },
                ParticipationLevel::Low => {
                    guidelines.push("Create space for observation and gradual engagement".to_string());
                },
            }
        }
        
        guidelines
    }
}

/// Participant role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantRole {
    /// Role type
    pub role_type: String,
    
    /// Number of participants in this role
    pub count: usize,
    
    /// Responsibilities in this role
    pub responsibilities: Vec<String>,
}

/// Participant demographics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantDemographics {
    /// Age ranges
    pub age_ranges: Vec<AgeRange>,
    
    /// Gender representation
    pub gender_representation: Vec<String>,
    
    /// Cultural backgrounds
    pub cultural_backgrounds: Vec<String>,
    
    /// Language preferences
    pub language_preferences: Vec<String>,
}

/// Age range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgeRange {
    /// Range description (e.g., "18-25", "26-35")
    pub range: String,
    
    /// Count in this range
    pub count: usize,
}

/// Engagement level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngagementLevel {
    /// Level of participation
    pub level: ParticipationLevel,
    
    /// Description of engagement
    pub description: String,
    
    /// Number of participants at this level
    pub count: usize,
}

/// Participation levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParticipationLevel {
    Low,
    Medium,
    High,
}

/// Celebration activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CelebrationActivity {
    /// Activity name
    pub name: String,
    
    /// Activity description
    pub description: String,
    
    /// Duration of the activity
    pub duration: String,
    
    /// Resources needed for this activity
    pub resources_needed: Vec<String>,
    
    /// Facilitator requirements
    pub facilitator_requirements: Vec<String>,
    
    /// Participant requirements
    pub participant_requirements: Vec<String>,
    
    /// Time required
    pub time_required: String,
    
    /// Activity outcomes
    pub outcomes: Vec<String>,
}

/// Celebration emotion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CelebrationEmotion {
    /// Emotion type
    pub emotion_type: EmotionType,
    
    /// Intensity of the emotion (0.0 to 1.0)
    pub intensity: f64,
    
    /// Description of the emotional experience
    pub description: String,
    
    /// When this emotion was prominent
    pub timestamp: DateTime<Utc>,
    
    /// Who experienced this emotion
    pub experienced_by: Vec<String>,
}

/// Emotion types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmotionType {
    Joy,
    Gratitude,
    Pride,
    Hope,
    Inspiration,
    Connection,
    Accomplishment,
    Reflection,
    Other(String),
}

/// Celebration outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CelebrationOutcome {
    /// Outcome description
    pub description: String,
    
    /// Type of outcome
    pub outcome_type: OutcomeType,
    
    /// When this outcome was observed
    pub timestamp: DateTime<Utc>,
    
    /// Evidence supporting this outcome
    pub evidence: Vec<String>,
    
    /// Long-term significance
    pub long_term_significance: Option<String>,
}

/// Outcome types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutcomeType {
    CommunityBonding,
    Motivational,
    Learning,
    Transformative,
    Practical,
    Other(String),
}

/// Celebration documentation content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CelebrationDocumentationContent {
    /// Visual documentation (photos, videos)
    pub visual_documentation: Vec<VisualDocumentation>,
    
    /// Written documentation
    pub written_documentation: Vec<WrittenDocumentation>,
    
    /// Audio documentation
    pub audio_documentation: Vec<AudioDocumentation>,
    
    /// Participant testimonials
    pub testimonials: Vec<Testimonial>,
    
    /// Key moments captured
    pub key_moments: Vec<KeyMoment>,
}

impl Default for CelebrationDocumentationContent {
    fn default() -> Self {
        Self {
            visual_documentation: Vec::new(),
            written_documentation: Vec::new(),
            audio_documentation: Vec::new(),
            testimonials: Vec::new(),
            key_moments: Vec::new(),
        }
    }
}

/// Visual documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualDocumentation {
    /// Documentation title
    pub title: String,
    
    /// Type of visual documentation
    pub doc_type: VisualDocType,
    
    /// Description of what's shown
    pub description: String,
    
    /// File path or reference
    pub file_reference: String,
    
    /// Timestamp when captured
    pub captured_at: DateTime<Utc>,
}

/// Visual documentation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VisualDocType {
    Photo,
    Video,
    Diagram,
    Infographic,
    Screenshot,
    Other(String),
}

/// Written documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WrittenDocumentation {
    /// Document title
    pub title: String,
    
    /// Document content
    pub content: String,
    
    /// Author of the document
    pub author: String,
    
    /// Type of document
    pub doc_type: WrittenDocType,
    
    /// When written
    pub written_at: DateTime<Utc>,
}

/// Written documentation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WrittenDocType {
    Reflection,
    Summary,
    Story,
    Poem,
    Song,
    Other(String),
}

/// Audio documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDocumentation {
    /// Recording title
    pub title: String,
    
    /// Description of content
    pub description: String,
    
    /// File path or reference
    pub file_reference: String,
    
    /// Duration of recording
    pub duration: String,
    
    /// When recorded
    pub recorded_at: DateTime<Utc>,
}

/// Participant testimonial
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Testimonial {
    /// Person giving testimonial
    pub person: String,
    
    /// Their role in community
    pub role: String,
    
    /// Testimonial content
    pub content: String,
    
    /// When given
    pub timestamp: DateTime<Utc>,
}

/// Key moment captured
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMoment {
    /// Moment title
    pub title: String,
    
    /// Moment description
    pub description: String,
    
    /// Significance of this moment
    pub significance: String,
    
    /// When this moment occurred
    pub timestamp: DateTime<Utc>,
}

/// Cultural significance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalSignificance {
    /// Cultural context
    pub cultural_context: String,
    
    /// Cultural traditions incorporated
    pub traditions_incorporated: Vec<String>,
    
    /// Cultural considerations
    pub considerations: Vec<String>,
    
    /// Cultural sensitivities respected
    pub sensitivities_respected: Vec<String>,
}

/// Shared element that can be used by other communities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedElement {
    /// Element name
    pub name: String,
    
    /// Element description
    pub description: String,
    
    /// Type of element
    pub element_type: SharedElementType,
    
    /// How to adapt this element
    pub adaptation_guidelines: Vec<String>,
    
    /// Success factors for this element
    pub success_factors: Vec<String>,
    
    /// Potential challenges
    pub potential_challenges: Vec<String>,
}

/// Shared element types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SharedElementType {
    Activity,
    Ritual,
    Practice,
    Symbol,
    Song,
    Story,
    Other(String),
}

/// Sharing status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SharingStatus {
    Internal,
    Shared,
    Featured,
}

/// Celebration practice that can be shared
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CelebrationPractice {
    /// Practice identifier
    pub id: Uuid,
    
    /// Practice name
    pub name: String,
    
    /// Practice description
    pub description: String,
    
    /// Type of celebration this practice supports
    pub celebration_type: CelebrationType,
    
    /// Community that originated this practice
    pub source_community: String,
    
    /// Activities in this practice
    pub activities: Vec<CelebrationActivity>,
    
    /// Resources required for this practice
    pub required_resources: Vec<String>,
    
    /// Time requirements
    pub time_requirements: Vec<String>,
    
    /// Participant guidelines
    pub participant_guidelines: Vec<String>,
    
    /// Cultural considerations
    pub cultural_considerations: Vec<String>,
    
    /// Effectiveness rating (0.0 to 1.0)
    pub effectiveness_rating: f64,
    
    /// Communities using this practice
    pub communities_using: Vec<String>,
    
    /// Celebration this practice was created from
    pub created_from: Uuid,
    
    /// When this practice was created
    pub created_at: DateTime<Utc>,
}

/// Cultural tradition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalTradition {
    /// Tradition identifier
    pub id: Uuid,
    
    /// Tradition name
    pub name: String,
    
    /// Cultural origin
    pub cultural_origin: String,
    
    /// Tradition description
    pub description: String,
    
    /// How this tradition can be adapted for celebrations
    pub adaptation_suggestions: Vec<String>,
    
    /// Important cultural considerations
    pub cultural_considerations: Vec<String>,
    
    /// Communities that have adapted this tradition
    pub adapting_communities: Vec<String>,
}

/// Celebration template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CelebrationTemplate {
    /// Template identifier
    pub id: Uuid,
    
    /// Template name
    pub name: String,
    
    /// Template description
    pub description: String,
    
    /// Type of celebration this template creates
    pub celebration_type: CelebrationType,
    
    /// Suggested activities
    pub suggested_activities: Vec<String>,
    
    /// Suggested celebration structure
    pub suggested_structure: CelebrationStructure,
    
    /// Estimated duration
    pub estimated_duration: String,
    
    /// Suggested participant range
    pub participant_range: String,
}

/// Celebration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CelebrationStructure {
    /// Opening activities
    pub opening: String,
    
    /// Main activities
    pub main_activities: Vec<String>,
    
    /// Closing activities
    pub closing: String,
}