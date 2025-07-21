use axum::{Router, routing::post};
use std::sync::Arc;
use rmp_serde::Serializer;
use serde::Serialize;
use anyhow::Result;

mod http_handlers;
mod file_utils;

struct AppState {
    file_processor: file_utils::FileProcessor,
}

impl AppState {
    fn new() -> Self {
        // For development: use a fixed encryption key
        // In production, this should be securely generated and stored
        let encryption_key = [0u8; 32];
        AppState {
            file_processor: file_utils::FileProcessor::new(encryption_key),
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create shared application state
    let app_state = Arc::new(AppState::new());

    // Build our application with a route
    let app = Router::new()
        .route("/publish", post(http_handlers::publish_handler))
        .with_state(app_state);

    // Run our app with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}