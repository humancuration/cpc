//! Community Learning Exchange System
//!
//! This module provides tools for facilitating learning between communities,
//! sharing transformation insights, and building a collective knowledge base.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::transformation::story_collection::TransformationStory;
use crate::transformation::adaptation_tracking::CommunityAdaptation;
use crate::transformation::milestone_recognition::TransformationMilestone;

/// Community learning exchange system
pub struct LearningExchange {
    /// Learning opportunities
    learning_opportunities: HashMap<Uuid, LearningOpportunity>,
    
    /// Community learning profiles
    community_profiles: HashMap<String, CommunityLearningProfile>,
    
    /// Learning connections between communities
    learning_connections: Vec<LearningConnection>,
    
    /// Knowledge base of transformation insights
    knowledge_base: TransformationKnowledgeBase,
}

impl LearningExchange {
    /// Create a new learning exchange system
    pub fn new() -> Self {
        Self {
            learning_opportunities: HashMap::new(),
            community_profiles: HashMap::new(),
            learning_connections: Vec::new(),
            knowledge_base: TransformationKnowledgeBase::new(),
        }
    }

    /// Initialize the learning exchange system
    pub fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Initialize with some default knowledge categories
        self.knowledge_base.initialize()?;
        Ok(())
    }

    /// Create a community learning profile
    pub fn create_community_profile(&mut self, community_id: String, community_name: String) -> Uuid {
        let profile = CommunityLearningProfile {
            id: Uuid::new_v4(),
            community_id,
            community_name,
            learning_interests: Vec::new(),
            expertise_areas: Vec::new(),
            transformation_phase: TransformationPhase::Exploring,
            learning_preferences: LearningPreferences::default(),
            participation_history: Vec::new(),
            contributions: Vec::new(),
            connection_requests: Vec::new(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
        };
        
        let profile_id = profile.id;
        self.community_profiles.insert(profile.community_id.clone(), profile);
        profile_id
    }

    /// Get a community learning profile
    pub fn get_community_profile(&self, community_id: &str) -> Option<&CommunityLearningProfile> {
        self.community_profiles.get(community_id)
    }

    /// Update community learning interests
    pub fn update_learning_interests(
        &mut self,
        community_id: &str,
        interests: Vec<LearningInterest>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(profile) = self.community_profiles.get_mut(community_id) {
            profile.learning_interests = interests;
            profile.last_updated = Utc::now();
            Ok(())
        } else {
            Err("Community profile not found".into())
        }
    }

    /// Update community expertise areas
    pub fn update_expertise_areas(
        &mut self,
        community_id: &str,
        expertise_areas: Vec<ExpertiseArea>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(profile) = self.community_profiles.get_mut(community_id) {
            profile.expertise_areas = expertise_areas;
            profile.last_updated = Utc::now();
            Ok(())
        } else {
            Err("Community profile not found".into())
        }
    }

    /// Update transformation phase
    pub fn update_transformation_phase(
        &mut self,
        community_id: &str,
        phase: TransformationPhase,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(profile) = self.community_profiles.get_mut(community_id) {
            profile.transformation_phase = phase;
            profile.last_updated = Utc::now();
            Ok(())
        } else {
            Err("Community profile not found".into())
        }
    }

    /// Create a learning opportunity
    pub fn create_learning_opportunity(
        &mut self,
        host_community_id: String,
        title: String,
        description: String,
        opportunity_type: LearningOpportunityType,
        format: LearningFormat,
    ) -> Uuid {
        let opportunity = LearningOpportunity {
            id: Uuid::new_v4(),
            host_community_id,
            title,
            description,
            opportunity_type,
            format,
            topics: Vec::new(),
            target_audience: Vec::new(),
            prerequisites: Vec::new(),
            schedule: None,
            capacity: None,
            participants: Vec::new(),
            materials: Vec::new(),
            learning_objectives: Vec::new(),
            outcomes: Vec::new(),
            status: OpportunityStatus::Draft,
            created_at: Utc::now(),
            last_updated: Utc::now(),
        };
        
        let opportunity_id = opportunity.id;
        self.learning_opportunities.insert(opportunity_id, opportunity);
        opportunity_id
    }

    /// Get a learning opportunity by ID
    pub fn get_learning_opportunity(&self, opportunity_id: Uuid) -> Option<&LearningOpportunity> {
        self.learning_opportunities.get(&opportunity_id)
    }

    /// Find learning opportunities for a community
    pub fn find_learning_opportunities(&self, community_id: &str) -> Vec<&LearningOpportunity> {
        if let Some(profile) = self.community_profiles.get(community_id) {
            self.learning_opportunities
                .values()
                .filter(|opportunity| {
                    // Filter based on community's interests and phase
                    opportunity.topics.iter().any(|topic| {
                        profile.learning_interests.iter().any(|interest| {
                            interest.topic == *topic || 
                            interest.related_topics.contains(&topic)
                        })
                    }) ||
                    opportunity.status == OpportunityStatus::Open
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Register for a learning opportunity
    pub fn register_for_opportunity(
        &mut self,
        opportunity_id: Uuid,
        participant_community_id: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(opportunity) = self.learning_opportunities.get_mut(&opportunity_id) {
            if opportunity.status != OpportunityStatus::Open {
                return Err("Opportunity is not open for registration".into());
            }
            
            if let Some(capacity) = opportunity.capacity {
                if opportunity.participants.len() >= capacity {
                    return Err("Opportunity has reached capacity".into());
                }
            }
            
            if !opportunity.participants.contains(&participant_community_id) {
                opportunity.participants.push(participant_community_id.clone());
                
                // Update community participation history
                if let Some(profile) = self.community_profiles.get_mut(&participant_community_id) {
                    profile.participation_history.push(ParticipationRecord {
                        opportunity_id,
                        participation_date: Utc::now(),
                        role: ParticipantRole::Learner,
                        feedback: None,
                        outcomes: Vec::new(),
                    });
                    profile.last_updated = Utc::now();
                }
                
                opportunity.last_updated = Utc::now();
                Ok(())
            } else {
                Err("Community already registered for this opportunity".into())
            }
        } else {
            Err("Learning opportunity not found".into())
        }
    }

    /// Create a learning connection between communities
    pub fn create_learning_connection(
        &mut self,
        requesting_community_id: String,
        target_community_id: String,
        purpose: String,
        connection_type: ConnectionType,
    ) -> Uuid {
        let connection = LearningConnection {
            id: Uuid::new_v4(),
            requesting_community_id,
            target_community_id,
            purpose,
            connection_type,
            status: ConnectionStatus::Pending,
            topics: Vec::new(),
            activities: Vec::new(),
            outcomes: Vec::new(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
        };
        
        let connection_id = connection.id;
        self.learning_connections.push(connection);
        
        // Add to community's connection requests
        if let Some(profile) = self.community_profiles.get_mut(&connection.requesting_community_id) {
            profile.connection_requests.push(connection_id);
            profile.last_updated = Utc::now();
        }
        
        connection_id
    }

    /// Accept a learning connection request
    pub fn accept_connection(&mut self, connection_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(connection) = self.learning_connections.iter_mut().find(|c| c.id == connection_id) {
            connection.status = ConnectionStatus::Active;
            connection.last_updated = Utc::now();
            Ok(())
        } else {
            Err("Connection not found".into())
        }
    }

    /// Reject a learning connection request
    pub fn reject_connection(&mut self, connection_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(connection) = self.learning_connections.iter_mut().find(|c| c.id == connection_id) {
            connection.status = ConnectionStatus::Rejected;
            connection.last_updated = Utc::now();
            Ok(())
        } else {
            Err("Connection not found".into())
        }
    }

    /// Share a transformation story for learning
    pub fn share_transformation_story(
        &mut self,
        story: TransformationStory,
    ) -> Result<Uuid, Box<dyn std::error::Error>> {
        let knowledge_id = self.knowledge_base.add_knowledge_item(
            story.title.clone(),
            story.narrative.challenge.clone(),
            vec![KnowledgeCategory::CommunityStory],
            story.community_id.clone(),
        )?;
        
        // Link story to community's contributions
        if let Some(profile) = self.community_profiles.get_mut(&story.community_id) {
            profile.contributions.push(Contribution {
                id: Uuid::new_v4(),
                contribution_type: ContributionType::TransformationStory,
                knowledge_id,
                title: story.title,
                description: story.narrative.challenge.clone(),
                created_at: Utc::now(),
            });
            profile.last_updated = Utc::now();
        }
        
        Ok(knowledge_id)
    }

    /// Share a community adaptation for learning
    pub fn share_adaptation(
        &mut self,
        adaptation: CommunityAdaptation,
    ) -> Result<Uuid, Box<dyn std::error::Error>> {
        let knowledge_id = self.knowledge_base.add_knowledge_item(
            adaptation.name.clone(),
            adaptation.description.clone(),
            vec![KnowledgeCategory::CommunityAdaptation],
            adaptation.community_id.clone(),
        )?;
        
        // Link adaptation to community's contributions
        if let Some(profile) = self.community_profiles.get_mut(&adaptation.community_id) {
            profile.contributions.push(Contribution {
                id: Uuid::new_v4(),
                contribution_type: ContributionType::CommunityAdaptation,
                knowledge_id,
                title: adaptation.name,
                description: adaptation.description,
                created_at: Utc::now(),
            });
            profile.last_updated = Utc::now();
        }
        
        Ok(knowledge_id)
    }

    /// Share a transformation milestone for learning
    pub fn share_milestone(
        &mut self,
        milestone: TransformationMilestone,
    ) -> Result<Uuid, Box<dyn std::error::Error>> {
        let knowledge_id = self.knowledge_base.add_knowledge_item(
            milestone.title.clone(),
            milestone.description.clone(),
            vec![KnowledgeCategory::MilestoneAchievement],
            milestone.community_id.clone(),
        )?;
        
        // Link milestone to community's contributions
        if let Some(profile) = self.community_profiles.get_mut(&milestone.community_id) {
            profile.contributions.push(Contribution {
                id: Uuid::new_v4(),
                contribution_type: ContributionType::MilestoneAchievement,
                knowledge_id,
                title: milestone.title,
                description: milestone.description,
                created_at: Utc::now(),
            });
            profile.last_updated = Utc::now();
        }
        
        Ok(knowledge_id)
    }

    /// Search for knowledge items
    pub fn search_knowledge(&self, query: &str, categories: Option<Vec<KnowledgeCategory>>) -> Vec<&KnowledgeItem> {
        self.knowledge_base.search(query, categories)
    }

    /// Get recommended learning opportunities for a community
    pub fn get_recommended_opportunities(&self, community_id: &str) -> Vec<&LearningOpportunity> {
        if let Some(profile) = self.community_profiles.get(community_id) {
            let mut recommendations = Vec::new();
            
            // Based on learning interests
            for interest in &profile.learning_interests {
                for opportunity in self.learning_opportunities.values() {
                    if opportunity.topics.contains(&interest.topic) && 
                       opportunity.status == OpportunityStatus::Open &&
                       !opportunity.participants.contains(&community_id.to_string()) {
                        recommendations.push(opportunity);
                    }
                }
            }
            
            // Based on transformation phase
            let phase_opportunities: Vec<&LearningOpportunity> = self.learning_opportunities
                .values()
                .filter(|opportunity| {
                    opportunity.target_audience.contains(&profile.transformation_phase) &&
                    opportunity.status == OpportunityStatus::Open &&
                    !opportunity.participants.contains(&community_id.to_string())
                })
                .collect();
            
            recommendations.extend(phase_opportunities);
            
            // Remove duplicates
            recommendations.sort_by(|a, b| a.created_at.cmp(&b.created_at));
            recommendations.dedup_by(|a, b| a.id == b.id);
            
            recommendations
        } else {
            Vec::new()
        }
    }

    /// Get recommended community connections for a community
    pub fn get_recommended_connections(&self, community_id: &str) -> Vec<&CommunityLearningProfile> {
        if let Some(requesting_profile) = self.community_profiles.get(community_id) {
            let mut recommendations = Vec::new();
            
            // Based on similar learning interests
            for profile in self.community_profiles.values() {
                if profile.community_id != community_id {
                    // Calculate interest overlap
                    let interest_overlap = requesting_profile.learning_interests.iter()
                        .filter(|interest| profile.expertise_areas.iter().any(|expertise| {
                            expertise.area == interest.topic || 
                            expertise.related_areas.contains(&interest.topic)
                        }))
                        .count();
                    
                    // Calculate expertise overlap
                    let expertise_overlap = requesting_profile.expertise_areas.iter()
                        .filter(|expertise| profile.learning_interests.iter().any(|interest| {
                            interest.topic == expertise.area || 
                            interest.related_topics.contains(&expertise.area)
                        }))
                        .count();
                    
                    // If there's meaningful overlap, recommend
                    if interest_overlap > 0 || expertise_overlap > 0 {
                        recommendations.push(profile);
                    }
                }
            }
            
            recommendations
        } else {
            Vec::new()
        }
    }

    /// Get active learning connections for a community
    pub fn get_active_connections(&self, community_id: &str) -> Vec<&LearningConnection> {
        self.learning_connections
            .iter()
            .filter(|connection| 
                (connection.requesting_community_id == community_id || 
                 connection.target_community_id == community_id) &&
                connection.status == ConnectionStatus::Active
            )
            .collect()
    }

    /// Get pending connection requests for a community
    pub fn get_pending_requests(&self, community_id: &str) -> Vec<&LearningConnection> {
        self.learning_connections
            .iter()
            .filter(|connection| 
                connection.target_community_id == community_id &&
                connection.status == ConnectionStatus::Pending
            )
            .collect()
    }
}

/// Community learning profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityLearningProfile {
    /// Profile identifier
    pub id: Uuid,
    
    /// Community identifier
    pub community_id: String,
    
    /// Community name
    pub community_name: String,
    
    /// Learning interests
    pub learning_interests: Vec<LearningInterest>,
    
    /// Expertise areas
    pub expertise_areas: Vec<ExpertiseArea>,
    
    /// Current transformation phase
    pub transformation_phase: TransformationPhase,
    
    /// Learning preferences
    pub learning_preferences: LearningPreferences,
    
    /// Participation history
    pub participation_history: Vec<ParticipationRecord>,
    
    /// Contributions to knowledge base
    pub contributions: Vec<Contribution>,
    
    /// Connection requests
    pub connection_requests: Vec<Uuid>,
    
    /// When this profile was created
    pub created_at: DateTime<Utc>,
    
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

/// Learning interest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningInterest {
    /// Topic of interest
    pub topic: String,
    
    /// Related topics
    pub related_topics: Vec<String>,
    
    /// Priority level
    pub priority: PriorityLevel,
    
    /// Why this is important
    pub importance: String,
}

/// Expertise area
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpertiseArea {
    /// Area of expertise
    pub area: String,
    
    /// Related areas
    pub related_areas: Vec<String>,
    
    /// Proficiency level
    pub proficiency: ProficiencyLevel,
    
    /// Experience description
    pub experience: String,
    
    /// Willingness to mentor
    pub willing_to_mentor: bool,
}

/// Priority level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PriorityLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Proficiency level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProficiencyLevel {
    Beginner,
    Developing,
    Competent,
    Proficient,
    Expert,
}

/// Learning preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPreferences {
    /// Preferred learning formats
    pub preferred_formats: Vec<LearningFormat>,
    
    /// Preferred session length
    pub preferred_session_length: String,
    
    /// Best times for learning
    pub best_times: Vec<String>,
    
    /// Group size preference
    pub group_size_preference: GroupSizePreference,
    
    /// Accessibility needs
    pub accessibility_needs: Vec<String>,
}

impl Default for LearningPreferences {
    fn default() -> Self {
        Self {
            preferred_formats: vec![LearningFormat::Workshop, LearningFormat::Discussion],
            preferred_session_length: "60-90 minutes".to_string(),
            best_times: vec!["Weekdays after 5pm".to_string(), "Weekends".to_string()],
            group_size_preference: GroupSizePreference::Medium,
            accessibility_needs: Vec::new(),
        }
    }
}

/// Learning formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LearningFormat {
    Workshop,
    Discussion,
    Presentation,
    Mentorship,
    CoWorking,
    Documentation,
    Other(String),
}

/// Group size preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GroupSizePreference {
    Small,
    Medium,
    Large,
    Any,
}

