//! Implementation of the consent service

use async_trait::async_trait;
use lru::LruCache;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{
    domain::{
        model::Visibility,
        service::consent_service::{ConsentService, ConsentError},
    },
    domain::repository::RelationshipRepository,
};

/// Implementation of the ConsentService trait
pub struct ConsentServiceImpl<R> {
    relationship_repo: Arc<R>,
    cache: Arc<Mutex<LruCache<(Uuid, Uuid), bool>>>,
}

impl<R: RelationshipRepository> ConsentServiceImpl<R> {
    /// Create a new ConsentServiceImpl
    pub fn new(relationship_repo: Arc<R>) -> Self {
        Self {
            relationship_repo,
            cache: Arc::new(Mutex::new(LruCache::new(
                std::num::NonZeroUsize::new(1000).unwrap(),
            ))),
        }
    }

    /// Check if two users are friends
    async fn check_friendship(&self, user_id: Uuid, friend_id: Uuid) -> Result<bool, ConsentError> {
        // Check cache first
        {
            let mut cache = self.cache.lock().await;
            if let Some(&result) = cache.get(&(user_id, friend_id)) {
                return Ok(result);
            }
        }

        // Check if they are friends in the repository
        let relationships = self
            .relationship_repo
            .get_relationships_by_user(user_id)
            .await
            .map_err(|_| ConsentError::FriendshipCheckFailed)?;

        let is_friend = relationships
            .iter()
            .any(|r| r.target_user_id == friend_id && r.relationship_type == crate::domain::model::RelationshipType::Friend);

        // Cache the result
        {
            let mut cache = self.cache.lock().await;
            cache.put((user_id, friend_id), is_friend);
        }

        Ok(is_friend)
    }

    /// Check if a user is a member of a group
    /// For now, this is a placeholder implementation
    async fn check_group_membership(&self, _user_id: Uuid, _group_id: Uuid) -> Result<bool, ConsentError> {
        // In a real implementation, we would check group membership
        // For now, we'll return an error to indicate this is not implemented
        Err(ConsentError::GroupCheckFailed)
    }
}

#[async_trait]
impl<R: RelationshipRepository> ConsentService for ConsentServiceImpl<R> {
    async fn can_view_content(
        &self,
        viewer_id: Uuid,
        content_owner_id: Uuid,
        visibility: Visibility,
    ) -> Result<bool, ConsentError> {
        match visibility {
            Visibility::Public => Ok(true),
            Visibility::FriendsOnly => self.check_friendship(viewer_id, content_owner_id).await,
            Visibility::GroupMembers => self.check_group_membership(viewer_id, content_owner_id).await,
            Visibility::Private => Ok(viewer_id == content_owner_id),
        }
    }
}