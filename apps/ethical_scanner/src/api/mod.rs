//! GraphQL and gRPC interfaces for EthicalScanner

pub mod graphql;
pub mod grpc;

/// API error types
#[derive(Debug)]
pub enum ApiError {
    GraphQLInitializationError(String),
    GrpcInitializationError(String),
    DataSerializationError(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ApiError::GraphQLInitializationError(msg) => write!(f, "GraphQL initialization error: {}", msg),
            ApiError::GrpcInitializationError(msg) => write!(f, "gRPC initialization error: {}", msg),
            ApiError::DataSerializationError(msg) => write!(f, "Data serialization error: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}