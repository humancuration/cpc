//! Visualization routes for API Gateway

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

use crate::application::visualization::{
    cache::VisualizationCache,
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
pub struct VisualizationRouteState {
    pub cache: Arc<VisualizationCache>,
    pub bi_service_url: String,
}

/// Register visualization routes
pub fn register_routes(router: Router, state: VisualizationRouteState) -> Router {
    router
        .route("/visualizations/:id", get(get_visualization))
        .route("/visualizations/:id/image", get(get_visualization_image))
        .route("/visualizations/:id/ws", get(get_visualization_stream))
        .route("/visualizations", post(create_visualization))
        .with_state(state)
}

/// Get 3D visualization
async fn get_visualization(
    Path(id): Path<Uuid>,
    Query(params): Query<VisualizationQuery>,
    headers: HeaderMap,
    State(state): State<VisualizationRouteState>,
) -> Result<Json<VisualizationResponse>, StatusCode> {
    // Extract visualization context from headers
    let context = VisualizationContext::from_headers(&headers)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Check access permissions
    if !context.has_access(context.user_id) {
        return Err(StatusCode::FORBIDDEN);
    }
    
    // Generate cache key
    let viz_params = VisualizationParameters {
        width: params.width.unwrap_or(800),
        height: params.height.unwrap_or(600),
        lod_level: params.lod_level.unwrap_or(context.lod_level),
        visualization_type: "3d_scene".to_string(),
        accessibility_mode: context.accessibility_mode.clone(),
    };
    
    let cache_key = VisualizationCache::generate_cache_key(id, &viz_params, &context);
    
    // Try to get from cache
    if let Ok(Some(cached_data)) = state.cache.get(&cache_key) {
        if let Ok(cached_response) = bincode::deserialize::<VisualizationResponse>(&cached_data) {
            return Ok(Json(cached_response));
        }
    }
    
    // Create mock response (in a real implementation, this would call the BI service)
    let response = create_mock_visualization_response(id, &viz_params, &context);
    
    // Cache the response
    if let Ok(serialized) = bincode::serialize(&response) {
        let _ = state.cache.set(&cache_key, serialized, 300); // Cache for 5 minutes
    }
    
    Ok(Json(response))
}

/// Get visualization as image
async fn get_visualization_image(
    Path(id): Path<Uuid>,
    Query(params): Query<VisualizationQuery>,
    headers: HeaderMap,
    State(state): State<VisualizationRouteState>,
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
    State(state): State<VisualizationRouteState>,
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
    let response = create_mock_visualization_response(
        request.visualization_id,
        &request.parameters,
        &context,
    );
    
    Ok(Json(response))
}

/// Create a mock visualization response for testing
fn create_mock_visualization_response(
    id: Uuid,
    params: &VisualizationParameters,
    context: &VisualizationContext,
) -> VisualizationResponse {
    VisualizationResponse {
        visualization_data: VisualizationData::Scene3D {
            payload: serde_json::json!({
                "visualization_id": id.to_string(),
                "width": params.width,
                "height": params.height,
                "lod_level": params.lod_level,
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