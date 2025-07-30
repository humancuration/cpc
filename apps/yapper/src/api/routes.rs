use axum::{
    Router,
    routing::{post, get},
    extract::{Path, State},
    Json,
    http::StatusCode
};
use uuid::Uuid;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

use crate::application::post_service::PostService;
use crate::domain::post::{Yap, YapError};
use crate::domain::auth_service::AuthService;
use crate::api::handlers::auth_handler::{RegisterRequest, LoginRequest, PasswordResetRequest, PasswordResetConfirmRequest, register, login, logout, initiate_password_reset, confirm_password_reset};
use crate::api::handlers::oauth::{OAuthCallbackRequest, oauth_initiate, oauth_callback};
use crate::infrastructure::consent_manager::YapperConsentManager;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateYapRequest {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YapResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub like_count: u32,
    pub share_count: u32,
}

impl From<Yap> for YapResponse {
    fn from(yap: Yap) -> Self {
        Self {
            id: yap.id,
            user_id: yap.user_id,
            content: yap.content,
            created_at: yap.created_at,
            like_count: yap.like_count,
            share_count: yap.share_count,
        }
    }
}

pub fn routes(
    post_service: Arc<PostService>,
    auth_service: Arc<dyn AuthService>,
    consent_manager: Arc<YapperConsentManager>,
) -> Router {
    Router::new()
        .route("/yap", post(create_yap))
        .route("/yap/:id", get(get_yap))
        .route("/yap/:id/like", post(like_yap))
        .route("/yap/:id/share", post(share_yap))
        // Authentication routes
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/logout", post(logout))
        .route("/auth/password-reset", post(initiate_password_reset))
        .route("/auth/password-reset/confirm", post(confirm_password_reset))
        // OAuth routes
        .route("/oauth/initiate", post(oauth_initiate))
        .route("/oauth/callback", post(oauth_callback))
        .with_state((post_service, auth_service, consent_manager))
}

async fn create_yap(
    State((post_service, _auth_service, _consent_manager)): State<(Arc<PostService>, Arc<dyn AuthService>, Arc<YapperConsentManager>)>,
    Json(payload): Json<CreateYapRequest>,
) -> Result<(StatusCode, Json<YapResponse>), (StatusCode, String)> {
    // For demo purposes, we'll use a fixed user ID
    let user_id = Uuid::nil(); // In a real implementation, this would come from auth
    
    match post_service.create_yap(user_id, payload.content) {
        Ok(yap) => Ok((StatusCode::CREATED, Json(YapResponse::from(yap)))),
        Err(YapError::ContentTooLong) => Err((StatusCode::BAD_REQUEST, "Content too long".to_string())),
        Err(YapError::ContentEmpty) => Err((StatusCode::BAD_REQUEST, "Content empty".to_string())),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to create yap".to_string())),
    }
}

async fn get_yap(
    State((post_service, _auth_service, _consent_manager)): State<(Arc<PostService>, Arc<dyn AuthService>, Arc<YapperConsentManager>)>,
    Path(id): Path<Uuid>,
) -> Result<Json<Option<YapResponse>>, (StatusCode, String)> {
    match post_service.get_yap(id) {
        Ok(Some(yap)) => Ok(Json(Some(YapResponse::from(yap)))),
        Ok(None) => Ok(Json(None)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

async fn like_yap(
    State((post_service, _auth_service, _consent_manager)): State<(Arc<PostService>, Arc<dyn AuthService>, Arc<YapperConsentManager>)>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    // For demo purposes, we'll use a fixed user ID
    let user_id = Uuid::nil(); // In a real implementation, this would come from auth
    
    match post_service.like_yap(id, user_id) {
        Ok(()) => Ok(StatusCode::OK),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

async fn share_yap(
    State((post_service, _auth_service, _consent_manager)): State<(Arc<PostService>, Arc<dyn AuthService>, Arc<YapperConsentManager>)>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    // For demo purposes, we'll use a fixed user ID
    let user_id = Uuid::nil(); // In a real implementation, this would come from auth
    
    match post_service.share_yap(id, user_id) {
        Ok(()) => Ok(StatusCode::OK),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}