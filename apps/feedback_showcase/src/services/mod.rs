//! Services for the feedback showcase

pub mod generator_service;
pub mod data_generator;
pub mod federation;
pub mod collaboration;

pub use generator_service::GeneratorService;
pub use data_generator::DataGeneratorService;
pub use federation::share_visualization;
pub use collaboration::CollaborationService;