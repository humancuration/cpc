//! Visualization routing for the API Gateway
//!
//! This module handles routing of visualization requests to the appropriate services.

use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;
use visualization_context::VisualizationContext;

use cpc_api_integration::application::visualization::{
    request::{VisualizationRequest, VisualizationParameters, RequestContext},
    response::{VisualizationResponse, VisualizationData, AccessibilityMetadata, ResponseMetadata},
};

/// Query parameters for visualization requests
#[derive(Debug, Deserialize)]
pub struct VisualizationQuery {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub lod_level: Option<u8>,
}

/// State for visualization routes
#[derive(Clone)]
pub struct VisualizationState {
    pub bi_service_url: String,
}

/// Query parameters for visualization requests
#[derive(Debug, Deserialize)]
pub struct VisualizationQuery {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub lod_level: Option<u8>,
}

/// State for visualization routes
#[derive(Clone)]
pub struct VisualizationState {
    pub bi_service_url: String,
}

/// Register visualization routes with the router
pub fn register_routes(router: Router, state: VisualizationState) -> Router {
    router
        .route("/visualizations/:id", get(get_visualization))
        .route("/visualizations/:id/image", get(get_visualization_image))
        .route("/visualizations/:id/ws", get(get_visualization_stream))
        .route("/visualizations", post(create_visualization))
        .with_state(state)
}

/// Get visualization data
async fn get_visualization(
    Path(id): Path<Uuid>,
    Query(params): Query<VisualizationQuery>,
    headers: HeaderMap,
    State(state): State<VisualizationState>,
) -> Result<Json<VisualizationResponse>, StatusCode> {
    // Extract visualization context from headers
    let context = VisualizationContext::from_headers(&headers)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Check access permissions
    if !context.has_access(context.user_id) {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // In a real implementation, this would forward the request to the BI service
    // For now, we'll return a mock response
    let response = create_mock_visualization_response(id, &params, &context);
    
    Ok(Json(response))
}

/// Get visualization as image
async fn get_visualization_image(
    Path(id): Path<Uuid>,
    Query(params): Query<VisualizationQuery>,
    headers: HeaderMap,
    State(state): State<VisualizationState>,
) -> Result<Response, StatusCode> {
    // Extract visualization context from headers
    let context = VisualizationContext::from_headers(&headers)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Check access permissions
    if !context.has_access(context.user_id) {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // In a real implementation, this would generate an image from the visualization
    // For now, return a placeholder
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/png")
        .body(axum::body::Body::from("fake_image_data"))
        .unwrap())
}

/// Get visualization stream (WebSocket)
async fn get_visualization_stream(
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    State(state): State<VisualizationState>,
) -> Result<Response, StatusCode> {
    // Extract visualization context from headers
    let context = VisualizationContext::from_headers(&headers)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Check access permissions
    if !context.has_access(context.user_id) {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // In a real implementation, this would upgrade to a WebSocket connection
    // For now, return not implemented
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// Create a new visualization
async fn create_visualization(
    Json(request): Json<VisualizationRequest>,
    headers: HeaderMap,
) -> Result<Json<VisualizationResponse>, StatusCode> {
    // Extract visualization context from headers
    let context = VisualizationContext::from_headers(&headers)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Check access permissions
    if !context.has_access(context.user_id) {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // In a real implementation, this would create a new visualization
    // For now, return a mock response
    let response = create_mock_visualization_response_from_request(&request, &context);
    
    Ok(Json(response))
}

/// Create a mock visualization response for testing
fn create_mock_visualization_response(
    id: Uuid,
    params: &VisualizationQuery,
    context: &VisualizationContext,
) -> VisualizationResponse {
    VisualizationResponse {
        visualization_data: VisualizationData::Scene3D {
            payload: serde_json::json!({
                "visualization_id": id.to_string(),
                "width": params.width.unwrap_or(800),
                "height": params.height.unwrap_or(600),
                "lod_level": params.lod_level.unwrap_or(context.lod_level as u8),
                "type": "mock_3d_scene"
            }),
            accessibility: AccessibilityMetadata {
                alt_text: format!("3D visualization of report {}", id),
                navigation_map: Default::default(),
                aria_properties: Default::default(),
            },
        },
        metadata: ResponseMetadata::default(),
    }
}

/// Create a mock visualization response from request
fn create_mock_visualization_response_from_request(
    request: &VisualizationRequest,
    context: &VisualizationContext,
) -> VisualizationResponse {
    VisualizationResponse {
        visualization_data: VisualizationData::Scene3D {
            payload: serde_json::json!({
                "visualization_id": request.visualization_id.to_string(),
                "width": request.parameters.width,
                "height": request.parameters.height,
                "lod_level": request.parameters.lod_level,
                "type": request.parameters.visualization_type,
            }),
            accessibility: AccessibilityMetadata {
                alt_text: format!("3D visualization of report {}", request.visualization_id),
                navigation_map: Default::default(),
                aria_properties: Default::default(),
            },
        },
        metadata: ResponseMetadata::default(),
    }
}