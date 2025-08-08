//! Community facilitator preparation tools
//!
//! This module provides tools and resources for community facilitators
//! to help prepare for and conduct dashboard launch activities.

use tracing::info;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Facilitator toolkit for dashboard launch preparation
pub struct FacilitatorToolkit {
    resources: HashMap<String, FacilitatorResource>,
    training_progress: HashMap<String, TrainingProgress>,
}

impl FacilitatorToolkit {
    /// Create a new facilitator toolkit
    pub fn new() -> Self {
        let mut toolkit = Self {
            resources: HashMap::new(),
            training_progress: HashMap::new(),
        };
        
        // Initialize with default resources
        toolkit.initialize_default_resources();
        toolkit
    }
    
    /// Initialize default facilitator resources
    fn initialize_default_resources(&mut self) {
        // Workshop templates
        self.resources.insert(
            "workshop_template_introduction".to_string(),
            FacilitatorResource::new(
                "workshop_template_introduction".to_string(),
                "Introduction to Interconnected Impact Workshop Template".to_string(),
                "Template for introducing the dashboard and interconnected impact concepts to community members".to_string(),
                ResourceType::WorkshopTemplate,
                ResourceContent::Text(include_str!("../../docs/templates/workshop_introduction_template.md")),
            )
        );
        
        self.resources.insert(
            "workshop_template_validation".to_string(),
            FacilitatorResource::new(
                "workshop_template_validation".to_string(),
                "Community Validation Workshop Template".to_string(),
                "Template for facilitating community validation sessions using the dashboard".to_string(),
                ResourceType::WorkshopTemplate,
                ResourceContent::Text(include_str!("../../docs/templates/workshop_validation_template.md")),
            )
        );
        
        // Facilitation guides
        self.resources.insert(
            "facilitation_guide_basics".to_string(),
            FacilitatorResource::new(
                "facilitation_guide_basics".to_string(),
                "Dashboard Facilitation Basics".to_string(),
                "Fundamental principles and techniques for facilitating dashboard activities".to_string(),
                ResourceType::FacilitationGuide,
                ResourceContent::Text(include_str!("../../docs/guides/facilitation_basics.md")),
            )
        );
        
        self.resources.insert(
            "facilitation_guide_troubleshooting".to_string(),
            FacilitatorResource::new(
                "facilitation_guide_troubleshooting".to_string(),
                "Troubleshooting Common Issues".to_string(),
                "Guide for handling common technical and conceptual challenges during dashboard sessions".to_string(),
                ResourceType::FacilitationGuide,
                ResourceContent::Text(include_str!("../../docs/guides/troubleshooting.md")),
            )
        );
        
        // Training materials
        self.resources.insert(
            "training_video_overview".to_string(),
            FacilitatorResource::new(
                "training_video_overview".to_string(),
                "Dashboard Overview Training Video".to_string(),
                "Comprehensive video walkthrough of dashboard features and capabilities".to_string(),
                ResourceType::TrainingVideo,
                ResourceContent::Video("https://example.com/training/dashboard_overview.mp4".to_string()),
            )
        );
        
        self.resources.insert(
            "training_video_validation".to_string(),
            FacilitatorResource::new(
                "training_video_validation".to_string(),
                "Community Validation Training Video".to_string(),
                "Video tutorial on facilitating community validation processes".to_string(),
                ResourceType::TrainingVideo,
                ResourceContent::Video("https://example.com/training/validation_workflows.mp4".to_string()),
            )
        );
        
        // Customization templates
        self.resources.insert(
            "customization_template_community".to_string(),
            FacilitatorResource::new(
                "customization_template_community".to_string(),
                "Community-Specific Customization Template".to_string(),
                "Template for adapting dashboard materials to specific community contexts".to_string(),
                ResourceType::CustomizationTemplate,
                ResourceContent::Text(include_str!("../../docs/templates/community_customization.md")),
            )
        );
    }
    
    /// Get a specific resource by ID
    pub fn get_resource(&self, resource_id: &str) -> Option<&FacilitatorResource> {
        self.resources.get(resource_id)
    }
    
    /// Get all resources of a specific type
    pub fn get_resources_by_type(&self, resource_type: ResourceType) -> Vec<&FacilitatorResource> {
        self.resources.values()
            .filter(|resource| resource.resource_type == resource_type)
            .collect()
    }
    
    /// Search resources by keyword
    pub fn search_resources(&self, keyword: &str) -> Vec<&FacilitatorResource> {
        self.resources.values()
            .filter(|resource| {
                resource.title.to_lowercase().contains(&keyword.to_lowercase()) ||
                resource.description.to_lowercase().contains(&keyword.to_lowercase())
            })
            .collect()
    }
    
    /// Record training progress for a facilitator
    pub fn record_training_progress(&mut self, facilitator_id: String, resource_id: String, completed: bool) {
        let progress = TrainingProgress::new(facilitator_id.clone(), resource_id, completed);
        self.training_progress.insert(
            format!("{}_{}", facilitator_id, progress.resource_id),
            progress
        );
        info!("Recorded training progress for facilitator {}: {} - {}", 
              facilitator_id, progress.resource_id, if completed { "completed" } else { "in progress" });
    }
    
