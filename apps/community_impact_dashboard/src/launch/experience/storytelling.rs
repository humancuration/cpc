//! Community storytelling integration for the Unified Community Impact Dashboard
//!
//! This module integrates community stories, celebrates transformation moments,
//! and creates space for collective joy in discoveries during the launch process.

use tracing::info;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Community storytelling integration system
pub struct StoryIntegration {
    community_stories: Vec<CommunityStory>,
    story_collections: HashMap<String, StoryCollection>,
    story_contributions: Vec<StoryContribution>,
    featured_stories: Vec<FeaturedStory>,
}

impl StoryIntegration {
    /// Create a new story integration system
    pub fn new() -> Self {
        Self {
            community_stories: Vec::new(),
            story_collections: HashMap::new(),
            story_contributions: Vec::new(),
            featured_stories: Vec::new(),
        }
    }

    /// Create a launch story collection
    pub fn create_launch_story_collection(&mut self, community_name: String) -> String {
        let collection_id = "launch_stories".to_string();
        
        let collection = StoryCollection::new(
            collection_id.clone(),
            format!("{} Launch Stories", community_name),
            "Stories from our community's dashboard launch journey".to_string(),
            StoryCollectionType::Launch,
        );
        
        self.story_collections.insert(collection_id.clone(), collection);
        collection_id
    }

    /// Add a community story to the launch collection
    pub fn add_community_story(&mut self, story: CommunityStory) -> Result<Uuid, StoryError> {
        // Check if story already exists
        if self.community_stories.iter().any(|s| s.id == story.id) {
            return Err(StoryError::StoryAlreadyExists(story.id));
        }
        
        self.community_stories.push(story.clone());
        
        // Add to launch collection if it exists
        if let Some(collection) = self.story_collections.get_mut("launch_stories") {
            collection.story_ids.push(story.id);
        }
        
        info!("Added community story: {}", story.title);
        Ok(story.id)
    }

    /// Contribute to an existing story
    pub fn contribute_to_story(&mut self, contribution: StoryContribution) -> Result<Uuid, StoryError> {
        // Verify the story exists
        if !self.community_stories.iter().any(|s| s.id == contribution.story_id) {
            return Err(StoryError::StoryNotFound(contribution.story_id));
        }
        
        self.story_contributions.push(contribution.clone());
        info!("Added story contribution to story: {}", contribution.story_id);
        Ok(contribution.id)
    }

    /// Feature a story for community celebration
    pub fn feature_story(&mut self, story_id: Uuid, reason: String) -> Result<Uuid, StoryError> {
        // Verify the story exists
        let story = self.community_stories.iter()
            .find(|s| s.id == story_id)
            .ok_or(StoryError::StoryNotFound(story_id))?;
        
        let featured = FeaturedStory::new(
            story_id,
            story.title.clone(),
            reason,
            FeaturedStoryType::CommunityCelebration,
        );
        
        self.featured_stories.push(featured.clone());
        info!("Featured story: {}", story.title);
        Ok(featured.id)
    }

