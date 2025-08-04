//! Repository implementation for reactions using the social_interactions package

use shared_packages::messenger::models::{Reaction as MessengerReaction};
use shared_packages::messenger::errors::MessengerError;
use shared_packages::social_interactions::domain::models::{Reaction as SocialReaction, ReactionType as SocialReactionType, TargetType as SocialTargetType};
use shared_packages::social_interactions::domain::service::{ReactionService as SocialReactionService, ServiceError as SocialServiceError};
use shared_packages::social_interactions::domain::repository::RepositoryError as SocialRepositoryError;
use async_trait::async_trait;
use uuid::Uuid;
use std::sync::Arc;

/// Repository for reaction operations that delegates to the social_interactions package
pub struct ReactionRepository {
    social_reaction_service: Arc<dyn SocialReactionService>,
}

impl ReactionRepository {
    /// Create a new ReactionRepository
    pub fn new(social_reaction_service: Arc<dyn SocialReactionService>) -> Self {
        Self {
            social_reaction_service,
        }
    }
    
    /// Convert a social reaction to a messenger reaction
    fn social_to_messenger_reaction(social_reaction: SocialReaction) -> MessengerReaction {
        MessengerReaction {
            id: social_reaction.id,
            message_id: social_reaction.target_id,
            user_id: social_reaction.user_id,
            reaction_type: social_reaction.reaction_type.to_string(),
            created_at: social_reaction.created_at,
        }
    }
    
    /// Convert a messenger reaction type to a social reaction type
    fn messenger_to_social_reaction_type(reaction_type: &str) -> Result<SocialReactionType, MessengerError> {
        match reaction_type {
            "like" => Ok(SocialReactionType::Like),
            "heart" => Ok(SocialReactionType::Heart),
            "celebrate" => Ok(SocialReactionType::Celebrate),
            "insightful" => Ok(SocialReactionType::Insightful),
            "funny" => Ok(SocialReactionType::Funny),
            "sad" => Ok(SocialReactionType::Sad),
            "angry" => Ok(SocialReactionType::Angry),
            _ => Err(MessengerError::InvalidInput { 
                message: format!("Unsupported reaction type: {}", reaction_type) 
            }),
        }
    }
    
    /// Convert a social service error to a messenger error
    fn social_error_to_messenger_error(error: SocialServiceError) -> MessengerError {
        match error {
            SocialServiceError::RepositoryError(SocialRepositoryError::NotFound) => {
                MessengerError::ReactionNotFound { id: Uuid::nil() } // We don't have the ID here
            }
            SocialServiceError::Unauthorized => {
                MessengerError::PermissionDenied { 
                    user_id: Uuid::nil(), // We don't have the user ID here
                    action: "reaction".to_string()
                }
            }
            SocialServiceError::ValidationError(msg) => {
                MessengerError::InvalidInput { message: msg }
            }
            _ => {
                MessengerError::StorageError { message: error.to_string() }
            }
        }
    }
}

impl ReactionRepository {
    /// Add a reaction to a message
    pub async fn add_reaction(&self, message_id: Uuid, user_id: Uuid, reaction_type: String) -> Result<MessengerReaction, MessengerError> {
        let social_reaction_type = self.messenger_to_social_reaction_type(&reaction_type)?;
        
        let social_reaction = self.social_reaction_service
            .add_reaction(
                user_id,
                message_id,
                SocialTargetType::Post, // Using Post as a placeholder, we might need a Message target type
                social_reaction_type
            )
            .await
            .map_err(Self::social_error_to_messenger_error)?;
            
        Ok(Self::social_to_messenger_reaction(social_reaction))
    }
    
    /// Remove a reaction from a message
    pub async fn remove_reaction(&self, reaction_id: Uuid, user_id: Uuid) -> Result<(), MessengerError> {
        self.social_reaction_service
            .remove_reaction(user_id, reaction_id)
            .await
            .map_err(Self::social_error_to_messenger_error)
    }
    
    /// Get all reactions for a message
    pub async fn get_message_reactions(&self, message_id: Uuid) -> Result<Vec<MessengerReaction>, MessengerError> {
        let social_reactions = self.social_reaction_service
            .get_reactions_for_target(message_id, SocialTargetType::Post) // Using Post as a placeholder
            .await
            .map_err(Self::social_error_to_messenger_error)?;
            
        Ok(social_reactions
            .into_iter()
            .map(Self::social_to_messenger_reaction)
            .collect())
    }
}