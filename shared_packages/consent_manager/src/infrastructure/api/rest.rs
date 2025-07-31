//! REST API implementation for the consent manager.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::{
    domain::{
        consent::{DataSharingLevel, Domain},
        audit::Actor,
    },
    application::service::ConsentService,
};

/// REST API state
#[derive(Clone)]
pub struct AppState {
    pub consent_service: Arc<ConsentService>,
}

/// Request to get consent level
#[derive(Deserialize)]
pub struct GetConsentLevelRequest {
    pub user_id: String,
    pub domain: String,
}

/// Response for consent level
#[derive(Serialize)]
pub struct ConsentLevelResponse {
    pub level: String,
}

/// Request to update consent level
#[derive(Deserialize)]
pub struct UpdateConsentLevelRequest {
    pub user_id: String,
    pub domain: String,
    pub level: String,
    pub actor_type: String,
    pub actor_id: String,
}

/// Response for update operations
#[derive(Serialize)]
pub struct UpdateResponse {
    pub success: bool,
}

/// Request to revoke domain consent
#[derive(Deserialize)]
pub struct RevokeDomainRequest {
    pub user_id: String,
    pub domain: String,
    pub actor_type: String,
    pub actor_id: String,
}

/// Audit event response
#[derive(Serialize)]
pub struct AuditEventResponse {
    pub id: String,
    pub user_id: String,
    pub domain: String,
    pub action: String,
    pub previous_level: Option<String>,
    pub new_level: String,
    pub actor_type: String,
    pub actor_id: String,
    pub timestamp: String,
}

/// Response for audit events
#[derive(Serialize)]
pub struct AuditEventsResponse {
    pub events: Vec<AuditEventResponse>,
}

/// Create the REST API router
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/consent/:user_id/:domain", get(get_consent_level))
        .route("/consent", post(update_consent_level))
        .route("/consent/revoke", delete(revoke_domain))
        .route("/consent/audit/:user_id", get(get_audit_events))
        .with_state(state)
}