/// Transformation phases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransformationPhase {
    Exploring,
    Planning,
    Implementing,
    Refining,
    Sustaining,
    Evolving,
}

/// Participation record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipationRecord {
    /// Learning opportunity ID
    pub opportunity_id: Uuid,
    
    /// When the community participated
    pub participation_date: DateTime<Utc>,
    
    /// Role in the learning opportunity
    pub role: ParticipantRole,
    
    /// Feedback provided
    pub feedback: Option<Feedback>,
    
    /// Outcomes from participation
    pub outcomes: Vec<String>,
}

/// Participant roles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParticipantRole {
    Learner,
    Contributor,
    Facilitator,
    Mentor,
    Observer,
}

/// Feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feedback {
    /// Overall rating (1-5)
    pub overall_rating: u8,
    
    /// Content relevance rating (1-5)
    pub content_relevance: u8,
    
    /// Facilitation quality rating (1-5)
    pub facilitation_quality: u8,
    
    /// Most valuable aspects
    pub most_valuable: Vec<String>,
    
    /// Areas for improvement
    pub improvement_areas: Vec<String>,
    
    /// Additional comments
    pub comments: String,
}

/// Contribution to knowledge base
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contribution {
    /// Contribution identifier
    pub id: Uuid,
    
    /// Type of contribution
    pub contribution_type: ContributionType,
    
    /// Knowledge item ID
    pub knowledge_id: Uuid,
    
    /// Contribution title
    pub title: String,
    
    /// Contribution description
    pub description: String,
    
    /// When contributed
    pub created_at: DateTime<Utc>,
}

