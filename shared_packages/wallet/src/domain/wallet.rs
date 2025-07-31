//! Wallet domain model for the CPC platform
//!
//! This module provides functionality for managing user wallets with dabloons,
//! the internal currency of the CPC platform.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use super::primitives::{Money, Currency, FinancialError};

/// A user's wallet for storing dabloons
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Wallet {
    /// Unique identifier for the wallet
    pub id: Uuid,
    
    /// User who owns this wallet
    pub user_id: Uuid,
    
    /// Current balance in dabloons
    pub balance: Money,
    
    /// When the wallet was created
    pub created_at: DateTime<Utc>,
    
    /// When the wallet was last updated
    pub updated_at: DateTime<Utc>,
}

impl Wallet {
    /// Create a new wallet for a user
    pub fn new(user_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            balance: Money::zero(Currency::Dabloons),
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Create a wallet with a specific balance
    pub fn with_balance(user_id: Uuid, balance: Money) -> Result<Self, FinancialError> {
        if balance.currency != Currency::Dabloons {
            return Err(FinancialError::CurrencyMismatch {
                expected: Currency::Dabloons.code().to_string(),
                actual: balance.currency.code().to_string(),
            });
        }
        
        let now = Utc::now();
        Ok(Self {
            id: Uuid::new_v4(),
            user_id,
            balance,
            created_at: now,
            updated_at: now,
        })
    }

    /// Add dabloons to the wallet
    pub fn add_dabloons(&mut self, amount: Money) -> Result<(), FinancialError> {
        if amount.currency != Currency::Dabloons {
            return Err(FinancialError::CurrencyMismatch {
                expected: Currency::Dabloons.code().to_string(),
                actual: amount.currency.code().to_string(),
            });
        }
        
        self.balance = self.balance.add(&amount)?;
        self.updated_at = Utc::now();
        Ok(())
    }
    
    /// Subtract dabloons from the wallet
    pub fn subtract_dabloons(&mut self, amount: Money) -> Result<(), FinancialError> {
        if amount.currency != Currency::Dabloons {
            return Err(FinancialError::CurrencyMismatch {
                expected: Currency::Dabloons.code().to_string(),
                actual: amount.currency.code().to_string(),
            });
        }
        
        if self.balance.amount < amount.amount {
            return Err(FinancialError::InsufficientFunds(Currency::Dabloons));
        }
        
        self.balance = self.balance.subtract(&amount)?;
        self.updated_at = Utc::now();
        Ok(())
    }
    
    /// Check if the wallet has sufficient balance
    pub fn has_sufficient_balance(&self, amount: &Money) -> Result<bool, FinancialError> {
        if amount.currency != Currency::Dabloons {
            return Err(FinancialError::CurrencyMismatch {
                expected: Currency::Dabloons.code().to_string(),
                actual: amount.currency.code().to_string(),
            });
        }
        
        Ok(self.balance.amount >= amount.amount)
    }
    
    /// Add traditional currency to the wallet
    pub fn add_traditional_currency(&mut self, amount: Money) -> Result<(), FinancialError> {
        if amount.currency == Currency::Dabloons {
            return Err(FinancialError::CurrencyMismatch {
                expected: "Traditional Currency".to_string(),
                actual: amount.currency.code().to_string(),
            });
        }
        
        // For traditional currency, we don't track the balance in the wallet
        // This is just a placeholder implementation
        self.updated_at = Utc::now();
        Ok(())
    }
    
    /// Subtract traditional currency from the wallet
    pub fn subtract_traditional_currency(&mut self, amount: Money) -> Result<(), FinancialError> {
        if amount.currency == Currency::Dabloons {
            return Err(FinancialError::CurrencyMismatch {
                expected: "Traditional Currency".to_string(),
                actual: amount.currency.code().to_string(),
            });
        }
        
        // For traditional currency, we don't track the balance in the wallet
        // This is just a placeholder implementation
        self.updated_at = Utc::now();
        Ok(())
    }
}

/// Transaction type for wallet operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionType {
    /// Adding dabloons to wallet (e.g., purchase, reward)
    Credit,
    
    /// Removing dabloons from wallet (e.g., purchase, transfer)
    Debit,
}

/// A transaction record for wallet operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WalletTransaction {
    /// Unique identifier for the transaction
    pub id: Uuid,
    
    /// Wallet this transaction belongs to
    pub wallet_id: Uuid,
    
    /// Type of transaction
    pub transaction_type: TransactionType,
    
    /// Amount of dabloons involved
    pub amount: Money,
    
    /// Optional description of the transaction
    pub description: Option<String>,
    
    /// When the transaction occurred
    pub timestamp: DateTime<Utc>,
}

impl WalletTransaction {
    /// Create a new wallet transaction
    pub fn new(
        wallet_id: Uuid,
        transaction_type: TransactionType,
        amount: Money,
        description: Option<String>,
    ) -> Result<Self, FinancialError> {
        if amount.currency != Currency::Dabloons {
            return Err(FinancialError::CurrencyMismatch {
                expected: Currency::Dabloons.code().to_string(),
                actual: amount.currency.code().to_string(),
            });
        }
        
        Ok(Self {
            id: Uuid::new_v4(),
            wallet_id,
            transaction_type,
            amount,
            description,
            timestamp: Utc::now(),
        })
    }
}

