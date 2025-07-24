//! GraphQL mutations and queries for social features
//!
//! Implements the GraphQL schema extensions for social features
//! including experience sharing, friend invitations, and comments.

use async_graphql::*;
use std::sync::Arc;
use uuid::Uuid;
use crate::services::social::{SocialFeaturesService, ShareExperienceInput, PostCommentInput};
use crate::services::permissions::{PermissionManager, Permission, PermissionContext};

/// Input for sharing an experience
#[derive(InputObject, Clone)]
pub struct ShareExperienceInputGql {
    pub experience_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub visibility: Visibility,
    pub content_hash: String,
    pub file_size: u64,
}

/// Input for inviting a friend
#[derive(InputObject, Clone)]
pub struct InviteFriendInput {
    pub user_id: Uuid,
}

/// Input for adding a comment
#[derive(InputObject, Clone)]
pub struct AddCommentInput {
    pub experience_id: Uuid,
    pub content: String,
}

/// Response payload for sharing an experience
#[derive(SimpleObject, Clone)]
pub struct ShareExperiencePayload {
    pub experience: Experience,
    pub share_url: String,
}

/// Response payload for friend invitation
#[derive(SimpleObject, Clone)]
pub struct InviteFriendPayload {
    pub invitation_id: Uuid,
    pub invitation_code: String,
    pub status: String,
}

/// Response payload for comment
#[derive(SimpleObject, Clone)]
pub struct CommentPayload {
    pub comment: Comment,
}

/// Experience type for GraphQL
#[derive(SimpleObject, Clone)]
pub struct Experience {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub visibility: Visibility,
    pub content_hash: String,
    pub file_size: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Comment type for GraphQL
#[derive(SimpleObject, Clone)]
pub struct Comment {
    pub id: Uuid,
    pub experience_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Visibility enum for GraphQL
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum Visibility {
    Public,
    Friends,
    Private,
}

impl From<cpc_core::models::social::post::Visibility> for Visibility {
    fn from(v: cpc_core::models::social::post::Visibility) -> Self {
        match v {
            cpc_core::models::social::post::Visibility::Public => Visibility::Public,
            cpc_core::models::social::post::Visibility::Friends => Visibility::Friends,
            cpc_core::models::social::post::Visibility::Private => Visibility::Private,
        }
    }
}

impl From<Visibility> for cpc_core::models::social::post::Visibility {
    fn from(v: Visibility) -> Self {
        match v {
            Visibility::Public => cpc_core::models::social::post::Visibility::Public,
            Visibility::Friends => cpc_core::models::social::post::Visibility::Friends,
            Visibility::Private => cpc_core::models::social::post::Visibility::Private,
        }
    }
}

/// Social features mutations
#[derive(Default)]
pub struct SocialFeaturesMutation;

#[Object]
impl SocialFeaturesMutation {
    /// Share an experience with specified visibility
    async fn share_experience(
        &self,
        ctx: &Context<'_>,
        input: ShareExperienceInputGql,
    ) -> Result<ShareExperiencePayload> {
        let service = ctx.data::<Arc<SocialFeaturesService>>()?;
        let permission_manager = ctx.data::<Arc<PermissionManager>>()?;
        
        let user_id = get_current_user_id(ctx)?;
        
        // Check permissions
        let context = PermissionContext {
            user_id,
            target_user_id: None,
            target_experience_id: Some(input.experience_id),
        };
        
        let permission_result = permission_manager
            .check_permission(Permission::ShareExperience, &context)
            .await;
        
        match permission_result {
            crate::services::permissions::PermissionResult::Granted => {
                let share_input = ShareExperienceInput {
                    experience_id: input.experience_id,
                    title: input.title,
                    description: input.description,
                    visibility: input.visibility.into(),
                    content_hash: input.content_hash,
                    file_size: input.file_size,
                };
                
                let experience = service
                    .share_experience(user_id, share_input)
                    .await
                    .map_err(|e| Error::new(format!("Failed to share experience: {:?}", e)))?;
                
                let share_url = format!("/experiences/{}", experience.id);
                
                Ok(ShareExperiencePayload {
                    experience: Experience {
                        id: experience.id,
                        owner_id: experience.owner_id,
                        title: experience.title,
                        description: experience.description,
                        visibility: experience.visibility.into(),
                        content_hash: experience.content_hash,
                        file_size: experience.file_size,
                        created_at: experience.created_at,
                        updated_at: experience.updated_at,
                    },
                    share_url,
                })
            }
            crate::services::permissions::PermissionResult::Denied(reason) => {
                Err(Error::new(format!("Permission denied: {}", reason)))
            }
        }
    }

    /// Invite a friend to view experiences
    async fn invite_friend(
        &self,
        ctx: &Context<'_>,
        input: InviteFriendInput,
    ) -> Result<InviteFriendPayload> {
        let service = ctx.data::<Arc<SocialFeaturesService>>()?;
        let permission_manager = ctx.data::<Arc<PermissionManager>>()?;
        
        let user_id = get_current_user_id(ctx)?;
        
        // Check permissions
        let context = PermissionContext {
            user_id,
            target_user_id: Some(input.user_id),
            target_experience_id: None,
        };
        
        let permission_result = permission_manager
            .check_permission(Permission::InviteFriend, &context)
            .await;
        
        match permission_result {
            crate::services::permissions::PermissionResult::Granted => {
                let invitation = service
                    .invite_friend(user_id, input.user_id)
                    .await
                    .map_err(|e| Error::new(format!("Failed to invite friend: {:?}", e)))?;
                
                Ok(InviteFriendPayload {
                    invitation_id: invitation.id,
                    invitation_code: format!("INV-{}", invitation.id),
                    status: match invitation.status {
                        crate::services::social::InvitationStatus::Pending => "PENDING".to_string(),
                        crate::services::social::InvitationStatus::Accepted => "ACCEPTED".to_string(),
                        crate::services::social::InvitationStatus::Rejected => "REJECTED".to_string(),
                    },
                })
            }
            crate::services::permissions::PermissionResult::Denied(reason) => {
                Err(Error::new(format!("Permission denied: {}", reason)))
            }
        }
    }

