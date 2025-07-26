use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{post, get},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use jsonwebtoken::{encode, EncodingKey, Header, Algorithm};
// bcrypt is handled by the IdentityService
use cpc_core::services::identity::IdentityService;
use crate::auth::{Claims, AuthState};
use crate::routes::social::ApiResponse;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct SocialLoginRequest {
    pub provider: String,
    pub access_token: String,
}

#[derive(Debug, Deserialize)]
pub struct PasswordlessInitiateRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct PasswordlessVerifyRequest {
    pub email: String,
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub expires_at: DateTime<Utc>,
}

pub async fn register(
    State(identity_service): State<Arc<IdentityService>>,
    State(auth_state): State<Arc<AuthState>>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, StatusCode> {
    // Validate input
    if request.username.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    if request.email.trim().is_empty() || !request.email.contains('@') {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    if request.password.len() < 8 {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // Create user
    let new_user = cpc_core::models::user::NewUser {
        username: request.username.trim().to_string(),
        email: request.email.trim().to_lowercase(),
        password: request.password,
        display_name: None,
    };
    
    match identity_service.register(new_user).await {
        Ok((user, token)) => {
            let expires_at = Utc::now() + Duration::hours(24);
            
            let auth_response = AuthResponse {
                token,
                user_id: user.id,
                username: user.username,
                email: user.email,
                expires_at,
            };
            
            Ok(Json(ApiResponse::success(auth_response)))
        }
        Err(e) => {
            tracing::error!("Failed to create user: {:?}", e);
            // Check if it's a duplicate user error
            if e.to_string().contains("duplicate") || e.to_string().contains("unique") {
                Err(StatusCode::CONFLICT)
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

}

pub async fn social_login(
    State(identity_service): State<Arc<IdentityService>>,
    State(auth_state): State<Arc<AuthState>>,
    Json(request): Json<SocialLoginRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, StatusCode> {
    if request.provider.trim().is_empty() || request.access_token.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    match identity_service.social_login(&request.provider, &request.access_token).await {
        Ok((user, token)) => {
            let expires_at = Utc::now() + Duration::hours(24);
            
            let auth_response = AuthResponse {
                token,
                user_id: user.id,
                username: user.username,
                email: user.email,
                expires_at,
            };
            
            Ok(Json(ApiResponse::success(auth_response)))
        }
        Err(e) => {
            tracing::error!("Social login failed: {:?}", e);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

pub async fn passwordless_initiate(
    State(identity_service): State<Arc<IdentityService>>,
    Json(request): Json<PasswordlessInitiateRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    if request.email.trim().is_empty() || !request.email.contains('@') {
        return Err(StatusCode::BAD_REQUEST);
    }

    match identity_service.initiate_passwordless_login(&request.email).await {
        Ok(_) => Ok(Json(ApiResponse::success(()))),
        Err(e) => {
            tracing::error!("Failed to initiate passwordless login: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn passwordless_verify(
    State(identity_service): State<Arc<IdentityService>>,
    State(auth_state): State<Arc<AuthState>>,
    Json(request): Json<PasswordlessVerifyRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, StatusCode> {
    if request.email.trim().is_empty() || request.token.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    match identity_service.verify_passwordless_login(&request.email, &request.token).await {
        Ok((user, token)) => {
            let expires_at = Utc::now() + Duration::hours(24);
            
            let auth_response = AuthResponse {
                token,
                user_id: user.id,
                username: user.username,
                email: user.email,
                expires_at,
            };
            
            Ok(Json(ApiResponse::success(auth_response)))
        }
        Err(e) => {
            tracing::error!("Passwordless verification failed: {:?}", e);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}
pub async fn login(
    State(identity_service): State<Arc<IdentityService>>,
    State(auth_state): State<Arc<AuthState>>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, StatusCode> {
    // Validate input
    if request.email.trim().is_empty() || request.password.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // Login user
    match identity_service.login(&request.email.trim().to_lowercase(), &request.password).await {
        Ok((user, token)) => {
            let expires_at = Utc::now() + Duration::hours(24);
            
            let auth_response = AuthResponse {
                token,
                user_id: user.id,
                username: user.username,
                email: user.email,
                expires_at,
            };
            
            Ok(Json(ApiResponse::success(auth_response)))
        }
        Ok(None) => Err(StatusCode::UNAUTHORIZED),
        Err(e) => {
            tracing::error!("Failed to get user by email: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn refresh_token(
    State(identity_service): State<Arc<IdentityService>>,
    State(auth_state): State<Arc<AuthState>>,
    user: crate::auth::AuthenticatedUser,
) -> Result<Json<ApiResponse<AuthResponse>>, StatusCode> {
    // Get fresh user data
    match identity_service.get_user_by_id(user.user_id).await {
        Ok(Some(user_data)) => {
            // Generate new JWT token
            let expires_at = Utc::now() + Duration::hours(24);
            let claims = Claims {
                user_id: user.user_id,
                exp: expires_at.timestamp() as usize,
                iat: Utc::now().timestamp() as usize,
                roles: user.roles.clone(),
            };
            
            let token = encode(
                &Header::new(Algorithm::HS256),
                &claims,
                &EncodingKey::from_secret(auth_state.jwt_secret.as_bytes()),
            ).map_err(|e| {
                tracing::error!("Failed to generate JWT token: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
            
            let auth_response = AuthResponse {
                token,
                user_id: user_data.id,
                username: user_data.username,
                email: user_data.email,
                expires_at,
            };
            
            Ok(Json(ApiResponse::success(auth_response)))
        }
        Ok(None) => Err(StatusCode::UNAUTHORIZED),
        Err(e) => {
            tracing::error!("Failed to get user by ID: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }

}

pub fn router() -> Router<(Arc<IdentityService>, Arc<AuthState>)> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh_token))
        .route("/social/login", post(social_login))
        .route("/passwordless/initiate", post(passwordless_initiate))
        .route("/passwordless/verify", post(passwordless_verify))
}