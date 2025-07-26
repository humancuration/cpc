use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use cpc_core::services::social::SocialService;
use crate::auth::AuthenticatedUser;

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub content: String,
    pub visibility: String,
    pub community_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    pub content: String,
    pub visibility: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
    pub parent_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct FollowRequest {
    pub user_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

pub async fn create_post(
    State(social_service): State<Arc<SocialService>>,
    user: AuthenticatedUser,
    Json(request): Json<CreatePostRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    match social_service.create_post(
        user.user_id,
        request.content,
        request.visibility.parse().map_err(|_| StatusCode::BAD_REQUEST)?,
        request.community_id,
    ).await {
        Ok(post) => Ok(Json(ApiResponse::success(serde_json::to_value(post).unwrap()))),
        Err(e) => {
            tracing::error!("Failed to create post: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_post(
    State(social_service): State<Arc<SocialService>>,
    user: Option<AuthenticatedUser>,
    Path(post_id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let viewer_id = user.map(|u| u.user_id);
    
    match social_service.get_post(post_id, viewer_id).await {
        Ok(Some(post)) => Ok(Json(ApiResponse::success(serde_json::to_value(post).unwrap()))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get post: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_post(
    State(social_service): State<Arc<SocialService>>,
    user: AuthenticatedUser,
    Path(post_id): Path<Uuid>,
    Json(request): Json<UpdatePostRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    match social_service.update_post(
        post_id,
        user.user_id,
        request.content,
        request.visibility.parse().map_err(|_| StatusCode::BAD_REQUEST)?,
    ).await {
        Ok(post) => Ok(Json(ApiResponse::success(serde_json::to_value(post).unwrap()))),
        Err(e) => {
            tracing::error!("Failed to update post: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_post(
    State(social_service): State<Arc<SocialService>>,
    user: AuthenticatedUser,
    Path(post_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    match social_service.delete_post(post_id, user.user_id).await {
        Ok(_) => Ok(Json(ApiResponse::success(()))),
        Err(e) => {
            tracing::error!("Failed to delete post: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_user_feed(
    State(social_service): State<Arc<SocialService>>,
    user: AuthenticatedUser,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    match social_service.get_user_feed(user.user_id, 20, 0).await {
        Ok(posts) => Ok(Json(ApiResponse::success(serde_json::to_value(posts).unwrap()))),
        Err(e) => {
            tracing::error!("Failed to get user feed: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn follow_user(
    State(social_service): State<Arc<SocialService>>,
    user: AuthenticatedUser,
    Json(request): Json<FollowRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    match social_service.follow_user(user.user_id, request.user_id).await {
        Ok(_) => Ok(Json(ApiResponse::success(()))),
        Err(e) => {
            tracing::error!("Failed to follow user: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn unfollow_user(
    State(social_service): State<Arc<SocialService>>,
    user: AuthenticatedUser,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    match social_service.unfollow_user(user.user_id, user_id).await {
        Ok(_) => Ok(Json(ApiResponse::success(()))),
        Err(e) => {
            tracing::error!("Failed to unfollow user: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_comment(
    State(social_service): State<Arc<SocialService>>,
    user: AuthenticatedUser,
    Path(post_id): Path<Uuid>,
    Json(request): Json<CreateCommentRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    match social_service.create_comment(
        user.user_id,
        post_id,
        request.content,
        request.parent_id,
    ).await {
        Ok(comment) => Ok(Json(ApiResponse::success(serde_json::to_value(comment).unwrap()))),
        Err(e) => {
            tracing::error!("Failed to create comment: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_post_comments(
    State(social_service): State<Arc<SocialService>>,
    user: Option<AuthenticatedUser>,
    Path(post_id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let viewer_id = user.map(|u| u.user_id);
    
    match social_service.get_post_comments(post_id, viewer_id).await {
        Ok(comments) => Ok(Json(ApiResponse::success(serde_json::to_value(comments).unwrap()))),
        Err(e) => {
            tracing::error!("Failed to get post comments: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub fn router() -> Router<Arc<SocialService>> {
    Router::new()
        .route("/posts", post(create_post))
        .route("/posts/:id", get(get_post))
        .route("/posts/:id", put(update_post))
        .route("/posts/:id", delete(delete_post))
        .route("/posts/:id/comments", post(create_comment))
        .route("/posts/:id/comments", get(get_post_comments))
        .route("/feed", get(get_user_feed))
        .route("/follow", post(follow_user))
        .route("/unfollow/:user_id", delete(unfollow_user))
}