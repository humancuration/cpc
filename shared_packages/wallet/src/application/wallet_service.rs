//! Wallet service for managing user wallets and transactions

use async_trait::async_trait;
use uuid::Uuid;
use tokio::sync::broadcast;
use crate::domain::{
    wallet::{Wallet, WalletTransaction, TransactionType, TipSentEvent},
    primitives::{Money, Currency, FinancialError},
};
use common_utils::financial::MonetaryValue;

// Conditionally import common_utils logging or fallback to tracing
#[cfg(feature = "common-utils-integration")]
use common_utils::logging::{info, warn, error, debug};
#[cfg(not(feature = "common-utils-integration"))]
use tracing::{info, warn, error, debug};

/// Repository trait for wallet persistence
#[async_trait]
pub trait WalletRepository {
    /// Save a wallet to the repository
    async fn save_wallet(&self, wallet: &Wallet) -> Result<(), FinancialError>;
    
    /// Find a wallet by user ID
    async fn find_wallet_by_user_id(&self, user_id: Uuid) -> Result<Option<Wallet>, FinancialError>;
    
    /// Save a wallet transaction
    async fn save_transaction(&self, transaction: &WalletTransaction) -> Result<(), FinancialError>;
    
    /// Find transactions for a wallet
    async fn find_transactions_by_wallet_id(&self, wallet_id: Uuid) -> Result<Vec<WalletTransaction>, FinancialError>;
}

/// Service trait for wallet operations
#[async_trait]
pub trait WalletService {
    /// Get or create a wallet for a user
    async fn get_or_create_wallet(&self, user_id: Uuid) -> Result<Wallet, FinancialError>;
    
    /// Add dabloons to a user's wallet
    async fn add_dabloons(&self, user_id: Uuid, amount: Money, description: Option<String>) -> Result<Wallet, FinancialError>;
    
    /// Subtract dabloons from a user's wallet
    async fn subtract_dabloons(&self, user_id: Uuid, amount: Money, description: Option<String>) -> Result<Wallet, FinancialError>;
    
    /// Transfer dabloons between users
    async fn transfer_dabloons(&self, from_user_id: Uuid, to_user_id: Uuid, amount: Money, description: Option<String>) -> Result<(Wallet, Wallet), FinancialError>;
    
    /// Send a tip to another user
    async fn send_tip(&self, from_user_id: Uuid, to_user_id: Uuid, amount: Money, note: Option<String>) -> Result<(), FinancialError>;
    
    /// Get transaction history for a user's wallet
    async fn get_transaction_history(&self, user_id: Uuid) -> Result<Vec<WalletTransaction>, FinancialError>;
    
    /// Distribute Universal Income to a user's wallet
    async fn distribute_universal_income(&self, user_id: Uuid, amount: Money, distribution_date: chrono::NaiveDate) -> Result<Wallet, FinancialError>;
    
    /// Credit Dabloons from volunteer hours conversion
    async fn credit_volunteer_dabloons(&self, user_id: Uuid, amount: Money, hours_converted: rust_decimal::Decimal) -> Result<Wallet, FinancialError>;
}

/// Implementation of the WalletService
pub struct WalletServiceImpl {
    wallet_repo: std::sync::Arc<dyn WalletRepository>,
    tip_events: broadcast::Sender<TipSentEvent>,
}

impl WalletServiceImpl {
    /// Create a new wallet service
    pub fn new(wallet_repo: std::sync::Arc<dyn WalletRepository>) -> Self {
        let (tip_events, _) = broadcast::channel(100);
        Self { wallet_repo, tip_events }
    }
    
    /// Subscribe to tip events
    pub fn subscribe_tip_events(&self) -> broadcast::Receiver<TipSentEvent> {
        self.tip_events.subscribe()
    }
    
    /// Create a transaction record
    async fn create_transaction(&self, wallet_id: Uuid, transaction_type: TransactionType, amount: Money, description: Option<String>) -> Result<(), FinancialError> {
        let transaction = WalletTransaction::new(wallet_id, transaction_type, amount, description)?;
        self.wallet_repo.save_transaction(&transaction).await
    }
}

