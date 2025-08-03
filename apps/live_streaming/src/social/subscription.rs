//! Subscription system implementation

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Subscription service for channel subscriptions
pub struct SubscriptionService {
    /// In-memory storage of subscriptions (in a real implementation, this would be in a database)
    subscriptions: HashMap<Uuid, Subscription>,
    
    /// In-memory storage of subscription tiers
    tiers: HashMap<Uuid, SubscriptionTier>,
}

/// Represents a channel subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    /// Unique identifier for the subscription
    pub id: Uuid,
    
    /// ID of the user who subscribed
    pub subscriber_id: Uuid,
    
    /// ID of the channel owner
    pub channel_owner_id: Uuid,
    
    /// Tier of the subscription
    pub tier_id: Uuid,
    
    /// When the subscription started
    pub subscribed_at: DateTime<Utc>,
    
    /// When the subscription renews
    pub renews_at: DateTime<Utc>,
    
    /// Whether the subscription is active
    pub is_active: bool,
    
    /// Whether the subscription is a gift
    pub is_gift: bool,
    
    /// ID of the user who gifted the subscription (if applicable)
    pub gifted_by: Option<Uuid>,
}

/// Represents a subscription tier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionTier {
    /// Unique identifier for the tier
    pub id: Uuid,
    
    /// Channel ID this tier belongs to
    pub channel_id: Uuid,
    
    /// Tier name (e.g., "Tier 1", "Tier 2", "Tier 3")
    pub name: String,
    
    /// Tier description
    pub description: String,
    
    /// Monthly price in cents
    pub price_cents: u32,
    
    /// Tier level (1, 2, 3, etc.)
    pub level: u8,
    
    /// Benefits included in this tier
    pub benefits: SubscriptionBenefits,
}

/// Benefits included in a subscription tier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionBenefits {
    /// Whether subscribers can use subscriber-only emotes
    pub subscriber_emotes: bool,
    
    /// Whether subscribers get ad-free viewing
    pub ad_free: bool,
    
    /// Whether subscribers can stream in higher quality
    pub higher_quality: bool,
    
    /// Whether subscribers can use custom badges
    pub custom_badges: bool,
    
    /// Whether subscribers can participate in subscriber-only chat
    pub subscriber_chat: bool,
    
    /// Whether subscribers get a special badge
    pub special_badge: bool,
    
    /// Custom benefits specific to this channel
    pub custom_benefits: Vec<String>,
}

impl SubscriptionService {
    /// Create a new subscription service
    pub fn new() -> Self {
        Self {
            subscriptions: HashMap::new(),
            tiers: HashMap::new(),
        }
    }
    
    /// Create a new subscription tier for a channel
    pub fn create_tier(
        &mut self,
        channel_id: Uuid,
        name: String,
        description: String,
        price_cents: u32,
        level: u8,
        benefits: SubscriptionBenefits,
    ) -> SubscriptionTier {
        let tier = SubscriptionTier {
            id: Uuid::new_v4(),
            channel_id,
            name,
            description,
            price_cents,
            level,
            benefits,
        };
        
        self.tiers.insert(tier.id, tier.clone());
        tier
    }
    
    /// Subscribe a user to a channel
    pub fn subscribe_user(
        &mut self,
        subscriber_id: Uuid,
        channel_owner_id: Uuid,
        tier_id: Uuid,
        is_gift: bool,
        gifted_by: Option<Uuid>,
    ) -> Result<Subscription, SubscriptionError> {
        // Check if the tier exists
        if !self.tiers.contains_key(&tier_id) {
            return Err(SubscriptionError::TierNotFound);
        }
        
        // Check if user is already subscribed to this channel
        for subscription in self.subscriptions.values() {
            if subscription.subscriber_id == subscriber_id 
                && subscription.channel_owner_id == channel_owner_id 
                && subscription.is_active {
                return Err(SubscriptionError::AlreadySubscribed);
            }
        }
        
        let now = Utc::now();
        let subscription = Subscription {
            id: Uuid::new_v4(),
            subscriber_id,
            channel_owner_id,
            tier_id,
            subscribed_at: now,
            renews_at: now + chrono::Duration::days(30), // Monthly subscription
            is_active: true,
            is_gift,
            gifted_by,
        };
        
        self.subscriptions.insert(subscription.id, subscription.clone());
        Ok(subscription)
    }
    
    /// Cancel a subscription
    pub fn cancel_subscription(&mut self, subscription_id: Uuid) -> Result<Subscription, SubscriptionError> {
        if let Some(subscription) = self.subscriptions.get_mut(&subscription_id) {
            subscription.is_active = false;
            Ok(subscription.clone())
        } else {
            Err(SubscriptionError::SubscriptionNotFound)
        }
    }
    
    /// Get all subscriptions for a user
    pub fn get_user_subscriptions(&self, subscriber_id: Uuid) -> Vec<Subscription> {
        self.subscriptions
            .values()
            .filter(|s| s.subscriber_id == subscriber_id && s.is_active)
            .cloned()
            .collect()
    }
    
    /// Get all subscribers for a channel
    pub fn get_channel_subscribers(&self, channel_owner_id: Uuid) -> Vec<Subscription> {
        self.subscriptions
            .values()
            .filter(|s| s.channel_owner_id == channel_owner_id && s.is_active)
            .cloned()
            .collect()
    }
    
    /// Get subscription by ID
    pub fn get_subscription(&self, subscription_id: Uuid) -> Option<Subscription> {
        self.subscriptions.get(&subscription_id).cloned()
    }
    
    /// Get all tiers for a channel
    pub fn get_channel_tiers(&self, channel_id: Uuid) -> Vec<SubscriptionTier> {
        self.tiers
            .values()
            .filter(|t| t.channel_id == channel_id)
            .cloned()
            .collect()
    }
    
    /// Get a specific tier
    pub fn get_tier(&self, tier_id: Uuid) -> Option<SubscriptionTier> {
        self.tiers.get(&tier_id).cloned()
    }
    
    /// Update subscription benefits for a tier
    pub fn update_tier_benefits(&mut self, tier_id: Uuid, benefits: SubscriptionBenefits) -> Result<SubscriptionTier, SubscriptionError> {
        if let Some(tier) = self.tiers.get_mut(&tier_id) {
            tier.benefits = benefits;
            Ok(tier.clone())
        } else {
            Err(SubscriptionError::TierNotFound)
        }
    }
}

/// Errors that can occur with subscriptions
#[derive(Debug, Clone)]
pub enum SubscriptionError {
    /// Subscription not found
    SubscriptionNotFound,
    
    /// Tier not found
    TierNotFound,
    
    /// User is already subscribed
    AlreadySubscribed,
    
    /// Payment processing error
    PaymentError(String),
}

impl std::fmt::Display for SubscriptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubscriptionError::SubscriptionNotFound => write!(f, "Subscription not found"),
            SubscriptionError::TierNotFound => write!(f, "Tier not found"),
            SubscriptionError::AlreadySubscribed => write!(f, "User is already subscribed"),
            SubscriptionError::PaymentError(msg) => write!(f, "Payment error: {}", msg),
        }
    }
}

impl std::error::Error for SubscriptionError {}

impl Default for SubscriptionService {
    fn default() -> Self {
        Self::new()
    }
}