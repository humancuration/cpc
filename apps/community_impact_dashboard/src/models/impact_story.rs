//! Impact Story Model
//!
//! This module defines data structures for community impact stories that
//! connect individual actions to collective outcomes.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use impact_viz::core::{ImpactMetric, VisualizationType};

/// Community Impact Story
/// 
/// A narrative that connects individual actions to collective community outcomes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactStory {
    /// Unique identifier for this story
    pub id: Uuid,
    
    /// Timestamp when this story was created
    pub timestamp: DateTime<Utc>,
    
    /// Title of the story
    pub title: String,
    
    /// Narrative description of the impact
    pub narrative: String,
    
    /// Author/member who contributed this story
    pub author: StoryAuthor,
    
    /// Related impact metrics
    pub metrics: Vec<ImpactMetric>,
    
    /// Community member quotes or testimonials
    pub testimonials: Vec<StoryTestimonial>,
    
    /// Visual elements to support the story
    pub visual_elements: Vec<StoryVisualElement>,
    
    /// Tags categorizing the story
    pub tags: Vec<String>,
    
    /// Cooperative values demonstrated in this story
    pub values_demonstrated: Vec<String>,
    
    /// Community validation status
    pub community_validated: bool,
    
    /// Number of community reactions
    pub reaction_count: u32,
    
    /// Featured status
    pub featured: bool,
}

/// Story Author
/// 
/// Information about the author of an impact story
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryAuthor {
    /// Author ID (hashed for privacy)
    pub id: String,
    
    /// Author name (may be anonymized)
    pub name: String,
    
    /// Author role in the community
    pub role: String,
    
    /// Author's impact domains
    pub domains: Vec<String>,
}

/// Story Testimonial
/// 
/// A quote or testimonial from a community member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryTestimonial {
    /// Testimonial ID
    pub id: Uuid,
    
    /// Person giving the testimonial
    pub person: String,
    
    /// Testimonial content
    pub content: String,
    
    /// Relationship to the story
    pub relationship: String,
}

/// Story Visual Element
/// 
/// A visual element that supports an impact story
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryVisualElement {
    /// Element ID
    pub id: Uuid,
    
    /// Type of visualization
    pub viz_type: VisualizationType,
    
    /// Description of what the visualization shows
    pub description: String,
    
    /// Data for the visualization
    pub data: VisualizationData,
}

/// Visualization Data
/// 
/// Data structure for visualization in a story
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationData {
    /// JSON representation of the data
    pub json_data: String,
    
    /// Binary representation if applicable (e.g., for images)
    pub binary_data: Option<Vec<u8>>,
}

/// Story Collection
/// 
/// A collection of related impact stories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryCollection {
    /// Collection ID
    pub id: Uuid,
    
    /// Collection title
    pub title: String,
    
    /// Collection description
    pub description: String,
    
    /// Stories in this collection
    pub stories: Vec<ImpactStory>,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
    
    /// Tags for the collection
    pub tags: Vec<String>,
}

/// Story Contribution
/// 
/// A contribution to an existing impact story
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryContribution {
    /// Contribution ID
    pub id: Uuid,
    
    /// Story this contribution is for
    pub story_id: Uuid,
    
    /// Contributor information
    pub contributor: StoryAuthor,
    
    /// Contribution content
    pub content: String,
    
    /// Type of contribution
    pub contribution_type: ContributionType,
    
    /// Timestamp of contribution
    pub timestamp: DateTime<Utc>,
}

/// Contribution Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContributionType {
    /// Additional narrative
    Narrative,
    
    /// Additional metrics
    Metrics,
    
    /// Additional testimonial
    Testimonial,
    
    /// Additional visualization
    Visualization,
    
    /// Correction or clarification
    Correction,
    
    /// Question or discussion point
    Question,
}

impl ImpactStory {
    /// Create a new ImpactStory
    pub fn new(
        title: String,
        narrative: String,
        author: StoryAuthor,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            title,
            narrative,
            author,
            metrics: Vec::new(),
            testimonials: Vec::new(),
            visual_elements: Vec::new(),
            tags: Vec::new(),
            values_demonstrated: Vec::new(),
            community_validated: false,
            reaction_count: 0,
            featured: false,
        }
    }
    
    /// Add an impact metric to this story
    pub fn add_metric(mut self, metric: ImpactMetric) -> Self {
        self.metrics.push(metric);
        self
    }
    
    /// Add a testimonial to this story
    pub fn add_testimonial(mut self, testimonial: StoryTestimonial) -> Self {
        self.testimonials.push(testimonial);
        self
    }
    
    /// Add a visual element to this story
    pub fn add_visual_element(mut self, element: StoryVisualElement) -> Self {
        self.visual_elements.push(element);
        self
    }
    
    /// Add a tag to this story
    pub fn add_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }
    
    /// Add a demonstrated value to this story
    pub fn add_value_demonstrated(mut self, value: String) -> Self {
        self.values_demonstrated.push(value);
        self
    }
    
    /// Mark this story as community validated
    pub fn mark_validated(mut self) -> Self {
        self.community_validated = true;
        self
    }
    
    /// Increment reaction count
    pub fn add_reaction(mut self) -> Self {
        self.reaction_count += 1;
        self
    }
    
    /// Mark this story as featured
    pub fn mark_featured(mut self) -> Self {
        self.featured = true;
        self
    }
}