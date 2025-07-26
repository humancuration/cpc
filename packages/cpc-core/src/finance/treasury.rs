//! Treasury module for managing platform funds and profit distributions
//!
//! Provides core treasury functionality including revenue recording,
//! profit distribution, and financial tracking. Follows hexagonal
//! architecture with storage-agnostic design.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

use super::transactions::{Transaction, TransactionLedger, TransactionType};

/// Core treasury entity managing platform funds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Treasury {
    /// Current treasury balance
    pub balance: Decimal,
    /// Currency code (e.g., "USD")
    pub currency: String,
    /// Transaction history for this treasury
    pub transaction_history: Vec<Transaction>,
    /// Minimum amount for profit distributions
    pub min_payout: Decimal,
}

impl Treasury {
    /// Creates a new treasury with default values
    pub fn new() -> Self {
        Self {
            balance: Decimal::ZERO,
            currency: "USD".to_string(),
            transaction_history: Vec::new(),
            min_payout: dec!(0.01), // Default minimum payout of $0.01
        }
    }

    /// Creates a new treasury with custom parameters
    pub fn with_currency_and_min_payout(currency: impl Into<String>, min_payout: Decimal) -> Self {
        Self {
            balance: Decimal::ZERO,
            currency: currency.into(),
            transaction_history: Vec::new(),
            min_payout,
        }
    }

    /// Adds a transaction to the treasury history
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transaction_history.push(transaction);
    }

    /// Updates the treasury balance based on transaction type
    pub fn update_balance(&mut self, amount: Decimal, transaction_type: TransactionType) {
        match transaction_type {
            TransactionType::AdRevenue
            | TransactionType::InAppPurchase
            | TransactionType::Subscription
            | TransactionType::Revenue => {
                // Revenue increases treasury balance
                self.balance += amount;
            }
            TransactionType::ProfitDistribution | TransactionType::UniversalIncome => {
                // Distributions decrease treasury balance
                self.balance -= amount;
            }
            TransactionType::Tax | TransactionType::System => {
                // Tax and system transactions may increase or decrease based on context
                // For now, we'll treat system as revenue for treasury
                self.balance += amount;
            }
        }
    }

    /// Gets total revenue (all incoming transactions)
    pub fn get_total_revenue(&self) -> Decimal {
        self.transaction_history
            .iter()
            .filter(|tx| {
                matches!(
                    tx.transaction_type,
                    TransactionType::AdRevenue
                        | TransactionType::InAppPurchase
                        | TransactionType::Subscription
                        | TransactionType::Revenue
                )
            })
            .map(|tx| tx.amount)
            .sum()
    }

    /// Gets total distributions made from treasury
    pub fn get_total_distributions(&self) -> Decimal {
        self.transaction_history
            .iter()
            .filter(|tx| tx.transaction_type == TransactionType::ProfitDistribution)
            .map(|tx| tx.amount)
            .sum()
    }
}

/// Profit distribution record for tracking user payouts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProfitDistribution {
    /// Unique distribution identifier
    pub id: Uuid,
    /// Total amount distributed
    pub amount: Decimal,
    /// When the distribution occurred
    pub distributed_at: DateTime<Utc>,
    /// Mapping of user IDs to their transaction IDs
    pub transaction_ids: HashMap<Uuid, String>,
}

impl ProfitDistribution {
    /// Creates a new profit distribution
    pub fn new(amount: Decimal) -> Self {
        Self {
            id: Uuid::new_v4(),
            amount,
            distributed_at: Utc::now(),
            transaction_ids: HashMap::new(),
        }
    }

    /// Adds a user transaction to the distribution
    pub fn add_user_transaction(&mut self, user_id: Uuid, transaction_id: String) {
        self.transaction_ids.insert(user_id, transaction_id);
    }
}

/// Comprehensive treasury error handling
#[derive(Debug, thiserror::Error)]
pub enum TreasuryError {
    /// Insufficient funds in treasury
    #[error("Insufficient funds in treasury: required {required}, available {available}")]
    InsufficientFunds {
        required: Decimal,
        available: Decimal,
    },
    
    /// Invalid transaction amount
    #[error("Invalid transaction amount: must be positive")]
    InvalidAmount,
    
