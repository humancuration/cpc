//! Web routes for the BI & Analytics module

use axum::{
    routing::{get, post},
    Router,
    extract::State,
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::presentation::web::graphql::BiAnalyticsGraphQLSchema;
use tracing::{info, error};

/// Module-specific state
#[derive(Clone)]
pub struct BiAnalyticsState {
    pub graphql_schema: BiAnalyticsGraphQLSchema,
}

/// Health check endpoint
pub async fn health_check() -> impl IntoResponse {
    info!("Health check requested");
    (StatusCode::OK, "BI Analytics service is healthy")
}

/// GraphQL endpoint
pub async fn graphql_handler(
    State(state): State<BiAnalyticsState>,
    req: async_graphql_axum::GraphQLRequest,
) -> impl IntoResponse {
    info!("GraphQL request received");
    let response = state.graphql_schema.execute(req.into_inner()).await;
    let json = serde_json::to_string(&response).unwrap();
    axum::response::Html(json)
}

/// Create the BI Analytics web router
pub fn create_router(state: BiAnalyticsState) -> Router {
    info!("Creating BI Analytics web router");
    
    Router::new()
        .route("/health", get(health_check))
        .route("/graphql", post(graphql_handler))
        .with_state(state)
}

/// Response structure for API endpoints
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

/// Error response structure
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: String,
}

/// Dataset response structure
#[derive(Serialize)]
pub struct DatasetResponse {
    pub id: Uuid,
    pub name: String,
    pub source: String,
    pub description: Option<String>,
}

/// Report response structure
#[derive(Serialize)]
pub struct ReportResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

/// Dashboard response structure
#[derive(Serialize)]
pub struct DashboardResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, Method},
    };
    use tower::ServiceExt; // for `call`
    
    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }
    
    // Note: Testing the GraphQL endpoint would require a more complex setup
    // including creating mock services and a full GraphQL schema
}