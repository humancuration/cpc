use async_trait::async_trait;
use crate::domain::vote::{Vote, VoteType, VoteEvent};
use crate::domain::auth_service::AuthService;
use crate::infrastructure::repositories::vote_repo::VoteRepository;
use crate::infrastructure::repositories::post_repo::PostRepository;
use uuid::Uuid;
use std::sync::Arc;
use crate::application::error::ApplicationError;

#[derive(Debug, Clone)]
pub struct VotePostInput {
    pub user_id: Uuid,
    pub post_id: Uuid,
    pub vote_type: VoteType,
}

#[async_trait]
pub trait VoteService: Send + Sync {
    async fn vote_post(&self, input: VotePostInput) -> Result<i32, ApplicationError>;
    async fn vote_comment(&self, input: VotePostInput) -> Result<i32, ApplicationError>;
    async fn remove_vote(&self, user_id: Uuid, post_id: Uuid) -> Result<i32, ApplicationError>;
}

pub struct VoteServiceImpl {
    vote_repo: Arc<dyn VoteRepository>,
    post_repo: Arc<dyn PostRepository>,
    auth_service: Arc<dyn AuthService>,
}

impl VoteServiceImpl {
    pub fn new(
        vote_repo: Arc<dyn VoteRepository>,
        post_repo: Arc<dyn PostRepository>,
        auth_service: Arc<dyn AuthService>,
    ) -> Self {
        Self {
            vote_repo,
            post_repo,
            auth_service,
        }
    }
}

#[async_trait]
impl VoteService for VoteServiceImpl {
    async fn vote_post(&self, input: VotePostInput) -> Result<i32, ApplicationError> {
        // Check if post exists
        if self.post_repo.find_by_id(input.post_id).await?.is_none() {
            return Err(ApplicationError::InvalidInput("Post not found".to_string()));
        }
        
        // Check if user is trying to vote on their own post
        let post = self.post_repo.find_by_id(input.post_id).await?
            .ok_or(ApplicationError::InvalidInput("Post not found".to_string()))?;
        
        if post.user_id == input.user_id {
            return Err(ApplicationError::InvalidInput("Cannot vote on your own post".to_string()));
        }
        
        // Check if user has already voted on this post
        match self.vote_repo.find_by_user_and_post(input.user_id, input.post_id).await? {
            Some(existing_vote) => {
                // If it's the same vote type, remove the vote (toggle off)
                if existing_vote.vote_type == input.vote_type {
                    self.vote_repo.delete(input.user_id, input.post_id).await?;
                } else {
                    // Update the existing vote
                    let updated_vote = Vote::new(input.user_id, input.post_id, input.vote_type);
                    self.vote_repo.update(&updated_vote).await?;
                }
            }
            None => {
                // Create a new vote
                let vote = Vote::new(input.user_id, input.post_id, input.vote_type);
                self.vote_repo.create(&vote).await?;
            }
        }
        
        // Update user karma based on vote
        let karma_delta = match input.vote_type {
            VoteType::Upvote => 1,
            VoteType::Downvote => -1,
        };
        
        // Create and handle vote event
        let vote_event = VoteEvent::new(post.user_id, input.vote_type);
        self.auth_service.handle_vote_event(vote_event).await
            .map_err(|e| ApplicationError::InvalidInput(format!("Failed to update karma: {:?}", e)))?;
        
        // Get the updated vote count
        let vote_count = self.vote_repo.get_vote_count(input.post_id).await?;
        
        Ok(vote_count)
    }
    
    async fn vote_comment(&self, input: VotePostInput) -> Result<i32, ApplicationError> {
        // For now, we'll use the same logic as voting on posts
        // In a more complex implementation, we might want to differentiate
        self.vote_post(input).await
    }
    
    async fn remove_vote(&self, user_id: Uuid, post_id: Uuid) -> Result<i32, ApplicationError> {
        // Check if post exists
        if self.post_repo.find_by_id(post_id).await?.is_none() {
            return Err(ApplicationError::InvalidInput("Post not found".to_string()));
        }
        
        // Remove the vote
        self.vote_repo.delete(user_id, post_id).await?;
        
        // Get the updated vote count
        let vote_count = self.vote_repo.get_vote_count(post_id).await?;
        
        Ok(vote_count)
    }
}