    /// Currency mismatch
    #[error("Currency mismatch: expected {expected}, got {actual}")]
    CurrencyMismatch {
        expected: String,
        actual: String,
    },
    
    /// Transaction storage error
    #[error("Transaction storage error: {0}")]
    StorageError(String),
    
    /// Distribution amount below minimum
    #[error("Distribution amount {amount} below minimum {min_payout}")]
    BelowMinimumPayout {
        amount: Decimal,
        min_payout: Decimal,
    },
    
    /// No eligible users for distribution
    #[error("No eligible users for profit distribution")]
    NoEligibleUsers,
}

/// Treasury service for managing platform financial operations
pub struct TreasuryService<L: TransactionLedger> {
    treasury: Arc<RwLock<Treasury>>,
    ledger: Arc<RwLock<L>>,
}

impl<L: TransactionLedger + Send + Sync + 'static> TreasuryService<L> {
    /// Creates a new treasury service
    pub fn new(ledger: L) -> Self {
        Self {
            treasury: Arc::new(RwLock::new(Treasury::new())),
            ledger: Arc::new(RwLock::new(ledger)),
        }
    }

    /// Creates a new treasury service with custom parameters
    pub fn with_treasury(treasury: Treasury, ledger: L) -> Self {
        Self {
            treasury: Arc::new(RwLock::new(treasury)),
            ledger: Arc::new(RwLock::new(ledger)),
        }
    }

    /// Records revenue in the treasury
    pub fn record_revenue(&self, amount: Decimal, currency: &str) -> Result<(), TreasuryError> {
        if amount <= Decimal::ZERO {
            return Err(TreasuryError::InvalidAmount);
        }

        let mut treasury = self.treasury.write().map_err(|_| {
            TreasuryError::StorageError("Treasury lock poisoned".to_string())
        })?;

        if currency != treasury.currency {
            return Err(TreasuryError::CurrencyMismatch {
                expected: treasury.currency.clone(),
                actual: currency.to_string(),
            });
        }

        // Create revenue transaction
        let transaction = Transaction::new(
            None, // System transaction
            amount,
            currency,
            TransactionType::Revenue,
            "external".to_string(), // Source is external revenue
            "treasury".to_string(), // Destination is treasury
        );

        // Record transaction in ledger
        let mut ledger = self.ledger.write().map_err(|_| {
            TreasuryError::StorageError("Ledger lock poisoned".to_string())
        })?;

        ledger.record_transaction(transaction.clone())
            .map_err(|e| TreasuryError::StorageError(e.to_string()))?;

        // Update treasury state
        treasury.add_transaction(transaction.clone());
        treasury.update_balance(amount, transaction.transaction_type);

        Ok(())
    }

    /// Distributes profits to users
    pub fn distribute_profits(
        &self,
        distribution: ProfitDistribution,
    ) -> Result<(), TreasuryError> {
        let mut treasury = self.treasury.write().map_err(|_| {
            TreasuryError::StorageError("Treasury lock poisoned".to_string())
        })?;

        if distribution.amount <= Decimal::ZERO {
            return Err(TreasuryError::InvalidAmount);
        }

        if distribution.amount < treasury.min_payout {
            return Err(TreasuryError::BelowMinimumPayout {
                amount: distribution.amount,
                min_payout: treasury.min_payout,
            });
        }

        if distribution.amount > treasury.balance {
            return Err(TreasuryError::InsufficientFunds {
                required: distribution.amount,
                available: treasury.balance,
            });
        }

        if distribution.transaction_ids.is_empty() {
            return Err(TreasuryError::NoEligibleUsers);
        }

        let mut ledger = self.ledger.write().map_err(|_| {
            TreasuryError::StorageError("Ledger lock poisoned".to_string())
        })?;

        // Create individual user transactions
        let amount_per_user = distribution.amount / Decimal::from(distribution.transaction_ids.len());
        
        for (user_id, _transaction_id) in &distribution.transaction_ids {
            let user_transaction = Transaction::new(
                Some(*user_id),
                amount_per_user,
                &treasury.currency,
                TransactionType::ProfitDistribution,
                "treasury".to_string(),
                format!("user_{}", user_id), // User's wallet ID
            );

            ledger.record_transaction(user_transaction)
                .map_err(|e| TreasuryError::StorageError(e.to_string()))?;
            
            treasury.add_transaction(user_transaction);
            treasury.update_balance(amount_per_user, TransactionType::ProfitDistribution);
        }

        Ok(())
    }

    /// Gets the current treasury balance
    pub fn get_treasury_balance(&self) -> Decimal {
        self.treasury
            .read()
            .map(|treasury| treasury.balance)
            .unwrap_or(Decimal::ZERO)
    }

    /// Gets total revenue recorded in the treasury
    pub fn get_total_revenue(&self) -> Decimal {
        self.treasury
            .read()
            .map(|treasury| treasury.get_total_revenue())
            .unwrap_or(Decimal::ZERO)
    }

    /// Gets profit distribution history
    pub fn get_profit_distribution_history(&self) -> Vec<ProfitDistribution> {
        // This would typically query the ledger for profit distributions
        // For now, we'll return an empty vec - implement based on storage needs
        Vec::new()
    }

    /// Gets the treasury with current state
    pub fn get_treasury(&self) -> Treasury {
        self.treasury
            .read()
            .map(|treasury| treasury.clone())
            .unwrap_or_else(|_| Treasury::new())
    }

    /// Updates the minimum payout amount
    pub fn set_min_payout(&self, min_payout: Decimal) -> Result<(), TreasuryError> {
        if min_payout <= Decimal::ZERO {
            return Err(TreasuryError::InvalidAmount);
        }

        let mut treasury = self.treasury.write().map_err(|_| {
            TreasuryError::StorageError("Treasury lock poisoned".to_string())
        })?;
        
        treasury.min_payout = min_payout;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finance::transactions::InMemoryTransactionLedger;
    use rust_decimal_macros::dec;

    #[test]
    fn test_treasury_creation() {
        let treasury = Treasury::new();
        assert_eq!(treasury.balance, Decimal::ZERO);
        assert_eq!(treasury.currency, "USD");
        assert_eq!(treasury.min_payout, dec!(0.01));
    }

    #[test]
    fn test_record_revenue() {
        let ledger = InMemoryTransactionLedger::new();
        let service = TreasuryService::new(ledger);

        let result = service.record_revenue(dec!(100.0), "USD");
        assert!(result.is_ok());
        assert_eq!(service.get_treasury_balance(), dec!(100.0));
        assert_eq!(service.get_total_revenue(), dec!(100.0));
    }

    #[test]
    fn test_record_revenue_invalid_currency() {
        let ledger = InMemoryTransactionLedger::new();
        let service = TreasuryService::new(ledger);

        let result = service.record_revenue(dec!(100.0), "EUR");
        assert!(matches!(result, Err(TreasuryError::CurrencyMismatch { .. })));
    }

    #[test]
    fn test_distribute_profits() {
        let ledger = InMemoryTransactionLedger::new();
        let service = TreasuryService::new(ledger);

        // Record revenue first
        service.record_revenue(dec!(1000.0), "USD").unwrap();

        // Create distribution
        let mut distribution = ProfitDistribution::new(dec!(500.0));
        distribution.add_user_transaction(Uuid::new_v4(), "tx1".to_string());
        distribution.add_user_transaction(Uuid::new_v4(), "tx2".to_string());

        let result = service.distribute_profits(distribution);
        assert!(result.is_ok());
        assert_eq!(service.get_treasury_balance(), dec!(500.0));
    }

    #[test]
    fn test_distribute_profits_insufficient_funds() {
        let ledger = InMemoryTransactionLedger::new();
        let service = TreasuryService::new(ledger);

        let mut distribution = ProfitDistribution::new(dec!(1000.0));
        distribution.add_user_transaction(Uuid::new_v4(), "tx1".to_string());

        let result = service.distribute_profits(distribution);
        assert!(matches!(result, Err(TreasuryError::InsufficientFunds { .. })));
    }

    #[test]
    fn test_distribute_profits_below_minimum() {
        let ledger = InMemoryTransactionLedger::new();
        let service = TreasuryService::new(ledger);

        service.set_min_payout(dec!(10.0)).unwrap();

        let mut distribution = ProfitDistribution::new(dec!(5.0));
        distribution.add_user_transaction(Uuid::new_v4(), "tx1".to_string());

        let result = service.distribute_profits(distribution);
        assert!(matches!(result, Err(TreasuryError::BelowMinimumPayout { .. })));
    }
}