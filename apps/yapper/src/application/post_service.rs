use crate::domain::post::{Yap, YapError};
use crate::domain::events::{EventPublisher, YapperEvent};
use crate::infrastructure::post_repository::PostRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct PostService {
    post_repo: PostRepository,
    event_publisher: Arc<dyn EventPublisher>,
}

impl PostService {
    pub fn new(post_repo: PostRepository, event_publisher: Arc<dyn EventPublisher>) -> Self {
        Self { post_repo, event_publisher }
    }

    pub fn create_yap(&self, user_id: Uuid, content: String) -> Result<Yap, YapError> {
        let yap = Yap::new(user_id, content)?;
        self.post_repo.save(&yap).map_err(|e| YapError::ContentEmpty)?; // Using ContentEmpty as a placeholder
        self.event_publisher.publish(YapperEvent::YapCreated {
            id: yap.id,
            user_id: yap.user_id,
            content: yap.content.clone(),
        });
        Ok(yap)
    }

    pub fn like_yap(&self, yap_id: Uuid, user_id: Uuid) -> Result<(), String> {
        // In a real implementation, this would like a yap
        self.event_publisher.publish(YapperEvent::YapLiked {
            yap_id,
            user_id,
        });
        Ok(())
    }

    pub fn share_yap(&self, yap_id: Uuid, user_id: Uuid) -> Result<(), String> {
        // In a real implementation, this would share a yap
        self.event_publisher.publish(YapperEvent::YapShared {
            yap_id,
            user_id,
        });
        Ok(())
    }

    pub fn get_yap(&self, id: Uuid) -> Result<Option<Yap>, String> {
        self.post_repo.find_by_id(id)
    }
}