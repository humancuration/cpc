//! Services module for the Unified Community Impact Dashboard
//!
//! This module contains services for loading, processing, and managing
//! impact data from all four measurement systems.

pub mod impact_data_service;
pub mod interconnection_analyzer;
pub mod community_wellbeing_calculator;
pub mod story_collector;
pub mod visualization_generator;
pub mod community_validation_service;
pub mod mock_data;

pub use impact_data_service::ImpactDataService;
pub use interconnection_analyzer::InterconnectionAnalyzer;
pub use community_wellbeing_calculator::CommunityWellbeingCalculator;
pub use story_collector::StoryCollector;
pub use visualization_generator::VisualizationGenerator;
pub use community_validation_service::CommunityValidationService;