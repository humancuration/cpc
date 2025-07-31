// This file has been deprecated as tip functionality has been moved to the wallet package
// The file is kept for reference but is no longer used in the codebase
//! Tip service for social integration

use uuid::Uuid;
use crate::domain::tip_transaction::TipTransaction;
use crate::infrastructure::repositories::TipTransactionRepository;
use cpc_wallet::application::wallet_service::WalletService;
use cpc_wallet::domain::primitives::{Money, Currency};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

/// Service for handling voluntary tipping between users
#[derive(Debug)]
pub struct TipService {
    wallet_service: Box<dyn WalletService + Send + Sync>,
    tip_transaction_repository: Box<dyn TipTransactionRepository + Send + Sync>,
}

impl TipService {
    /// Create a new tip service
    pub fn new(
        wallet_service: Box<dyn WalletService + Send + Sync>,
        tip_transaction_repository: Box<dyn TipTransactionRepository + Send + Sync>,
    ) -> Self {
        Self {
            wallet_service,
            tip_transaction_repository,
        }
    }

    /// Send a tip from one user to another
    pub async fn send_tip(
        &self,
        sender_id: Uuid,
        recipient_id: Uuid,
        amount: Money,
        note: Option<String>
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Validate amount
        if amount.amount <= Decimal::ZERO {
            return Err("Tip amount must be positive".into());
        }

        // Transfer funds using wallet service
        self.wallet_service
            .transfer_dabloons(sender_id, recipient_id, amount.clone(), note.clone())
            .await?;

        // Record transaction
        self.tip_transaction_repository
            .record_transaction(
                sender_id,
                recipient_id,
                amount,
                "tip",
                note.unwrap_or_else(|| "User tip".to_string())
            )
            .await?;

        Ok(())
    }
    
    /// Get tip transactions for a user with pagination
    pub async fn get_tip_transactions_for_user(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<TipTransaction>, Box<dyn std::error::Error + Send + Sync>> {
        self.tip_transaction_repository
            .get_transactions_for_user(user_id, limit, offset)
            .await
    }
}