/// Contribution types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContributionType {
    TransformationStory,
    CommunityAdaptation,
    MilestoneAchievement,
    LearningResource,
    BestPractice,
    Other(String),
}

/// Learning opportunity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningOpportunity {
    /// Opportunity identifier
    pub id: Uuid,
    
    /// Host community ID
    pub host_community_id: String,
    
    /// Opportunity title
    pub title: String,
    
    /// Opportunity description
    pub description: String,
    
    /// Type of learning opportunity
    pub opportunity_type: LearningOpportunityType,
    
    /// Format of the learning opportunity
    pub format: LearningFormat,
    
    /// Topics covered
    pub topics: Vec<String>,
    
    /// Target audience
    pub target_audience: Vec<TransformationPhase>,
    
    /// Prerequisites
    pub prerequisites: Vec<String>,
    
    /// Schedule information
    pub schedule: Option<Schedule>,
    
    /// Capacity limit
    pub capacity: Option<usize>,
    
    /// Participant communities
    pub participants: Vec<String>,
    
    /// Learning materials
    pub materials: Vec<LearningMaterial>,
    
    /// Learning objectives
    pub learning_objectives: Vec<String>,
    
    /// Expected outcomes
    pub outcomes: Vec<String>,
    
    /// Current status
    pub status: OpportunityStatus,
    
    /// When this opportunity was created
    pub created_at: DateTime<Utc>,
    
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

