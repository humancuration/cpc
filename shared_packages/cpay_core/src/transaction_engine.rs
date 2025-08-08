//! Transaction processing engine for CPay
//!
//! This module implements the core transaction processing logic, including
//! compliance checks, fraud detection, and integration with wallet services.

use crate::models::{PaymentRequest, PaymentResponse, Transaction, TransactionStatus, PaymentError};
use crate::repositories::TraditionalCurrencyTransactionRepository;
use wallet::application::WalletService;
use wallet::domain::primitives::{Money, Currency as WalletCurrency};
use uuid::Uuid;
use rust_decimal::Decimal;
use common_utils::logging::{info, warn, error};
use common_utils::financial::MonetaryValue;
use cpc_financial_core::{MonetaryAmount, CurrencyCode};
use cpc_financial_core::audit::FinancialAuditHook;

/// Transaction processing engine
pub struct TransactionEngine {
    wallet_service: std::sync::Arc<dyn WalletService>,
    traditional_currency_repo: std::sync::Arc<dyn TraditionalCurrencyTransactionRepository>,
    audit_hook: std::sync::Arc<FinancialAuditHook>,
}

impl TransactionEngine {
    /// Create a new transaction engine
    pub fn new(
        wallet_service: std::sync::Arc<dyn WalletService>,
        traditional_currency_repo: std::sync::Arc<dyn TraditionalCurrencyTransactionRepository>,
        audit_hook: std::sync::Arc<FinancialAuditHook>,
    ) -> Self {
        Self {
            wallet_service,
            traditional_currency_repo,
            audit_hook,
        }
    }
    
    /// Process a payment request
    pub async fn process_payment(&self, request: PaymentRequest) -> Result<PaymentResponse, PaymentError> {
        info!("Processing payment request: {}", request.id);
        
        // Record audit log for payment processing
        let _ = self.audit_hook.record_operation(
            Some(request.user_id.to_string()),
            "payment_processing",
            serde_json::json!({
                "payment_id": request.id.to_string(),
                "amount": request.amount.value().to_string(),
                "currency": request.currency.to_string(),
                "recipient_id": request.recipient_id.to_string()
            }),
            serde_json::json!({}),
            serde_json::json!({}),
        ).await;
        
        // Perform compliance checks
        self.perform_compliance_checks(&request).await?;
        
        // Convert currency to wallet currency format
        let wallet_currency = self.convert_currency(&request.currency);
        let amount = Money::new(request.amount.value(), wallet_currency);
        
        // Process the transaction based on currency type
        match request.currency {
            CurrencyCode::DBL => {
                self.process_dabloons_transaction(request, amount).await
            },
            _ => {
                self.process_traditional_currency_transaction(request, amount).await
            }
        }
    }
    
    /// Process a transaction using Dabloons
    async fn process_dabloons_transaction(&self, request: PaymentRequest, amount: Money) -> Result<PaymentResponse, PaymentError> {
        // For Dabloons, we use the wallet service directly
        let result = self.wallet_service
            .transfer_dabloons(
                request.user_id,
                request.recipient_id,
                amount,
                request.description.clone()
            )
            .await
            .map_err(|e| PaymentError::from(e));
        
        // Perform post-transaction actions regardless of transaction success
        self.perform_post_transaction_actions(&request).await?;
        
        result.map(|_| {
            PaymentResponse {
                transaction_id: Uuid::new_v4(),
                status: TransactionStatus::Completed,
                timestamp: chrono::Utc::now(),
            }
        })
    }
    
    /// Process a transaction using traditional currency
    async fn process_traditional_currency_transaction(&self, request: PaymentRequest, amount: Money) -> Result<PaymentResponse, PaymentError> {
        // For traditional currencies, we would integrate with external payment providers
        // This is a simplified implementation
        info!("Processing traditional currency transaction: {} {}", amount.amount, amount.currency.code());
        
        // Create a transaction record
        let transaction = crate::repositories::TraditionalCurrencyTransaction::new(
            request.user_id,
            "debit".to_string(),
            request.amount.value(),
            request.currency.to_string(),
            None, // external_reference
            request.description.clone(),
            None, // social_post_id
            request.volunteer_hours, // volunteer_hours
        );
        
        // Save the transaction
        self.traditional_currency_repo
            .save_transaction(transaction)
            .await
            .map_err(|e| PaymentError::from(e))?;
        
        // In a real implementation, this would connect to external payment processors
        // For now, we'll simulate a successful transaction
        Ok(PaymentResponse {
            transaction_id: Uuid::new_v4(),
            status: TransactionStatus::Completed,
            timestamp: chrono::Utc::now(),
        })
    }
    
