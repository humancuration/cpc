use async_graphql::{Context, Object, Result, ID};
use crate::application::{
    community_service::CommunityService,
    post_service::PostService,
    comment_service::CommentService,
    search_service::{SearchService, SearchCriteria},
};
use crate::api::objects::{
    community::CommunityObject,
    post::PostObject,
    comment::CommentObject,
    input::SearchCriteriaInput,
};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn community(&self, ctx: &Context<'_>, id: ID) -> Result<Option<CommunityObject>> {
        let service = ctx.data::<std::sync::Arc<dyn CommunityService>>()?;
        let uuid = uuid::Uuid::parse_str(&id.to_string())?;
        let community = service.get_community(uuid).await?;
        
        Ok(community.map(CommunityObject::from))
    }
    
    async fn communities(&self, ctx: &Context<'_>) -> Result<Vec<CommunityObject>> {
        // This would require a new method in the community service
        // For now, we'll return an empty vector
        Ok(vec![])
    }
    
    async fn post(&self, ctx: &Context<'_>, id: ID) -> Result<Option<PostObject>> {
        let service = ctx.data::<std::sync::Arc<dyn PostService>>()?;
        let uuid = uuid::Uuid::parse_str(&id.to_string())?;
        let post = service.get_post(uuid).await?;
        
        Ok(post.map(PostObject::from))
    }
    
    async fn posts(&self, ctx: &Context<'_>, community_id: ID) -> Result<Vec<PostObject>> {
        let service = ctx.data::<std::sync::Arc<dyn PostService>>()?;
        let uuid = uuid::Uuid::parse_str(&community_id.to_string())?;
        let posts = service.get_posts_by_community(uuid).await?;
        
        Ok(posts.into_iter().map(PostObject::from).collect())
    }
    
    async fn comment(&self, ctx: &Context<'_>, id: ID) -> Result<Option<CommentObject>> {
        let service = ctx.data::<std::sync::Arc<dyn CommentService>>()?;
        let uuid = uuid::Uuid::parse_str(&id.to_string())?;
        let comment = service.get_comment(uuid).await?;
        
        Ok(comment.map(CommentObject::from))
    }
    
    async fn comment_thread(&self, ctx: &Context<'_>, comment_id: ID) -> Result<Vec<CommentObject>> {
        let service = ctx.data::<std::sync::Arc<dyn CommentService>>()?;
        let uuid = uuid::Uuid::parse_str(&comment_id.to_string())?;
        let comments = service.get_comment_thread(uuid).await?;
        
        Ok(comments.into_iter().map(CommentObject::from).collect())
    }
    
    async fn search_communities(&self, ctx: &Context<'_>, query: String) -> Result<Vec<CommunityObject>> {
        // This is the old simple search - we'll keep it for backward compatibility
        let service = ctx.data::<std::sync::Arc<dyn CommunityService>>()?;
        let communities = service.search_communities(query).await?;
        
        Ok(communities.into_iter().map(CommunityObject::from).collect())
    }
    
    async fn search_posts(&self, ctx: &Context<'_>, query: String) -> Result<Vec<PostObject>> {
        // This is the old simple search - we'll keep it for backward compatibility
        let service = ctx.data::<std::sync::Arc<dyn PostService>>()?;
        let posts = service.search_posts(query).await?;
        
        Ok(posts.into_iter().map(PostObject::from).collect())
    }
    
    // New advanced search queries
    async fn search_communities_advanced(&self, ctx: &Context<'_>, criteria: SearchCriteriaInput) -> Result<Vec<CommunityObject>> {
        let service = ctx.data::<std::sync::Arc<dyn SearchService>>()?;
        let communities = service.search_communities(criteria.query).await?;
        
        Ok(communities.into_iter().map(CommunityObject::from).collect())
    }
    
    async fn search_posts_advanced(&self, ctx: &Context<'_>, criteria: SearchCriteriaInput) -> Result<Vec<PostObject>> {
        let service = ctx.data::<std::sync::Arc<dyn SearchService>>()?;
        
        let search_criteria = SearchCriteria {
            query: criteria.query,
            community_id: criteria.community_id,
            author_id: criteria.author_id,
            date_from: criteria.date_from,
            date_to: criteria.date_to,
            limit: criteria.limit.map(|l| l as u32),
            offset: criteria.offset.map(|o| o as u32),
        };
        
        let posts = service.search_posts(search_criteria).await?;
        Ok(posts.into_iter().map(PostObject::from).collect())
    }
}