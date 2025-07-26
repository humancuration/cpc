//! Transaction management module for Constellation Personal Cooperative
//! 
//! Provides core transaction functionality including recording, validation,
//! and querying financial transactions. Follows hexagonal architecture
//! with storage-agnostic design.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

use super::wallets::WalletId;

/// Transaction type enumeration based on business requirements
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
    /// Revenue from advertisements
    AdRevenue,
    /// In-app purchases
    InAppPurchase,
    /// Subscription payments
    Subscription,
    /// Universal basic income distribution
    UniversalIncome,
    /// Profit distribution to users
    ProfitDistribution,
    /// Tax collection
    Tax,
    /// Revenue from content sales
    Revenue,
    /// System transactions
    System,
}

impl TransactionType {
    /// Returns true if this transaction type affects treasury balance
    pub fn affects_treasury(&self) -> bool {
        match self {
            TransactionType::AdRevenue | 
            TransactionType::InAppPurchase | 
            TransactionType::Subscription |
            TransactionType::Revenue => true,
            TransactionType::UniversalIncome |
            TransactionType::ProfitDistribution |
            TransactionType::Tax |
            TransactionType::System => false,
        }
    }
}

/// Core transaction entity representing a financial movement
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    /// Unique transaction identifier
    pub id: Uuid,
    /// User ID (None for system transactions)
    pub user_id: Option<Uuid>,
    /// Transaction amount
    pub amount: Decimal,
    /// Currency code (e.g., "USD", "EUR")
    pub currency: String,
    /// Type of transaction
    pub transaction_type: TransactionType,
    /// Transaction timestamp
    pub timestamp: DateTime<Utc>,
    /// Optional note/description
    pub note: Option<String>,
    /// Source wallet ID
    pub source_wallet: WalletId,
    /// Destination wallet ID
    pub destination_wallet: WalletId,
    /// Transaction status
    pub status: TransactionStatus,
}

impl Transaction {
    /// Creates a new transaction with default values
    pub fn new(
        user_id: Option<Uuid>,
        amount: Decimal,
        currency: impl Into<String>,
        transaction_type: TransactionType,
        source_wallet: WalletId,
        destination_wallet: WalletId,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            amount,
            currency: currency.into(),
            transaction_type,
            timestamp: Utc::now(),
            note: None,
            source_wallet,
            destination_wallet,
            status: TransactionStatus::Pending,
        }
    }

    /// Sets the transaction note
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }

    /// Sets the transaction timestamp
    pub fn with_timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = timestamp;
        self
    }

    /// Sets the transaction status
    pub fn with_status(mut self, status: TransactionStatus) -> Self {
        self.status = status;
        self
    }
}

/// Transaction lifecycle status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionStatus {
    /// Transaction is pending validation
    Pending,
    /// Transaction has been successfully processed
    Completed,
    /// Transaction processing failed
    Failed,
    /// Transaction has been reversed
    Reversed,
}

/// Comprehensive transaction error handling
#[derive(Debug, thiserror::Error)]
pub enum TransactionError {
    /// Invalid transaction amount (zero or negative)
    #[error("Invalid transaction amount: must be positive")]
    InvalidAmount,
    /// Source and destination wallets are the same
    #[error("Source and destination wallets cannot be the same")]
    SameSourceAndDestination,
    /// Insufficient funds in source wallet
    #[error("Insufficient funds in source wallet")]
    InsufficientFunds,
    /// Specified wallet not found
    #[error("Wallet not found: {0}")]
    WalletNotFound(WalletId),
    /// Invalid cryptographic signature
    #[error("Invalid transaction signature")]
    InvalidSignature,
    /// Storage-related error
    #[error("Storage error: {0}")]
    StorageError(String),
    /// Transaction validation error
    #[error("Validation error: {0}")]
    ValidationError(String),
    /// Duplicate transaction ID
    #[error("Duplicate transaction ID: {0}")]
    DuplicateTransaction(Uuid),
}

/// Transaction ledger trait defining storage-agnostic interface
///
/// This trait follows hexagonal architecture principles by defining
/// core business logic without coupling to specific storage implementations.
pub trait TransactionLedger {
    /// Records a new transaction in the ledger
    fn record_transaction(&mut self, transaction: Transaction) -> Result<(), TransactionError>;
    
    /// Retrieves transaction history for a specific user
    fn get_transaction_history(&self, user_id: Uuid) -> Vec<Transaction>;
    
    /// Retrieves transaction history for a specific wallet
    fn get_wallet_transactions(&self, wallet_id: &WalletId) -> Vec<Transaction>;
    
    /// Calculates current balance for a wallet
    fn get_balance(&self, wallet_id: &WalletId) -> Decimal;
    
