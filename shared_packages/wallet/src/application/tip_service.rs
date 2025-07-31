//! Tip service for handling tip transactions between users
//! 
//! This service provides functionality for sending tips between users,
//! handling wallet balance updates, and recording transactions.
//! 
//! # Sequence Diagram
//! Refer to docs/diagrams/tip_sequence.mmd for the complete flow.

use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::{
    wallet::{Wallet, WalletTransaction, TransactionType, TipSentEvent},
    primitives::{Money, Currency, FinancialError},
};
use crate::application::WalletRepository;

/// Service for handling tip operations
#[derive(Debug)]
pub struct TipService {
    wallet_repo: std::sync::Arc<dyn WalletRepository>,
}

impl TipService {
    /// Create a new tip service
    pub fn new(wallet_repo: std::sync::Arc<dyn WalletRepository>) -> Self {
        Self { wallet_repo }
    }
    
    /// Send a tip from one user to another
    /// 
    /// This method:
    /// 1. Deducts amount from sender's wallet
    /// 2. Adds amount to recipient's wallet
    /// 3. Records transaction in wallet history
    /// 4. Broadcasts TipSentEvent
    /// 
    /// # Arguments
    /// * `sender_id` - UUID of the user sending the tip
    /// * `recipient_id` - UUID of the user receiving the tip
    /// * `amount` - Amount of dabloons to tip
    /// * `note` - Optional note for the tip
    /// 
    /// # Returns
    /// Result with TipSentEvent on success, FinancialError on failure
    /// 
    /// # Errors
    /// Returns FinancialError if:
    /// - Currency is not Dabloons
    /// - Amount is not positive
    /// - Sender has insufficient funds
    /// - Database operations fail
    pub async fn send_tip(
        &self,
        sender_id: Uuid,
        recipient_id: Uuid,
        amount: Money,
        note: Option<String>,
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
        
        // Get both wallets
        let mut sender_wallet = self.get_or_create_wallet(sender_id).await?;
        let mut recipient_wallet = self.get_or_create_wallet(recipient_id).await?;
        
        // Check if sender has sufficient balance
        if !sender_wallet.has_sufficient_balance(&amount)? {
            return Err(FinancialError::InsufficientFunds(Currency::Dabloons));
        }
        
        // Deduct from sender
        sender_wallet.subtract_dabloons(amount.clone())?;
        self.wallet_repo.save_wallet(&sender_wallet).await?;
        
        // Add to recipient
        recipient_wallet.add_dabloons(amount.clone())?;
        self.wallet_repo.save_wallet(&recipient_wallet).await?;
        
        // Record transactions
        let transaction_id = Uuid::new_v4();
        let timestamp = chrono::Utc::now();
        
        let sender_desc = format!("Tip to user {}: {}", recipient_id, note.clone().unwrap_or_else(|| "Tip".to_string()));
        let recipient_desc = format!("Tip from user {}: {}", sender_id, note.clone().unwrap_or_else(|| "Tip".to_string()));
        
        // Record sender transaction
        let sender_transaction = WalletTransaction::new(
            sender_wallet.id,
            TransactionType::Debit,
            amount.clone(),
            Some(sender_desc),
        )?;
        self.wallet_repo.save_transaction(&sender_transaction).await?;
        
        // Record recipient transaction
        let recipient_transaction = WalletTransaction::new(
            recipient_wallet.id,
            TransactionType::Credit,
            amount.clone(),
            Some(recipient_desc),
        )?;
        self.wallet_repo.save_transaction(&recipient_transaction).await?;
        
        // Create and return the tip event
        let event = TipSentEvent {
            transaction_id,
            sender_id,
            recipient_id,
            amount,
            note,
            timestamp,
        };
        
        Ok(event)
    }
    