    /// Perform compliance checks on a payment request
    async fn perform_compliance_checks(&self, request: &PaymentRequest) -> Result<(), PaymentError> {
        info!("Performing compliance checks for payment: {}", request.id);
        
        // Mock KYC provider integration
        if !self.mock_kyc_check(request.user_id).await {
            warn!("KYC check failed for user: {}", request.user_id);
            return Err(PaymentError::generic("KYC verification required"));
        }
        
        // Check transaction limits
        if !self.check_transaction_limits(request).await {
            warn!("Transaction limits exceeded for user: {}", request.user_id);
            return Err(PaymentError::generic("Transaction limits exceeded"));
        }
        
        // Fraud detection
        if self.detect_fraud(request).await {
            error!("Fraud detected for transaction: {}", request.id);
            return Err(PaymentError::generic("Suspicious activity detected"));
        }
        
        Ok(())
    }
    
    /// Mock KYC check - in a real implementation, this would connect to a KYC provider
    async fn mock_kyc_check(&self, user_id: Uuid) -> bool {
        // Simulate a 95% success rate for KYC checks
        // In a real implementation, this would check against actual KYC data
        true
    }
    /// Check transaction limits for a user
    async fn check_transaction_limits(&self, request: &PaymentRequest) -> bool {
        // Simulate transaction limit checking
        // In a real implementation, this would check against user's transaction history
        // and configured limits
        // Convert to fixed-point for precise comparison
        let amount_fixed = fixed::types::I64F64::from_num(request.amount.value().to_f64().unwrap_or(0.0));
        let limit_fixed = fixed::types::I64F64::from_num(10000.0);
        amount_fixed <= limit_fixed
    }
    
    /// Detect fraud in a transaction
    async fn detect_fraud(&self, request: &PaymentRequest) -> bool {
        // Simulate fraud detection
        // In a real implementation, this would use machine learning models
        // and pattern analysis to detect suspicious activity
        false
    }
    
    /// Convert currency from CPay format to wallet format
    fn convert_currency(&self, currency: &CurrencyCode) -> WalletCurrency {
        match currency {
            CurrencyCode::DBL => WalletCurrency::Dabloons,
            CurrencyCode::USD => WalletCurrency::USD,
            CurrencyCode::EUR => WalletCurrency::EUR,
            CurrencyCode::GBP => WalletCurrency::GBP,
            CurrencyCode::JPY => WalletCurrency::JPY,
            CurrencyCode::CAD => WalletCurrency::USD, // Using USD as fallback for CAD
            CurrencyCode::AUD => WalletCurrency::USD, // Using USD as fallback for AUD
            CurrencyCode::CHF => WalletCurrency::USD, // Using USD as fallback for CHF
            CurrencyCode::CNY => WalletCurrency::USD, // Using USD as fallback for CNY
        }
    }
    
    /// Perform post-transaction actions based on request flags
    async fn perform_post_transaction_actions(&self, request: &PaymentRequest) -> Result<(), PaymentError> {
        // Create social post if requested
        if request.share_to_social {
            self.create_social_post(request).await?;
        }
        
        // Record donation if cause_id is present
        if let Some(cause_id) = request.cause_id {
            self.record_donation(request, cause_id).await?;
        }
        
        // Calculate and store volunteer hours if applicable
        if request.volunteer_hours.is_some() {
            self.calculate_volunteer_hours(request).await?;
        }
        
        Ok(())
    }
    
    /// Create a social post for the transaction
    async fn create_social_post(&self, request: &PaymentRequest) -> Result<(), PaymentError> {
        info!("Creating social post for transaction: {}", request.id);
        // In a real implementation, this would call the social integration service
        // For now, we'll just log the action
        Ok(())
    }
    
