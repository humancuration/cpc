use async_trait::async_trait;
use crate::domain::post::Post;
use crate::domain::media_asset::MediaAsset;
use crate::infrastructure::repositories::post_repo::PostRepository;
use crate::infrastructure::repositories::community_repo::CommunityRepository;
use uuid::Uuid;
use std::sync::Arc;
use crate::application::error::ApplicationError;

#[derive(Debug, Clone)]
pub struct CreatePostInput {
    pub community_id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
    pub media_assets: Vec<MediaAsset>,
}

#[derive(Debug, Clone)]
pub struct UpdatePostInput {
    pub title: Option<String>,
    pub content: Option<String>,
    pub media_assets: Option<Vec<MediaAsset>>,
}

#[async_trait]
pub trait PostService: Send + Sync {
    async fn create_post(&self, input: CreatePostInput) -> Result<Post, ApplicationError>;
    async fn update_post(&self, id: Uuid, input: UpdatePostInput) -> Result<Post, ApplicationError>;
    async fn delete_post(&self, id: Uuid) -> Result<bool, ApplicationError>;
    async fn get_post(&self, id: Uuid) -> Result<Option<Post>, ApplicationError>;
    async fn get_posts_by_community(&self, community_id: Uuid) -> Result<Vec<Post>, ApplicationError>;
    async fn search_posts(&self, query: String) -> Result<Vec<Post>, ApplicationError>;
}

pub struct PostServiceImpl {
    post_repo: Arc<dyn PostRepository>,
    community_repo: Arc<dyn CommunityRepository>,
}

impl PostServiceImpl {
    pub fn new(
        post_repo: Arc<dyn PostRepository>,
        community_repo: Arc<dyn CommunityRepository>,
    ) -> Self {
        Self { post_repo, community_repo }
    }
}

#[async_trait]
impl PostService for PostServiceImpl {
    async fn create_post(&self, input: CreatePostInput) -> Result<Post, ApplicationError> {
        // Validate input
        if input.title.is_empty() {
            return Err(ApplicationError::InvalidInput("Post title cannot be empty".to_string()));
        }
        
        // Check if community exists
        if self.community_repo.find_by_id(input.community_id).await?.is_none() {
            return Err(ApplicationError::InvalidInput("Community not found".to_string()));
        }
        
        // Create post
        let post = Post::new(
            input.community_id,
            input.user_id,
            input.title,
            input.content,
            None, // Posts don't have parent_id, that's for comments
            input.media_assets,
        );
        
        self.post_repo.create(&post).await?;
        
        Ok(post)
    }
    
    async fn update_post(&self, id: Uuid, input: UpdatePostInput) -> Result<Post, ApplicationError> {
        // Find existing post
        let mut post = self.post_repo.find_by_id(id).await?
            .ok_or(ApplicationError::NotFound)?;
        
        // Update fields if provided
        if let Some(title) = input.title {
            if !title.is_empty() {
                post.title = title;
            }
        }
        
        if let Some(content) = input.content {
            post.content = content;
        }
        
        if let Some(media_assets) = input.media_assets {
            post.media_assets = media_assets;
        }
        
        // Update timestamp
        post.updated_at = chrono::Utc::now();
        
        // Save updated post
        self.post_repo.update(&post).await?;
        
        Ok(post)
    }
    
    async fn delete_post(&self, id: Uuid) -> Result<bool, ApplicationError> {
        // Check if post exists
        if self.post_repo.find_by_id(id).await?.is_none() {
            return Err(ApplicationError::NotFound);
        }
        
        // Delete post
        self.post_repo.delete(id).await?;
        
        Ok(true)
    }
    
    async fn get_post(&self, id: Uuid) -> Result<Option<Post>, ApplicationError> {
        self.post_repo.find_by_id(id).await.map_err(ApplicationError::from)
    }
    
    async fn get_posts_by_community(&self, community_id: Uuid) -> Result<Vec<Post>, ApplicationError> {
        // Check if community exists
        if self.community_repo.find_by_id(community_id).await?.is_none() {
            return Err(ApplicationError::InvalidInput("Community not found".to_string()));
        }
        
        self.post_repo.find_by_community(community_id).await.map_err(ApplicationError::from)
    }
    
    async fn search_posts(&self, query: String) -> Result<Vec<Post>, ApplicationError> {
        // For now, we'll implement a simple search that returns all posts
        // In a real implementation, this would search by title or content
        // This would require a new repository method or a different approach
        todo!("Implement search_posts")
    }
}