    /// Get training completion percentage for a facilitator
    pub fn get_training_completion(&self, facilitator_id: &str) -> f64 {
        let facilitator_trainings: Vec<&TrainingProgress> = self.training_progress.values()
            .filter(|progress| progress.facilitator_id == facilitator_id)
            .collect();
        
        if facilitator_trainings.is_empty() {
            return 0.0;
        }
        
        let completed_count = facilitator_trainings.iter()
            .filter(|progress| progress.completed)
            .count();
        
        (completed_count as f64 / facilitator_trainings.len() as f64) * 100.0
    }
    
    /// Get all resources that need facilitator preparation
    pub fn get_preparation_resources(&self) -> Vec<&FacilitatorResource> {
        vec![
            self.resources.get("workshop_template_introduction").unwrap(),
            self.resources.get("workshop_template_validation").unwrap(),
            self.resources.get("facilitation_guide_basics").unwrap(),
            self.resources.get("facilitation_guide_troubleshooting").unwrap(),
        ]
    }
}

impl Default for FacilitatorToolkit {
    fn default() -> Self {
        Self::new()
    }
}

/// Facilitator resource structure
#[derive(Debug, Clone)]
pub struct FacilitatorResource {
    /// Unique identifier for the resource
    pub id: String,
    
    /// Title of the resource
    pub title: String,
    
    /// Description of the resource
    pub description: String,
    
    /// Type of resource
    pub resource_type: ResourceType,
    
    /// Content of the resource
    pub content: ResourceContent,
    
    /// When the resource was created
    pub created_at: DateTime<Utc>,
    
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
}

impl FacilitatorResource {
    /// Create a new facilitator resource
    pub fn new(id: String, title: String, description: String, resource_type: ResourceType, content: ResourceContent) -> Self {
        let now = Utc::now();
        Self {
            id,
            title,
            description,
            resource_type,
            content,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Types of facilitator resources
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceType {
    /// Workshop template
    WorkshopTemplate,
    
    /// Facilitation guide
    FacilitationGuide,
    
    /// Training material
    TrainingVideo,
    
    /// Customization template
    CustomizationTemplate,
    
    /// Reference document
    ReferenceDocument,
}

/// Content of a facilitator resource
#[derive(Debug, Clone)]
pub enum ResourceContent {
    /// Text content (markdown, plain text, etc.)
    Text(String),
    
    /// Video content (URL to video resource)
    Video(String),
    
    /// Document content (URL to document)
    Document(String),
    
    /// Interactive tool
    InteractiveTool(String),
}

/// Training progress tracking
#[derive(Debug, Clone)]
pub struct TrainingProgress {
    /// Facilitator identifier
    pub facilitator_id: String,
    
    /// Resource identifier
    pub resource_id: String,
    
    /// Whether the training is completed
    pub completed: bool,
    
    /// When progress was recorded
    pub recorded_at: DateTime<Utc>,
}

impl TrainingProgress {
    /// Create new training progress record
    pub fn new(facilitator_id: String, resource_id: String, completed: bool) -> Self {
        Self {
            facilitator_id,
            resource_id,
            completed,
            recorded_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_toolkit_initialization() {
        let toolkit = FacilitatorToolkit::new();
        assert!(!toolkit.resources.is_empty());
        assert!(toolkit.resources.contains_key("workshop_template_introduction"));
        assert!(toolkit.resources.contains_key("facilitation_guide_basics"));
    }
    
    #[test]
    fn test_get_resource() {
        let toolkit = FacilitatorToolkit::new();
        let resource = toolkit.get_resource("workshop_template_introduction");
        assert!(resource.is_some());
        assert_eq!(resource.unwrap().resource_type, ResourceType::WorkshopTemplate);
    }
    
    #[test]
    fn test_get_resources_by_type() {
        let toolkit = FacilitatorToolkit::new();
        let workshop_templates = toolkit.get_resources_by_type(ResourceType::WorkshopTemplate);
        assert!(!workshop_templates.is_empty());
        assert!(workshop_templates.iter().all(|r| r.resource_type == ResourceType::WorkshopTemplate));
    }
    
    #[test]
    fn test_record_training_progress() {
        let mut toolkit = FacilitatorToolkit::new();
        toolkit.record_training_progress(
            "facilitator123".to_string(),
            "workshop_template_introduction".to_string(),
            true
        );
        
        let completion = toolkit.get_training_completion("facilitator123");
        assert!(completion > 0.0);
    }
    
    #[test]
    fn test_search_resources() {
        let toolkit = FacilitatorToolkit::new();
        let results = toolkit.search_resources("workshop");
        assert!(!results.is_empty());
        assert!(results.iter().all(|r| 
            r.title.to_lowercase().contains("workshop") || 
            r.description.to_lowercase().contains("workshop")
        ));
    }
}