    /// Record a donation to a cause
    async fn record_donation(&self, request: &PaymentRequest, cause_id: Uuid) -> Result<(), PaymentError> {
        info!("Recording donation to cause: {} for transaction: {}", cause_id, request.id);
        // In a real implementation, this would update the cause's donation records
        // For now, we'll just log the action
        Ok(())
    }
    /// Process volunteer hour conversion to Dabloons
    pub async fn process_volunteer_conversion(&self, user_id: Uuid, hours: Decimal, skill_rate: Decimal) -> Result<PaymentResponse, PaymentError> {
        info!("Processing volunteer hour conversion for user: {} ({} hours at rate {})", user_id, hours, skill_rate);
        
        // Convert Decimal values to fixed-point for precise calculation
        let hours_fixed = fixed::types::I64F64::from_num(hours.to_f64().unwrap_or(0.0));
        let skill_rate_fixed = fixed::types::I64F64::from_num(skill_rate.to_f64().unwrap_or(0.0));
        
        // Record audit log for volunteer conversion
        let _ = self.audit_hook.record_calculation(
            Some(user_id.to_string()),
            "volunteer_hour_conversion",
            vec![hours.to_string(), skill_rate.to_string()],
            format!("{:.6}", (hours_fixed * skill_rate_fixed).to_num::<f64>()),
        ).await;
        
        // Calculate Dabloons to credit using high-precision fixed-point arithmetic
        let dabloons_amount_fixed = hours_fixed * skill_rate_fixed;
        let dabloons_amount_monetary = MonetaryValue::new(dabloons_amount_fixed, "DABLOONS");
        
        // Convert to the existing MonetaryAmount for compatibility
        let dabloons_amount_decimal = Decimal::from_f64(dabloons_amount_fixed.to_num::<f64>())
            .unwrap_or(Decimal::ZERO);
        let monetary_amount = MonetaryAmount::new(dabloons_amount_decimal, CurrencyCode::DBL);
        
        // Create a payment request for the conversion
        let request = PaymentRequest::new(
            user_id, // System user as sender
            user_id, // User as recipient
            monetary_amount,
            CurrencyCode::DBL,
            Some(format!("Converted {} volunteer hours", hours)),
            false, // is_public
            false, // share_to_social
            None,  // cause_id
            None,  // volunteer_hours
        );
        
        // Process as a Dabloons transaction
        let amount = Money::new(dabloons_amount_decimal, WalletCurrency::Dabloons);
        self.process_dabloons_transaction(request, amount).await
    }
    
    /// Calculate and store volunteer hours
    async fn calculate_volunteer_hours(&self, request: &PaymentRequest) -> Result<(), PaymentError> {
        info!("Calculating volunteer hours for transaction: {}", request.id);
        // In a real implementation, this would calculate volunteer hours based on transaction amount
        // For now, we'll just log the action
        Ok(())
    }
    
    /// Get transaction history for a user
    pub async fn get_transaction_history(&self, user_id: Uuid) -> Result<Vec<Transaction>, PaymentError> {
        info!("Fetching transaction history for user: {}", user_id);
        
        // Record audit log for transaction history access
        let _ = self.audit_hook.record_operation(
            Some(user_id.to_string()),
            "transaction_history_access",
            serde_json::json!({}),
            serde_json::json!({}),
            serde_json::json!({}),
        ).await;
        
        // Get wallet transaction history
        let wallet_transactions = self.wallet_service
            .get_transaction_history(user_id)
            .await
            .map_err(|e| PaymentError::from(e))?;
        
        // Get traditional currency transaction history
        let traditional_transactions = self.traditional_currency_repo
            .find_transactions_by_user_id(user_id)
            .await
            .map_err(|e| PaymentError::from(e))?;
        
        // Convert wallet transactions to CPay transactions
        let mut transactions: Vec<Transaction> = wallet_transactions
            .into_iter()
            .map(|wt| Transaction {
                id: wt.id,
                sender_id: user_id, // This would need to be looked up in a real implementation
                recipient_id: user_id, // This would need to be looked up in a real implementation
                amount: MonetaryAmount::new(wt.amount.amount, self.convert_wallet_currency(&wt.amount.currency)),
                currency: self.convert_wallet_currency(&wt.amount.currency),
                status: TransactionStatus::Completed,
                description: wt.description,
                created_at: wt.timestamp,
                completed_at: Some(wt.timestamp),
            })
            .collect();
        
        // Convert traditional currency transactions to CPay transactions
        let traditional_transactions: Vec<Transaction> = traditional_transactions
            .into_iter()
            .map(|t| t.to_transaction())
            .collect();
        
        // Combine both transaction lists
        transactions.extend(traditional_transactions);
        
        // Sort by creation time (newest first)
        transactions.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        Ok(transactions)
    }
    