/// Event emitted when a tip is sent
#[derive(Debug, Clone)]
pub struct TipSentEvent {
    pub transaction_id: Uuid,
    pub sender_id: Uuid,
    pub recipient_id: Uuid,
    pub amount: Money,
    pub note: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl Wallet {
    /// Send dabloons as a tip to another user
    pub fn send_tip(
        &mut self,
        recipient_id: Uuid,
        amount: Money,
        note: Option<String>
    ) -> Result<TipSentEvent, FinancialError> {
        // Validate currency
        if amount.currency != Currency::Dabloons {
            return Err(FinancialError::CurrencyMismatch {
                expected: Currency::Dabloons.code().to_string(),
                actual: amount.currency.code().to_string(),
            });
        }
        
        // Validate amount is positive
        if !amount.is_positive() {
            return Err(FinancialError::InvalidAmount);
        }
        
        // Check if the wallet has sufficient balance
        if !self.has_sufficient_balance(&amount)? {
            return Err(FinancialError::InsufficientFunds(Currency::Dabloons));
        }
        
        // Subtract the amount from the wallet
        self.subtract_dabloons(amount.clone())?;
        
        // Create the tip event
        let event = TipSentEvent {
            transaction_id: Uuid::new_v4(),
            sender_id: self.user_id,
            recipient_id,
            amount,
            note,
            timestamp: Utc::now(),
        };
        
        Ok(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_wallet_creation() {
        let user_id = Uuid::new_v4();
        let wallet = Wallet::new(user_id);
        
        assert_eq!(wallet.user_id, user_id);
        assert_eq!(wallet.balance.currency, Currency::Dabloons);
        assert_eq!(wallet.balance.amount, Decimal::ZERO);
    }
    
    #[test]
    fn test_wallet_with_balance() {
        let user_id = Uuid::new_v4();
        let balance = Money::new(dec!(100), Currency::Dabloons);
        let wallet = Wallet::with_balance(user_id, balance.clone()).unwrap();
        
        assert_eq!(wallet.user_id, user_id);
        assert_eq!(wallet.balance, balance);
    }
    
    #[test]
    fn test_wallet_with_balance_wrong_currency() {
        let user_id = Uuid::new_v4();
        let balance = Money::new(dec!(100), Currency::USD);
        let result = Wallet::with_balance(user_id, balance);
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_add_dabloons() {
        let user_id = Uuid::new_v4();
        let mut wallet = Wallet::new(user_id);
        let amount = Money::new(dec!(50), Currency::Dabloons);
        
        assert!(wallet.add_dabloons(amount.clone()).is_ok());
        assert_eq!(wallet.balance.amount, dec!(50));
    }
    
    #[test]
    fn test_add_dabloons_wrong_currency() {
        let user_id = Uuid::new_v4();
        let mut wallet = Wallet::new(user_id);
        let amount = Money::new(dec!(50), Currency::USD);
        
        let result = wallet.add_dabloons(amount);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_subtract_dabloons() {
        let user_id = Uuid::new_v4();
        let balance = Money::new(dec!(100), Currency::Dabloons);
        let mut wallet = Wallet::with_balance(user_id, balance).unwrap();
        let amount = Money::new(dec!(30), Currency::Dabloons);
        
        assert!(wallet.subtract_dabloons(amount.clone()).is_ok());
        assert_eq!(wallet.balance.amount, dec!(70));
    }
    
    #[test]
    fn test_subtract_dabloons_insufficient_balance() {
        let user_id = Uuid::new_v4();
        let mut wallet = Wallet::new(user_id);
        let amount = Money::new(dec!(50), Currency::Dabloons);
        
        let result = wallet.subtract_dabloons(amount);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_subtract_dabloons_wrong_currency() {
        let user_id = Uuid::new_v4();
        let mut wallet = Wallet::new(user_id);
        let amount = Money::new(dec!(50), Currency::USD);
        
        let result = wallet.subtract_dabloons(amount);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_has_sufficient_balance() {
        let user_id = Uuid::new_v4();
        let balance = Money::new(dec!(100), Currency::Dabloons);
        let wallet = Wallet::with_balance(user_id, balance).unwrap();
        let amount = Money::new(dec!(50), Currency::Dabloons);
        
        assert!(wallet.has_sufficient_balance(&amount).unwrap());
    }
    
    #[test]
    fn test_has_insufficient_balance() {
        let user_id = Uuid::new_v4();
        let balance = Money::new(dec!(30), Currency::Dabloons);
        let wallet = Wallet::with_balance(user_id, balance).unwrap();
        let amount = Money::new(dec!(50), Currency::Dabloons);
        
        assert!(!wallet.has_sufficient_balance(&amount).unwrap());
    }
    
    #[test]
    fn test_transaction_creation() {
        let wallet_id = Uuid::new_v4();
        let amount = Money::new(dec!(100), Currency::Dabloons);
        let transaction = WalletTransaction::new(
            wallet_id,
            TransactionType::Credit,
            amount,
            Some("Reward for completing a task".to_string())
        ).unwrap();
        
        assert_eq!(transaction.wallet_id, wallet_id);
        assert_eq!(transaction.transaction_type, TransactionType::Credit);
        assert_eq!(transaction.amount.currency, Currency::Dabloons);
        assert_eq!(transaction.amount.amount, dec!(100));
        assert_eq!(transaction.description, Some("Reward for completing a task".to_string()));
    }
    
    #[test]
    fn test_transaction_creation_wrong_currency() {
        let wallet_id = Uuid::new_v4();
        let amount = Money::new(dec!(100), Currency::USD);
        let result = WalletTransaction::new(
            wallet_id,
            TransactionType::Credit,
            amount,
            None
        );
        
        assert!(result.is_err());
    }
}