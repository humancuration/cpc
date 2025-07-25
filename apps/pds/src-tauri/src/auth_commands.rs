use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use cpc_core::models::user::{User, UserProfile, UserRelationship, UserRelationshipType};
use cpc_core::services::identity::IdentityService;
use cpc_core::error::CpcError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub display_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginResponse {
    pub user: User,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateProfileRequest {
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
}

/// Tauri command to authenticate user login
#[tauri::command]
pub async fn login(
    request: LoginRequest,
    identity_service: State<'_, IdentityService>,
) -> Result<AuthResponse<LoginResponse>, String> {
    match identity_service.authenticate(&request.username, &request.password).await {
        Ok((user, token)) => Ok(AuthResponse {
            success: true,
            data: Some(LoginResponse { user, token }),
            error: None,
        }),
        Err(e) => Ok(AuthResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

/// Tauri command to register a new user
#[tauri::command]
pub async fn register(
    request: RegisterRequest,
    identity_service: State<'_, IdentityService>,
) -> Result<AuthResponse<User>, String> {
    match identity_service.create_user(
        &request.username,
        &request.email,
        &request.password,
        request.display_name.as_deref(),
    ).await {
        Ok(user) => Ok(AuthResponse {
            success: true,
            data: Some(user),
            error: None,
        }),
        Err(e) => Ok(AuthResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

/// Tauri command to get current user profile
#[tauri::command]
pub async fn get_current_user(
    user_id: Uuid,
    identity_service: State<'_, IdentityService>,
) -> Result<AuthResponse<User>, String> {
    match identity_service.get_user_by_id(user_id).await {
        Ok(Some(user)) => Ok(AuthResponse {
            success: true,
            data: Some(user),
            error: None,
        }),
        Ok(None) => Ok(AuthResponse {
            success: false,
            data: None,
            error: Some("User not found".to_string()),
        }),
        Err(e) => Ok(AuthResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

/// Tauri command to update user profile
#[tauri::command]
pub async fn update_profile(
    user_id: Uuid,
    request: UpdateProfileRequest,
    identity_service: State<'_, IdentityService>,
) -> Result<AuthResponse<User>, String> {
    match identity_service.update_user_profile(
        user_id,
        request.display_name.as_deref(),
        request.bio.as_deref(),
        request.avatar_url.as_deref(),
    ).await {
        Ok(user) => Ok(AuthResponse {
            success: true,
            data: Some(user),
            error: None,
        }),
        Err(e) => Ok(AuthResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

/// Tauri command to logout user
#[tauri::command]
pub async fn logout(
    token: String,
    identity_service: State<'_, IdentityService>,
) -> Result<AuthResponse<bool>, String> {
    match identity_service.invalidate_token(&token).await {
        Ok(_) => Ok(AuthResponse {
            success: true,
            data: Some(true),
            error: None,
        }),
        Err(e) => Ok(AuthResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

/// Tauri command to validate authentication token
#[tauri::command]
pub async fn validate_token(
    token: String,
    identity_service: State<'_, IdentityService>,
) -> Result<AuthResponse<User>, String> {
    match identity_service.validate_token(&token).await {
        Ok(Some(user)) => Ok(AuthResponse {
            success: true,
            data: Some(user),
            error: None,
        }),
        Ok(None) => Ok(AuthResponse {
            success: false,
            data: None,
            error: Some("Invalid token".to_string()),
        }),
        Err(e) => Ok(AuthResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

/// Tauri command to change user password
#[tauri::command]
pub async fn change_password(
    user_id: Uuid,
    current_password: String,
    new_password: String,
    identity_service: State<'_, IdentityService>,
) -> Result<AuthResponse<bool>, String> {
    match identity_service.change_password(user_id, &current_password, &new_password).await {
        Ok(_) => Ok(AuthResponse {
            success: true,
            data: Some(true),
            error: None,
        }),
        Err(e) => Ok(AuthResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

/// Tauri command to get user profile by ID
#[tauri::command]
pub async fn get_user_profile(
    user_id: Uuid,
    identity_service: State<'_, IdentityService>,
) -> Result<AuthResponse<UserProfile>, String> {
    match identity_service.get_user_profile(user_id).await {
        Ok(Some(profile)) => Ok(AuthResponse {
            success: true,
            data: Some(profile),
            error: None,
        }),
        Ok(None) => Ok(AuthResponse {
            success: false,
            data: None,
            error: Some("User profile not found".to_string()),
        }),
        Err(e) => Ok(AuthResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

/// Tauri command to search users by username or display name
#[tauri::command]
pub async fn search_users(
    query: String,
    limit: Option<i32>,
    offset: Option<i32>,
    identity_service: State<'_, IdentityService>,
) -> Result<AuthResponse<Vec<User>>, String> {
    match identity_service.search_users(&query, limit.unwrap_or(10), offset.unwrap_or(0)).await {
        Ok(users) => Ok(AuthResponse {
            success: true,
            data: Some(users),
            error: None,
        }),
        Err(e) => Ok(AuthResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

/// Tauri command to get user relationships
#[tauri::command]
pub async fn get_user_relationships(
    user_id: Uuid,
    relationship_type: Option<String>,
    identity_service: State<'_, IdentityService>,
) -> Result<AuthResponse<Vec<UserRelationship>>, String> {
    let rel_type = relationship_type.as_ref().map(|t| match t.as_str() {
        "following" => UserRelationshipType::Following,
        "blocked" => UserRelationshipType::Blocked,
        "muted" => UserRelationshipType::Muted,
        _ => UserRelationshipType::Following,
    });

    match identity_service.get_user_relationships(user_id, rel_type).await {
        Ok(relationships) => Ok(AuthResponse {
            success: true,
            data: Some(relationships),
            error: None,
        }),
        Err(e) => Ok(AuthResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

/// Tauri command to refresh authentication token
#[tauri::command]
pub async fn refresh_token(
    refresh_token: String,
    identity_service: State<'_, IdentityService>,
) -> Result<AuthResponse<LoginResponse>, String> {
    match identity_service.refresh_token(&refresh_token).await {
        Ok((user, new_token)) => Ok(AuthResponse {
            success: true,
            data: Some(LoginResponse { user, token: new_token }),
            error: None,
        }),
        Err(e) => Ok(AuthResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}