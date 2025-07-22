use axum::{Router, routing::{get, post}};
use std::net::SocketAddr;

mod routes;
mod graphql;
mod auth;
mod file_utils;
mod config;

use crate::graphql::schema::{build_schema, Mutation, Query};
use async_graphql::Schema;
use axum::{Extension, middleware};
use axum::http::HeaderValue;
use axum::routing::post;
use std::sync::Arc;
use crate::file_utils::FileProcessor;
use crate::config::{Config, ConfigError};
use jsonwebtoken::DecodingKey;  // Add missing import

pub struct AppState {
    file_processor: FileProcessor,
    decoding_key: DecodingKey<'static>,
}

impl AppState {
    fn new(encryption_key: [u8; 32], jwt_secret: String) -> Self {
        let decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
        AppState {
            file_processor: FileProcessor::new(encryption_key),
            decoding_key,
        }
    }
}

#[tokio::main]
async fn main() {
    // Load configuration from environment variables
    let config = Config::from_env()
        .expect("Failed to load configuration");

    let schema = build_schema().finish();
    let app_state = Arc::new(AppState::new(config.encryption_key, config.jwt_secret));  // Add jwt_secret

    let app = Router::new()
        .route("/health", get(routes::health_check))
        .route("/api/update/check", post(routes::update::check_for_updates))
        .route("/graphql", post(graphql::handler))
        .route("/publish", post(routes::publish_handler)
            .route_layer(middleware::from_fn_with_state(app_state.clone(), auth::auth_middleware)))
        .layer(Extension(schema))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}