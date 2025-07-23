use crate::services::social_service::SocialService;
use async_graphql::{Context, Object, Result};
use cpc_core::models::social::post::{Post, Visibility}; // Make sure Visibility is imported
use std::sync::Arc;
use uuid::Uuid;

#[derive(Default)]
pub struct SocialQuery;

#[derive(MergedObject, Default)]
pub struct SocialMutation {
    create_post: create_post_mutation::CreatePostMutation,
    follow_user: follow_user_mutation::FollowUserMutation,
    unfollow_user: unfollow_user_mutation::UnfollowUserMutation,
}

#[derive(Default)]
pub struct FollowUserMutation;

#[Object]
impl FollowUserMutation {
    async fn follow_user(&self, ctx: &Context<'_>, user_id: Uuid) -> Result<User> {
        let social_service = ctx.data::<Arc<SocialService>>()?;
        let current_user_id = get_current_user_id(ctx)?;
        
        social_service.follow_user(current_user_id, user_id).await
            .map_err(|e| async_graphql::Error::new(format!("Follow failed: {}", e)))?;
        
        // TODO: Implement actual user fetching
        Ok(User { id: user_id })
    }
}

#[derive(Default)]
pub struct UnfollowUserMutation;

#[Object]
impl UnfollowUserMutation {
    async fn unfollow_user(&self, ctx: &Context<'_>, user_id: Uuid) -> Result<bool> {
        let social_service = ctx.data::<Arc<SocialService>>()?;
        let current_user_id = get_current_user_id(ctx)?;
        
        social_service.unfollow_user(current_user_id, user_id).await
            .map_err(|e| async_graphql::Error::new(format!("Unfollow failed: {}", e)))?;
            
        Ok(true)
    }
}

// Add to top of file:
use crate::services::social_service::SocialService;
use async_graphql::MergedObject;
use cpc_core::models::social::relationship::Relationship;
use cpc_core::models::user::User;

#[Object]
impl SocialQuery {
    async fn post(&self, ctx: &Context<'_>, id: Uuid) -> Result<Post> {
        let service = ctx.data::<Arc<SocialService>>()?;

        service
            .get_post(id)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to get post: {:?}", e)))
    }
}

#[Object]
impl SocialMutation {
    async fn create_post(
        &self,
        ctx: &Context<'_>,
        author_id: Uuid,
        content: String,
        visibility: Visibility, // Changed from String
    ) -> Result<Post> {
        let service = ctx.data::<Arc<SocialService>>()?;

        let post = service
            .create_post(author_id, content, visibility)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to create post: {:?}", e)))?;

        Ok(post)
    }
}
// Helper function to get authenticated user ID (temporary implementation)
fn get_current_user_id(ctx: &Context<'_>) -> Result<Uuid> {
    // TODO: Replace with actual authentication logic
    // For now, return a placeholder user ID
    Ok(Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap())
}