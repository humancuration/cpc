// DEPRECATED: This file has been replaced by tip_service.rs
//! Reward service for social integration

use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::social_event::SocialEvent;
use crate::infrastructure::repositories::RewardTransactionRepository;
use cpc_wallet::application::wallet_service::WalletService;
use cpc_wallet::domain::primitives::{Money, Currency};

/// Configuration for social rewards
#[derive(Debug, Clone)]
pub struct RewardConfig {
    /// Dabloons awarded for creating a post
    pub post_creation_reward: Money,
    
    /// Dabloons awarded for creating a comment
    pub comment_creation_reward: Money,
    
    /// Dabloons awarded per 10 upvotes
    pub upvote_reward: Money,
    
    /// Dabloons awarded per share
    pub share_reward: Money,
    
    /// Dabloons awarded for following a user
    pub follow_reward: Money,
}

impl Default for RewardConfig {
    fn default() -> Self {
        Self {
            post_creation_reward: Money::new(rust_decimal::Decimal::new(5, 0), Currency::Dabloons),
            comment_creation_reward: Money::new(rust_decimal::Decimal::new(2, 0), Currency::Dabloons),
            upvote_reward: Money::new(rust_decimal::Decimal::new(1, 0), Currency::Dabloons),
            share_reward: Money::new(rust_decimal::Decimal::new(3, 0), Currency::Dabloons),
            follow_reward: Money::new(rust_decimal::Decimal::new(1, 0), Currency::Dabloons),
        }
    }
}
/// Service for handling social rewards
#[derive(Debug)]
pub struct RewardService {
    wallet_service: Box<dyn WalletService + Send + Sync>,
    reward_transaction_repository: Box<dyn RewardTransactionRepository + Send + Sync>,
    config: RewardConfig,
}
impl RewardService {
    /// Create a new reward service
    pub fn new(
        wallet_service: Box<dyn WalletService + Send + Sync>,
        reward_transaction_repository: Box<dyn RewardTransactionRepository + Send + Sync>,
        config: RewardConfig,
    ) -> Self {
        Self {
            wallet_service,
            reward_transaction_repository,
            config,
        }
    }
        }
    }
    
    /// Process a social event and award rewards if applicable
    pub async fn process_event(&self, event: SocialEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let reward_amount = self.calculate_reward(&event)?;
        
        if !reward_amount.is_zero() {
            let user_id = *event.user_id();
            
            // Record the transaction
            self.reward_transaction_repository
                .record_transaction(
                    user_id,
                    reward_amount.clone(),
                    self.event_type(&event),
                    self.event_description(&event)
                )
                .await?;
            
            // Add dabloons to user's wallet
            self.wallet_service
                .add_dabloons(user_id, reward_amount, Some(self.event_description(&event)))
                .await?;
        }
        
        Ok(())
    }
    
    /// Calculate the reward amount for a social event
    fn calculate_reward(&self, event: &SocialEvent) -> Result<Money, Box<dyn std::error::Error + Send + Sync>> {
        let reward = match event {
            SocialEvent::PostCreated { .. } => self.config.post_creation_reward.clone(),
            SocialEvent::CommentCreated { .. } => self.config.comment_creation_reward.clone(),
            SocialEvent::PostVoted { vote_type, .. } => {
                match vote_type {
                    crate::domain::social_event::VoteType::Upvote => self.config.upvote_reward.clone(),
                    crate::domain::social_event::VoteType::Downvote => Money::zero(Currency::Dabloons),
                }
            },
            SocialEvent::PostShared { .. } => self.config.share_reward.clone(),
            SocialEvent::UserFollowed { .. } => self.config.follow_reward.clone(),
        };
        
        Ok(reward)
    }
    
    /// Get a description for a social event
    fn event_description(&self, event: &SocialEvent) -> String {
        match event {
            SocialEvent::PostCreated { .. } => "Reward for creating a post".to_string(),
            SocialEvent::CommentCreated { .. } => "Reward for creating a comment".to_string(),
            SocialEvent::PostVoted { vote_type, .. } => {
                match vote_type {
                    crate::domain::social_event::VoteType::Upvote => "Reward for receiving an upvote".to_string(),
                    crate::domain::social_event::VoteType::Downvote => "Reward for receiving a downvote".to_string(),
                }
            },
            SocialEvent::PostShared { .. } => "Reward for sharing a post".to_string(),
            SocialEvent::UserFollowed { .. } => "Reward for following a user".to_string(),
        }
    }
    
    /// Get the event type for a social event
    fn event_type(&self, event: &SocialEvent) -> String {
        match event {
            SocialEvent::PostCreated { .. } => "post_created".to_string(),
            SocialEvent::CommentCreated { .. } => "comment_created".to_string(),
            SocialEvent::PostVoted { vote_type, .. } => {
                match vote_type {
                    crate::domain::social_event::VoteType::Upvote => "post_upvoted".to_string(),
                    crate::domain::social_event::VoteType::Downvote => "post_downvoted".to_string(),
                }
            },
            SocialEvent::PostShared { .. } => "post_shared".to_string(),
            SocialEvent::UserFollowed { .. } => "user_followed".to_string(),
        }
    }
}