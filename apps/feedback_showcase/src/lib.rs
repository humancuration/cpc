//! Feedback Showcase - Demonstration of feedback system integration

pub mod data_generator;
pub mod components;
pub mod services;
pub mod utils;

// Re-export visualization components for easy access
pub use components::visualization::{Summary, RatingsChart, WordCloud, Sentiment};
pub use components::{DemoControlPanel, Playground, PresetSelector};
pub use components::social_sharing::{
    ShareButtonGroup,
    generate_embed_code,
    export_as_image,
    SocialSharingDialog,
    EmbedCodeDialog,
    EmbedPreview,
    AnnotationManager,
    EmbeddedVisualization
};
pub use services::collaboration::CollaborationService;