    /// Get stories from the launch collection
    pub fn get_launch_stories(&self) -> Vec<&CommunityStory> {
        if let Some(collection) = self.story_collections.get("launch_stories") {
            collection.story_ids.iter()
                .filter_map(|id| self.community_stories.iter().find(|s| s.id == *id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get featured stories
    pub fn get_featured_stories(&self) -> &Vec<FeaturedStory> {
        &self.featured_stories
    }

    /// Get story contributions for a specific story
    pub fn get_story_contributions(&self, story_id: Uuid) -> Vec<&StoryContribution> {
        self.story_contributions.iter()
            .filter(|c| c.story_id == story_id)
            .collect()
    }

    /// Create a transformation story from launch experience
    pub fn create_transformation_story(
        &mut self,
        title: String,
        description: String,
        participants: Vec<String>,
        impact_domains: Vec<ImpactDomain>,
    ) -> Result<Uuid, StoryError> {
        let story = CommunityStory::new(
            title,
            description,
            StoryType::Transformation,
            participants,
            impact_domains,
            StoryVisibility::Community,
        );
        
        self.add_community_story(story)
    }

    /// Create a celebration story from launch milestones
    pub fn create_celebration_story(
        &mut self,
        title: String,
        description: String,
        participants: Vec<String>,
        achievements: Vec<String>,
    ) -> Result<Uuid, StoryError> {
        let story = CommunityStory::celebration(
            title,
            description,
            participants,
            achievements,
        );
        
        self.add_community_story(story)
    }
}

/// Community story representing a meaningful experience
#[derive(Debug, Clone)]
pub struct CommunityStory {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub story_type: StoryType,
    pub participants: Vec<String>,
    pub impact_domains: Vec<ImpactDomain>,
    pub visibility: StoryVisibility,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub additional_data: Option<StoryData>,
}

impl CommunityStory {
    /// Create a new community story
    pub fn new(
        title: String,
        description: String,
        story_type: StoryType,
        participants: Vec<String>,
        impact_domains: Vec<ImpactDomain>,
        visibility: StoryVisibility,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            story_type,
            participants,
            impact_domains,
            visibility,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            additional_data: None,
        }
    }

    /// Create a celebration story
    pub fn celebration(
        title: String,
        description: String,
        participants: Vec<String>,
        achievements: Vec<String>,
    ) -> Self {
        let mut story = Self::new(
            title,
            description,
            StoryType::Celebration,
            participants,
            vec![ImpactDomain::Community],
            StoryVisibility::Community,
        );
        
        story.tags = achievements;
        story
    }

    /// Add tags to the story
    pub fn add_tags(&mut self, tags: Vec<String>) {
        self.tags.extend(tags);
        self.updated_at = Utc::now();
    }

    /// Add additional data to the story
    pub fn add_data(&mut self, data: StoryData) {
        self.additional_data = Some(data);
        self.updated_at = Utc::now();
    }
}

/// Types of community stories
#[derive(Debug, Clone)]
pub enum StoryType {
    PersonalJourney,
    CommunityImpact,
    Transformation,
    Celebration,
    Learning,
    Innovation,
}

/// Impact domains for story categorization
#[derive(Debug, Clone)]
pub enum ImpactDomain {
    Learning,
    Volunteering,
    Financial,
    CauseAdvocacy,
    Community,
    Interconnected,
}

/// Visibility levels for stories
#[derive(Debug, Clone)]
pub enum StoryVisibility {
    Private,
    Community,
    Public,
}

/// Additional data that can be attached to stories
#[derive(Debug, Clone)]
pub enum StoryData {
    Media(String), // URL to media
    Document(String), // URL to document
    Metrics(HashMap<String, f64>), // Key metrics related to the story
}

/// Collection of related stories
#[derive(Debug, Clone)]
pub struct StoryCollection {
    pub id: String,
    pub name: String,
    pub description: String,
    pub collection_type: StoryCollectionType,
    pub story_ids: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub featured: bool,
}

impl StoryCollection {
    /// Create a new story collection
    pub fn new(
        id: String,
        name: String,
        description: String,
        collection_type: StoryCollectionType,
    ) -> Self {
        Self {
            id,
            name,
            description,
            collection_type,
            story_ids: Vec::new(),
            created_at: Utc::now(),
            featured: false,
        }
    }
}

/// Types of story collections
#[derive(Debug, Clone)]
pub enum StoryCollectionType {
    Launch,
    ThemeBased,
    TimeBased,
    UserGenerated,
}

/// Contribution to an existing story
#[derive(Debug, Clone)]
pub struct StoryContribution {
    pub id: Uuid,
    pub story_id: Uuid,
    pub contributor: String,
    pub content: String,
    pub contribution_type: ContributionType,
    pub created_at: DateTime<Utc>,
}

impl StoryContribution {
    /// Create a new story contribution
    pub fn new(
        story_id: Uuid,
        contributor: String,
        content: String,
        contribution_type: ContributionType,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            story_id,
            contributor,
            content,
            contribution_type,
            created_at: Utc::now(),
        }
    }
}

/// Types of story contributions
#[derive(Debug, Clone)]
pub enum ContributionType {
    Reflection,
    Extension,
    Correction,
    Celebration,
}

/// Featured story for community highlighting
#[derive(Debug, Clone)]
pub struct FeaturedStory {
    pub id: Uuid,
    pub story_id: Uuid,
    pub title: String,
    pub reason: String,
    pub featured_type: FeaturedStoryType,
    pub featured_at: DateTime<Utc>,
}

impl FeaturedStory {
    /// Create a new featured story
    pub fn new(
        story_id: Uuid,
        title: String,
        reason: String,
        featured_type: FeaturedStoryType,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            story_id,
            title,
            reason,
            featured_type,
            featured_at: Utc::now(),
        }
    }
}

/// Types of featured stories
#[derive(Debug, Clone)]
pub enum FeaturedStoryType {
    CommunityCelebration,
    LearningHighlight,
    InnovationShowcase,
    TransformationMoment,
}

/// Error types for story integration
#[derive(Debug)]
pub enum StoryError {
    StoryNotFound(Uuid),
    StoryAlreadyExists(Uuid),
    CollectionNotFound(String),
    ContributionError(String),
}

impl std::fmt::Display for StoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoryError::StoryNotFound(id) => write!(f, "Story not found: {}", id),
            StoryError::StoryAlreadyExists(id) => write!(f, "Story already exists: {}", id),
            StoryError::CollectionNotFound(id) => write!(f, "Collection not found: {}", id),
            StoryError::ContributionError(msg) => write!(f, "Contribution error: {}", msg),
        }
    }
}

impl std::error::Error for StoryError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_story_integration_initialization() {
        let story_integration = StoryIntegration::new();
        assert!(story_integration.community_stories.is_empty());
        assert!(story_integration.story_collections.is_empty());
    }

    #[test]
    fn test_create_launch_story_collection() {
        let mut story_integration = StoryIntegration::new();
        let collection_id = story_integration.create_launch_story_collection("Test Community".to_string());
        assert_eq!(collection_id, "launch_stories");
        assert!(story_integration.story_collections.contains_key("launch_stories"));
    }

    #[test]
    fn test_add_community_story() {
        let mut story_integration = StoryIntegration::new();
        story_integration.create_launch_story_collection("Test Community".to_string());
        
        let story = CommunityStory::new(
            "Test Story".to_string(),
            "This is a test story".to_string(),
            StoryType::CommunityImpact,
            vec!["user1".to_string()],
            vec![ImpactDomain::Learning],
            StoryVisibility::Community,
        );
        
        let result = story_integration.add_community_story(story);
        assert!(result.is_ok());
        assert_eq!(story_integration.community_stories.len(), 1);
    }

    #[test]
    fn test_feature_story() {
        let mut story_integration = StoryIntegration::new();
        
        let story = CommunityStory::new(
            "Test Story".to_string(),
            "This is a test story".to_string(),
            StoryType::CommunityImpact,
            vec!["user1".to_string()],
            vec![ImpactDomain::Learning],
            StoryVisibility::Community,
        );
        
        let story_id = story.id;
        story_integration.add_community_story(story).unwrap();
        
        let result = story_integration.feature_story(
            story_id,
            "Great story for launch celebration".to_string()
        );
        assert!(result.is_ok());
        assert_eq!(story_integration.featured_stories.len(), 1);
    }
}