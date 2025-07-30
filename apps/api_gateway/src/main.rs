//! Central API Gateway for CPC platform
//!
//! This gateway serves as the single entry point for all client requests,
//! routing them to the appropriate backend services including visualization services.

use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Starting CPC API Gateway");

    // Create the application router
    let app = cpc_api_gateway::create_router()
        .route("/health", get(health_check));

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    info!("API Gateway listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

/// Health check endpoint
async fn health_check() -> &'static str {
    "API Gateway is healthy"
}