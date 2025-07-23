use async_trait::async_trait;
use sqlx::Error;
use std::sync::Arc;
use uuid::Uuid;
use cpc_core::models::social::post::{Post, Visibility};
use crate::repositories::social::{
    post_repository::{PostRepository, NewPost},
    relationship_repository::RelationshipRepository
};

#[derive(Debug)]
pub enum SocialServiceError {
    PostNotFound,
    DatabaseError(sqlx::Error),
    RelationshipNotFound,
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
        let new_post = NewPost {
            author_id,
            content,
            visibility,
            cooperative_id: None,
        };

        self.post_repo.create_post(new_post).await
            .map_err(SocialServiceError::from)
    }

    pub async fn get_post(&self, post_id: Uuid) -> Result<Post> {
        self.post_repo
            .get_post_by_id(post_id)
            .await
            .map_err(SocialServiceError::from)?
            .ok_or(SocialServiceError::PostNotFound)
    }

    pub async fn get_posts_by_user(&self, user_id: Uuid) -> Result<Vec<Post>> {
        self.post_repo
            .get_posts_by_user(user_id)
            .await
            .map_err(SocialServiceError::from)
    }

    pub async fn follow_user(
        &self,
        follower_id: Uuid,
        followed_id: Uuid
    ) -> Result<Relationship> {
        self.relationship_repo
            .follow(follower_id, followed_id)
            .await
            .map_err(SocialServiceError::from)
    }

    pub async fn unfollow_user(
        &self,
        follower_id: Uuid,
        followed_id: Uuid
    ) -> Result<()> {
        self.relationship_repo
            .unfollow(follower_id, followed_id)
            .await
            .map_err(SocialServiceError::from)
    }

    pub async fn get_followers(&self, user_id: Uuid) -> Result<Vec<Uuid>> {
        self.relationship_repo
            .get_followers(user_id)
            .await
            .map_err(SocialServiceError::from)
    }

    pub async fn get_following(&self, user_id: Uuid) -> Result<Vec<Uuid>> {
        self.relationship_repo
            .get_following(user_id)
            .await
            .map_err(SocialServiceError::from)
    }
}