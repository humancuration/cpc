use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use cpc_core::services::governance::GovernanceService;
use crate::auth::{AuthenticatedUser, require_role};
use crate::routes::social::ApiResponse;

#[derive(Debug, Deserialize)]
pub struct CreateProposalRequest {
    pub title: String,
    pub description: String,
    pub proposal_type: String,
    pub voting_deadline: chrono::DateTime<chrono::Utc>,
    pub options: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct VoteRequest {
    pub option_index: usize,
    pub weight: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProposalRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub voting_deadline: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn create_proposal(
    State(governance_service): State<Arc<GovernanceService>>,
    user: AuthenticatedUser,
    Json(request): Json<CreateProposalRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    // Check if user has permission to create proposals
    require_role("proposal_creator")(user.clone())?;
    
    match governance_service.create_proposal(
        user.user_id,
        request.title,
        request.description,
        request.proposal_type.parse().map_err(|_| StatusCode::BAD_REQUEST)?,
        request.voting_deadline,
        request.options,
    ).await {
        Ok(proposal) => Ok(Json(ApiResponse::success(serde_json::to_value(proposal).unwrap()))),
        Err(e) => {
            tracing::error!("Failed to create proposal: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_proposal(
    State(governance_service): State<Arc<GovernanceService>>,
    user: Option<AuthenticatedUser>,
    Path(proposal_id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let viewer_id = user.map(|u| u.user_id);
    
    match governance_service.get_proposal(proposal_id, viewer_id).await {
        Ok(Some(proposal)) => Ok(Json(ApiResponse::success(serde_json::to_value(proposal).unwrap()))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get proposal: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_proposal(
    State(governance_service): State<Arc<GovernanceService>>,
    user: AuthenticatedUser,
    Path(proposal_id): Path<Uuid>,
    Json(request): Json<UpdateProposalRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    match governance_service.update_proposal(
        proposal_id,
        user.user_id,
        request.title,
        request.description,
        request.voting_deadline,
    ).await {
        Ok(proposal) => Ok(Json(ApiResponse::success(serde_json::to_value(proposal).unwrap()))),
        Err(e) => {
            tracing::error!("Failed to update proposal: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn vote_on_proposal(
    State(governance_service): State<Arc<GovernanceService>>,
    user: AuthenticatedUser,
    Path(proposal_id): Path<Uuid>,
    Json(request): Json<VoteRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    match governance_service.vote_on_proposal(
        proposal_id,
        user.user_id,
        request.option_index,
        request.weight,
    ).await {
        Ok(vote) => Ok(Json(ApiResponse::success(serde_json::to_value(vote).unwrap()))),
        Err(e) => {
            tracing::error!("Failed to vote on proposal: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_proposal_results(
    State(governance_service): State<Arc<GovernanceService>>,
    user: Option<AuthenticatedUser>,
    Path(proposal_id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let viewer_id = user.map(|u| u.user_id);
    
    match governance_service.get_proposal_results(proposal_id, viewer_id).await {
        Ok(results) => Ok(Json(ApiResponse::success(serde_json::to_value(results).unwrap()))),
        Err(e) => {
            tracing::error!("Failed to get proposal results: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_active_proposals(
    State(governance_service): State<Arc<GovernanceService>>,
    user: Option<AuthenticatedUser>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let viewer_id = user.map(|u| u.user_id);
    
    match governance_service.get_active_proposals(viewer_id, 20, 0).await {
        Ok(proposals) => Ok(Json(ApiResponse::success(serde_json::to_value(proposals).unwrap()))),
        Err(e) => {
            tracing::error!("Failed to get active proposals: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_user_votes(
    State(governance_service): State<Arc<GovernanceService>>,
    user: AuthenticatedUser,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    match governance_service.get_user_votes(user.user_id, 20, 0).await {
        Ok(votes) => Ok(Json(ApiResponse::success(serde_json::to_value(votes).unwrap()))),
        Err(e) => {
            tracing::error!("Failed to get user votes: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn close_proposal(
    State(governance_service): State<Arc<GovernanceService>>,
    user: AuthenticatedUser,
    Path(proposal_id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    // Check if user has permission to close proposals
    require_role("proposal_moderator")(user.clone())?;
    
    match governance_service.close_proposal(proposal_id, user.user_id).await {
        Ok(proposal) => Ok(Json(ApiResponse::success(serde_json::to_value(proposal).unwrap()))),
        Err(e) => {
            tracing::error!("Failed to close proposal: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub fn router() -> Router<Arc<GovernanceService>> {
    Router::new()
        .route("/proposals", post(create_proposal))
        .route("/proposals", get(get_active_proposals))
        .route("/proposals/:id", get(get_proposal))
        .route("/proposals/:id", put(update_proposal))
        .route("/proposals/:id/vote", post(vote_on_proposal))
        .route("/proposals/:id/results", get(get_proposal_results))
        .route("/proposals/:id/close", put(close_proposal))
        .route("/votes", get(get_user_votes))
}