/// Learning opportunity types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LearningOpportunityType {
    KnowledgeSharing,
    SkillBuilding,
    CollaborativeProject,
    PeerMentorship,
    CommunityVisit,
    CaseStudy,
    Other(String),
}

/// Schedule information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    /// Start date and time
    pub start_time: DateTime<Utc>,
    
    /// End date and time
    pub end_time: DateTime<Utc>,
    
    /// Recurrence pattern
    pub recurrence: Option<Recurrence>,
    
    /// Time zone
    pub time_zone: String,
}

/// Recurrence patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Recurrence {
    Once,
    Daily,
    Weekly,
    Monthly,
    Custom(String),
}

/// Learning material
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningMaterial {
    /// Material title
    pub title: String,
    
    /// Material description
    pub description: String,
    
    /// Material type
    pub material_type: MaterialType,
    
    /// File path or reference
    pub file_reference: String,
    
    /// When added
    pub added_at: DateTime<Utc>,
}

/// Material types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MaterialType {
    Document,
    Video,
    Image,
    Audio,
    Link,
    Exercise,
    Other(String),
}

/// Opportunity status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OpportunityStatus {
    Draft,
    Open,
    InProgress,
    Completed,
    Cancelled,
}

/// Learning connection between communities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningConnection {
    /// Connection identifier
    pub id: Uuid,
    
    /// Requesting community ID
    pub requesting_community_id: String,
    
    /// Target community ID
    pub target_community_id: String,
    
    /// Purpose of the connection
    pub purpose: String,
    
    /// Type of connection
    pub connection_type: ConnectionType,
    
    /// Current status
    pub status: ConnectionStatus,
    
    /// Topics for learning exchange
    pub topics: Vec<String>,
    
    /// Activities in the connection
    pub activities: Vec<ConnectionActivity>,
    
    /// Outcomes from the connection
    pub outcomes: Vec<String>,
    
    /// When this connection was created
    pub created_at: DateTime<Utc>,
    
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

