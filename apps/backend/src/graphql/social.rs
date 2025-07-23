use crate::services::social_service::SocialService;
use async_graphql::{Context, Object, Result};
use cpc_core::models::social::post::{Post, Visibility}; // Make sure Visibility is imported
use std::sync::Arc;
use uuid::Uuid;

#[derive(Default)]
pub struct SocialQuery;

#[derive(MergedObject, Default)]
pub struct SocialMutation;

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
}

#[Object]
impl SocialMutation {
    async fn create_post(
        &self,
        ctx: &Context<'_>,
        author_id: Uuid,
        content: String,
        visibility: Visibility,
    ) -> Result<Post> {
        let service = ctx.data::<Arc<SocialService>>()?;

        let post = service
            .create_post(author_id, content, visibility)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to create post: {:?}", e)))?;

        Ok(post)
    }

    async fn create_post(
        &self,
        ctx: &Context<'_>,
        author_id: Uuid,
        content: String,
        visibility: Visibility,
    ) -> Result<Post> {
        let service = ctx.data::<Arc<SocialService>>()?;

        let post = service
            .create_post(author_id, content, visibility)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to create post: {:?}", e)))?;

        async fn follow_user(
            &self,
            ctx: &Context<'_>,
            follower_id: Uuid,
            following_id: Uuid
        ) -> Result<Relationship> {
            let service = ctx.data::<Arc<SocialService>>()?;
            let current_user_id = get_current_user_id(ctx)?;
            
            if current_user_id != follower_id {
                return Err(async_graphql::Error::new("You can only follow as yourself"));
            }
            
            let relationship = service.follow_user(follower_id, following_id).await
                .map_err(|e| async_graphql::Error::new(format!("Follow failed: {}", e)))?;
                
            Ok(relationship)
        }
            
        Ok(())
    }

    async fn unfollow_user(
        &self,
        ctx: &Context<'_>,
        follower_id: Uuid,
        following_id: Uuid
    ) -> Result<bool> {
        let service = ctx.data::<Arc<SocialService>>()?;
        let current_user_id = get_current_user_id(ctx)?;
        
        if current_user_id != follower_id {
            return Err(async_graphql::Error::new("You can only unfollow as yourself"));
        }
        
        service.unfollow_user(follower_id, following_id).await
            .map_err(|e| async_graphql::Error::new(format!("Unfollow failed: {}", e)))?;
            
        Ok(true)
    }
}
// Helper function to get authenticated user ID
fn get_current_user_id(ctx: &Context<'_>) -> Result<Uuid> {
    // Extract user ID from authentication context
    let auth_data = ctx.data::<auth::AuthData>()?;
    Ok(auth_data.user_id)
}