//! Wallet service for managing user wallets and transactions

use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::{
    wallet::{Wallet, WalletTransaction, TransactionType},
    primitives::{Money, Currency},
    FinanceError,
};

/// Repository trait for wallet persistence
#[async_trait]
pub trait WalletRepository {
    /// Save a wallet to the repository
    async fn save_wallet(&self, wallet: &Wallet) -> Result<(), FinanceError>;
    
    /// Find a wallet by user ID
    async fn find_wallet_by_user_id(&self, user_id: Uuid) -> Result<Option<Wallet>, FinanceError>;
    
    /// Save a wallet transaction
    async fn save_transaction(&self, transaction: &WalletTransaction) -> Result<(), FinanceError>;
    
    /// Find transactions for a wallet
    async fn find_transactions_by_wallet_id(&self, wallet_id: Uuid) -> Result<Vec<WalletTransaction>, FinanceError>;
}

/// Service trait for wallet operations
#[async_trait]
pub trait WalletService {
    /// Get or create a wallet for a user
    async fn get_or_create_wallet(&self, user_id: Uuid) -> Result<Wallet, FinanceError>;
    
    /// Add dabloons to a user's wallet
    async fn add_dabloons(&self, user_id: Uuid, amount: Money, description: Option<String>) -> Result<Wallet, FinanceError>;
    
    /// Subtract dabloons from a user's wallet
    async fn subtract_dabloons(&self, user_id: Uuid, amount: Money, description: Option<String>) -> Result<Wallet, FinanceError>;
    
    /// Transfer dabloons between users
    /// Add traditional currency to a user's wallet
    async fn add_traditional_currency(&self, user_id: Uuid, amount: Money, description: Option<String>) -> Result<Wallet, FinanceError>;
    
    /// Subtract traditional currency from a user's wallet
    async fn subtract_traditional_currency(&self, user_id: Uuid, amount: Money, description: Option<String>) -> Result<Wallet, FinanceError>;
    
    /// Transfer dabloons between users
    async fn transfer_dabloons(&self, from_user_id: Uuid, to_user_id: Uuid, amount: Money, description: Option<String>) -> Result<(Wallet, Wallet), FinanceError>;
    
    /// Get transaction history for a user's wallet
    async fn get_transaction_history(&self, user_id: Uuid) -> Result<Vec<WalletTransaction>, FinanceError>;
    
    /// Link wallet spending to a budget category
    async fn link_to_budget(&self, user_id: Uuid, category: &str, amount: Money) -> Result<(), FinanceError>;
    
    /// Get budgets linked to this wallet
    /// Distribute Universal Income to a user's wallet
    async fn distribute_universal_income(&self, user_id: Uuid, amount: Money, distribution_date: chrono::NaiveDate) -> Result<Wallet, FinanceError>;
    
    async fn get_linked_budgets(&self, user_id: Uuid) -> Result<Vec<(String, Money)>, FinanceError>;
}

/// Implementation of the WalletService
pub struct WalletServiceImpl {
    wallet_repo: std::sync::Arc<dyn WalletRepository>,
}

impl WalletServiceImpl {
    /// Create a new wallet service
    pub fn new(wallet_repo: std::sync::Arc<dyn WalletRepository>) -> Self {
        Self { wallet_repo }
    }
    
    /// Create a transaction record
    async fn create_transaction(&self, wallet_id: Uuid, transaction_type: TransactionType, amount: Money, description: Option<String>) -> Result<(), FinanceError> {
        let transaction = WalletTransaction::new(wallet_id, transaction_type, amount, description)?;
        self.wallet_repo.save_transaction(&transaction).await
    }
}

#[async_trait]
impl WalletService for WalletServiceImpl {
    async fn get_or_create_wallet(&self, user_id: Uuid) -> Result<Wallet, FinanceError> {
        match self.wallet_repo.find_wallet_by_user_id(user_id).await? {
            Some(wallet) => Ok(wallet),
            None => {
                let wallet = Wallet::new(user_id);
                self.wallet_repo.save_wallet(&wallet).await?;
                Ok(wallet)
            }
        }
    }
    
    async fn add_dabloons(&self, user_id: Uuid, amount: Money, description: Option<String>) -> Result<Wallet, FinanceError> {
        if amount.currency != Currency::Dabloons {
            return Err(FinanceError::FinancialError(FinancialError::CurrencyMismatch {
                expected: Currency::Dabloons.code().to_string(),
                actual: amount.currency.code().to_string(),
            }));
        }
        
        let mut wallet = self.get_or_create_wallet(user_id).await?;
        wallet.add_dabloons(amount.clone())?;
        self.wallet_repo.save_wallet(&wallet).await?;
        
        // Record the transaction
        self.create_transaction(
            wallet.id,
            TransactionType::Credit,
            amount,
            description
        ).await?;
        
        Ok(wallet)
    }
    