#[async_trait]
impl WalletService for WalletServiceImpl {
    async fn get_or_create_wallet(&self, user_id: Uuid) -> Result<Wallet, FinancialError> {
        match self.wallet_repo.find_wallet_by_user_id(user_id).await? {
            Some(wallet) => Ok(wallet),
            None => {
                let wallet = Wallet::new(user_id);
                self.wallet_repo.save_wallet(&wallet).await?;
                Ok(wallet)
            }
        }
    
    async fn add_dabloons(&self, user_id: Uuid, amount: Money, description: Option<String>) -> Result<Wallet, FinancialError> {
        if amount.currency != Currency::Dabloons {
            return Err(FinancialError::CurrencyMismatch {
                expected: Currency::Dabloons.code().to_string(),
                actual: amount.currency.code().to_string(),
            });
        }
        
        let mut wallet = self.get_or_create_wallet(user_id).await?;
        
        // Use fixed-point arithmetic for precise balance calculation
        let current_balance = wallet.dabloons_balance;
        let amount_to_add = amount.amount;
        
        let current_balance_fixed = fixed::types::I64F64::from_num(current_balance.to_f64().unwrap_or(0.0));
        let amount_to_add_fixed = fixed::types::I64F64::from_num(amount_to_add.to_f64().unwrap_or(0.0));
        let new_balance_fixed = current_balance_fixed + amount_to_add_fixed;
        
        // Update the wallet with the new balance
        wallet.dabloons_balance = rust_decimal::Decimal::from_f64(new_balance_fixed.to_num::<f64>())
            .unwrap_or(rust_decimal::Decimal::ZERO);
        
        self.wallet_repo.save_wallet(&wallet).await?;
        
        // Record the transaction
        self.create_transaction(
            wallet.id,
            TransactionType::Credit,
            amount,
            description
        ).await?;
        
        Ok(wallet)
        Ok(wallet)
    }
    
    async fn subtract_dabloons(&self, user_id: Uuid, amount: Money, description: Option<String>) -> Result<Wallet, FinancialError> {
        if amount.currency != Currency::Dabloons {
            return Err(FinancialError::CurrencyMismatch {
                expected: Currency::Dabloons.code().to_string(),
                actual: amount.currency.code().to_string(),
            });
        }
        
        let mut wallet = self.get_or_create_wallet(user_id).await?;
        
        // Use fixed-point arithmetic for precise balance calculation
        let current_balance = wallet.dabloons_balance;
        let amount_to_subtract = amount.amount;
        
        let current_balance_fixed = fixed::types::I64F64::from_num(current_balance.to_f64().unwrap_or(0.0));
        let amount_to_subtract_fixed = fixed::types::I64F64::from_num(amount_to_subtract.to_f64().unwrap_or(0.0));
        let new_balance_fixed = current_balance_fixed - amount_to_subtract_fixed;
        
        // Update the wallet with the new balance
        wallet.dabloons_balance = rust_decimal::Decimal::from_f64(new_balance_fixed.to_num::<f64>())
            .unwrap_or(rust_decimal::Decimal::ZERO);
        
        self.wallet_repo.save_wallet(&wallet).await?;
        
        // Record the transaction
        self.create_transaction(
            wallet.id,
            TransactionType::Debit,
            amount,
            description
        ).await?;
        
        Ok(wallet)
    }
    
    /// Send a tip to another user
    async fn send_tip(&self, from_user_id: Uuid, to_user_id: Uuid, amount: Money, note: Option<String>) -> Result<(), FinancialError> {
        // Validate currency
        if amount.currency != Currency::Dabloons {
            return Err(FinancialError::CurrencyMismatch {
                expected: Currency::Dabloons.code().to_string(),
                actual: amount.currency.code().to_string(),
            });
        }
        
        // Get the sender's wallet
        let mut from_wallet = self.get_or_create_wallet(from_user_id).await?;
        
        // Send the tip and get the event
        let event = from_wallet.send_tip(to_user_id, amount.clone(), note)?;
        
        // Save the updated wallet
        self.wallet_repo.save_wallet(&from_wallet).await?;
        
        // Record the transaction
        let transfer_desc = event.note.clone().unwrap_or_else(|| "Tip".to_string());
        let from_desc = format!("Tip to user {}: {}", to_user_id, transfer_desc);
        
        self.create_transaction(
            from_wallet.id,
            TransactionType::Debit,
            amount,
            Some(from_desc)
        ).await?;
        
        // Broadcast the tip event
        let _ = self.tip_events.send(event);
        
        Ok(())
    }
    
    async fn transfer_dabloons(&self, from_user_id: Uuid, to_user_id: Uuid, amount: Money, description: Option<String>) -> Result<(Wallet, Wallet), FinancialError> {
        if amount.currency != Currency::Dabloons {
            return Err(FinancialError::CurrencyMismatch {
                expected: Currency::Dabloons.code().to_string(),
                actual: amount.currency.code().to_string(),
            });
        }
        
        // Get both wallets
        let mut from_wallet = self.get_or_create_wallet(from_user_id).await?;
        let mut to_wallet = self.get_or_create_wallet(to_user_id).await?;
        
        // Check if the sender has sufficient balance
        if !from_wallet.has_sufficient_balance(&amount)? {
            return Err(FinancialError::InsufficientFunds(Currency::Dabloons));
        }
        
        // Use fixed-point arithmetic for precise balance calculations
        let from_balance = from_wallet.dabloons_balance;
        let to_balance = to_wallet.dabloons_balance;
        let transfer_amount = amount.amount;
        
        let from_balance_fixed = fixed::types::I64F64::from_num(from_balance.to_f64().unwrap_or(0.0));
        let to_balance_fixed = fixed::types::I64F64::from_num(to_balance.to_f64().unwrap_or(0.0));
        let transfer_amount_fixed = fixed::types::I64F64::from_num(transfer_amount.to_f64().unwrap_or(0.0));
        
        let new_from_balance_fixed = from_balance_fixed - transfer_amount_fixed;
        let new_to_balance_fixed = to_balance_fixed + transfer_amount_fixed;
        
        // Update both wallets with the new balances
        from_wallet.dabloons_balance = rust_decimal::Decimal::from_f64(new_from_balance_fixed.to_num::<f64>())
            .unwrap_or(rust_decimal::Decimal::ZERO);
        to_wallet.dabloons_balance = rust_decimal::Decimal::from_f64(new_to_balance_fixed.to_num::<f64>())
            .unwrap_or(rust_decimal::Decimal::ZERO);
        
        // Save both wallets
        self.wallet_repo.save_wallet(&from_wallet).await?;
        self.wallet_repo.save_wallet(&to_wallet).await?;
        
        // Record the transactions
        let transfer_desc = description.clone().unwrap_or_else(|| "Transfer".to_string());
        let from_desc = format!("Transfer to user {}: {}", to_user_id, transfer_desc);
        let to_desc = format!("Transfer from user {}: {}", from_user_id, transfer_desc);
        
        self.create_transaction(
            from_wallet.id,
            TransactionType::Debit,
            amount.clone(),
            Some(from_desc)
        ).await?;
        
        self.create_transaction(
            to_wallet.id,
            TransactionType::Credit,
            amount,
            Some(to_desc)
        ).await?;
        
        Ok((from_wallet, to_wallet))
    }
    
    async fn get_transaction_history(&self, user_id: Uuid) -> Result<Vec<WalletTransaction>, FinancialError> {
        let wallet = self.get_or_create_wallet(user_id).await?;
        self.wallet_repo.find_transactions_by_wallet_id(wallet.id).await
    }
    
    async fn distribute_universal_income(&self, user_id: Uuid, amount: Money, distribution_date: chrono::NaiveDate) -> Result<Wallet, FinancialError> {
        if amount.currency != Currency::Dabloons {
            return Err(FinancialError::CurrencyMismatch {
                expected: Currency::Dabloons.code().to_string(),
                actual: amount.currency.code().to_string(),
            });
        }
        
        let mut wallet = self.get_or_create_wallet(user_id).await?;
        
        // Use fixed-point arithmetic for precise balance calculation
        let current_balance = wallet.dabloons_balance;
        let amount_to_add = amount.amount;
        
        let current_balance_fixed = fixed::types::I64F64::from_num(current_balance.to_f64().unwrap_or(0.0));
        let amount_to_add_fixed = fixed::types::I64F64::from_num(amount_to_add.to_f64().unwrap_or(0.0));
        let new_balance_fixed = current_balance_fixed + amount_to_add_fixed;
        
        // Update the wallet with the new balance
        wallet.dabloons_balance = rust_decimal::Decimal::from_f64(new_balance_fixed.to_num::<f64>())
            .unwrap_or(rust_decimal::Decimal::ZERO);
        
        self.wallet_repo.save_wallet(&wallet).await?;
        
        // Record the transaction with a special description for Universal Income
        let description = Some(format!("Universal Income distribution for {}", distribution_date));
        self.create_transaction(
            wallet.id,
            TransactionType::Credit,
            amount,
            description
        ).await?;
        
        Ok(wallet)
    }
    
    async fn credit_volunteer_dabloons(&self, user_id: Uuid, amount: Money, hours_converted: rust_decimal::Decimal) -> Result<Wallet, FinancialError> {
        if amount.currency != Currency::Dabloons {
            return Err(FinancialError::CurrencyMismatch {
                expected: Currency::Dabloons.code().to_string(),
                actual: amount.currency.code().to_string(),
            });
        }
        
        let mut wallet = self.get_or_create_wallet(user_id).await?;
        
        // Use fixed-point arithmetic for precise balance calculation
        let current_balance = wallet.dabloons_balance;
        let amount_to_add = amount.amount;
        
        let current_balance_fixed = fixed::types::I64F64::from_num(current_balance.to_f64().unwrap_or(0.0));
        let amount_to_add_fixed = fixed::types::I64F64::from_num(amount_to_add.to_f64().unwrap_or(0.0));
        let new_balance_fixed = current_balance_fixed + amount_to_add_fixed;
        
        // Update the wallet with the new balance
        wallet.dabloons_balance = rust_decimal::Decimal::from_f64(new_balance_fixed.to_num::<f64>())
            .unwrap_or(rust_decimal::Decimal::ZERO);
        
        self.wallet_repo.save_wallet(&wallet).await?;
        
        // Record the transaction with a special description for volunteer conversion
        let description = Some(format!("Converted {} volunteer hours to Dabloons", hours_converted));
        self.create_transaction(
            wallet.id,
            TransactionType::Credit,
            amount,
            description
        ).await?;
        
        Ok(wallet)
    }
}