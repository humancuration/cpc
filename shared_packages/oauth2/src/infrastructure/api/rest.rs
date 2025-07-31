//! REST API handlers for OAuth2

use axum::{
    extract::{Query, Path},
    response::Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use crate::domain::{OAuthProvider, AuthError};
use crate::application::AuthService;
use tracing::{info, error};

/// REST API handlers for OAuth2

/// Request to start OAuth authentication
#[derive(Deserialize)]
pub struct StartAuthRequest {
    pub provider: String,
    pub redirect_uri: Option<String>,
}

/// Response for starting OAuth authentication
#[derive(Serialize)]
pub struct StartAuthResponse {
    pub auth_url: String,
    pub state: String,
}

/// Query parameters for OAuth callback
#[derive(Deserialize)]
pub struct CallbackQuery {
    pub code: String,
    pub state: String,
}

/// Response for handling OAuth callback
#[derive(Serialize)]
pub struct CallbackResponse {
    pub user_id: String,
    pub success: bool,
    pub message: String,
}

/// Start OAuth authentication flow
pub async fn start_auth(
    axum::extract::Extension(auth_service): axum::extract::Extension<std::sync::Arc<AuthService>>,
    axum::extract::Extension(user_id): axum::extract::Extension<Uuid>,
    axum::extract::Json(request): axum::extract::Json<StartAuthRequest>,
) -> Result<Json<StartAuthResponse>, (StatusCode, String)> {
    info!(user_id = %user_id, provider = %request.provider, "REST StartAuth request received");
    
    let provider = OAuthProvider::from_str(&request.provider)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid provider: {}", e)))?;
    
    match auth_service.start_auth(user_id, provider, request.redirect_uri).await {
        Ok(auth_request) => {
            let response = StartAuthResponse {
                auth_url: auth_request.auth_url,
                state: auth_request.state,
            };
            Ok(Json(response))
        }
        Err(e) => {
            error!(error = %e, "Failed to start OAuth authentication");
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to start OAuth authentication: {}", e)))
        }
    }
}

/// Handle OAuth callback
pub async fn handle_callback(
    axum::extract::Extension(auth_service): axum::extract::Extension<std::sync::Arc<AuthService>>,
    axum::extract::Query(query): axum::extract::Query<CallbackQuery>,
) -> Result<Json<CallbackResponse>, (StatusCode, String)> {
    info!(code = %query.code, state = %query.state, "REST HandleCallback request received");
    
    match auth_service.handle_callback(query.code, query.state).await {
        Ok((user_id, _token, _profile)) => {
            let response = CallbackResponse {
                user_id: user_id.to_string(),
                success: true,
                message: "Authentication successful".to_string(),
            };
            Ok(Json(response))
        }
        Err(AuthError::InvalidState) => {
            Err((StatusCode::BAD_REQUEST, "Invalid state parameter".to_string()))
        }
        Err(AuthError::InvalidAuthorizationCode) => {
            Err((StatusCode::BAD_REQUEST, "Invalid authorization code".to_string()))
        }
        Err(e) => {
            error!(error = %e, "Failed to handle OAuth callback");
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to handle OAuth callback: {}", e)))
        }
    }
}

/// Refresh OAuth token
pub async fn refresh_token(
    axum::extract::Extension(auth_service): axum::extract::Extension<std::sync::Arc<AuthService>>,
    axum::extract::Extension(user_id): axum::extract::Extension<Uuid>,
    axum::extract::Path(provider): axum::extract::Path<String>,
) -> Result<Json<HashMap<String, String>>, (StatusCode, String)> {
    info!(user_id = %user_id, provider = %provider, "REST RefreshToken request received");
    
    let provider = OAuthProvider::from_str(&provider)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid provider: {}", e)))?;
    
    match auth_service.refresh_token(user_id, provider).await {
        Ok(_token) => {
            let mut response = HashMap::new();
            response.insert("status".to_string(), "success".to_string());
            response.insert("message".to_string(), "Token refreshed successfully".to_string());
            Ok(Json(response))
        }
        Err(AuthError::TokenExpired) => {
            Err((StatusCode::UNAUTHORIZED, "Token expired".to_string()))
        }
        Err(AuthError::TokenRefreshFailed(msg)) => {
            Err((StatusCode::PRECONDITION_FAILED, msg))
        }
        Err(e) => {
            error!(error = %e, "Failed to refresh OAuth token");
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to refresh OAuth token: {}", e)))
        }
    }
}