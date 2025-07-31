//! In-memory repository for unified posts

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use crate::domain::post::{UnifiedPost, AppSource};
use crate::application::social_integration_service::UnifiedPostRepository;

/// In-memory repository for unified posts
#[derive(Debug)]
pub struct InMemoryUnifiedPostRepository {
    posts: Arc<RwLock<HashMap<Uuid, UnifiedPost>>>,
}

impl InMemoryUnifiedPostRepository {
    /// Create a new in-memory unified post repository
    pub fn new() -> Self {
        Self {
            posts: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl UnifiedPostRepository for InMemoryUnifiedPostRepository {
    async fn save(&self, post: &UnifiedPost) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut posts = self.posts.write().await;
        posts.insert(post.id, post.clone());
        Ok(())
    }
    
    async fn find_by_id(&self, id: Uuid) -> Result<Option<UnifiedPost>, Box<dyn std::error::Error + Send + Sync>> {
        let posts = self.posts.read().await;
        Ok(posts.get(&id).cloned())
    }
    
    async fn find_by_author(&self, author_id: Uuid) -> Result<Vec<UnifiedPost>, Box<dyn std::error::Error + Send + Sync>> {
        let posts = self.posts.read().await;
        let result = posts
            .values()
            .filter(|post| post.author_id == author_id)
            .cloned()
            .collect();
        Ok(result)
    }
    
    async fn find_by_source(&self, source: AppSource) -> Result<Vec<UnifiedPost>, Box<dyn std::error::Error + Send + Sync>> {
        let posts = self.posts.read().await;
        let result = posts
            .values()
            .filter(|post| post.source == source)
            .cloned()
            .collect();
        Ok(result)
    }
}

impl Default for InMemoryUnifiedPostRepository {
    fn default() -> Self {
        Self::new()
    }
}