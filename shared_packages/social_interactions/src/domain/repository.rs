//! Repository traits for social interactions
//!
//! This module defines the traits for data access operations.

use crate::domain::models::{Reaction, Comment, Share, TargetType, ContentType};
use async_trait::async_trait;
use uuid::Uuid;
use std::collections::HashMap;

/// Error types for repository operations
#[derive(Debug, Clone, PartialEq)]
pub enum RepositoryError {
    NotFound,
    DatabaseError(String),
    ValidationError(String),
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryError::NotFound => write!(f, "Not found"),
            RepositoryError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            RepositoryError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for RepositoryError {}

/// Repository trait for reactions
#[async_trait]
pub trait ReactionRepository: Send + Sync {
    /// Add a new reaction
    async fn add_reaction(&self, reaction: &Reaction) -> Result<(), RepositoryError>;
    
    /// Remove a reaction by ID
    async fn remove_reaction(&self, reaction_id: Uuid) -> Result<(), RepositoryError>;
    
    /// Get all reactions for a target
    async fn get_reactions_for_target(
        &self,
        target_id: Uuid,
        target_type: TargetType,
    ) -> Result<Vec<Reaction>, RepositoryError>;
    
    /// Get reaction summary for a target (count by reaction type)
    async fn get_reaction_summary(
        &self,
        target_id: Uuid,
        target_type: TargetType,
    ) -> Result<HashMap<String, usize>, RepositoryError>;
    
    /// Check if a user has already reacted to a target
    async fn user_has_reacted(
        &self,
        user_id: Uuid,
        target_id: Uuid,
        target_type: TargetType,
    ) -> Result<bool, RepositoryError>;
}

/// Repository trait for comments
#[async_trait]
pub trait CommentRepository: Send + Sync {
    /// Add a new comment
    async fn add_comment(&self, comment: &Comment) -> Result<(), RepositoryError>;
    
    /// Update an existing comment
    async fn update_comment(&self, comment: &Comment) -> Result<(), RepositoryError>;
    
    /// Delete a comment by ID
    async fn delete_comment(&self, comment_id: Uuid) -> Result<(), RepositoryError>;
    
    /// Get a comment by ID
    async fn get_comment(&self, comment_id: Uuid) -> Result<Option<Comment>, RepositoryError>;
    
    /// Get all comments for a target (with optional depth for nested comments)
    async fn get_comments_for_target(
        &self,
        target_id: Uuid,
        target_type: TargetType,
        max_depth: Option<usize>,
    ) -> Result<Vec<Comment>, RepositoryError>;
    
    /// Get all replies to a comment
    async fn get_replies(&self, comment_id: Uuid) -> Result<Vec<Comment>, RepositoryError>;
}

/// Repository trait for shares
#[async_trait]
pub trait ShareRepository: Send + Sync {
    /// Add a new share
    async fn add_share(&self, share: &Share) -> Result<(), RepositoryError>;
    
    /// Get shares by user
    async fn get_shares_by_user(&self, user_id: Uuid) -> Result<Vec<Share>, RepositoryError>;
    
    /// Get shares of specific content
    async fn get_shares_of_content(
        &self,
        content_id: Uuid,
        content_type: ContentType,
    ) -> Result<Vec<Share>, RepositoryError>;
    
    /// Delete a share by ID
    async fn delete_share(&self, share_id: Uuid) -> Result<(), RepositoryError>;
}