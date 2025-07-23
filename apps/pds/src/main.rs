use axum::{Router, routing::post, middleware, routing::get};
use std::sync::Arc;
use rmp_serde::Serializer;
use serde::Serialize;
use anyhow::Result;
use uuid::Uuid;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use async_graphql::*;
use axum::response::IntoResponse;

mod http_handlers;
mod file_utils;
mod auth;
mod impact;
mod graphql;

struct AppState {
    file_processor: file_utils::FileProcessor,
    impact_repo: impact::ImpactRepository,
}

impl AppState {
    fn new() -> Self {
        // For development: use a fixed encryption key
        // In production, this should be securely generated and stored
        let encryption_key = [0u8; 32];
        AppState {
            file_processor: file_utils::FileProcessor::new(encryption_key),
            impact_repo: impact::ImpactRepository::new(),
        }
    }
}

async fn graphql_handler(schema: axum::Extension<graphql::Schema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl axum::response::IntoResponse {
    axum::response::Html(async_graphql::http::GraphiQLSource::build()
        .endpoint("/graphql")
        .subscription_endpoint("/graphql/ws")
        .finish())
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create shared application state
    let app_state = Arc::new(AppState::new());
    
    // Create GraphQL schema
    let schema = graphql::Schema::build(
        graphql::QueryRoot,
        graphql::MutationRoot,
        graphql::SubscriptionRoot,
    )
    .data(app_state.impact_repo.clone())
    .finish();

    // Build our application with routes and middleware
    let app = Router::new()
        .route("/publish", post(http_handlers::publish_handler)
            .route_layer(middleware::from_fn(auth::auth_middleware)))
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .route("/graphql/ws", get(GraphQLSubscription::new(schema.clone())))
        .with_state(app_state);

    // Run our app with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}