//! Service traits for social interactions
//!
//! This module defines the traits for business logic operations.

use crate::domain::models::{Reaction, Comment, Share, ReactionType, TargetType, ContentType};
use crate::domain::repository::RepositoryError;
use async_trait::async_trait;
use uuid::Uuid;
use std::collections::HashMap;

/// Error types for service operations
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceError {
    RepositoryError(RepositoryError),
    ValidationError(String),
    Unauthorized,
}

impl From<RepositoryError> for ServiceError {
    fn from(error: RepositoryError) -> Self {
        ServiceError::RepositoryError(error)
    }
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::RepositoryError(err) => write!(f, "Repository error: {}", err),
            ServiceError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            ServiceError::Unauthorized => write!(f, "Unauthorized"),
        }
    }
}

impl std::error::Error for ServiceError {}

/// Service trait for reactions
#[async_trait]
pub trait ReactionService: Send + Sync {
    /// Add a reaction to a target
    async fn add_reaction(
        &self,
        user_id: Uuid,
        target_id: Uuid,
        target_type: TargetType,
        reaction_type: ReactionType,
    ) -> Result<Reaction, ServiceError>;
    
    /// Remove a reaction
    async fn remove_reaction(&self, user_id: Uuid, reaction_id: Uuid) -> Result<(), ServiceError>;
    
    /// Get all reactions for a target
    async fn get_reactions_for_target(
        &self,
        target_id: Uuid,
        target_type: TargetType,
    ) -> Result<Vec<Reaction>, ServiceError>;
    
    /// Get reaction summary for a target (count by reaction type)
    async fn get_reaction_summary(
        &self,
        target_id: Uuid,
        target_type: TargetType,
    ) -> Result<HashMap<String, usize>, ServiceError>;
}

/// Service trait for comments
#[async_trait]
pub trait CommentService: Send + Sync {
    /// Add a comment to a target
    async fn add_comment(
        &self,
        user_id: Uuid,
        target_id: Uuid,
        target_type: TargetType,
        content: String,
        parent_id: Option<Uuid>,
    ) -> Result<Comment, ServiceError>;
    
    /// Edit an existing comment
    async fn edit_comment(
        &self,
        user_id: Uuid,
        comment_id: Uuid,
        new_content: String,
    ) -> Result<Comment, ServiceError>;
    
    /// Delete a comment
    async fn delete_comment(&self, user_id: Uuid, comment_id: Uuid) -> Result<(), ServiceError>;
    
    /// Get comments for a target
    async fn get_comments_for_target(
        &self,
        target_id: Uuid,
        target_type: TargetType,
        max_depth: Option<usize>,
    ) -> Result<Vec<Comment>, ServiceError>;
}

/// Service trait for shares
#[async_trait]
pub trait ShareService: Send + Sync {
    /// Share content with another user or publicly
    async fn share_content(
        &self,
        user_id: Uuid,
        content_id: Uuid,
        content_type: ContentType,
        shared_with: Option<Uuid>, // None for public, Some(user_id) for private
    ) -> Result<Share, ServiceError>;
    
    /// Get shares by a user
    async fn get_shares_by_user(&self, user_id: Uuid) -> Result<Vec<Share>, ServiceError>;
    
    /// Get shares of specific content
    async fn get_shares_of_content(
        &self,
        content_id: Uuid,
        content_type: ContentType,
    ) -> Result<Vec<Share>, ServiceError>;
    
    /// Unshare content
    async fn unshare_content(&self, user_id: Uuid, share_id: Uuid) -> Result<(), ServiceError>;
}