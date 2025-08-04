//! Comment service implementation
//!
//! This module provides the concrete implementation of the CommentService trait.

use crate::domain::models::{Comment, TargetType};
use crate::domain::repository::{CommentRepository, RepositoryError};
use crate::domain::service::{CommentService, ServiceError};
use async_trait::async_trait;
use uuid::Uuid;
use std::sync::Arc;

/// Implementation of CommentService
pub struct CommentServiceImpl {
    comment_repository: Arc<dyn CommentRepository>,
}

impl CommentServiceImpl {
    /// Create a new CommentServiceImpl
    pub fn new(comment_repository: Arc<dyn CommentRepository>) -> Self {
        Self {
            comment_repository,
        }
    }
}

#[async_trait]
impl CommentService for CommentServiceImpl {
    async fn add_comment(
        &self,
        user_id: Uuid,
        target_id: Uuid,
        target_type: TargetType,
        content: String,
        parent_id: Option<Uuid>,
    ) -> Result<Comment, ServiceError> {
        // Validate content
        if content.trim().is_empty() {
            return Err(ServiceError::ValidationError(
                "Comment content cannot be empty".to_string()
            ));
        }
        
        // If this is a reply, verify the parent comment exists
        if let Some(parent_id) = parent_id {
            if self.comment_repository
                .get_comment(parent_id)
                .await
                .map_err(ServiceError::from)?
                .is_none()
            {
                return Err(ServiceError::ValidationError(
                    "Parent comment not found".to_string()
                ));
            }
        }
        
        let comment = Comment::new(user_id, target_id, target_type, content, parent_id);
        self.comment_repository
            .add_comment(&comment)
            .await
            .map_err(ServiceError::from)?;
            
        Ok(comment)
    }
    
    async fn edit_comment(
        &self,
        user_id: Uuid,
        comment_id: Uuid,
        new_content: String,
    ) -> Result<Comment, ServiceError> {
        // Validate content
        if new_content.trim().is_empty() {
            return Err(ServiceError::ValidationError(
                "Comment content cannot be empty".to_string()
            ));
        }
        
        // Get the existing comment
        let mut comment = self.comment_repository
            .get_comment(comment_id)
            .await
            .map_err(ServiceError::from)?
            .ok_or(ServiceError::RepositoryError(RepositoryError::NotFound))?;
            
        // Check ownership
        if comment.user_id != user_id {
            return Err(ServiceError::Unauthorized);
        }
        
        // Update content
        comment.update_content(new_content);
        self.comment_repository
            .update_comment(&comment)
            .await
            .map_err(ServiceError::from)?;
            
        Ok(comment)
    }
    
    async fn delete_comment(&self, user_id: Uuid, comment_id: Uuid) -> Result<(), ServiceError> {
        // Get the existing comment
        let comment = self.comment_repository
            .get_comment(comment_id)
            .await
            .map_err(ServiceError::from)?
            .ok_or(ServiceError::RepositoryError(RepositoryError::NotFound))?;
            
        // Check ownership
        if comment.user_id != user_id {
            return Err(ServiceError::Unauthorized);
        }
        
        // Delete the comment
        self.comment_repository
            .delete_comment(comment_id)
            .await
            .map_err(ServiceError::from)
    }
    
    async fn get_comments_for_target(
        &self,
        target_id: Uuid,
        target_type: TargetType,
        max_depth: Option<usize>,
    ) -> Result<Vec<Comment>, ServiceError> {
        self.comment_repository
            .get_comments_for_target(target_id, target_type, max_depth)
            .await
            .map_err(ServiceError::from)
    }
}