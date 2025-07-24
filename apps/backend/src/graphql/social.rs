use crate::services::social_service::SocialService;
use async_graphql::*;
use cpc_core::models::social::post::{Post, Visibility};
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Default)]
pub struct SocialQuery;

#[derive(Default)]
pub struct SocialMutation;

#[derive(MergedObject, Default)]
pub struct SocialRootQuery(
    SocialQuery,
    super::post::PostQuery,
    super::user::UserQuery,
);

#[derive(MergedObject, Default)]
pub struct SocialRootMutation(
    SocialMutation,
    super::post::PostMutation,
    super::user::UserMutation,
);

/// Input type for timeline filters
#[derive(InputObject, Clone, Default)]
pub struct TimelineFilterInput {
    pub content_type: Option<String>,
    pub author_id: Option<Uuid>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub visibility: Option<Visibility>,
    pub cooperative_only: Option<bool>,
}

/// Pagination input for timeline queries
#[derive(InputObject, Clone, Default)]
pub struct PaginationInput {
    pub limit: i32,
    pub offset: Option<i32>,
    pub after: Option<Uuid>,
}

/// Timeline response type
#[derive(SimpleObject)]
pub struct TimelineResponse {
    pub posts: Vec<Post>,
    pub has_more: bool,
    pub total_count: i64,
}

#[Object]
impl SocialQuery {
    async fn post(&self, ctx: &Context<'_>, id: Uuid) -> Result<Post> {
        let service = ctx.data::<Arc<SocialService>>()?;

        service
            .get_post(id)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to get post: {:?}", e)))
    }
    
    async fn get_posts_by_user(&self, ctx: &Context<'_>, user_id: Uuid) -> Result<Vec<Post>> {
        let service = ctx.data::<Arc<SocialService>>()?;
        
        service
            .get_posts_by_user(user_id)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to get user posts: {:?}", e)))
    }
    
    async fn get_followers(&self, ctx: &Context<'_>, user_id: Uuid) -> Result<Vec<Uuid>> {
        let service = ctx.data::<Arc<SocialService>>()?;
        
        service
            .get_followers(user_id)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to get followers: {:?}", e)))
    }
    
    async fn get_following(&self, ctx: &Context<'_>, user_id: Uuid) -> Result<Vec<Uuid>> {
        let service = ctx.data::<Arc<SocialService>>()?;
        
        service
            .get_following(user_id)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to get following: {:?}", e)))
    }
    
    /// Get timeline posts with filtering and pagination
    async fn timeline(
        &self,
        ctx: &Context<'_>,
        filters: Option<TimelineFilterInput>,
        pagination: Option<PaginationInput>,
    ) -> Result<TimelineResponse> {
        let service = ctx.data::<Arc<SocialService>>()?;
        
        let limit = pagination.as_ref().and_then(|p| Some(p.limit)).unwrap_or(20);
        let offset = pagination.as_ref().and_then(|p| p.offset).unwrap_or(0);
        let after = pagination.as_ref().and_then(|p| p.after);
        
        service
            .get_timeline(filters, limit, offset, after)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to get timeline: {:?}", e)))
    }
}

/// Input for creating a post
#[derive(InputObject, Clone)]
pub struct CreatePostInput {
    pub content: String,
    pub media_urls: Vec<String>,
    pub visibility: Visibility,
    pub cooperative_id: Option<Uuid>,
    pub media_ids: Vec<Uuid>,
}

/// Input for creating a comment
#[derive(InputObject, Clone)]
pub struct CreateCommentInput {
    pub post_id: Uuid,
    pub content: String,
}

/// Input for social interactions
#[derive(InputObject, Clone)]
pub struct SocialInteractionInput {
    pub post_id: Uuid,
    pub interaction_type: String, // "like", "unlike", "share"
}

#[Object]
impl SocialMutation {
    async fn create_post(
        &self,
        ctx: &Context<'_>,
        input: CreatePostInput,
    ) -> Result<Post> {
        let service = ctx.data::<Arc<SocialService>>()?;
        let current_user_id = get_current_user_id(ctx)?;

        let post = service
            .create_post(
                current_user_id,
                input.content,
                input.visibility,
                input.media_urls,
                input.cooperative_id,
            )
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to create post: {:?}", e)))?;

        // Associate media items with the post
        if !input.media_ids.is_empty() {
            service
                .associate_media_with_post(post.id, input.media_ids)
                .await
                .map_err(|e| async_graphql::Error::new(format!("Failed to associate media with post: {:?}", e)))?;
        }

        Ok(post)
    }

    async fn like_post(&self, ctx: &Context<'_>, post_id: Uuid) -> Result<bool> {
        let service = ctx.data::<Arc<SocialService>>()?;
        let current_user_id = get_current_user_id(ctx)?;

        service
            .like_post(current_user_id, post_id)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to like post: {:?}", e)))?;

        Ok(true)
    }

    async fn unlike_post(&self, ctx: &Context<'_>, post_id: Uuid) -> Result<bool> {
        let service = ctx.data::<Arc<SocialService>>()?;
        let current_user_id = get_current_user_id(ctx)?;

        service
            .unlike_post(current_user_id, post_id)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to unlike post: {:?}", e)))?;

        Ok(true)
    }

    async fn share_post(&self, ctx: &Context<'_>, post_id: Uuid) -> Result<bool> {
        let service = ctx.data::<Arc<SocialService>>()?;
        let current_user_id = get_current_user_id(ctx)?;

        service
            .share_post(current_user_id, post_id)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to share post: {:?}", e)))?;

        Ok(true)
    }

    async fn create_comment(
        &self,
        ctx: &Context<'_>,
        input: CreateCommentInput,
    ) -> Result<Post> {
        let service = ctx.data::<Arc<SocialService>>()?;
        let current_user_id = get_current_user_id(ctx)?;

        let comment = service
            .create_comment(current_user_id, input.post_id, input.content)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to create comment: {:?}", e)))?;

        // Return the updated post
        service
            .get_post(input.post_id)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to get post: {:?}", e)))
    }

    async fn follow_user(
        &self,
        ctx: &Context<'_>,
        following_id: Uuid,
    ) -> Result<Relationship> {
        let service = ctx.data::<Arc<SocialService>>()?;
        let current_user_id = get_current_user_id(ctx)?;

        let relationship = service
            .follow_user(current_user_id, following_id)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Follow failed: {}", e)))?;

        Ok(relationship)
    }

    async fn unfollow_user(
        &self,
        ctx: &Context<'_>,
        following_id: Uuid,
    ) -> Result<bool> {
        let service = ctx.data::<Arc<SocialService>>()?;
        let current_user_id = get_current_user_id(ctx)?;

        service
            .unfollow_user(current_user_id, following_id)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Unfollow failed: {}", e)))?;

        Ok(true)
    }
}

// Helper function to get authenticated user ID
fn get_current_user_id(ctx: &Context<'_>) -> Result<Uuid> {
    // Extract user ID from authentication context
    let auth_data = ctx.data::<crate::auth::AuthData>()?;
    Ok(auth_data.user_id)
}