pub mod domain;
pub mod application;
pub mod infrastructure;

pub use domain::*;
pub use application::*;
pub use infrastructure::*;

// Re-export key types and services for easier access
pub use infrastructure::grpc::GrpcServer;

// Re-export gRPC client and server traits
tonic::include_proto!("learning_platform");