    async fn subtract_dabloons(&self, user_id: Uuid, amount: Money, description: Option<String>) -> Result<Wallet, FinanceError> {
        if amount.currency != Currency::Dabloons {
            return Err(FinanceError::FinancialError(FinancialError::CurrencyMismatch {
                expected: Currency::Dabloons.code().to_string(),
                actual: amount.currency.code().to_string(),
            }));
        }
        
        let mut wallet = self.get_or_create_wallet(user_id).await?;
        wallet.subtract_dabloons(amount.clone())?;
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
    
    async fn transfer_dabloons(&self, from_user_id: Uuid, to_user_id: Uuid, amount: Money, description: Option<String>) -> Result<(Wallet, Wallet), FinanceError> {
        if amount.currency != Currency::Dabloons {
            return Err(FinanceError::FinancialError(FinancialError::CurrencyMismatch {
                expected: Currency::Dabloons.code().to_string(),
                actual: amount.currency.code().to_string(),
            }));
        }
        
        // Get both wallets
        let mut from_wallet = self.get_or_create_wallet(from_user_id).await?;
        let mut to_wallet = self.get_or_create_wallet(to_user_id).await?;
        
        // Check if the sender has sufficient balance
        if !from_wallet.has_sufficient_balance(&amount)? {
            return Err(FinanceError::InsufficientFunds(Currency::Dabloons));
        }
        
        // Perform the transfer
        from_wallet.subtract_dabloons(amount.clone())?;
        to_wallet.add_dabloons(amount.clone())?;
        
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
    
    async fn get_transaction_history(&self, user_id: Uuid) -> Result<Vec<WalletTransaction>, FinanceError> {
        let wallet = self.get_or_create_wallet(user_id).await?;
        self.wallet_repo.find_transactions_by_wallet_id(wallet.id).await
    }
    
    async fn link_to_budget(&self, user_id: Uuid, category: &str, amount: Money) -> Result<(), FinanceError> {
        // In a real implementation, this would update the budget service
        // For now, we'll just validate the inputs
        if amount.currency != Currency::Dabloons {
            return Err(FinanceError::FinancialError(crate::domain::primitives::FinancialError::InvalidCurrency));
        }
        
        // This is a placeholder implementation
        // In a real implementation, this would:
        // 1. Call the budget service to update the spent amount for the category
        // 2. Handle any errors (e.g., budget exceeded)
        // 3. Return appropriate results
        
        Ok(())
    }
    
    async fn get_linked_budgets(&self, user_id: Uuid) -> Result<Vec<(String, Money)>, FinanceError> {
        // This is a placeholder implementation
        // In a real implementation, this would query the budget service
        // to get all budgets linked to this user's wallet
        
        Ok(vec![])
        
        async fn add_traditional_currency(&self, user_id: Uuid, amount: Money, description: Option<String>) -> Result<Wallet, FinanceError> {
            if amount.currency == Currency::Dabloons {
                return Err(FinanceError::FinancialError(FinancialError::CurrencyMismatch {
                    expected: "Traditional Currency".to_string(),
                    actual: amount.currency.code().to_string(),
                }));
            }
            
            let mut wallet = self.get_or_create_wallet(user_id).await?;
            wallet.add_traditional_currency(amount.clone())?;
            self.wallet_repo.save_wallet(&wallet).await?;
            
            // Record the transaction
            self.create_transaction(
                wallet.id,
                TransactionType::Credit,
                amount,
                description
            ).await?;
            
            Ok(wallet)
        }
        
        async fn subtract_traditional_currency(&self, user_id: Uuid, amount: Money, description: Option<String>) -> Result<Wallet, FinanceError> {
            if amount.currency == Currency::Dabloons {
                return Err(FinanceError::FinancialError(FinancialError::CurrencyMismatch {
                    expected: "Traditional Currency".to_string(),
                    actual: amount.currency.code().to_string(),
                }));
            }
            
            let mut wallet = self.get_or_create_wallet(user_id).await?;
            wallet.subtract_traditional_currency(amount.clone())?;
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
    }
    
    async fn distribute_universal_income(&self, user_id: Uuid, amount: Money, distribution_date: chrono::NaiveDate) -> Result<Wallet, FinanceError> {
        if amount.currency != Currency::Dabloons {
            return Err(FinanceError::FinancialError(crate::domain::primitives::FinancialError::CurrencyMismatch {
                expected: Currency::Dabloons.code().to_string(),
                actual: amount.currency.code().to_string(),
            }));
        }
        
        let mut wallet = self.get_or_create_wallet(user_id).await?;
        wallet.add_dabloons(amount.clone())?;
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
}