//! # CPC API Gateway
//!
//! Central API Gateway for the CPC platform that serves as the single entry point
//! for all client requests, routing them to the appropriate backend services.
//!
//! ## Features
//!
//! - Request routing and load balancing
//! - Authentication and authorization
//! - Rate limiting and throttling
//! - Caching
//! - Monitoring and metrics collection
//! - Protocol translation between clients and backend services

pub mod visualization;

use axum::Router;

/// Create the main application router
pub fn create_router() -> Router {
    let visualization_state = visualization::VisualizationState {
        bi_service_url: "http://localhost:3000".to_string(), // BI service URL
    };
    
    Router::new()
        .merge(visualization::register_routes(Router::new(), visualization_state))
        // TODO: Add routes for other services
}

// Re-export key types
pub use visualization::{VisualizationState, VisualizationQuery};