    /// Get or create a wallet for a user
    async fn get_or_create_wallet(&self, user_id: Uuid) -> Result<Wallet, FinancialError> {
        match self.wallet_repo.find_wallet_by_user_id(user_id).await? {
            Some(wallet) => Ok(wallet),
            None => {
                let wallet = Wallet::new(user_id);
                self.wallet_repo.save_wallet(&wallet).await?;
                Ok(wallet)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use std::sync::{Arc, Mutex};
    use std::collections::HashMap;
    use tokio;
    
    // Mock repository for testing
    struct MockWalletRepository {
        wallets: Mutex<HashMap<Uuid, Wallet>>,
        transactions: Mutex<Vec<WalletTransaction>>,
    }
    
    impl MockWalletRepository {
        fn new() -> Self {
            Self {
                wallets: Mutex::new(HashMap::new()),
                transactions: Mutex::new(Vec::new()),
            }
        }
    }
    
    #[async_trait]
    impl WalletRepository for MockWalletRepository {
        async fn save_wallet(&self, wallet: &Wallet) -> Result<(), FinancialError> {
            let mut wallets = self.wallets.lock().unwrap();
            wallets.insert(wallet.user_id, wallet.clone());
            Ok(())
        }
        
        async fn find_wallet_by_user_id(&self, user_id: Uuid) -> Result<Option<Wallet>, FinancialError> {
            let wallets = self.wallets.lock().unwrap();
            Ok(wallets.get(&user_id).cloned())
        }
        
        async fn save_transaction(&self, transaction: &WalletTransaction) -> Result<(), FinancialError> {
            let mut transactions = self.transactions.lock().unwrap();
            transactions.push(transaction.clone());
            Ok(())
        }
        
        async fn find_transactions_by_wallet_id(&self, _wallet_id: Uuid) -> Result<Vec<WalletTransaction>, FinancialError> {
            let transactions = self.transactions.lock().unwrap();
            Ok(transactions.clone())
        }
    }
    #[tokio::test]
    async fn test_send_tip_success() {
        let repo = Arc::new(MockWalletRepository::new());
        let tip_service = TipService::new(repo.clone());
        
        let sender_id = Uuid::new_v4();
        let recipient_id = Uuid::new_v4();
        let amount = Money::new(dec!(10), Currency::Dabloons);
        let note = Some("Thanks for the great post!".to_string());
        
        // Create sender wallet with initial balance
        let mut sender_wallet = Wallet::new(sender_id);
        sender_wallet.add_dabloons(Money::new(dec!(50), Currency::Dabloons)).unwrap();
        repo.save_wallet(&sender_wallet).await.unwrap();
        
        // Create recipient wallet
        let recipient_wallet = Wallet::new(recipient_id);
        repo.save_wallet(&recipient_wallet).await.unwrap();
        
        // Send tip
        let result = tip_service.send_tip(sender_id, recipient_id, amount.clone(), note.clone()).await;
        assert!(result.is_ok());
        
        // Verify the tip event
        let event = result.unwrap();
        assert_eq!(event.sender_id, sender_id);
        assert_eq!(event.recipient_id, recipient_id);
        assert_eq!(event.amount, amount);
        assert_eq!(event.note, note);
        
        // Verify sender wallet balance
        let updated_sender_wallet = tip_service.get_or_create_wallet(sender_id).await.unwrap();
        assert_eq!(updated_sender_wallet.balance.amount, dec!(40)); // 50 - 10
        
        // Verify recipient wallet balance
        let updated_recipient_wallet = tip_service.get_or_create_wallet(recipient_id).await.unwrap();
        assert_eq!(updated_recipient_wallet.balance.amount, dec!(10)); // 0 + 10
    }
    
    #[tokio::test]
    async fn test_send_tip_wrong_currency() {
        let repo = Arc::new(MockWalletRepository::new());
        let tip_service = TipService::new(repo);
        
        let sender_id = Uuid::new_v4();
        let recipient_id = Uuid::new_v4();
        let amount = Money::new(dec!(10), Currency::USD);
        let note = Some("Thanks for the great post!".to_string());
        
        let result = tip_service.send_tip(sender_id, recipient_id, amount, note).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), FinancialError::CurrencyMismatch { .. }));
    }
    
    #[tokio::test]
    async fn test_send_tip_negative_amount() {
        let repo = Arc::new(MockWalletRepository::new());
        let tip_service = TipService::new(repo);
        
        let sender_id = Uuid::new_v4();
        let recipient_id = Uuid::new_v4();
        let amount = Money::new(dec!(-10), Currency::Dabloons);
        let note = Some("Thanks for the great post!".to_string());
        
        let result = tip_service.send_tip(sender_id, recipient_id, amount, note).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), FinancialError::InvalidAmount));
    }
    
    #[tokio::test]
    async fn test_send_tip_zero_amount() {
        let repo = Arc::new(MockWalletRepository::new());
        let tip_service = TipService::new(repo);
        
        let sender_id = Uuid::new_v4();
        let recipient_id = Uuid::new_v4();
        let amount = Money::new(Decimal::ZERO, Currency::Dabloons);
        let note = Some("Thanks for the great post!".to_string());
        
        let result = tip_service.send_tip(sender_id, recipient_id, amount, note).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), FinancialError::InvalidAmount));
    }
}