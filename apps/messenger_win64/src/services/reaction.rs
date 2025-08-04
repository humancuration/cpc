//! Implementation of the ReactionService trait

use shared_packages::messenger::{
    models::{Reaction, Message},
    services::ReactionService,
    errors::MessengerError
};
use crate::repositories::reaction::ReactionRepository;
use async_trait::async_trait;
use uuid::Uuid;
use std::sync::Arc;

/// Implementation of the ReactionService
pub struct ReactionServiceImpl {
    reaction_repository: Arc<ReactionRepository>,
}

impl ReactionServiceImpl {
    /// Create a new ReactionService implementation
    pub fn new(reaction_repository: Arc<ReactionRepository>) -> Self {
        Self {
            reaction_repository,
        }
    }
}

#[async_trait]
impl ReactionService for ReactionServiceImpl {
    async fn add_reaction(&self, message_id: Uuid, user_id: Uuid, reaction_type: String) -> Result<Reaction, MessengerError> {
        self.reaction_repository
            .add_reaction(message_id, user_id, reaction_type)
            .await
    }
    
    async fn remove_reaction(&self, message_id: Uuid, user_id: Uuid, reaction_type: String) -> Result<(), MessengerError> {
        // In a real implementation, we would need to first find the reaction ID
        // For now, we'll use a placeholder
        Ok(())
    }
    
    async fn get_message_reactions(&self, message_id: Uuid) -> Result<Vec<Reaction>, MessengerError> {
        self.reaction_repository
            .get_message_reactions(message_id)
            .await
    }
}