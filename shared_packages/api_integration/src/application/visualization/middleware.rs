//! Visualization middleware for request processing

use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use tokio::time::Instant;
use tracing::{info, warn};
use uuid::Uuid;
use visualization_context::VisualizationContext;

use crate::application::{
    monitoring::MetricsCollector,
    visualization::cache::VisualizationCache,
};

/// Shared state for visualization middleware
#[derive(Clone)]
pub struct VisualizationMiddlewareState {
    pub cache: Arc<VisualizationCache>,
    pub metrics: Arc<MetricsCollector>,
}

/// Middleware for processing visualization requests
pub async fn visualization_middleware(
    State(state): State<VisualizationMiddlewareState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let start = Instant::now();
    
    // Extract visualization context from headers
    let context = match VisualizationContext::from_headers(&headers) {
        Ok(ctx) => ctx,
        Err(e) => {
            warn!("Failed to parse visualization context: {}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    
    // Add context to request extensions for downstream handlers
    request.extensions_mut().insert(context);
    
    // Process the request
    let response = next.run(request).await;
    
    // Record metrics
    let duration = start.elapsed();
    state.metrics.record_request(
        &headers
            .get("X-Originating-App")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown"),
        response.status().is_success(),
        duration,
    );
    
    Ok(response)
}

/// Rate limiting middleware
pub async fn rate_limit_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract app ID from headers
    let app_id = headers
        .get("X-Originating-App")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");
    
    // Check rate limits (simplified implementation)
    // In a real implementation, this would check against a rate limiting service
    let is_rate_limited = false; // Always allow for now
    
    if is_rate_limited {
        Err(StatusCode::TOO_MANY_REQUESTS)
    } else {
        Ok(next.run(request).await)
    }
}