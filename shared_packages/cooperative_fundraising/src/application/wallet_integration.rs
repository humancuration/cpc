//! Integration with wallet for fund management

use async_trait::async_trait;
use uuid::Uuid;
use rust_decimal::Decimal;
use cpc_wallet::application::WalletService;
use cpc_wallet::domain::primitives::{Money, Currency};

#[async_trait]
pub trait WalletIntegration: Send + Sync {
    /// Record a donation in the wallet system
    async fn record_donation(
        &self,
        user_id: Uuid,
        campaign_id: Uuid,
        amount: Decimal,
        contribution_id: &Uuid,
    ) -> Result<(), WalletIntegrationError>;
    
    /// Allocate funds for a campaign
    async fn allocate_funds(
        &self,
        campaign_id: Uuid,
        amount: Decimal,
        purpose: &str,
    ) -> Result<(), WalletIntegrationError>;
}

pub struct WalletIntegrationImpl {
    wallet_service: std::sync::Arc<dyn WalletService>,
}

impl WalletIntegrationImpl {
    pub fn new(wallet_service: std::sync::Arc<dyn WalletService>) -> Self {
        Self {
            wallet_service,
        }
    }
}

#[async_trait]
impl WalletIntegration for WalletIntegrationImpl {
    async fn record_donation(
        &self,
        user_id: Uuid,
        _campaign_id: Uuid,
        amount: Decimal,
        contribution_id: &Uuid,
    ) -> Result<(), WalletIntegrationError> {
        // Record the donation in the user's wallet
        // In a real implementation, this would create a wallet transaction
        tracing::info!(
            "Recording donation from user {} for amount {} with contribution ID {}",
            user_id,
            amount,
            contribution_id
        );
        
        // Convert to Money type
        let money = Money::new(amount, Currency::USD); // Assuming USD for now
        
        // In a real implementation, we would call the wallet service
        // For now, we'll just log the action
        tracing::info!("Would record {} in user {}'s wallet", money.amount, user_id);
        
        Ok(())
    }
    
    async fn allocate_funds(
        &self,
        campaign_id: Uuid,
        amount: Decimal,
        purpose: &str,
    ) -> Result<(), WalletIntegrationError> {
        // Allocate funds for the campaign
        tracing::info!(
            "Allocating {} funds for campaign {} for purpose: {}",
            amount,
            campaign_id,
            purpose
        );
        
        // In a real implementation, this would create a fund allocation record
        // and potentially move funds to a dedicated campaign account
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WalletIntegrationError {
    #[error("Insufficient funds")]
    InsufficientFunds,
    
    #[error("Wallet error: {0}")]
    WalletError(String),
    
    #[error("Integration error: {0}")]
    IntegrationError(String),
}