//! Integration tests for the visualization functionality in the API Gateway

use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt; // for `call`

use cpc_api_gateway::visualization;

#[tokio::test]
async fn test_visualization_routes() {
    // Create the router with visualization routes
    let visualization_state = visualization::VisualizationState {
        bi_service_url: "http://localhost:3000".to_string(),
    };
    
    let app = visualization::register_routes(Router::new(), visualization_state);
    
    // Test health check endpoint
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND); // Health route is not registered in visualization module
}

#[tokio::test]
async fn test_visualization_get_route() {
    // Create the router with visualization routes
    let visualization_state = visualization::VisualizationState {
        bi_service_url: "http://localhost:3000".to_string(),
    };
    
    let app = visualization::register_routes(Router::new(), visualization_state);
    
    // Test getting a visualization (this will fail due to missing headers)
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/visualizations/123e4567-e89b-12d3-a456-426614174000")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should fail due to missing headers
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}