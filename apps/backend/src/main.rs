use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

mod routes;
mod services;
mod bi;
mod scheduled_jobs;  // Add scheduled_jobs module

use crate::bi::{BIService, BIConfig};
use crate::scheduled_jobs::start_scheduled_jobs;  // Import start function

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Initialize services
    let bi_config = BIConfig::default();
    let bi_service = Arc::new(BIService::new(bi_config));

    // Start scheduled jobs
    start_scheduled_jobs().await;

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(routes::health_check))
        .nest("/api", create_api_router())
        .nest("/bi", bi_service.clone())
        .layer(CorsLayer::permissive());

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn create_api_router() -> Router {
    Router::new()
        .route("/upload", post(routes::upload::upload_image))
        .merge(routes::publish::router())
        .merge(routes::update::router())
        .merge(routes::impact::router())
}