/// Get consent level for a user and domain
pub async fn get_consent_level(
    Path((user_id, domain)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Result<Json<ConsentLevelResponse>, StatusCode> {
    // Convert domain string to enum
    let domain_enum = match domain.as_str() {
        "financial" => Domain::FinancialData,
        "health" => Domain::HealthData,
        "calendar" => Domain::CalendarData,
        "crm" => Domain::CrmData,
        "scm" => Domain::ScmData,
        "document" => Domain::DocumentData,
        "website" => Domain::WebsiteData,
        "recruitment" => Domain::RecruitmentData,
        "datalakehouse" => Domain::DataLakehouse,
        "forecasting" => Domain::ForecastingData,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    // Get consent level
    let level = state.consent_service
        .get_consent_level(&user_id, domain_enum)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Convert level to string
    let level_str = match level {
        DataSharingLevel::None => "none",
        DataSharingLevel::Minimal => "minimal",
        DataSharingLevel::Standard => "standard",
        DataSharingLevel::Full => "full",
    };

    Ok(Json(ConsentLevelResponse {
        level: level_str.to_string(),
    }))
}

/// Update consent level
pub async fn update_consent_level(
    State(state): State<AppState>,
    Json(payload): Json<UpdateConsentLevelRequest>,
) -> Result<Json<UpdateResponse>, StatusCode> {
    // Convert domain string to enum
    let domain_enum = match payload.domain.as_str() {
        "financial" => Domain::FinancialData,
        "health" => Domain::HealthData,
        "calendar" => Domain::CalendarData,
        "crm" => Domain::CrmData,
        "scm" => Domain::ScmData,
        "document" => Domain::DocumentData,
        "website" => Domain::WebsiteData,
        "recruitment" => Domain::RecruitmentData,
        "datalakehouse" => Domain::DataLakehouse,
        "forecasting" => Domain::ForecastingData,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    // Convert level string to enum
    let level_enum = match payload.level.as_str() {
        "none" => DataSharingLevel::None,
        "minimal" => DataSharingLevel::Minimal,
        "standard" => DataSharingLevel::Standard,
        "full" => DataSharingLevel::Full,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    // Convert actor
    let actor = match payload.actor_type.as_str() {
        "user" => Actor::User(payload.actor_id),
        "service" => Actor::Service(payload.actor_id),
        "admin" => Actor::Admin(payload.actor_id),
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    // Update consent level
    state.consent_service
        .update_consent_level(&payload.user_id, domain_enum, level_enum, actor)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(UpdateResponse { success: true }))
}

/// Revoke domain consent
pub async fn revoke_domain(
    State(state): State<AppState>,
    Json(payload): Json<RevokeDomainRequest>,
) -> Result<Json<UpdateResponse>, StatusCode> {
    // Convert domain string to enum
    let domain_enum = match payload.domain.as_str() {
        "financial" => Domain::FinancialData,
        "health" => Domain::HealthData,
        "calendar" => Domain::CalendarData,
        "crm" => Domain::CrmData,
        "scm" => Domain::ScmData,
        "document" => Domain::DocumentData,
        "website" => Domain::WebsiteData,
        "recruitment" => Domain::RecruitmentData,
        "datalakehouse" => Domain::DataLakehouse,
        "forecasting" => Domain::ForecastingData,
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    // Convert actor
    let actor = match payload.actor_type.as_str() {
        "user" => Actor::User(payload.actor_id),
        "service" => Actor::Service(payload.actor_id),
        "admin" => Actor::Admin(payload.actor_id),
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    // Revoke domain
    state.consent_service
        .revoke_domain(&payload.user_id, domain_enum, actor)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(UpdateResponse { success: true }))
}

/// Get audit events for a user
pub async fn get_audit_events(
    Path(user_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<AuditEventsResponse>, StatusCode> {
    // Get audit events
    let events = state.consent_service
        .get_audit_events(&user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Convert to response format
    let response_events: Vec<AuditEventResponse> = events
        .into_iter()
        .map(|event| {
            let domain_str = match event.domain {
                Domain::FinancialData => "financial",
                Domain::HealthData => "health",
                Domain::CalendarData => "calendar",
                Domain::CrmData => "crm",
                Domain::ScmData => "scm",
                Domain::DocumentData => "document",
                Domain::WebsiteData => "website",
                Domain::RecruitmentData => "recruitment",
                Domain::DataLakehouse => "datalakehouse",
                Domain::ForecastingData => "forecasting",
            };

            let action_str = match event.action {
                crate::domain::audit::ConsentAction::Granted => "granted",
                crate::domain::audit::ConsentAction::Revoked => "revoked",
                crate::domain::audit::ConsentAction::Modified => "modified",
            };

            let previous_level_str = event.previous_level.map(|level| match level {
                DataSharingLevel::None => "none",
                DataSharingLevel::Minimal => "minimal",
                DataSharingLevel::Standard => "standard",
                DataSharingLevel::Full => "full",
            });

            let new_level_str = match event.new_level {
                DataSharingLevel::None => "none",
                DataSharingLevel::Minimal => "minimal",
                DataSharingLevel::Standard => "standard",
                DataSharingLevel::Full => "full",
            };

            let (actor_type_str, actor_id_str) = match event.actor {
                Actor::User(id) => ("user", id),
                Actor::Service(name) => ("service", name),
                Actor::Admin(id) => ("admin", id),
            };

            AuditEventResponse {
                id: event.id,
                user_id: event.user_id,
                domain: domain_str.to_string(),
                action: action_str.to_string(),
                previous_level: previous_level_str,
                new_level: new_level_str.to_string(),
                actor_type: actor_type_str.to_string(),
                actor_id: actor_id_str,
                timestamp: event.timestamp.to_rfc3339(),
            }
        })
        .collect();

    Ok(Json(AuditEventsResponse {
        events: response_events,
    }))
}