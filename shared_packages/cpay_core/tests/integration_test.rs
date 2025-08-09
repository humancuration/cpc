//! Integration tests for CPay Core

use cpay_core::{
    CpayService,
    CpayServiceImpl,
    transaction_engine::TransactionEngine,
    repositories::mock::MockTraditionalCurrencyTransactionRepository,
    models::{PaymentRequest, CurrencyCode, TransactionStatus}
};
use cpc_financial_core::audit::FinancialAuditHook;
use cpc_wallet::application::{WalletService, WalletServiceImpl};
use std::sync::Arc;
use uuid::Uuid;
use rust_decimal::Decimal;

// Mock implementations for testing
struct MockWalletRepository;
struct MockNotificationService;
struct MockSocialService;

#[async_trait::async_trait]
impl cpc_wallet::application::WalletRepository for MockWalletRepository {
    async fn save_wallet(&self, _wallet: &cpc_wallet::Wallet) -> Result<(), cpc_wallet::domain::primitives::FinancialError> {
        Ok(())
    }
    
    async fn find_wallet_by_user_id(&self, _user_id: Uuid) -> Result<Option<cpc_wallet::Wallet>, cpc_wallet::domain::primitives::FinancialError> {
        Ok(None)
    }
    
    async fn save_transaction(&self, _transaction: &cpc_wallet::WalletTransaction) -> Result<(), cpc_wallet::domain::primitives::FinancialError> {
        Ok(())
    }
    
    async fn find_transactions_by_wallet_id(&self, _wallet_id: Uuid) -> Result<Vec<cpc_wallet::WalletTransaction>, cpc_wallet::domain::primitives::FinancialError> {
        Ok(vec![])
    }
}
#[async_trait::async_trait]
impl notification_core::application::service::NotificationService for MockNotificationService {
    async fn send_notification(&self, _notification: notification_core::Notification) -> Result<(), notification_core::NotificationError> {
        Ok(())
    }
    
    async fn get_user_preferences(&self, _user_id: &str) -> Result<notification_core::UserPreferences, notification_core::NotificationError> {
        Ok(notification_core::UserPreferences::default())
    }
    
    async fn update_user_preferences(&self, _user_id: &str, _preferences: notification_core::UserPreferences) -> Result<(), notification_core::NotificationError> {
        Ok(())
    }
}

#[async_trait::async_trait]
impl social_integration::application::social_integration_service::SocialIntegrationService for MockSocialService {
    async fn create_post(&self, _post: social_integration::domain::post::UnifiedPost) -> Result<(), social_integration::domain::social_event::SocialEventError> {
        Ok(())
    }
    
    async fn get_user_feed(&self, _user_id: &str, _limit: i64) -> Result<Vec<social_integration::domain::post::UnifiedPost>, social_integration::domain::social_event::SocialEventError> {
        Ok(vec![])
    }
    
    async fn follow_user(&self, _follower_id: &str, _followed_id: &str) -> Result<(), social_integration::domain::social_event::SocialEventError> {
        Ok(())
    }
    
    async fn unfollow_user(&self, _follower_id: &str, _followed_id: &str) -> Result<(), social_integration::domain::social_event::SocialEventError> {
        Ok(())
    }
}

// Mock audit service for testing
struct MockAuditService;

#[async_trait::async_trait]
impl audit_framework::application::service::AuditService for MockAuditService {
    async fn record_event(&self, _event: audit_framework::domain::event::AuditEvent) -> Result<(), audit_framework::AuditError> {
        // In a real implementation, this would record the event
        // For testing, we'll just return Ok
        Ok(())
    }
    
    async fn get_events_by_user(&self, _user_id: &str, _limit: Option<u32>) -> Result<Vec<audit_framework::domain::event::AuditEvent>, audit_framework::AuditError> {
        Ok(vec![])
    }
    
    async fn get_events_by_domain(&self, _domain: &str, _limit: Option<u32>) -> Result<Vec<audit_framework::domain::event::AuditEvent>, audit_framework::AuditError> {
        Ok(vec![])
    }
}

fn create_mock_audit_hook() -> FinancialAuditHook {
    let audit_service: Arc<dyn audit_framework::application::service::AuditService> = Arc::new(MockAuditService);
    FinancialAuditHook::new(audit_service)
}

#[tokio::test]
async fn test_cpay_service_creation() {
    // Arrange
    let wallet_repo = Arc::new(MockWalletRepository);
    let wallet_service: Arc<dyn WalletService> = Arc::new(WalletServiceImpl::new(wallet_repo));
    
    let traditional_currency_repo = Arc::new(MockTraditionalCurrencyTransactionRepository::new());
    // Create a mock audit hook for testing
    let audit_hook = Arc::new(create_mock_audit_hook());
    let transaction_engine = Arc::new(TransactionEngine::new(wallet_service, traditional_currency_repo, audit_hook));
    
    let notification_service: Arc<dyn notification_core::application::service::NotificationService> = Arc::new(MockNotificationService);
    let social_service: Arc<dyn social_integration::application::social_integration_service::SocialIntegrationService> = Arc::new(MockSocialService);
    
    // Act
    let cpay_service = CpayServiceImpl::new(
        notification_service,
        social_service,
        transaction_engine,
    );
    
    // Assert
    // If we get here without panicking, the service was created successfully
    assert!(true);
}

#[tokio::test]
async fn test_payment_request_creation() {
    // Arrange
    let user_id = Uuid::new_v4();
    let recipient_id = Uuid::new_v4();
    let amount = cpc_financial_core::MonetaryAmount::new(Decimal::from(100u64), CurrencyCode::USD);
    
    // Act
    let request = PaymentRequest::new(
        user_id,
        recipient_id,
        amount.clone(),
        CurrencyCode::USD,
        Some("Test payment".to_string()),
        false, // is_public
        false, // share_to_social
        None,  // cause_id
        None,  // volunteer_hours
    );
    
    // Assert
    assert_eq!(request.user_id, user_id);
    assert_eq!(request.recipient_id, recipient_id);
    assert_eq!(request.amount, amount);
    assert_eq!(request.currency, CurrencyCode::USD);
    assert_eq!(request.description, Some("Test payment".to_string()));
}