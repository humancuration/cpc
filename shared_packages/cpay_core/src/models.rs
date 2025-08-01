//! Data models for the CPay system
//! 
//! This module defines the core data structures used throughout the CPay system,
//! including payment requests, responses, transactions, and error types.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::fmt;

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
    pub amount: Decimal,
    
    /// Currency of the transaction
    pub currency: Currency,
    
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
        amount: Decimal,
        currency: Currency,
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
    pub amount: Decimal,
    
    /// Currency of the transaction
    pub currency: Currency,
    
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

/// Currency enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Currency {
    /// Dabloons - Internal CPC currency
    Dabloons,
    
    /// US Dollar
    USD,
    
    /// Euro
    EUR,
    
    /// British Pound
    GBP,
    
    /// Japanese Yen
    JPY,
}

impl Currency {
    /// Get the currency code as a string
    pub fn code(&self) -> &str {
        match self {
            Currency::Dabloons => "DBL",
            Currency::USD => "USD",
            Currency::EUR => "EUR",
            Currency::GBP => "GBP",
            Currency::JPY => "JPY",
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl std::str::FromStr for Currency {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DBL" => Ok(Currency::Dabloons),
            "USD" => Ok(Currency::USD),
            "EUR" => Ok(Currency::EUR),
            "GBP" => Ok(Currency::GBP),
            "JPY" => Ok(Currency::JPY),
            _ => Err(format!("Unknown currency: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use std::str::FromStr;
    
    #[test]
    fn test_currency_code() {
        assert_eq!(Currency::Dabloons.code(), "DBL");
        assert_eq!(Currency::USD.code(), "USD");
        assert_eq!(Currency::EUR.code(), "EUR");
        assert_eq!(Currency::GBP.code(), "GBP");
        assert_eq!(Currency::JPY.code(), "JPY");
    }
    
    #[test]
    fn test_currency_display() {
        assert_eq!(format!("{}", Currency::Dabloons), "DBL");
        assert_eq!(format!("{}", Currency::USD), "USD");
        assert_eq!(format!("{}", Currency::EUR), "EUR");
        assert_eq!(format!("{}", Currency::GBP), "GBP");
        assert_eq!(format!("{}", Currency::JPY), "JPY");
    }
    
    #[test]
    fn test_currency_from_str() {
        assert_eq!(Currency::from_str("DBL").unwrap(), Currency::Dabloons);
        assert_eq!(Currency::from_str("USD").unwrap(), Currency::USD);
        assert_eq!(Currency::from_str("EUR").unwrap(), Currency::EUR);
        assert_eq!(Currency::from_str("GBP").unwrap(), Currency::GBP);
        assert_eq!(Currency::from_str("JPY").unwrap(), Currency::JPY);
        assert!(Currency::from_str("XYZ").is_err());
    }
    
    #[test]
    fn test_payment_request_creation() {
        let user_id = Uuid::new_v4();
        let recipient_id = Uuid::new_v4();
        let amount = dec!(100.50);
        
        let request = PaymentRequest::new(
            user_id,
            recipient_id,
            amount,
            Currency::USD,
            Some("Test payment".to_string()),
        );
        
        assert_eq!(request.user_id, user_id);
        assert_eq!(request.recipient_id, recipient_id);
        assert_eq!(request.amount, amount);
        assert_eq!(request.currency, Currency::USD);
        assert_eq!(request.description, Some("Test payment".to_string()));
    }
    
    #[test]
    fn test_transaction_status_display() {
        assert_eq!(format!("{}", TransactionStatus::Pending), "Pending");
        assert_eq!(format!("{}", TransactionStatus::Completed), "Completed");
        assert_eq!(format!("{}", TransactionStatus::Failed), "Failed");
        assert_eq!(format!("{}", TransactionStatus::Cancelled), "Cancelled");
    }
}

/// Payment error types
#[derive(Debug, thiserror::Error)]
pub enum PaymentError {
    /// Insufficient funds for the transaction
    #[error("Insufficient funds for currency: {0}")]
    InsufficientFunds(Currency),
    
    /// Invalid amount (e.g., negative value)
    #[error("Invalid amount")]
    InvalidAmount,
    
    /// Currency mismatch in transaction
    #[error("Currency mismatch: expected {expected}, got {actual}")]
    CurrencyMismatch { expected: String, actual: String },
    
    /// User not found
    #[error("User not found: {0}")]
    UserNotFound(Uuid),
    
    /// Database error
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    /// Wallet error
    #[error("Wallet error: {0}")]
    WalletError(#[from] wallet::domain::primitives::FinancialError),
    
    /// Notification error
    #[error("Notification error: {0}")]
    NotificationError(#[from] notification_core::NotificationError),
    
    /// General error
    #[error("Payment processing error: {0}")]
    General(String),
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
    pub total_donations: Decimal,
}

/// Skill rate structure for volunteer hour conversion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillRate {
    /// Name of the skill
    pub skill_name: String,
    
    /// Rate per hour for this skill
    pub rate_per_hour: Decimal,
    
    /// Currency for the rate
    pub currency: Currency,
}