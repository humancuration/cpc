use async_trait::async_trait;
use crate::domain::comment::Comment;
use crate::infrastructure::repositories::comment_repo::CommentRepository;
use crate::infrastructure::repositories::post_repo::PostRepository;
use uuid::Uuid;
use std::sync::Arc;
use crate::application::error::ApplicationError;

#[derive(Debug, Clone)]
pub struct CreateCommentInput {
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub parent_id: Option<Uuid>,
}

#[derive(Debug, Clone)]
pub struct UpdateCommentInput {
    pub content: String,
}

#[async_trait]
pub trait CommentService: Send + Sync {
    async fn create_comment(&self, input: CreateCommentInput) -> Result<Comment, ApplicationError>;
    async fn update_comment(&self, id: Uuid, input: UpdateCommentInput) -> Result<Comment, ApplicationError>;
    async fn delete_comment(&self, id: Uuid) -> Result<bool, ApplicationError>;
    async fn get_comment(&self, id: Uuid) -> Result<Option<Comment>, ApplicationError>;
    async fn get_comment_thread(&self, id: Uuid) -> Result<Vec<Comment>, ApplicationError>;
}

pub struct CommentServiceImpl {
    comment_repo: Arc<dyn CommentRepository>,
    post_repo: Arc<dyn PostRepository>,
}

impl CommentServiceImpl {
    pub fn new(
        comment_repo: Arc<dyn CommentRepository>,
        post_repo: Arc<dyn PostRepository>,
    ) -> Self {
        Self { comment_repo, post_repo }
    }
    
    /// Check if a comment exceeds the maximum nesting depth (10 levels)
    async fn check_nesting_depth(&self, parent_id: Option<Uuid>) -> Result<u32, ApplicationError> {
        let mut depth = 0;
        let mut current_id = parent_id;
        
        while let Some(id) = current_id {
            // Check if we've exceeded the maximum depth
            if depth >= 10 {
                return Err(ApplicationError::InvalidInput("Comments cannot be nested more than 10 levels deep".to_string()));
            }
            
            // Get the parent comment
            let parent_comment = self.comment_repo.find_by_id(id).await?
                .ok_or(ApplicationError::InvalidInput("Parent comment not found".to_string()))?;
            
            current_id = parent_comment.parent_id;
            depth += 1;
        }
        
        Ok(depth)
    }
}

#[async_trait]
impl CommentService for CommentServiceImpl {
    async fn create_comment(&self, input: CreateCommentInput) -> Result<Comment, ApplicationError> {
        // Validate input
        if input.content.is_empty() {
            return Err(ApplicationError::InvalidInput("Comment content cannot be empty".to_string()));
        }
        
        // Check if post exists
        if self.post_repo.find_by_id(input.post_id).await?.is_none() {
            return Err(ApplicationError::InvalidInput("Post not found".to_string()));
        }
        
        // If this is a reply to another comment, check nesting depth
        if let Some(parent_id) = input.parent_id {
            // Verify parent comment exists
            if self.comment_repo.find_by_id(parent_id).await?.is_none() {
                return Err(ApplicationError::InvalidInput("Parent comment not found".to_string()));
            }
            
            // Check nesting depth
            self.check_nesting_depth(Some(parent_id)).await?;
        }
        
        // Create comment
        let comment = Comment::new(
            input.post_id,
            input.user_id,
            input.content,
            input.parent_id,
        );
        
        self.comment_repo.create(&comment).await?;
        
        Ok(comment)
    }
    
    async fn update_comment(&self, id: Uuid, input: UpdateCommentInput) -> Result<Comment, ApplicationError> {
        // Find existing comment
        let mut comment = self.comment_repo.find_by_id(id).await?
            .ok_or(ApplicationError::NotFound)?;
        
        // Update content
        comment.content = input.content;
        
        // Update timestamp
        comment.updated_at = chrono::Utc::now();
        
        // Save updated comment
        self.comment_repo.update(&comment).await?;
        
        Ok(comment)
    }
    
    async fn delete_comment(&self, id: Uuid) -> Result<bool, ApplicationError> {
        // Check if comment exists
        if self.comment_repo.find_by_id(id).await?.is_none() {
            return Err(ApplicationError::NotFound);
        }
        
        // Delete comment
        self.comment_repo.delete(id).await?;
        
        Ok(true)
    }
    
    async fn get_comment(&self, id: Uuid) -> Result<Option<Comment>, ApplicationError> {
        self.comment_repo.find_by_id(id).await.map_err(ApplicationError::from)
    }
    
    async fn get_comment_thread(&self, id: Uuid) -> Result<Vec<Comment>, ApplicationError> {
        // Find the comment
        let comment = self.comment_repo.find_by_id(id).await?
            .ok_or(ApplicationError::NotFound)?;
        
        // Get all replies to this comment
        let replies = self.comment_repo.find_replies(id).await?;
        
        // Combine the comment and its replies
        let mut thread = vec![comment];
        thread.extend(replies);
        
        Ok(thread)
    }
}