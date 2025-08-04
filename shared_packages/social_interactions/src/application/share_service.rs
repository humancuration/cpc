//! Share service implementation
//!
//! This module provides the concrete implementation of the ShareService trait.

use crate::domain::models::{Share, ContentType};
use crate::domain::repository::{ShareRepository, RepositoryError};
use crate::domain::service::{ShareService, ServiceError};
use async_trait::async_trait;
use uuid::Uuid;
use std::sync::Arc;

/// Implementation of ShareService
pub struct ShareServiceImpl {
    share_repository: Arc<dyn ShareRepository>,
}

impl ShareServiceImpl {
    /// Create a new ShareServiceImpl
    pub fn new(share_repository: Arc<dyn ShareRepository>) -> Self {
        Self {
            share_repository,
        }
    }
}

#[async_trait]
impl ShareService for ShareServiceImpl {
    async fn share_content(
        &self,
        user_id: Uuid,
        content_id: Uuid,
        content_type: ContentType,
        shared_with: Option<Uuid>,
    ) -> Result<Share, ServiceError> {
        // Create the share
        let share = Share::new(user_id, content_id, content_type, shared_with);
        self.share_repository
            .add_share(&share)
            .await
            .map_err(ServiceError::from)?;
            
        Ok(share)
    }
    
    async fn get_shares_by_user(&self, user_id: Uuid) -> Result<Vec<Share>, ServiceError> {
        self.share_repository
            .get_shares_by_user(user_id)
            .await
            .map_err(ServiceError::from)
    }
    
    async fn get_shares_of_content(
        &self,
        content_id: Uuid,
        content_type: ContentType,
    ) -> Result<Vec<Share>, ServiceError> {
        self.share_repository
            .get_shares_of_content(content_id, content_type)
            .await
            .map_err(ServiceError::from)
    }
    
    async fn unshare_content(&self, user_id: Uuid, share_id: Uuid) -> Result<(), ServiceError> {
        // Get the existing share
        let shares = self.share_repository
            .get_shares_by_user(user_id)
            .await
            .map_err(ServiceError::from)?;
            
        let share = shares
            .into_iter()
            .find(|s| s.id == share_id)
            .ok_or(ServiceError::RepositoryError(RepositoryError::NotFound))?;
            
        // Check ownership
        if share.user_id != user_id {
            return Err(ServiceError::Unauthorized);
        }
        
        // Delete the share
        self.share_repository
            .delete_share(share_id)
            .await
            .map_err(ServiceError::from)
    }
}