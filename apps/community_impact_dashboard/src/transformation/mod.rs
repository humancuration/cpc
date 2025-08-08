//! Community Transformation Documentation System
//!
//! This module provides comprehensive tools for documenting and celebrating
//! how communities are using the Unified Community Impact Dashboard to understand
//! and strengthen their interconnected impact.

pub mod story_collection;
pub mod impact_mapping;
pub mod adaptation_tracking;
pub mod milestone_recognition;
pub mod celebration_documentation;

pub use story_collection::*;
pub use impact_mapping::*;
pub use adaptation_tracking::*;
pub use milestone_recognition::*;
pub use celebration_documentation::*;

/// Main transformation system coordinator
pub struct TransformationSystem {
    pub story_collection: StoryCollectionSystem,
    pub impact_mapping: ImpactConnectionMapping,
    pub adaptation_tracking: CommunityAdaptationTracking,
    pub milestone_recognition: TransformationMilestoneRecognition,
    pub celebration_documentation: CelebrationDocumentation,
}

impl TransformationSystem {
    /// Create a new transformation system
    pub fn new() -> Self {
        Self {
            story_collection: StoryCollectionSystem::new(),
            impact_mapping: ImpactConnectionMapping::new(),
            adaptation_tracking: CommunityAdaptationTracking::new(),
            milestone_recognition: TransformationMilestoneRecognition::new(),
            celebration_documentation: CelebrationDocumentation::new(),
        }
    }

    /// Initialize the transformation system
    pub fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.story_collection.initialize()?;
        self.impact_mapping.initialize()?;
        self.adaptation_tracking.initialize()?;
        self.milestone_recognition.initialize()?;
        self.celebration_documentation.initialize()?;
        Ok(())
    }
}