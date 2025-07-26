use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use cpc_core::services::{
    identity::IdentityService,
    social::SocialService,
    forum::ForumService,
    governance::GovernanceService,
};
use crate::db::DbPool;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub services: ServiceHealthStatus,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct ServiceHealthStatus {
    pub database: ServiceStatus,
    pub identity: ServiceStatus,
    pub social: ServiceStatus,
    pub forum: ServiceStatus,
    pub governance: ServiceStatus,
}

#[derive(Debug, Serialize)]
pub struct ServiceStatus {
    pub status: String,
    pub message: Option<String>,
    pub response_time_ms: u64,
}

impl ServiceStatus {
    pub fn healthy(response_time_ms: u64) -> Self {
        Self {
            status: "healthy".to_string(),
            message: None,
            response_time_ms,
        }
    }

    pub fn unhealthy(message: String, response_time_ms: u64) -> Self {
        Self {
            status: "unhealthy".to_string(),
            message: Some(message),
            response_time_ms,
        }
    }
}

pub async fn detailed_health_check(
    State(db): State<DbPool>,
    State(identity_service): State<Arc<IdentityService>>,
    State(social_service): State<Arc<SocialService>>,
    State(forum_service): State<Arc<ForumService>>,
    State(governance_service): State<Arc<GovernanceService>>,
) -> Result<Json<HealthResponse>, StatusCode> {
    let start_time = std::time::Instant::now();

    // Check database health
    let db_status = check_database_health(&db).await;
    
    // Check service health
    let identity_status = check_identity_service_health(&identity_service).await;
    let social_status = check_social_service_health(&social_service).await;
    let forum_status = check_forum_service_health(&forum_service).await;
    let governance_status = check_governance_service_health(&governance_service).await;

    let services = ServiceHealthStatus {
        database: db_status,
        identity: identity_status,
        social: social_status,
        forum: forum_status,
        governance: governance_status,
    };

    // Determine overall status
    let overall_status = if services.database.status == "healthy" 
        && services.identity.status == "healthy"
        && services.social.status == "healthy"
        && services.forum.status == "healthy"
        && services.governance.status == "healthy" {
        "healthy"
    } else {
        "degraded"
    };

    let response = HealthResponse {
        status: overall_status.to_string(),
        services,
        timestamp: chrono::Utc::now(),
    };

    if overall_status == "healthy" {
        Ok(Json(response))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

async fn check_database_health(db: &DbPool) -> ServiceStatus {
    let start = std::time::Instant::now();
    
    match sqlx::query("SELECT 1").fetch_one(db).await {
        Ok(_) => ServiceStatus::healthy(start.elapsed().as_millis() as u64),
        Err(e) => ServiceStatus::unhealthy(
            format!("Database connection failed: {}", e),
            start.elapsed().as_millis() as u64,
        ),
    }
}

async fn check_identity_service_health(service: &IdentityService) -> ServiceStatus {
    let start = std::time::Instant::now();
    
    // Try to perform a simple operation to verify service health
    match service.health_check().await {
        Ok(_) => ServiceStatus::healthy(start.elapsed().as_millis() as u64),
        Err(e) => ServiceStatus::unhealthy(
            format!("Identity service error: {}", e),
            start.elapsed().as_millis() as u64,
        ),
    }
}

async fn check_social_service_health(service: &SocialService) -> ServiceStatus {
    let start = std::time::Instant::now();
    
    match service.health_check().await {
        Ok(_) => ServiceStatus::healthy(start.elapsed().as_millis() as u64),
        Err(e) => ServiceStatus::unhealthy(
            format!("Social service error: {}", e),
            start.elapsed().as_millis() as u64,
        ),
    }
}

async fn check_forum_service_health(service: &ForumService) -> ServiceStatus {
    let start = std::time::Instant::now();
    
    match service.health_check().await {
        Ok(_) => ServiceStatus::healthy(start.elapsed().as_millis() as u64),
        Err(e) => ServiceStatus::unhealthy(
            format!("Forum service error: {}", e),
            start.elapsed().as_millis() as u64,
        ),
    }
}

async fn check_governance_service_health(service: &GovernanceService) -> ServiceStatus {
    let start = std::time::Instant::now();
    
    match service.health_check().await {
        Ok(_) => ServiceStatus::healthy(start.elapsed().as_millis() as u64),
        Err(e) => ServiceStatus::unhealthy(
            format!("Governance service error: {}", e),
            start.elapsed().as_millis() as u64,
        ),
    }
}

pub fn router() -> Router<(
    DbPool,
    Arc<IdentityService>,
    Arc<SocialService>,
    Arc<ForumService>,
    Arc<GovernanceService>,
)> {
    Router::new()
        .route("/detailed", get(detailed_health_check))
}