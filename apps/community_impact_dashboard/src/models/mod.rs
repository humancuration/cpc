//! Data models for the Unified Community Impact Dashboard
//!
//! This module defines the core data structures used throughout the dashboard
//! to represent unified impact data from all four measurement systems.

pub mod impact_data;
pub mod interconnection;
pub mod community_wellbeing;
pub mod impact_story;
pub mod community_validation;

pub use impact_data::UnifiedImpactData;
pub use interconnection::ImpactInterconnection;
pub use community_wellbeing::CommunityWellbeing;
pub use impact_story::ImpactStory;
pub use community_validation::*;