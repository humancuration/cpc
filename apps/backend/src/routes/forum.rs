use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use cpc_core::services::forum::ForumService;
use crate::auth::{AuthenticatedUser, require_role};
use crate::routes::social::ApiResponse;

#[derive(Debug, Deserialize)]
pub struct CreateForumRequest {
    pub name: String,
    pub description: String,
    pub category: String,
    pub is_private: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateThreadRequest {
    pub title: String,
    pub content: String,
    pub is_pinned: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateReplyRequest {
    pub content: String,
    pub parent_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct VoteRequest {
    pub is_upvote: bool,
}

#[derive(Debug, Deserialize)]
pub struct ForumQueryParams {
    pub category: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct ThreadQueryParams {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub sort_by: Option<String>, // "recent", "popular", "pinned"
}

pub async fn create_forum(
    State(forum_service): State<Arc<ForumService>>,
    user: AuthenticatedUser,
    Json(request): Json<CreateForumRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    // Check if user has permission to create forums
    require_role("forum_creator")(user.clone())?;
    
    match forum_service.create_forum(
        user.user_id,
        request.name,
        request.description,
        request.category,
        request.is_private,
    ).await {
        Ok(forum) => Ok(Json(ApiResponse::success(serde_json::to_value(forum).unwrap()))),
        Err(e) => {
            tracing::error!("Failed to create forum: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_forums(
    State(forum_service): State<Arc<ForumService>>,
    user: Option<AuthenticatedUser>,
    Query(params): Query<ForumQueryParams>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let viewer_id = user.map(|u| u.user_id);
    let limit = params.limit.unwrap_or(20);
    let offset = params.offset.unwrap_or(0);
    
    match forum_service.get_forums(viewer_id, params.category, limit, offset).await {
        Ok(forums) => Ok(Json(ApiResponse::success(serde_json::to_value(forums).unwrap()))),
        Err(e) => {
            tracing::error!("Failed to get forums: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_forum(
    State(forum_service): State<Arc<ForumService>>,
    user: Option<AuthenticatedUser>,
    Path(forum_id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let viewer_id = user.map(|u| u.user_id);
    
    match forum_service.get_forum(forum_id, viewer_id).await {
        Ok(Some(forum)) => Ok(Json(ApiResponse::success(serde_json::to_value(forum).unwrap()))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get forum: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_thread(
    State(forum_service): State<Arc<ForumService>>,
    user: AuthenticatedUser,
    Path(forum_id): Path<Uuid>,
    Json(request): Json<CreateThreadRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    match forum_service.create_thread(
        user.user_id,
        forum_id,
        request.title,
        request.content,
        request.is_pinned,
    ).await {
        Ok(thread) => Ok(Json(ApiResponse::success(serde_json::to_value(thread).unwrap()))),
        Err(e) => {
            tracing::error!("Failed to create thread: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_forum_threads(
    State(forum_service): State<Arc<ForumService>>,
    user: Option<AuthenticatedUser>,
    Path(forum_id): Path<Uuid>,
    Query(params): Query<ThreadQueryParams>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let viewer_id = user.map(|u| u.user_id);
    let limit = params.limit.unwrap_or(20);
    let offset = params.offset.unwrap_or(0);
    let sort_by = params.sort_by.as_deref().unwrap_or("recent");
    
    match forum_service.get_forum_threads(forum_id, viewer_id, sort_by, limit, offset).await {
        Ok(threads) => Ok(Json(ApiResponse::success(serde_json::to_value(threads).unwrap()))),
        Err(e) => {
            tracing::error!("Failed to get forum threads: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_thread(
    State(forum_service): State<Arc<ForumService>>,
    user: Option<AuthenticatedUser>,
    Path(thread_id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let viewer_id = user.map(|u| u.user_id);
    
    match forum_service.get_thread(thread_id, viewer_id).await {
        Ok(Some(thread)) => Ok(Json(ApiResponse::success(serde_json::to_value(thread).unwrap()))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get thread: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_reply(
    State(forum_service): State<Arc<ForumService>>,
    user: AuthenticatedUser,
    Path(thread_id): Path<Uuid>,
    Json(request): Json<CreateReplyRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    match forum_service.create_reply(
        user.user_id,
        thread_id,
        request.content,
        request.parent_id,
    ).await {
        Ok(reply) => Ok(Json(ApiResponse::success(serde_json::to_value(reply).unwrap()))),
        Err(e) => {
            tracing::error!("Failed to create reply: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_thread_replies(
    State(forum_service): State<Arc<ForumService>>,
    user: Option<AuthenticatedUser>,
    Path(thread_id): Path<Uuid>,
    Query(params): Query<ThreadQueryParams>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let viewer_id = user.map(|u| u.user_id);
    let limit = params.limit.unwrap_or(50);
    let offset = params.offset.unwrap_or(0);
    
    match forum_service.get_thread_replies(thread_id, viewer_id, limit, offset).await {
        Ok(replies) => Ok(Json(ApiResponse::success(serde_json::to_value(replies).unwrap()))),
        Err(e) => {
            tracing::error!("Failed to get thread replies: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn vote_on_post(
    State(forum_service): State<Arc<ForumService>>,
    user: AuthenticatedUser,
    Path(post_id): Path<Uuid>,
    Json(request): Json<VoteRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    match forum_service.vote_on_post(user.user_id, post_id, request.is_upvote).await {
        Ok(vote) => Ok(Json(ApiResponse::success(serde_json::to_value(vote).unwrap()))),
        Err(e) => {
            tracing::error!("Failed to vote on post: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn pin_thread(
    State(forum_service): State<Arc<ForumService>>,
    user: AuthenticatedUser,
    Path(thread_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    // Check if user has moderator permissions
    require_role("forum_moderator")(user.clone())?;
    
    match forum_service.pin_thread(thread_id, user.user_id).await {
        Ok(_) => Ok(Json(ApiResponse::success(()))),
        Err(e) => {
            tracing::error!("Failed to pin thread: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn lock_thread(
    State(forum_service): State<Arc<ForumService>>,
    user: AuthenticatedUser,
    Path(thread_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    // Check if user has moderator permissions
    require_role("forum_moderator")(user.clone())?;
    
    match forum_service.lock_thread(thread_id, user.user_id).await {
        Ok(_) => Ok(Json(ApiResponse::success(()))),
        Err(e) => {
            tracing::error!("Failed to lock thread: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_post(
    State(forum_service): State<Arc<ForumService>>,
    user: AuthenticatedUser,
    Path(post_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    match forum_service.delete_post(post_id, user.user_id).await {
        Ok(_) => Ok(Json(ApiResponse::success(()))),
        Err(e) => {
            tracing::error!("Failed to delete post: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub fn router() -> Router<Arc<ForumService>> {
    Router::new()
        .route("/forums", post(create_forum))
        .route("/forums", get(get_forums))
        .route("/forums/:id", get(get_forum))
        .route("/forums/:id/threads", post(create_thread))
        .route("/forums/:id/threads", get(get_forum_threads))
        .route("/threads/:id", get(get_thread))
        .route("/threads/:id/replies", post(create_reply))
        .route("/threads/:id/replies", get(get_thread_replies))
        .route("/threads/:id/pin", put(pin_thread))
        .route("/threads/:id/lock", put(lock_thread))
        .route("/posts/:id/vote", post(vote_on_post))
        .route("/posts/:id", delete(delete_post))
}