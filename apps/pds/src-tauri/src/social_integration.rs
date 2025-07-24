//! Tauri integration for social features
//!
//! Provides Tauri commands for social features including experience sharing,
//! friend invitations, and comments.

use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::services::social::{SocialFeaturesService, ShareExperienceInput, PostCommentInput};
use crate::services::permissions::PermissionManager;

#[derive(Debug, Serialize, Deserialize)]
pub struct ShareExperienceRequest {
    pub experience_id: String,
    pub title: String,
    pub description: Option<String>,
    pub visibility: String,
    pub content_hash: String,
    pub file_size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShareExperienceResponse {
    pub success: bool,
    pub experience_id: String,
    pub share_url: String,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InviteFriendRequest {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InviteFriendResponse {
    pub success: bool,
    pub invitation_id: String,
    pub invitation_code: String,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddCommentRequest {
    pub experience_id: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddCommentResponse {
    pub success: bool,
    pub comment_id: String,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCommentsRequest {
    pub experience_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentResponse {
    pub id: String,
    pub experience_id: String,
    pub author_id: String,
    pub content: String,
    pub created_at: String,
}

/// Tauri command to share an experience
#[tauri::command]
pub async fn share_experience(
    request: ShareExperienceRequest,
    social_service: State<'_, Arc<RwLock<SocialFeaturesService>>>,
) -> Result<ShareExperienceResponse, String> {
    let service = social_service.read().await;
    
    let experience_id = Uuid::parse_str(&request.experience_id)
        .map_err(|e| format!("Invalid experience ID: {}", e))?;
    
    let visibility = match request.visibility.as_str() {
        "PUBLIC" => cpc_core::models::social::post::Visibility::Public,
        "FRIENDS" => cpc_core::models::social::post::Visibility::Friends,
        "PRIVATE" => cpc_core::models::social::post::Visibility::Private,
        _ => return Err("Invalid visibility setting".to_string()),
    };
    
    let input = ShareExperienceInput {
        experience_id,
        title: request.title,
        description: request.description,
        visibility,
        content_hash: request.content_hash,
        file_size: request.file_size,
    };
    
    // In a real implementation, we'd get the current user ID from auth
    let user_id = Uuid::new_v4(); // Mock user ID
    
    match service.share_experience(user_id, input).await {
        Ok(experience) => Ok(ShareExperienceResponse {
            success: true,
            experience_id: experience.id.to_string(),
            share_url: format!("/experiences/{}", experience.id),
            error: None,
        }),
        Err(e) => Ok(ShareExperienceResponse {
            success: false,
            experience_id: request.experience_id,
            share_url: String::new(),
            error: Some(e.to_string()),
        }),
    }
}

/// Tauri command to invite a friend
#[tauri::command]
pub async fn invite_friend(
    request: InviteFriendRequest,
    social_service: State<'_, Arc<RwLock<SocialFeaturesService>>>,
) -> Result<InviteFriendResponse, String> {
    let service = social_service.read().await;
    
    let user_id = Uuid::parse_str(&request.user_id)
        .map_err(|e| format!("Invalid user ID: {}", e))?;
    
    // In a real implementation, we'd get the current user ID from auth
    let from_user_id = Uuid::new_v4(); // Mock user ID
    
    match service.invite_friend(from_user_id, user_id).await {
        Ok(invitation) => Ok(InviteFriendResponse {
            success: true,
            invitation_id: invitation.id.to_string(),
            invitation_code: format!("INV-{}", invitation.id),
            error: None,
        }),
        Err(e) => Ok(InviteFriendResponse {
            success: false,
            invitation_id: String::new(),
            invitation_code: String::new(),
            error: Some(e.to_string()),
        }),
    }
}

/// Tauri command to add a comment
#[tauri::command]
pub async fn add_comment(
    request: AddCommentRequest,
    social_service: State<'_, Arc<RwLock<SocialFeaturesService>>>,
) -> Result<AddCommentResponse, String> {
    let service = social_service.read().await;
    
    let experience_id = Uuid::parse_str(&request.experience_id)
        .map_err(|e| format!("Invalid experience ID: {}", e))?;
    
    let input = PostCommentInput {
        experience_id,
        content: request.content,
    };
    
    // In a real implementation, we'd get the current user ID from auth
    let user_id = Uuid::new_v4(); // Mock user ID
    
    match service.add_comment(user_id, input).await {
        Ok(comment) => Ok(AddCommentResponse {
            success: true,
            comment_id: comment.id.to_string(),
            error: None,
        }),
        Err(e) => Ok(AddCommentResponse {
            success: false,
            comment_id: String::new(),
            error: Some(e.to_string()),
        }),
    }
}

/// Tauri command to get comments for an experience
#[tauri::command]
pub async fn get_comments(
    request: GetCommentsRequest,
    social_service: State<'_, Arc<RwLock<SocialFeaturesService>>>,
) -> Result<Vec<CommentResponse>, String> {
    let service = social_service.read().await;
    
    let experience_id = Uuid::parse_str(&request.experience_id)
        .map_err(|e| format!("Invalid experience ID: {}", e))?;
    
    // In a real implementation, we'd get the current user ID from auth
    let user_id = Uuid::new_v4(); // Mock user ID
    
    match service.get_comments(experience_id, Some(user_id)).await {
        Ok(comments) => Ok(comments.into_iter().map(|comment| CommentResponse {
            id: comment.id.to_string(),
            experience_id: comment.experience_id.to_string(),
            author_id: comment.author_id.to_string(),
            content: comment.content,
            created_at: comment.created_at.to_rfc3339(),
        }).collect()),
        Err(e) => Err(e.to_string()),
    }
}

/// Tauri command to get visible experiences
#[tauri::command]
pub async fn get_visible_experiences(
    user_id: Option<String>,
    social_service: State<'_, Arc<RwLock<SocialFeaturesService>>>,
) -> Result<Vec<serde_json::Value>, String> {
    let service = social_service.read().await;
    
    // In a real implementation, we'd get the current user ID from auth
    let current_user_id = Uuid::new_v4(); // Mock user ID
    
    let target_user_id = user_id
        .map(|id| Uuid::parse_str(&id).map_err(|e| e.to_string()))
        .transpose()?
        .unwrap_or(current_user_id);
    
    match service.get_visible_experiences(target_user_id, Some(current_user_id)).await {
        Ok(experiences) => {
            let experiences_json: Vec<serde_json::Value> = experiences.into_iter().map(|exp| {
                serde_json::json!({
                    "id": exp.id.to_string(),
                    "owner_id": exp.owner_id.to_string(),
                    "title": exp.title,
                    "description": exp.description,
                    "visibility": match exp.visibility {
                        cpc_core::models::social::post::Visibility::Public => "PUBLIC",
                        cpc_core::models::social::post::Visibility::Friends => "FRIENDS",
                        cpc_core::models::social::post::Visibility::Private => "PRIVATE",
                    },
                    "content_hash": exp.content_hash,
                    "file_size": exp.file_size,
                    "created_at": exp.created_at.to_rfc3339(),
                    "updated_at": exp.updated_at.to_rfc3339(),
                })
            }).collect();
            Ok(experiences_json)
        }
        Err(e) => Err(e.to_string()),
    }
}