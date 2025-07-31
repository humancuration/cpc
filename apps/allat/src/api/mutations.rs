use async_graphql::{Context, Object, Result, ID};
use crate::application::{
    community_service::{CommunityService, CreateCommunityInput as ServiceCreateCommunityInput, UpdateCommunityInput as ServiceUpdateCommunityInput},
    post_service::{PostService, CreatePostInput as ServiceCreatePostInput, UpdatePostInput as ServiceUpdatePostInput},
    comment_service::{CommentService, CreateCommentInput as ServiceCreateCommentInput, UpdateCommentInput as ServiceUpdateCommentInput},
    vote_service::{VoteService, VotePostInput as ServiceVotePostInput},
};
use crate::domain::vote::VoteType;
use crate::api::objects::{
    community::CommunityObject,
    post::PostObject,
    comment::CommentObject,
    input::{
        CreateCommunityInput,
        UpdateCommunityInput,
        CreatePostInput,
        UpdatePostInput,
        CreateCommentInput,
        UpdateCommentInput,
        VotePostInput,
    },
};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_community(&self, ctx: &Context<'_>, input: CreateCommunityInput) -> Result<CommunityObject> {
        let service = ctx.data::<std::sync::Arc<dyn CommunityService>>()?;
        let service_input = ServiceCreateCommunityInput {
            name: input.name,
            description: input.description,
            rules: input.rules,
        };
        let community = service.create_community(service_input).await?;
        
        Ok(CommunityObject::from(community))
    }
    
    async fn update_community(&self, ctx: &Context<'_>, id: ID, input: UpdateCommunityInput) -> Result<CommunityObject> {
        let service = ctx.data::<std::sync::Arc<dyn CommunityService>>()?;
        let uuid = uuid::Uuid::parse_str(&id.to_string())?;
        let service_input = ServiceUpdateCommunityInput {
            name: input.name,
            description: input.description,
            rules: input.rules,
        };
        let community = service.update_community(uuid, service_input).await?;
        
        Ok(CommunityObject::from(community))
    }
    
    async fn delete_community(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
        let service = ctx.data::<std::sync::Arc<dyn CommunityService>>()?;
        let uuid = uuid::Uuid::parse_str(&id.to_string())?;
        let result = service.delete_community(uuid).await?;
        
        Ok(result)
    }
    
    async fn create_post(&self, ctx: &Context<'_>, input: CreatePostInput) -> Result<PostObject> {
        let service = ctx.data::<std::sync::Arc<dyn PostService>>()?;
        let community_id = uuid::Uuid::parse_str(&input.community_id.to_string())?;
        // In a real implementation, we would get the user_id from the authentication context
        let user_id = uuid::Uuid::nil(); // Placeholder
        let service_input = ServiceCreatePostInput {
            community_id,
            user_id,
            title: input.title,
            content: input.content,
            media_assets: vec![], // For now, no media assets
        };
        let post = service.create_post(service_input).await?;
        
        Ok(PostObject::from(post))
    }
    
    async fn update_post(&self, ctx: &Context<'_>, id: ID, input: UpdatePostInput) -> Result<PostObject> {
        let service = ctx.data::<std::sync::Arc<dyn PostService>>()?;
        let uuid = uuid::Uuid::parse_str(&id.to_string())?;
        let service_input = ServiceUpdatePostInput {
            title: input.title,
            content: input.content,
            media_assets: None,
        };
        let post = service.update_post(uuid, service_input).await?;
        
        Ok(PostObject::from(post))
    }
    
    async fn delete_post(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
        let service = ctx.data::<std::sync::Arc<dyn PostService>>()?;
        let uuid = uuid::Uuid::parse_str(&id.to_string())?;
        let result = service.delete_post(uuid).await?;
        
        Ok(result)
    }
    
    async fn create_comment(&self, ctx: &Context<'_>, input: CreateCommentInput) -> Result<CommentObject> {
        let service = ctx.data::<std::sync::Arc<dyn CommentService>>()?;
        let post_id = uuid::Uuid::parse_str(&input.post_id.to_string())?;
