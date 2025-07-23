use cpc_core::models::social::post::{Post, Visibility};
use cpc_core::repositories::social::post_repository::{PostRepository, CreatePostData};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub enum SocialServiceError {
    PostNotFound,
    DatabaseError(sqlx::Error),
    // Add other specific errors as needed
}

// Allow converting sqlx::Error into our custom error type
impl From<sqlx::Error> for SocialServiceError {
    fn from(e: sqlx::Error) -> Self {
        SocialServiceError::DatabaseError(e)
    }
}

pub type Result<T> = std::result::Result<T, SocialServiceError>;

pub struct SocialService {
    post_repo: Arc<dyn PostRepository>,
    relationship_repo: Arc<dyn RelationshipRepository>,
}

impl SocialService {
    pub fn new(
        post_repo: Arc<dyn PostRepository>,
        relationship_repo: Arc<dyn RelationshipRepository>
    ) -> Self {
        Self { post_repo, relationship_repo }
    }

    pub async fn create_post(
        &self,
        author_id: Uuid,
        content: String,
        visibility: Visibility,
    ) -> Result<Post> {
        let create_data = CreatePostData {
            author_id,
            content,
            visibility,
            cooperative_id: None, // Or handle this based on visibility
        };

        let post = self.post_repo.create_post(create_data).await?;
        Ok(post)
    }

    pub async fn get_post(&self, post_id: Uuid) -> Result<Post> {
        self.post_repo
            .find_post_by_id(post_id)
            .await?
            .ok_or(SocialServiceError::PostNotFound)
    }

    pub async fn follow_user(
        &self,
        follower_id: Uuid,
        followed_id: Uuid
    ) -> Result<Relationship> {
        self.relationship_repo
            .follow_user(follower_id, followed_id)
            .await
            .map_err(SocialServiceError::from)
    }

    pub async fn unfollow_user(
        &self,
        follower_id: Uuid,
        followed_id: Uuid
    ) -> Result<()> {
        self.relationship_repo
            .unfollow_user(follower_id, followed_id)
            .await
            .map(|_| ())
            .map_err(SocialServiceError::from)
    }
}