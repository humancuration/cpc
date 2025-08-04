//! Reaction service implementation
//!
//! This module provides the concrete implementation of the ReactionService trait.

use crate::domain::models::{Reaction, ReactionType, TargetType};
use crate::domain::repository::{ReactionRepository, RepositoryError};
use crate::domain::service::{ReactionService, ServiceError};
use async_trait::async_trait;
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Arc;

/// Implementation of ReactionService
pub struct ReactionServiceImpl {
    reaction_repository: Arc<dyn ReactionRepository>,
}

impl ReactionServiceImpl {
    /// Create a new ReactionServiceImpl
    pub fn new(reaction_repository: Arc<dyn ReactionRepository>) -> Self {
        Self {
            reaction_repository,
        }
    }
}

#[async_trait]
impl ReactionService for ReactionServiceImpl {
    async fn add_reaction(
        &self,
        user_id: Uuid,
        target_id: Uuid,
        target_type: TargetType,
        reaction_type: ReactionType,
    ) -> Result<Reaction, ServiceError> {
        // Check if user has already reacted to this target
        let already_reacted = self.reaction_repository
            .user_has_reacted(user_id, target_id, target_type.clone())
            .await
            .map_err(ServiceError::from)?;
            
        if already_reacted {
            return Err(ServiceError::ValidationError(
                "User has already reacted to this target".to_string()
            ));
        }
        
        let reaction = Reaction::new(user_id, target_id, target_type, reaction_type);
        self.reaction_repository
            .add_reaction(&reaction)
            .await
            .map_err(ServiceError::from)?;
            
        Ok(reaction)
    }
    
    async fn remove_reaction(&self, user_id: Uuid, reaction_id: Uuid) -> Result<(), ServiceError> {
        // First, verify that the user owns this reaction
        let reaction = self.reaction_repository
            .get_reactions_for_target(reaction_id, TargetType::Post) // We don't know the target type, so we'll use a placeholder
            .await
            .map_err(ServiceError::from)?
            .into_iter()
            .find(|r| r.id == reaction_id);
            
        if let Some(reaction) = reaction {
            if reaction.user_id != user_id {
                return Err(ServiceError::Unauthorized);
            }
            
            self.reaction_repository
                .remove_reaction(reaction_id)
                .await
                .map_err(ServiceError::from)
        } else {
            Err(ServiceError::RepositoryError(RepositoryError::NotFound))
        }
    }
    
    async fn get_reactions_for_target(
        &self,
        target_id: Uuid,
        target_type: TargetType,
    ) -> Result<Vec<Reaction>, ServiceError> {
        self.reaction_repository
            .get_reactions_for_target(target_id, target_type)
            .await
            .map_err(ServiceError::from)
    }
    
    async fn get_reaction_summary(
        &self,
        target_id: Uuid,
        target_type: TargetType,
    ) -> Result<HashMap<String, usize>, ServiceError> {
        self.reaction_repository
            .get_reaction_summary(target_id, target_type)
            .await
            .map_err(ServiceError::from)
    }
}