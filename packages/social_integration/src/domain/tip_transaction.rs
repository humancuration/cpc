// This file has been deprecated as tip functionality has been moved to the wallet package
// The file is kept for reference but is no longer used in the codebase
//! Tip transaction domain model for social integration

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use cpc_wallet::domain::primitives::{Money, Currency};

/// A tip transaction between users
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TipTransaction {
    /// Unique identifier for the transaction
    pub id: Uuid,
    
    /// User who sent the tip
    pub sender_id: Uuid,
    
    /// User who received the tip
    pub recipient_id: Uuid,
    
    /// Amount of the tip
    pub amount: Money,
    
    /// Type of transaction (should be "tip")
    pub transaction_type: String,
    
    /// Optional description/note for the tip
    pub description: String,
    
    /// When the transaction occurred
    pub created_at: DateTime<Utc>,
}

impl TipTransaction {
    /// Create a new tip transaction
    pub fn new(
        sender_id: Uuid,
        recipient_id: Uuid,
        amount: Money,
        description: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            sender_id,
            recipient_id,
            amount,
            transaction_type: "tip".to_string(),
            description,
            created_at: Utc::now(),
        }
    }
}