use axum::{Router, routing::{get, post}};
use std::net::SocketAddr;

mod routes;
mod graphql;
mod auth;
mod file_utils;

use crate::graphql::schema::{build_schema, Mutation, Query};
use async_graphql::Schema;
use axum::{Extension, middleware};
use axum::http::HeaderValue;
use axum::routing::post;
use std::sync::Arc;
use crate::file_utils::FileProcessor;

pub struct AppState {
    file_processor: FileProcessor,
}

impl AppState {
    fn new() -> Self {
        // For development: use a fixed encryption key
        // In production, this should be securely generated and stored
        let encryption_key = [0u8; 32];
        AppState {
            file_processor: FileProcessor::new(encryption_key),
        }
    }
}

#[tokio::main]
async fn main() {
    let schema = build_schema().finish();
    let app_state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/health", get(routes::health_check))
        .route("/api/update/check", post(routes::update::check_for_updates))
        .route("/graphql", post(graphql::handler))
        .route("/publish", post(routes::publish_handler)
            .route_layer(middleware::from_fn_with_state(app_state.clone(), auth::auth_middleware)))
        .layer(Extension(schema))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}