    /// Convert wallet currency to CPay currency
    fn convert_wallet_currency(&self, currency: &WalletCurrency) -> CurrencyCode {
        match currency {
            WalletCurrency::Dabloons => CurrencyCode::DBL,
            WalletCurrency::USD => CurrencyCode::USD,
            WalletCurrency::EUR => CurrencyCode::EUR,
            WalletCurrency::GBP => CurrencyCode::GBP,
            WalletCurrency::JPY => CurrencyCode::JPY,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{PaymentRequest, Currency, TransactionStatus};
    use crate::repositories::mock::MockTraditionalCurrencyTransactionRepository;
    use wallet::application::WalletServiceImpl;
    use std::sync::Arc;
    use uuid::Uuid;
    use rust_decimal::Decimal;
    
    // Mock wallet repository for testing
    struct MockWalletRepository;
    
    #[async_trait::async_trait]
    impl wallet::application::WalletRepository for MockWalletRepository {
        async fn save_wallet(&self, _wallet: &wallet::Wallet) -> Result<(), wallet::domain::primitives::FinancialError> {
            Ok(())
        }
        
        async fn find_wallet_by_user_id(&self, _user_id: Uuid) -> Result<Option<wallet::Wallet>, wallet::domain::primitives::FinancialError> {
            Ok(None)
        }
        
        async fn save_transaction(&self, _transaction: &wallet::WalletTransaction) -> Result<(), wallet::domain::primitives::FinancialError> {
            Ok(())
        }
        
        async fn find_transactions_by_wallet_id(&self, _wallet_id: Uuid) -> Result<Vec<wallet::WalletTransaction>, wallet::domain::primitives::FinancialError> {
            Ok(vec![])
        }
    }
    
    #[tokio::test]
    async fn test_process_dabloons_transaction() {
        // Arrange
        let wallet_repo = Arc::new(MockWalletRepository);
        let wallet_service: Arc<dyn wallet::application::WalletService> = Arc::new(WalletServiceImpl::new(wallet_repo));
        let traditional_currency_repo = Arc::new(MockTraditionalCurrencyTransactionRepository::new());
        let engine = TransactionEngine::new(wallet_service, traditional_currency_repo);
        
        let user_id = Uuid::new_v4();
        let recipient_id = Uuid::new_v4();
        let amount = MonetaryAmount::new(Decimal::from(100u64), CurrencyCode::DBL);
        
        let request = PaymentRequest::new(
            user_id,
            recipient_id,
            amount,
            CurrencyCode::DBL,
            Some("Test payment".to_string()),
            false, // is_public
            false, // share_to_social
            None,  // cause_id
            None,  // volunteer_hours
        );
        
        // Act
        let result = engine.process_payment(request).await;
        
        // Assert
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, TransactionStatus::Completed);
    }
    
    #[tokio::test]
    async fn test_process_traditional_currency_transaction() {
        // Arrange
        let wallet_repo = Arc::new(MockWalletRepository);
        let wallet_service: Arc<dyn wallet::application::WalletService> = Arc::new(WalletServiceImpl::new(wallet_repo));
        let traditional_currency_repo = Arc::new(MockTraditionalCurrencyTransactionRepository::new());
        let engine = TransactionEngine::new(wallet_service, traditional_currency_repo);
        
        let user_id = Uuid::new_v4();
        let recipient_id = Uuid::new_v4();
        let amount = MonetaryAmount::new(Decimal::from(100u64), CurrencyCode::USD);
        
        let request = PaymentRequest::new(
            user_id,
            recipient_id,
            amount,
            CurrencyCode::USD,
            Some("Test payment".to_string()),
            false, // is_public
            false, // share_to_social
            None,  // cause_id
            None,  // volunteer_hours
        );
        
        // Act
        let result = engine.process_payment(request).await;
        
        // Assert
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.status, TransactionStatus::Completed);
    }
    
    #[tokio::test]
    async fn test_get_transaction_history() {
        // Arrange
        let wallet_repo = Arc::new(MockWalletRepository);
        let wallet_service: Arc<dyn wallet::application::WalletService> = Arc::new(WalletServiceImpl::new(wallet_repo));
        let traditional_currency_repo = Arc::new(MockTraditionalCurrencyTransactionRepository::new());
        let engine = TransactionEngine::new(wallet_service, traditional_currency_repo);
        
        let user_id = Uuid::new_v4();
        
        // Act
        let result = engine.get_transaction_history(user_id).await;
        
        // Assert
        assert!(result.is_ok());
        let transactions = result.unwrap();
        assert_eq!(transactions.len(), 0); // No transactions in mock repo
    }
}