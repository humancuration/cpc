//! Visualization routes for API Gateway

use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::Response,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;
use visualization_context::VisualizationContext;

use crate::application::visualization::{
    cache::VisualizationCache,
    request::{VisualizationRequest, VisualizationParameters},
    response::{VisualizationResponse, VisualizationData, AccessibilityMetadata},
};

/// Query parameters for visualization requests
#[derive(Debug, Deserialize)]
pub struct VisualizationQuery {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub lod_level: Option<u8>,
    pub format: Option<String>,
}

/// State for visualization routes
#[derive(Clone)]
pub struct VisualizationState {
    pub cache: Arc<VisualizationCache>,
    pub bi_service_url: String,
}

/// Register visualization routes with the router
pub fn register_routes(router: Router, state: VisualizationState) -> Router {
    router
        .route("/visualizations/:id", get(get_visualization))
        .route("/visualizations/:id/image", get(get_visualization_image))
        .route("/visualizations/:id/ws", get(ws_visualization_stream))
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
    
    // Build parameters
    let mut parameters = VisualizationParameters::default();
    if let Some(width) = params.width {
        parameters.width = width;
    }
    if let Some(height) = params.height {
        parameters.height = height;
    }
    if let Some(lod) = params.lod_level {
        parameters.lod_level = lod;
    }
    
    // TODO: Forward request to BI Analytics service
    // For now, return mock response
    let response = VisualizationResponse {
        visualization_data: VisualizationData::Scene3D {
            payload: serde_json::json!({
                "scene": "mock_3d_scene",
                "report_id": id,
                "parameters": parameters
            }),
            accessibility: AccessibilityMetadata {
                alt_text: format!("3D visualization for report {}", id),
                navigation_map: Default::default(),
                aria_properties: Default::default(),
            },
        },
        metadata: Default::default(),
    };
    
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
    
    // TODO: Generate actual image from BI service
    // For now, return placeholder
    let image_data = b"fake_png_data";
    
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/png")
        .body(axum::body::Body::from(image_data.as_ref()))
        .unwrap())
}

/// WebSocket endpoint for live visualization updates
async fn ws_visualization_stream(
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
    
    // TODO: Implement WebSocket upgrade
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// Create new visualization
async fn create_visualization(
    Json(request): Json<VisualizationRequest>,
    headers: HeaderMap,
) -> Result<Json<VisualizationResponse>, StatusCode> {
    // Extract visualization context from headers
    let context = VisualizationContext::from_headers(&headers)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // TODO: Implement visualization creation
    let response = VisualizationResponse {
        visualization_data: VisualizationData::Scene3D {
            payload: serde_json::json!({
                "created": true,
                "request": request
            }),
            accessibility: AccessibilityMetadata {
                alt_text: format!("Created visualization {}", request.visualization_id),
                navigation_map: Default::default(),
                aria_properties: Default::default(),
            },
        },
        metadata: Default::default(),
    };
    
    Ok(Json(response))
}