    /// Add a comment to an experience
    async fn add_comment(
        &self,
        ctx: &Context<'_>,
        input: AddCommentInput,
    ) -> Result<CommentPayload> {
        let service = ctx.data::<Arc<SocialFeaturesService>>()?;
        let permission_manager = ctx.data::<Arc<PermissionManager>>()?;
        
        let user_id = get_current_user_id(ctx)?;
        
        // Check permissions
        let context = PermissionContext {
            user_id,
            target_user_id: None,
            target_experience_id: Some(input.experience_id),
        };
        
        let permission_result = permission_manager
            .check_permission(Permission::CommentOnExperience, &context)
            .await;
        
        match permission_result {
            crate::services::permissions::PermissionResult::Granted => {
                let comment_input = PostCommentInput {
                    experience_id: input.experience_id,
                    content: input.content,
                };
                
                let comment = service
                    .add_comment(user_id, comment_input)
                    .await
                    .map_err(|e| Error::new(format!("Failed to add comment: {:?}", e)))?;
                
                Ok(CommentPayload {
                    comment: Comment {
                        id: comment.id,
                        experience_id: comment.experience_id,
                        author_id: comment.author_id,
                        content: comment.content,
                        created_at: comment.created_at,
                        updated_at: comment.updated_at,
                    },
                })
            }
            crate::services::permissions::PermissionResult::Denied(reason) => {
                Err(Error::new(format!("Permission denied: {}", reason)))
            }
        }
    }
}

/// Social features queries
#[derive(Default)]
pub struct SocialFeaturesQuery;

#[Object]
impl SocialFeaturesQuery {
    /// Get experiences visible to the current user
    async fn visible_experiences(
        &self,
        ctx: &Context<'_>,
        user_id: Option<Uuid>,
    ) -> Result<Vec<Experience>> {
        let service = ctx.data::<Arc<SocialFeaturesService>>()?;
        let permission_manager = ctx.data::<Arc<PermissionManager>>()?;
        
        let current_user_id = get_current_user_id(ctx)?;
        let target_user_id = user_id.unwrap_or(current_user_id);
        
        // Check permissions
        let context = PermissionContext {
            user_id: current_user_id,
            target_user_id: Some(target_user_id),
            target_experience_id: None,
        };
        
        let permission_result = permission_manager
            .check_permission(Permission::ViewPrivateContent, &context)
            .await;
        
        match permission_result {
            crate::services::permissions::PermissionResult::Granted => {
                let experiences = service
                    .get_visible_experiences(target_user_id, Some(current_user_id))
                    .await
                    .map_err(|e| Error::new(format!("Failed to get experiences: {:?}", e)))?;
                
                Ok(experiences.into_iter().map(|exp| Experience {
                    id: exp.id,
                    owner_id: exp.owner_id,
                    title: exp.title,
                    description: exp.description,
                    visibility: exp.visibility.into(),
                    content_hash: exp.content_hash,
                    file_size: exp.file_size,
                    created_at: exp.created_at,
                    updated_at: exp.updated_at,
                }).collect())
            }
            crate::services::permissions::PermissionResult::Denied(reason) => {
                Err(Error::new(format!("Permission denied: {}", reason)))
            }
        }
    }

    /// Get comments for an experience
    async fn experience_comments(
        &self,
        ctx: &Context<'_>,
        experience_id: Uuid,
    ) -> Result<Vec<Comment>> {
        let service = ctx.data::<Arc<SocialFeaturesService>>()?;
        let permission_manager = ctx.data::<Arc<PermissionManager>>()?;
        
        let user_id = get_current_user_id(ctx)?;
        
        // Check permissions
        let context = PermissionContext {
            user_id,
            target_user_id: None,
            target_experience_id: Some(experience_id),
        };
        
        let permission_result = permission_manager
            .check_permission(Permission::ViewExperience, &context)
            .await;
        
        match permission_result {
            crate::services::permissions::PermissionResult::Granted => {
                let comments = service
                    .get_comments(experience_id, Some(user_id))
                    .await
                    .map_err(|e| Error::new(format!("Failed to get comments: {:?}", e)))?;
                
                Ok(comments.into_iter().map(|comment| Comment {
                    id: comment.id,
                    experience_id: comment.experience_id,
                    author_id: comment.author_id,
                    content: comment.content,
                    created_at: comment.created_at,
                    updated_at: comment.updated_at,
                }).collect())
            }
            crate::services::permissions::PermissionResult::Denied(reason) => {
                Err(Error::new(format!("Permission denied: {}", reason)))
            }
        }
    }
}

// Helper function to get authenticated user ID
fn get_current_user_id(ctx: &Context<'_>) -> Result<Uuid> {
    // Extract user ID from authentication context
    let auth_data = ctx.data::<crate::auth::AuthData>()?;
    Ok(auth_data.user_id)
}