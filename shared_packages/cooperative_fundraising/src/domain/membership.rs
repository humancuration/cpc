//! Membership domain models

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// A membership share in the cooperative
/// 
/// Represents community involvement, not financial investment.
/// Strictly enforced: 1 membership share per person globally.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Membership {
    pub user_id: Uuid,
    pub campaign_id: Uuid,
    pub join_date: DateTime<Utc>,
}

impl Membership {
    /// Create a new membership
    pub fn new(user_id: Uuid, campaign_id: Uuid) -> Self {
        Self {
            user_id,
            campaign_id,
            join_date: Utc::now(),
        }
    }
    
    /// Check if this membership is active
    /// 
    /// In the current implementation, all memberships are considered active
    /// once granted. Future implementations might add expiration logic.
    pub fn is_active(&self) -> bool {
        true
    }
}