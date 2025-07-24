use std::sync::Arc;
use uuid::Uuid;
use cpc_core::models::social::post::Post;
use cpc_core::repositories::social::post_repository::PostRepository;
use cpc_core::repositories::social::media_repository::MediaRepository;

#[derive(Debug, thiserror::Error)]
pub enum FeedServiceError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Media service error: {0}")]
    MediaService(#[from] super::media_service::MediaServiceError),
}

pub struct FeedService {
    post_repo: Arc<dyn PostRepository>,
    media_repo: Arc<dyn MediaRepository>,
}

impl FeedService {
    pub fn new(
        post_repo: Arc<dyn PostRepository>,
        media_repo: Arc<dyn MediaRepository>,
    ) -> Self {
        Self {
            post_repo,
            media_repo,
        }
    }

    pub async fn get_user_feed(
        &self,
        user_id: Uuid,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Post>, FeedServiceError> {
        let posts = self.post_repo
            .get_feed_posts(user_id, limit, offset)
            .await?;
        
        Ok(posts)
    }

    pub async fn get_user_posts(
        &self,
        user_id: Uuid,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Post>, FeedServiceError> {
        let posts = self.post_repo
            .get_user_posts(user_id, limit, offset)
            .await?;
        
        Ok(posts)
    }

    pub async fn get_cooperative_feed(
        &self,
        cooperative_id: Uuid,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Post>, FeedServiceError> {
        let posts = self.post_repo
            .get_cooperative_posts(cooperative_id, limit, offset)
            .await?;
        
        Ok(posts)
    }

    pub async fn get_post_with_media(&self, post_id: Uuid) -> Result<(Post, Vec<cpc_core::models::social::post::MediaItem>), FeedServiceError> {
        let post = self.post_repo
            .find_post_by_id(post_id)
            .await?
            .ok_or_else(|| sqlx::Error::RowNotFound)?;
        
        let media_items = self.media_repo
            .find_media_by_post_id(post_id)
            .await?;
        
        Ok((post, media_items))
    }
}