    /// Retrieves a specific transaction by ID
    fn get_transaction(&self, id: Uuid) -> Option<Transaction>;
}

/// Transaction validation rules
#[derive(Debug, Clone)]
pub struct TransactionValidator;

impl TransactionValidator {
    /// Validates a transaction before processing
    pub fn validate(tx: &Transaction) -> Result<(), TransactionError> {
        // Validate amount is positive
        if tx.amount <= Decimal::ZERO {
            return Err(TransactionError::InvalidAmount);
        }
        
        // Validate source and destination are different
        if tx.source_wallet == tx.destination_wallet {
            return Err(TransactionError::SameSourceAndDestination);
        }
        
        // Validate currency format
        if tx.currency.is_empty() || tx.currency.len() != 3 {
            return Err(TransactionError::ValidationError(
                "Currency must be a 3-letter code".to_string()
            ));
        }
        
        Ok(())
    }

    /// Validates transaction signature (placeholder for future implementation)
    pub fn validate_signature(
        &self, 
        _tx: &Transaction, 
        _signature: &[u8]
    ) -> Result<(), TransactionError> {
        // TODO: Implement cryptographic signature validation
        Ok(())
    }
}

/// In-memory transaction ledger implementation
///
/// This is a temporary implementation for development/testing.
/// Will be replaced with p2panda-based storage in production.
#[derive(Debug, Default)]
pub struct InMemoryTransactionLedger {
    transactions: Vec<Transaction>,
    wallet_balances: HashMap<WalletId, Decimal>,
    user_transactions: HashMap<Uuid, Vec<Uuid>>,
}

impl InMemoryTransactionLedger {
    /// Creates a new empty ledger
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
            wallet_balances: HashMap::new(),
            user_transactions: HashMap::new(),
        }
    }

    /// Clears all transactions (for testing)
    pub fn clear(&mut self) {
        self.transactions.clear();
        self.wallet_balances.clear();
        self.user_transactions.clear();
    }

    /// Updates wallet balances based on transaction
    fn update_balances(&mut self, tx: &Transaction) -> Result<(), TransactionError> {
        // Update source wallet balance
        let source_balance = self.wallet_balances
            .get(&tx.source_wallet)
            .copied()
            .unwrap_or(Decimal::ZERO);
        
        let new_source_balance = source_balance - tx.amount;
        if new_source_balance < Decimal::ZERO && tx.source_wallet != "treasury" {
            return Err(TransactionError::InsufficientFunds);
        }
        
        self.wallet_balances.insert(tx.source_wallet.clone(), new_source_balance);

        // Update destination wallet balance
        let dest_balance = self.wallet_balances
            .get(&tx.destination_wallet)
            .copied()
            .unwrap_or(Decimal::ZERO);
        
        let new_dest_balance = dest_balance + tx.amount;
        self.wallet_balances.insert(tx.destination_wallet.clone(), new_dest_balance);

        Ok(())
    }
}

impl TransactionLedger for InMemoryTransactionLedger {
    fn record_transaction(&mut self, transaction: Transaction) -> Result<(), TransactionError> {
        // Validate transaction
        TransactionValidator::validate(&transaction)?;

        // Check for duplicate ID
        if self.transactions.iter().any(|tx| tx.id == transaction.id) {
            return Err(TransactionError::DuplicateTransaction(transaction.id));
        }

        // Update balances
        self.update_balances(&transaction)?;

        // Store transaction
        let tx_id = transaction.id;
        let user_id = transaction.user_id;
        
        self.transactions.push(transaction);

        // Index by user if user_id is provided
        if let Some(user_id) = user_id {
            self.user_transactions
                .entry(user_id)
                .or_default()
                .push(tx_id);
        }

        Ok(())
    }

