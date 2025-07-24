//! Impact report routes for the BI toolkit

use axum::{
    extract::{Path, Json},
    http::StatusCode,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::bi::BIService;

/// Request payload for impact report generation
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateReportRequest {
    pub user_id: Uuid,
}

/// Response payload for impact report
#[derive(Debug, Serialize, Deserialize)]
pub struct ImpactReportResponse {
    pub user_id: String,
    pub total_impact: f64,
    pub breakdown: Vec<ImpactBreakdown>,
    pub distribution: Vec<ImpactDistribution>,
    pub timeline: Vec<ImpactTimelinePoint>,
    pub generated_at: String,
}

/// Individual breakdown item response
#[derive(Debug, Serialize, Deserialize)]
pub struct ImpactBreakdown {
    pub category: String,
    pub amount: f64,
    pub item_name: String,
    pub contribution: f64,
    pub impact_score: f64,
}

/// Distribution data response
#[derive(Debug, Serialize, Deserialize)]
pub struct ImpactDistribution {
    pub category: String,
    pub weight: f64,
}

/// Timeline point response
#[derive(Debug, Serialize, Deserialize)]
pub struct ImpactTimelinePoint {
    pub date: String,
    pub description: String,
    pub impact_value: f64,
    pub timestamp: u64,
    pub score: f64,
}

/// Processing status response
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingStatusResponse {
    pub job_id: String,
    pub status: String,
    pub progress: f64,
    pub message: Option<String>,
    pub estimated_completion: Option<String>,
}

/// Get impact report for a user
pub async fn get_impact_report(
    Path(user_id): Path<Uuid>,
    bi_service: axum::extract::State<Arc<BIService>>,
) -> Result<Json<ImpactReportResponse>, StatusCode> {
    match bi_service.get_impact_report(user_id).await {
        Ok(report) => {
            let response = ImpactReportResponse {
                user_id: report.user_id.to_string(),
                total_impact: report.overall_score,
                breakdown: report.breakdown.into_iter()
                    .map(|item| ImpactBreakdown {
                        category: format!("{:?}", item.category),
                        amount: item.value,
                        item_name: item.name,
                        contribution: 0.0,
                        impact_score: item.ethical_score * 10.0,
                    })
                    .collect(),
                distribution: report.ethical_distribution.into_iter()
                    .map(|(category, weight)| ImpactDistribution {
                        category: format!("{:?}", category),
                        weight,
                    })
                    .collect(),
                timeline: report.timeline.into_iter()
                    .enumerate()
                    .map(|(i, point)| ImpactTimelinePoint {
                        date: point.timestamp.format("%Y-%m-%d").to_string(),
                        description: format!("Impact in {:?} category", point.category),
                        impact_value: point.value,
                        timestamp: point.timestamp.timestamp_millis() as u64,
                        score: 7.5 + (i as f64 * 0.5),
                    })
                    .collect(),
                generated_at: report.generated_at.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
            };
            
            Ok(Json(response))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Generate a new impact report for a user
pub async fn generate_impact_report(
    Json(payload): Json<GenerateReportRequest>,
    bi_service: axum::extract::State<Arc<BIService>>,
) -> Result<Json<ProcessingStatusResponse>, StatusCode> {
    match bi_service.generate_impact_report(payload.user_id).await {
        Ok(status) => {
            let response = ProcessingStatusResponse {
                job_id: status.job_id.to_string(),
                status: format!("{:?}", status.status).to_lowercase(),
                progress: status.progress,
                message: status.message,
                estimated_completion: status.estimated_completion
                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()),
            };
            
            Ok(Json(response))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Get processing status for a job
pub async fn get_processing_status(
    Path(job_id): Path<Uuid>,
    bi_service: axum::extract::State<Arc<BIService>>,
) -> Result<Json<ProcessingStatusResponse>, StatusCode> {
    match bi_service.get_processing_status(job_id).await {
        Ok(status) => {
            let response = ProcessingStatusResponse {
                job_id: status.job_id.to_string(),
                status: format!("{:?}", status.status).to_lowercase(),
                progress: status.progress,
                message: status.message,
                estimated_completion: status.estimated_completion
                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()),
            };
            
            Ok(Json(response))
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

/// Create router for impact-related routes
pub fn router() -> Router {
    Router::new()
        .route("/impact-report/:user_id", get(get_impact_report))
        .route("/generate-report", post(generate_impact_report))
        .route("/status/:job_id", get(get_processing_status))
}