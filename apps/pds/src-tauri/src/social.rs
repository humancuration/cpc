use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use cpc_core::models::social::post::{Post, Visibility};
use cpc_core::models::social::{Comment, Like, Share, Follow, Block, Mute, FeedType, FeedAlgorithm, LikeTargetType, ShareType, MuteType};
use cpc_core::services::social::{SocialService, CreatePostInput, CreateCommentInput, FeedParams, TimelineResult};
use cpc_core::repositories::social::post_repository::PostRepository;
use cpc_core::repositories::social::relationship_repository::RelationshipRepository;
use cpc_core::error::CpcError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatePostRequest {
    pub content: String,
    pub visibility: Visibility,
    pub cooperative_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateRelationshipRequest {
    pub following_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SocialResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[tauri::command]
pub async fn create_post(
    request: CreatePostRequest,
    post_repo: State<'_, PostRepository>,
) -> Result<SocialResponse<Post>, String> {
    match post_repo.create_post(&request.content, request.visibility, request.cooperative_id).await {
        Ok(post) => Ok(SocialResponse {
            success: true,
            data: Some(post),
            error: None,
        }),
        Err(e) => Ok(SocialResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

#[tauri::command]
pub async fn get_posts_by_user(
    user_id: Uuid,
    limit: Option<i32>,
    offset: Option<i32>,
    post_repo: State<'_, PostRepository>,
) -> Result<SocialResponse<Vec<Post>>, String> {
    match post_repo.get_posts_by_user(user_id, limit.unwrap_or(20), offset.unwrap_or(0)).await {
        Ok(posts) => Ok(SocialResponse {
            success: true,
            data: Some(posts),
            error: None,
        }),
        Err(e) => Ok(SocialResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

#[tauri::command]
pub async fn get_timeline(
    limit: Option<i32>,
    offset: Option<i32>,
    post_repo: State<'_, PostRepository>,
) -> Result<SocialResponse<Vec<Post>>, String> {
    match post_repo.get_timeline(limit.unwrap_or(20), offset.unwrap_or(0)).await {
        Ok(posts) => Ok(SocialResponse {
            success: true,
            data: Some(posts),
            error: None,
        }),
        Err(e) => Ok(SocialResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

#[tauri::command]
pub async fn update_post(
    post_id: Uuid,
    content: String,
    visibility: Visibility,
    post_repo: State<'_, PostRepository>,
) -> Result<SocialResponse<Post>, String> {
    match post_repo.update_post(post_id, &content, visibility).await {
        Ok(post) => Ok(SocialResponse {
            success: true,
            data: Some(post),
            error: None,
        }),
        Err(e) => Ok(SocialResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

#[tauri::command]
pub async fn delete_post(
    post_id: Uuid,
    post_repo: State<'_, PostRepository>,
) -> Result<SocialResponse<bool>, String> {
    match post_repo.delete_post(post_id).await {
        Ok(_) => Ok(SocialResponse {
            success: true,
            data: Some(true),
            error: None,
        }),
        Err(e) => Ok(SocialResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

#[tauri::command]
pub async fn follow_user(
    following_id: Uuid,
    relationship_repo: State<'_, RelationshipRepository>,
) -> Result<SocialResponse<bool>, String> {
    match relationship_repo.follow_user(following_id).await {
        Ok(_) => Ok(SocialResponse {
            success: true,
            data: Some(true),
            error: None,
        }),
        Err(e) => Ok(SocialResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

#[tauri::command]
pub async fn unfollow_user(
    following_id: Uuid,
    relationship_repo: State<'_, RelationshipRepository>,
) -> Result<SocialResponse<bool>, String> {
    match relationship_repo.unfollow_user(following_id).await {
        Ok(_) => Ok(SocialResponse {
            success: true,
            data: Some(true),
            error: None,
        }),
        Err(e) => Ok(SocialResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

#[tauri::command]
pub async fn get_followers(
    user_id: Uuid,
    relationship_repo: State<'_, RelationshipRepository>,
) -> Result<SocialResponse<Vec<cpc_core::models::social::relationship::Follower>>, String> {
    match relationship_repo.get_followers(user_id).await {
        Ok(followers) => Ok(SocialResponse {
            success: true,
            data: Some(followers),
            error: None,
        }),
        Err(e) => Ok(SocialResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

#[tauri::command]
pub async fn get_following(
    user_id: Uuid,
    relationship_repo: State<'_, RelationshipRepository>,
) -> Result<SocialResponse<Vec<cpc_core::models::social::relationship::Following>>, String> {
    match relationship_repo.get_following(user_id).await {
        Ok(following) => Ok(SocialResponse {
            success: true,
            data: Some(following),
            error: None,
        }),
        Err(e) => Ok(SocialResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

#[tauri::command]
pub async fn search_users(
    query: String,
    limit: Option<i32>,
    relationship_repo: State<'_, RelationshipRepository>,
) -> Result<SocialResponse<Vec<cpc_core::models::user::User>>, String> {
    match relationship_repo.search_users(&query, limit.unwrap_or(10)).await {
        Ok(users) => Ok(SocialResponse {
            success: true,
            data: Some(users),
            error: None,
        }),
        Err(e) => Ok(SocialResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}