use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use crate::domain::post::Post;
use crate::domain::community::Community;
use crate::infrastructure::repositories::post_repo::PostRepository;
use crate::infrastructure::repositories::community_repo::CommunityRepository;
use crate::application::error::ApplicationError;

#[derive(Debug, Clone)]
pub struct SearchCriteria {
    pub query: String,
    pub community_id: Option<Uuid>,
    pub author_id: Option<Uuid>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[async_trait]
pub trait SearchService: Send + Sync {
    async fn search_posts(&self, criteria: SearchCriteria) -> Result<Vec<Post>, ApplicationError>;
    async fn search_communities(&self, query: String) -> Result<Vec<Community>, ApplicationError>;
}

pub struct SearchServiceImpl {
    post_repo: Arc<dyn PostRepository>,
    community_repo: Arc<dyn CommunityRepository>,
}

impl SearchServiceImpl {
    pub fn new(
        post_repo: Arc<dyn PostRepository>,
        community_repo: Arc<dyn CommunityRepository>,
    ) -> Self {
        Self {
            post_repo,
            community_repo,
        }
    }
}

#[async_trait]
impl SearchService for SearchServiceImpl {
    async fn search_posts(&self, criteria: SearchCriteria) -> Result<Vec<Post>, ApplicationError> {
        // This will be implemented in the repository layer
        self.post_repo.search(criteria).await.map_err(ApplicationError::from)
    }
    
    async fn search_communities(&self, query: String) -> Result<Vec<Community>, ApplicationError> {
        // This will be implemented in the repository layer
        self.community_repo.search(&query).await.map_err(ApplicationError::from)
    }
}