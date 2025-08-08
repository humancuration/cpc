//! Cause Management Admin Dashboard
//!
//! A web-based admin dashboard for monitoring and analyzing cause impact metrics
//! across the CPC ecosystem.

use axum::{
    routing::{get, post},
    Router, Json, Extension,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

use cause_impact_tracker::{
    CauseImpactTracker,
    ImpactAnalyticsDashboard,
    FeedbackCollector,
    tracker::{VisualizationType, ValidationType},
    analytics::DashboardSummary,
};
use consent_manager::domain::consent::DataSharingLevel;
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Clone)]
struct AppState {
    tracker: Arc<RwLock<CauseImpactTracker>>,
    dashboard: Arc<RwLock<ImpactAnalyticsDashboard>>,
    feedback: Arc<RwLock<FeedbackCollector>>,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    timestamp: chrono::DateTime<Utc>,
}

#[derive(Deserialize)]
struct TrackEngagementRequest {
    user_id: String,
    component_id: String,
    viz_type: String,
    time_spent: f64,
    interaction_count: u32,
    quality_score: f64,
    confidence_score: Option<f64>,
}

#[derive(Deserialize)]
struct TrackCauseEngagementRequest {
    user_id: String,
    viz_usage: Vec<String>,
    engaged: bool,
    months_engaged: Option<u32>,
    satisfaction_rating: Option<u32>,
    contribution_amount: Option<f64>,
}

#[derive(Deserialize)]
struct RecordValidationRequest {
    user_id: String,
    viz_id: String,
    validation_type: String,
    content: String,
    context: Option<String>,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        timestamp: Utc::now(),
    })
}

async fn get_dashboard_summary(
    Extension(state): Extension<Arc<AppState>>,
) -> Json<ApiResponse<DashboardSummary>> {
    let metrics = {
        let tracker = state.tracker.read().unwrap();
        tracker.get_metrics()
    };
    
    // Create mock community data for the dashboard
    let community_data = skill_development::ml::CommunityData {
        member_count: 1500,
        skill_distribution: HashMap::new(),
        activity_levels: HashMap::new(),
        resource_availability: HashMap::new(),
    };
    
    let summary = {
        let dashboard = state.dashboard.read().unwrap();
        dashboard.generate_summary(&community_data)
    };
    
    Json(ApiResponse {
        success: true,
        data: Some(summary),
        error: None,
    })
}

async fn track_engagement(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<TrackEngagementRequest>,
) -> Json<ApiResponse<()>> {
    let viz_type = match payload.viz_type.as_str() {
        "comparative" => VisualizationType::Comparative,
        "trend_based" => VisualizationType::TrendBased,
        "narrative" => VisualizationType::Narrative,
        _ => VisualizationType::Comparative,
    };
    
    let result = {
        let mut tracker = state.tracker.write().unwrap();
        tracker.track_visualization_engagement(
            &payload.user_id,
            &payload.component_id,
            viz_type,
            payload.time_spent,
            payload.interaction_count,
            payload.quality_score,
            payload.confidence_score,
        )
    };
    
    match result {
        Ok(_) => Json(ApiResponse {
            success: true,
            data: Some(()),
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            data: None,
            error: Some(format!("Failed to track engagement: {}", e)),
        }),
    }
}

async fn track_cause_engagement(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<TrackCauseEngagementRequest>,
) -> Json<ApiResponse<()>> {
    let result = {
        let mut tracker = state.tracker.write().unwrap();
        tracker.track_engagement_correlation(
            &payload.user_id,
            payload.viz_usage,
            payload.engaged,
            payload.months_engaged.map(|m| m as f64),
            payload.satisfaction_rating.map(|r| r as u8),
            payload.contribution_amount,
        )
    };
    
    match result {
        Ok(_) => Json(ApiResponse {
            success: true,
            data: Some(()),
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            data: None,
            error: Some(format!("Failed to track cause engagement: {}", e)),
        }),
    }
}

async fn record_validation(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<RecordValidationRequest>,
) -> Json<ApiResponse<()>> {
    let validation_type = match payload.validation_type.as_str() {
        "endorsement" => ValidationType::Endorsement,
        "suggestion" => ValidationType::Suggestion,
        "critique" => ValidationType::Critique,
        _ => ValidationType::Endorsement,
    };
    
    let result = {
        let mut tracker = state.tracker.write().unwrap();
        tracker.record_community_validation(
            &payload.user_id,
            &payload.viz_id,
            validation_type,
            &payload.content,
            payload.context,
        )
    };
    
    match result {
        Ok(_) => Json(ApiResponse {
            success: true,
            data: Some(()),
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            data: None,
            error: Some(format!("Failed to record validation: {}", e)),
        }),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    // Create application state
    let tracker = CauseImpactTracker::new(DataSharingLevel::Standard);
    let dashboard = ImpactAnalyticsDashboard::new(tracker.get_metrics().clone());
    let feedback = FeedbackCollector::new(DataSharingLevel::Standard);
    
    let state = Arc::new(AppState {
        tracker: Arc::new(RwLock::new(tracker)),
        dashboard: Arc::new(RwLock::new(dashboard)),
        feedback: Arc::new(RwLock::new(feedback)),
    });
    
    // Build router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/dashboard/summary", get(get_dashboard_summary))
        .route("/api/tracking/engagement", post(track_engagement))
        .route("/api/tracking/cause-engagement", post(track_cause_engagement))
        .route("/api/tracking/validation", post(record_validation))
        .layer(CorsLayer::permissive())
        .layer(Extension(state));
    
    // Run server
    let listener = TcpListener::bind("0.0.0.0:3004").await?;
    println!("Cause Management Admin Dashboard listening on http://0.0.0.0:3004");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}