/// Connection types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionType {
    Mentorship,
    PeerLearning,
    Collaboration,
    KnowledgeExchange,
    Other(String),
}

/// Connection status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Pending,
    Active,
    Paused,
    Completed,
    Rejected,
}

/// Connection activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionActivity {
    /// Activity description
    pub description: String,
    
    /// Date of activity
    pub date: DateTime<Utc>,
    
    /// Participants from both communities
    pub participants: Vec<String>,
    
    /// Activity outcomes
    pub outcomes: Vec<String>,
}

/// Transformation knowledge base
pub struct TransformationKnowledgeBase {
    /// Knowledge items
    knowledge_items: HashMap<Uuid, KnowledgeItem>,
    
    /// Knowledge categories
    categories: Vec<KnowledgeCategory>,
    
    /// Knowledge tags
    tags: HashMap<String, Vec<Uuid>>,
}

impl TransformationKnowledgeBase {
    /// Create a new knowledge base
    pub fn new() -> Self {
        Self {
            knowledge_items: HashMap::new(),
            categories: Vec::new(),
            tags: HashMap::new(),
        }
    }

    /// Initialize the knowledge base
    pub fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Initialize with default categories
        self.categories = vec![
            KnowledgeCategory::CommunityStory,
            KnowledgeCategory::CommunityAdaptation,
            KnowledgeCategory::MilestoneAchievement,
            KnowledgeCategory::BestPractice,
            KnowledgeCategory::LearningResource,
        ];
        Ok(())
    }

    /// Add a knowledge item
    pub fn add_knowledge_item(
        &mut self,
        title: String,
        content: String,
        categories: Vec<KnowledgeCategory>,
        source_community: String,
    ) -> Result<Uuid, Box<dyn std::error::Error>> {
        let item = KnowledgeItem {
            id: Uuid::new_v4(),
            title,
            content,
            categories,
            tags: Vec::new(),
            source_community,
            contributing_communities: Vec::new(),
            related_items: Vec::new(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
            quality_rating: 0.0,
            usefulness_ratings: Vec::new(),
            view_count: 0,
            access_level: AccessLevel::AllCommunities,
        };
        
        let item_id = item.id;
        self.knowledge_items.insert(item_id, item);
        Ok(item_id)
    }

    /// Get a knowledge item by ID
    pub fn get_knowledge_item(&self, item_id: Uuid) -> Option<&KnowledgeItem> {
        self.knowledge_items.get(&item_id)
    }

    /// Search for knowledge items
    pub fn search(&self, query: &str, categories: Option<Vec<KnowledgeCategory>>) -> Vec<&KnowledgeItem> {
        let query_lower = query.to_lowercase();
        
        self.knowledge_items
            .values()
            .filter(|item| {
                // Filter by categories if specified
                if let Some(ref cats) = categories {
                    if !cats.iter().any(|cat| item.categories.contains(cat)) {
                        return false;
                    }
                }
                
                // Filter by query text
                item.title.to_lowercase().contains(&query_lower) ||
                item.content.to_lowercase().contains(&query_lower) ||
                item.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    /// Get knowledge items by category
    pub fn get_by_category(&self, category: &KnowledgeCategory) -> Vec<&KnowledgeItem> {
        self.knowledge_items
            .values()
            .filter(|item| item.categories.contains(category))
            .collect()
    }

    /// Get knowledge items by community
    pub fn get_by_community(&self, community_id: &str) -> Vec<&KnowledgeItem> {
        self.knowledge_items
            .values()
            .filter(|item| item.source_community == community_id)
            .collect()
    }

    /// Rate a knowledge item
    pub fn rate_knowledge_item(
        &mut self,
        item_id: Uuid,
        rating: f64,
        community_id: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(item) = self.knowledge_items.get_mut(&item_id) {
            item.usefulness_ratings.push(UsefulnessRating {
                rating,
                community_id,
                timestamp: Utc::now(),
            });
            
            // Update quality rating as average
            if !item.usefulness_ratings.is_empty() {
                let sum: f64 = item.usefulness_ratings.iter().map(|r| r.rating).sum();
                item.quality_rating = sum / item.usefulness_ratings.len() as f64;
            }
            
            item.last_updated = Utc::now();
            Ok(())
        } else {
            Err("Knowledge item not found".into())
        }
    }

    /// Record a view of a knowledge item
    pub fn record_view(&mut self, item_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(item) = self.knowledge_items.get_mut(&item_id) {
            item.view_count += 1;
            item.last_updated = Utc::now();
            Ok(())
        } else {
            Err("Knowledge item not found".into())
        }
    }
}

/// Knowledge item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeItem {
    /// Item identifier
    pub id: Uuid,
    
    /// Item title
    pub title: String,
    
    /// Item content
    pub content: String,
    
    /// Categories this item belongs to
    pub categories: Vec<KnowledgeCategory>,
    
    /// Tags for this item
    pub tags: Vec<String>,
    
    /// Community that contributed this item
    pub source_community: String,
    
    /// Other communities that contributed
    pub contributing_communities: Vec<String>,
    
    /// Related knowledge items
    pub related_items: Vec<Uuid>,
    
    /// When this item was created
    pub created_at: DateTime<Utc>,
    
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
    
    /// Quality rating (0.0 to 1.0)
    pub quality_rating: f64,
    
    /// Usefulness ratings from communities
    pub usefulness_ratings: Vec<UsefulnessRating>,
    
    /// Number of times this item has been viewed
    pub view_count: usize,
    
    /// Access level for this item
    pub access_level: AccessLevel,
}

/// Knowledge categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum KnowledgeCategory {
    CommunityStory,
    CommunityAdaptation,
    MilestoneAchievement,
    BestPractice,
    LearningResource,
    Other(String),
}

/// Usefulness rating
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsefulnessRating {
    /// Rating value (0.0 to 1.0)
    pub rating: f64,
    
    /// Community providing the rating
    pub community_id: String,
    
    /// When the rating was given
    pub timestamp: DateTime<Utc>,
}

/// Access levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessLevel {
    AllCommunities,
    SpecificCommunities(Vec<String>),
    Private,
}