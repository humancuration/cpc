pub mod domain;
pub mod application;
pub mod infrastructure;

pub use domain::*;
pub use application::*;
pub use infrastructure::*;

// Re-export key types and services for easier access
pub use infrastructure::grpc::GrpcServer;
pub use application::SkillTrackingService;
pub use application::LearningPathService;
pub use application::CertificationService;

pub mod optimization;
pub mod ml;