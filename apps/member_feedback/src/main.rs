//! Member Feedback App
//!
//! A web application for community members to provide feedback on financial visualizations
//! and contribute to continuous improvement efforts.

use axum::{
    routing::{get, post},
    Router, Json, Extension,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use std::sync::{Arc, RwLock};

use financial_impact_tracker::{
    FeedbackCollector,
    feedback::{QuickFeedback, DetailedFeedback},
};
use consent_manager::domain::consent::DataSharingLevel;
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Clone)]
struct AppState {
    feedback_collector: Arc<RwLock<FeedbackCollector>>,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    timestamp: chrono::DateTime<Utc>,
}

#[derive(Deserialize)]
struct QuickFeedbackRequest {
    user_id: String,
    component_id: String,
    helpful: bool,
}

#[derive(Deserialize)]
struct DetailedFeedbackRequest {
    user_id: String,
    component_id: String,
    rating: u32,
    comment: Option<String>,
    helpful: bool,
    impact_rating: Option<u32>,
    understanding_rating: Option<u32>,
    confidence_rating: Option<u32>,
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

async fn submit_quick_feedback(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<QuickFeedbackRequest>,
) -> Json<ApiResponse<()>> {
    let result = {
        let mut collector = state.feedback_collector.write().unwrap();
        collector.collect_quick_feedback(
            &payload.user_id,
            &payload.component_id,
            payload.helpful,
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
            error: Some(format!("Failed to collect feedback: {}", e)),
        }),
    }
}

async fn submit_detailed_feedback(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<DetailedFeedbackRequest>,
) -> Json<ApiResponse<()>> {
    let result = {
        let mut collector = state.feedback_collector.write().unwrap();
        collector.collect_detailed_feedback(
            &payload.user_id,
            &payload.component_id,
            payload.rating,
            payload.comment,
            payload.helpful,
            payload.impact_rating,
            payload.understanding_rating,
            payload.confidence_rating,
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
            error: Some(format!("Failed to collect detailed feedback: {}", e)),
        }),
    }
}

async fn get_feedback_stats(
    Extension(state): Extension<Arc<AppState>>,
) -> Json<ApiResponse<FeedbackStats>> {
    let stats = {
        let collector = state.feedback_collector.read().unwrap();
        let feedback = collector.get_feedback();
        
        FeedbackStats {
            total_quick_feedback: feedback.quick_feedback.len(),
            total_detailed_feedback: feedback.detailed_feedback.len(),
            avg_rating: calculate_avg_rating(&feedback.detailed_feedback),
            helpful_percentage: calculate_helpful_percentage(&feedback.quick_feedback, &feedback.detailed_feedback),
        }
    };
    
    Json(ApiResponse {
        success: true,
        data: Some(stats),
        error: None,
    })
}

#[derive(Serialize)]
struct FeedbackStats {
    total_quick_feedback: usize,
    total_detailed_feedback: usize,
    avg_rating: f64,
    helpful_percentage: f64,
}

fn calculate_avg_rating(detailed_feedback: &[DetailedFeedback]) -> f64 {
    if detailed_feedback.is_empty() {
        0.0
    } else {
        let sum: u32 = detailed_feedback.iter().map(|f| f.rating).sum();
        sum as f64 / detailed_feedback.len() as f64
    }
}

fn calculate_helpful_percentage(quick_feedback: &[QuickFeedback], detailed_feedback: &[DetailedFeedback]) -> f64 {
    let total = quick_feedback.len() + detailed_feedback.len();
    if total == 0 {
        0.0
    } else {
        let helpful_count = quick_feedback.iter().filter(|f| f.helpful).count() +
            detailed_feedback.iter().filter(|f| f.helpful).count();
        (helpful_count as f64 / total as f64) * 100.0
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    // Create application state
    let feedback_collector = FeedbackCollector::new(DataSharingLevel::Standard);
    
    let state = Arc::new(AppState {
        feedback_collector: Arc::new(RwLock::new(feedback_collector)),
    });
    
    // Build router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/feedback/quick", post(submit_quick_feedback))
        .route("/api/feedback/detailed", post(submit_detailed_feedback))
        .route("/api/feedback/stats", get(get_feedback_stats))
        .layer(CorsLayer::permissive())
        .layer(Extension(state));
    
    // Run server
    let listener = TcpListener::bind("0.0.0.0:3004").await?;
    println!("Member Feedback App listening on http://0.0.0.0:3004");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}