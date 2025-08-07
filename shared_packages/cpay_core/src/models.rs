//! Data models for the CPay system
//! 
//! This module defines the core data structures used throughout the CPay system,
//! including payment requests, responses, transactions, and error types.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::fmt;
use common_utils::error::CommonError;
use cpc_financial_core::MonetaryAmount;
use cpc_financial_core::currency::CurrencyCode;

/// Payment request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRequest {
    /// Unique identifier for the payment request
    pub id: Uuid,
    
    /// User initiating the payment
    pub user_id: Uuid,
    
    /// Recipient of the payment
    pub recipient_id: Uuid,
    
    /// Amount to be transferred
    pub amount: MonetaryAmount,
    
    /// Currency of the transaction
    pub currency: CurrencyCode,
    
    /// Optional description of the payment
    pub description: Option<String>,
    
    /// Whether the transaction is public
    pub is_public: bool,
    
    /// Whether to share the transaction to social media
    pub share_to_social: bool,
    
    /// Optional cause ID for donations
    pub cause_id: Option<Uuid>,
    
    /// Optional volunteer hours associated with the transaction
    pub volunteer_hours: Option<Decimal>,
    
    /// Timestamp when the request was created
    pub created_at: DateTime<Utc>,
}

impl PaymentRequest {
    /// Create a new payment request
    pub fn new(
        user_id: Uuid,
        recipient_id: Uuid,
        amount: MonetaryAmount,
        currency: CurrencyCode,
        description: Option<String>,
        is_public: bool,
        share_to_social: bool,
        cause_id: Option<Uuid>,
        volunteer_hours: Option<Decimal>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            recipient_id,
            amount,
            currency,
            description,
            is_public,
            share_to_social,
            cause_id,
            volunteer_hours,
            created_at: Utc::now(),
        }
    }
}

/// Payment response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentResponse {
    /// Unique identifier for the transaction
    pub transaction_id: Uuid,
    
    /// Status of the transaction
    pub status: TransactionStatus,
    
    /// Timestamp when the transaction was processed
    pub timestamp: DateTime<Utc>,
}

/// Transaction status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionStatus {
    /// Transaction is pending processing
    Pending,
    
    /// Transaction has been completed successfully
    Completed,
    
    /// Transaction has failed
    Failed,
    
    /// Transaction has been cancelled
    Cancelled,
}

impl fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionStatus::Pending => write!(f, "Pending"),
            TransactionStatus::Completed => write!(f, "Completed"),
            TransactionStatus::Failed => write!(f, "Failed"),
            TransactionStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

/// Transaction structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Unique identifier for the transaction
    pub id: Uuid,
    
    /// User initiating the transaction
    pub sender_id: Uuid,
    
    /// Recipient of the transaction
    pub recipient_id: Uuid,
    
    /// Amount transferred
    pub amount: MonetaryAmount,
    
    /// Currency of the transaction
    pub currency: CurrencyCode,
    
    /// Status of the transaction
    pub status: TransactionStatus,
    
    /// Optional description of the transaction
    pub description: Option<String>,
    
    /// Optional social post ID
    pub social_post_id: Option<Uuid>,
    
    /// Optional volunteer hours associated with the transaction
    pub volunteer_hours: Option<Decimal>,
    
    /// Timestamp when the transaction was created
    pub created_at: DateTime<Utc>,
    
    /// Timestamp when the transaction was completed
    pub completed_at: Option<DateTime<Utc>>,
}


#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use std::str::FromStr;
    
    
    
    
    #[test]
    fn test_payment_request_creation() {
        let user_id = Uuid::new_v4();
        let recipient_id = Uuid::new_v4();
        let amount = MonetaryAmount::new(dec!(100.50), CurrencyCode::USD);
        
        let request = PaymentRequest::new(
            user_id,
            recipient_id,
            amount.clone(),
            CurrencyCode::USD,
            Some("Test payment".to_string()),
            false, // is_public
            false, // share_to_social
            None,  // cause_id
            None,  // volunteer_hours
        );
        
        assert_eq!(request.user_id, user_id);
        assert_eq!(request.recipient_id, recipient_id);
        assert_eq!(request.amount, amount);
        assert_eq!(request.currency, CurrencyCode::USD);
        assert_eq!(request.description, Some("Test payment".to_string()));
    }
    
    #[test]
    fn test_transaction_status_display() {
        assert_eq!(format!("{}", TransactionStatus::Pending), "Pending");
        assert_eq!(format!("{}", TransactionStatus::Completed), "Completed");
        assert_eq!(format!("{}", TransactionStatus::Failed), "Failed");
        assert_eq!(format!("{}", TransactionStatus::Cancelled), "Cancelled");
    }
    
    /// Payment error type
    ///
    /// This is a type alias to CommonError for consistency with the common_utils crate.
    pub type PaymentError = CommonError;
}

/// Cause structure for donations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cause {
    /// Unique identifier for the cause
    pub id: Uuid,
    
    /// Name of the cause
    pub name: String,
    
    /// Description of the cause
    pub description: String,
    
    /// URL to the cause's image
    pub image_url: Option<String>,
    
    /// Total donations received
    pub total_donations: MonetaryAmount,
}

/// Skill rate structure for volunteer hour conversion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillRate {
    /// Name of the skill
    pub skill_name: String,
    
    /// Rate per hour for this skill
    pub rate_per_hour: Decimal,
    
    /// Currency for the rate
    pub currency: CurrencyCode,
}