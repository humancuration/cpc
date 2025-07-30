//! # CPC API & Integration Hub Module
//!
//! API & Integration Hub module for the CPC platform.
//!
//! This module provides a comprehensive solution for integrating with external systems
//! and exposing internal services through standardized APIs.
//!
//! # Architecture
//!
//! This module follows hexagonal (clean) architecture with vertical slices:
//!
//! - **Adapter Management Slice**: Registration and configuration of integration adapters
//! - **Request Routing Slice**: Routing of external requests to internal services
//! - **Rate Limiting Slice**: Rate limiting and throttling of API requests
//! - **Monitoring Slice**: Monitoring and logging of API activity
//!
//! # Key Features
//!
//! - Adapter registry for enterprise system integrations (SAP, Oracle, custom HTTP)
//! - Request routing with authentication and authorization
//! - Rate limiting with customizable rules
//! - Comprehensive monitoring and logging
//! - GraphQL and REST API endpoints
//! - gRPC internal service interface
//! - Developer portal with documentation

// Domain entities and business logic
pub mod domain;

// Application services for orchestrating domain logic
pub mod application;

// Infrastructure implementations (database, adapters, etc.)
pub mod infrastructure;

// Presentation layer (API endpoints, developer portal)
pub mod presentation;

// Test modules
#[cfg(test)]
mod tests;

// Re-export key types for convenience
pub use domain::{
    api_endpoint::ApiEndpoint,
    adapter_config::AdapterConfig,
};