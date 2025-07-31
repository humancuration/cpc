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
        let parent_id = input.parent_id.map(|id| uuid::Uuid::parse_str(&id.to_string()).unwrap());
        // In a real implementation, we would get the user_id from the authentication context
        let user_id = uuid::Uuid::nil(); // Placeholder
        let service_input = ServiceCreateCommentInput {
            post_id,
            user_id,
            content: input.content,
            parent_id,
        };
        let comment = service.create_comment(service_input).await?;
        
        Ok(CommentObject::from(comment))
    }
    
    async fn update_comment(&self, ctx: &Context<'_>, id: ID, input: UpdateCommentInput) -> Result<CommentObject> {
        let service = ctx.data::<std::sync::Arc<dyn CommentService>>()?;
        let uuid = uuid::Uuid::parse_str(&id.to_string())?;
        let service_input = ServiceUpdateCommentInput {
            content: input.content,
        };
        let comment = service.update_comment(uuid, service_input).await?;
        
        Ok(CommentObject::from(comment))
    }
    
    async fn delete_comment(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
        let service = ctx.data::<std::sync::Arc<dyn CommentService>>()?;
        let uuid = uuid::Uuid::parse_str(&id.to_string())?;
        let result = service.delete_comment(uuid).await?;
        
        Ok(result)
    }
    
    async fn vote_post(&self, ctx: &Context<'_>, input: VotePostInput) -> Result<i32> {
        let service = ctx.data::<std::sync::Arc<dyn VoteService>>()?;
        let post_id = uuid::Uuid::parse_str(&input.post_id.to_string())?;
        // In a real implementation, we would get the user_id from the authentication context
        let user_id = uuid::Uuid::nil(); // Placeholder
        let vote_type = match input.vote_type.as_str() {
            "UPVOTE" => VoteType::Upvote,
            "DOWNVOTE" => VoteType::Downvote,
            _ => return Err("Invalid vote type".into()),
        };
        let service_input = ServiceVotePostInput {
            user_id,
            post_id,
            vote_type,
        };
        let vote_count = service.vote_post(service_input).await?;
        
        Ok(vote_count)
    }
    
    async fn vote_comment(&self, ctx: &Context<'_>, input: VotePostInput) -> Result<i32> {
        let service = ctx.data::<std::sync::Arc<dyn VoteService>>()?;
        let post_id = uuid::Uuid::parse_str(&input.post_id.to_string())?;
        // In a real implementation, we would get the user_id from the authentication context
        let user_id = uuid::Uuid::nil(); // Placeholder
        let vote_type = match input.vote_type.as_str() {
            "UPVOTE" => VoteType::Upvote,
            "DOWNVOTE" => VoteType::Downvote,
            _ => return Err("Invalid vote type".into()),
        };
        let service_input = ServiceVotePostInput {
            user_id,
            post_id,
            vote_type,
        };
        let vote_count = service.vote_comment(service_input).await?;
        
        Ok(vote_count)
    }
}