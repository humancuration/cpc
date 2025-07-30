use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::auth_service::AuthService;
use crate::domain::credentials::Credentials;
use crate::domain::auth_error::AuthError;
use crate::infrastructure::consent_manager::YapperConsentManager;
use cpc_consent::ConsentLevel;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub consent_level: ConsentLevel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub session_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordResetRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordResetConfirmRequest {
    pub token: String,
    pub new_password: String,
}

pub async fn register(
    State((_, _, consent_manager)): State<(Arc<dyn AuthService>, Arc<YapperConsentManager>)>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), (StatusCode, String)> {
    let credentials = Credentials::new(payload.email, payload.password);
    
    match auth_service.register(credentials).await {
        Ok(user) => {
            // Set user consent
            consent_manager.set_consent(user.id, payload.consent_level);
            
            // For demo purposes, we'll create a session immediately after registration
            // In a real implementation, we would require email verification first
            let session = crate::domain::session::Session::new(user.id, "registration".to_string());
            Ok((
                StatusCode::CREATED,
                Json(AuthResponse {
                    session_id: session.id,
                    user_id: user.id,
                }),
            ))
        }
        Err(AuthError::DatabaseError(msg)) => Err((StatusCode::INTERNAL_SERVER_ERROR, msg)),
        Err(AuthError::InvalidCredentials) => Err((StatusCode::BAD_REQUEST, "Invalid credentials".to_string())),
        Err(_) => Err((StatusCode::BAD_REQUEST, "Registration failed".to_string())),
    }
}

pub async fn login(
    State((auth_service, _, _)): State<(Arc<dyn AuthService>, Arc<YapperConsentManager>)>,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), (StatusCode, String)> {
    let credentials = Credentials::new(payload.email, payload.password);
    
    match auth_service.login(credentials).await {
        Ok(session) => Ok((
            StatusCode::OK,
            Json(AuthResponse {
                session_id: session.id,
                user_id: session.user_id,
            }),
        )),
        Err(AuthError::InvalidCredentials) => Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string())),
        Err(AuthError::AccountNotVerified) => Err((StatusCode::FORBIDDEN, "Account not verified".to_string())),
        Err(AuthError::DatabaseError(msg)) => Err((StatusCode::INTERNAL_SERVER_ERROR, msg)),
        Err(_) => Err((StatusCode::UNAUTHORIZED, "Login failed".to_string())),
    }
}

pub async fn logout(
    State((auth_service, _, _)): State<(Arc<dyn AuthService>, Arc<YapperConsentManager>)>,
    Json(payload): Json<AuthResponse>,
) -> Result<StatusCode, (StatusCode, String)> {
    match auth_service.logout(payload.session_id).await {
        Ok(()) => Ok(StatusCode::OK),
        Err(AuthError::DatabaseError(msg)) => Err((StatusCode::INTERNAL_SERVER_ERROR, msg)),
        Err(_) => Err((StatusCode::BAD_REQUEST, "Logout failed".to_string())),
    }
}

pub async fn initiate_password_reset(
    State((auth_service, _, _)): State<(Arc<dyn AuthService>, Arc<YapperConsentManager>)>,
    Json(payload): Json<PasswordResetRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    match auth_service.initiate_password_reset(payload.email).await {
        Ok(()) => Ok(StatusCode::OK),
        Err(AuthError::UserNotFound) => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
        Err(AuthError::DatabaseError(msg)) => Err((StatusCode::INTERNAL_SERVER_ERROR, msg)),
        Err(_) => Err((StatusCode::BAD_REQUEST, "Password reset initiation failed".to_string())),
    }
}

pub async fn confirm_password_reset(
    State((auth_service, _, _)): State<(Arc<dyn AuthService>, Arc<YapperConsentManager>)>,
    Json(payload): Json<PasswordResetConfirmRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    match auth_service.confirm_password_reset(payload.token, payload.new_password).await {
        Ok(()) => Ok(StatusCode::OK),
        Err(AuthError::TokenInvalid) => Err((StatusCode::BAD_REQUEST, "Invalid token".to_string())),
        Err(AuthError::UserNotFound) => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
        Err(AuthError::DatabaseError(msg)) => Err((StatusCode::INTERNAL_SERVER_ERROR, msg)),
        Err(_) => Err((StatusCode::BAD_REQUEST, "Password reset confirmation failed".to_string())),
    }
}