    fn get_transaction_history(&self, user_id: Uuid) -> Vec<Transaction> {
        self.user_transactions
            .get(&user_id)
            .map(|tx_ids| {
                tx_ids.iter()
                    .filter_map(|&id| self.get_transaction(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    fn get_wallet_transactions(&self, wallet_id: &WalletId) -> Vec<Transaction> {
        self.transactions
            .iter()
            .filter(|tx| &tx.source_wallet == wallet_id || &tx.destination_wallet == wallet_id)
            .cloned()
            .collect()
    }

    fn get_balance(&self, wallet_id: &WalletId) -> Decimal {
        self.wallet_balances
            .get(wallet_id)
            .copied()
            .unwrap_or(Decimal::ZERO)
    }

    fn get_transaction(&self, id: Uuid) -> Option<Transaction> {
        self.transactions
            .iter()
            .find(|tx| tx.id == id)
            .cloned()
    }
}

/// Thread-safe wrapper for transaction ledger
#[derive(Debug, Clone)]
pub struct TransactionService {
    ledger: Arc<RwLock<dyn TransactionLedger + Send + Sync>>,
}

impl TransactionService {
    /// Creates a new transaction service with the given ledger
    pub fn new<L>(ledger: L) -> Self
    where
        L: TransactionLedger + Send + Sync + 'static,
    {
        Self {
            ledger: Arc::new(RwLock::new(ledger)),
        }
    }

    /// Creates a new transaction service with in-memory storage
    pub fn new_in_memory() -> Self {
        Self::new(InMemoryTransactionLedger::new())
    }

    /// Records a new transaction
    pub fn record_transaction(&self, transaction: Transaction) -> Result<(), TransactionError> {
        self.ledger
            .write()
            .map_err(|_| TransactionError::StorageError("Lock poisoned".to_string()))?
            .record_transaction(transaction)
    }

    /// Gets transaction history for a user
    pub fn get_user_transaction_history(&self, user_id: Uuid) -> Vec<Transaction> {
        self.ledger
            .read()
            .map_err(|_| TransactionError::StorageError("Lock poisoned".to_string()))
            .unwrap()
            .get_transaction_history(user_id)
    }

    /// Gets wallet transactions
    pub fn get_wallet_transactions(&self, wallet_id: &WalletId) -> Vec<Transaction> {
        self.ledger
            .read()
            .map_err(|_| TransactionError::StorageError("Lock poisoned".to_string()))
            .unwrap()
            .get_wallet_transactions(wallet_id)
    }

    /// Gets wallet balance
    pub fn get_balance(&self, wallet_id: &WalletId) -> Decimal {
        self.ledger
            .read()
            .map_err(|_| TransactionError::StorageError("Lock poisoned".to_string()))
            .unwrap()
            .get_balance(wallet_id)
    }

    /// Gets a specific transaction
    pub fn get_transaction(&self, id: Uuid) -> Option<Transaction> {
        self.ledger
            .read()
            .map_err(|_| TransactionError::StorageError("Lock poisoned".to_string()))
            .unwrap()
            .get_transaction(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_transaction_creation() {
        let tx = Transaction::new(
            Some(Uuid::new_v4()),
            dec!(100.50),
            "USD",
            TransactionType::Revenue,
            "wallet1".to_string(),
            "wallet2".to_string(),
        );

        assert_eq!(tx.amount, dec!(100.50));
        assert_eq!(tx.currency, "USD");
        assert_eq!(tx.transaction_type, TransactionType::Revenue);
        assert_eq!(tx.status, TransactionStatus::Pending);
    }

    #[test]
    fn test_transaction_validation() {
        let valid_tx = Transaction::new(
            Some(Uuid::new_v4()),
            dec!(100.0),
            "USD",
            TransactionType::Revenue,
            "wallet1".to_string(),
            "wallet2".to_string(),
        );

        assert!(TransactionValidator::validate(&valid_tx).is_ok());

        let invalid_amount = Transaction::new(
            Some(Uuid::new_v4()),
            dec!(-100.0),
            "USD",
            TransactionType::Revenue,
            "wallet1".to_string(),
            "wallet2".to_string(),
        );

        assert!(TransactionValidator::validate(&invalid_amount).is_err());

        let same_wallets = Transaction::new(
            Some(Uuid::new_v4()),
            dec!(100.0),
            "USD",
            TransactionType::Revenue,
            "wallet1".to_string(),
            "wallet1".to_string(),
        );

        assert!(TransactionValidator::validate(&same_wallets).is_err());
    }

    #[test]
    fn test_in_memory_ledger() {
        let mut ledger = InMemoryTransactionLedger::new();
        
        let user_id = Uuid::new_v4();
        let tx = Transaction::new(
            Some(user_id),
            dec!(50.0),
            "USD",
            TransactionType::Revenue,
            "wallet1".to_string(),
            "wallet2".to_string(),
        );

        ledger.record_transaction(tx.clone()).unwrap();
        
        let history = ledger.get_transaction_history(user_id);
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].id, tx.id);

        let balance = ledger.get_balance(&"wallet2".to_string());
        assert_eq!(balance, dec!(50.0));
    }

    #[test]
    fn test_transaction_service() {
        let service = TransactionService::new_in_memory();
        
        let user_id = Uuid::new_v4();
        let tx = Transaction::new(
            Some(user_id),
            dec!(75.0),
            "USD",
            TransactionType::ProfitDistribution,
            "treasury".to_string(),
            "user_wallet".to_string(),
        );

        service.record_transaction(tx.clone()).unwrap();
        
        let history = service.get_user_transaction_history(user_id);
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].transaction_type, TransactionType::ProfitDistribution);
    }
}