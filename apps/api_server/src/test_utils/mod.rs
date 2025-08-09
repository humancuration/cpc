//! Shared test utilities for integration tests
//!
//! ## Related Documentation
//! - ADR-0006: Concurrency Handling and Test Coverage Enhancement (`docs/ADR-0006.md`)

use uuid::Uuid;
use async_graphql::{Context, EmptyMutation, EmptySubscription, Schema};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Test user structure for testing
pub struct TestUser {
    pub id: Uuid,
}

impl TestUser {
    /// Create a new test user with a random ID
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
        }
    }
    
    /// Create a new test user with a specific ID
    pub fn with_id(id: Uuid) -> Self {
        Self { id }
    }
}

/// Create a test context with a user ID
pub fn create_test_context(user_id: Option<Uuid>) -> Context<'static> {
    let user_id = user_id.unwrap_or_else(Uuid::new_v4);
    
    // Create a simple schema to get a context
    let schema = Schema::build(EmptyMutation, EmptySubscription, EmptySubscription)
        .data(user_id)
        .finish();
    
    // Create a dummy request to get a context
    let req = async_graphql::Request::new("{ __typename }");
    let res = schema.execute(req).await;
    
    // Extract context from the result (this is a workaround since we can't directly create a context)
    // In real tests, we would insert our services into the context as well
    let ctx = Context::new();
    ctx.insert(user_id);
    
    ctx
}

/// Helper function to create a test context with specific services
pub fn create_test_context_with_services<T>(user_id: Option<Uuid>, service: T) -> Context<'static>
where
    T: 'static + Clone,
{
    let user_id = user_id.unwrap_or_else(Uuid::new_v4);
    let ctx = Context::new();
    ctx.insert(user_id);
    ctx.insert(Arc::new(service) as Arc<dyn std::any::Any + Send + Sync>);
    
    ctx
}

/// Mock volunteer service for testing
pub struct MockVolunteerService;

impl MockVolunteerService {
    pub fn new() -> Self {
        Self
    }
}

/// Mock skill exchange service for testing
pub struct MockSkillExchangeService;

impl MockSkillExchangeService {
    pub fn new() -> Self {
        Self
    }
}

/// Mock wallet service for testing
pub struct MockWalletService;

impl MockWalletService {
    pub fn new() -> Self {
        Self
    }
}

/// Mock notification service for testing
pub struct MockNotificationService;

impl MockNotificationService {
    pub fn new() -> Self {
        Self
    }
}

/// Mock social integration service for testing
pub struct MockSocialIntegrationService;

impl MockSocialIntegrationService {
    pub fn new() -> Self {
        Self
    }
}

/// Mock achievement service for testing
pub struct MockAchievementService;

impl MockAchievementService {
    pub fn new() -> Self {
        Self
    }
}

/// TransactionManager trait for concurrency control
#[async_trait::async_trait]
pub trait TransactionManager: Send + Sync {
    async fn begin_transaction(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn commit_transaction(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn rollback_transaction(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

/// Mock implementation of TransactionManager for testing
pub struct MockTransactionManager;

#[async_trait::async_trait]
impl TransactionManager for MockTransactionManager {
    async fn begin_transaction(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
    
    async fn commit_transaction(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
    
    async fn rollback_transaction(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
}

/// EventBus mock for cross-service testing
pub struct MockEventBus {
    events: Arc<Mutex<Vec<String>>>,
}

impl MockEventBus {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    pub async fn get_events(&self) -> Vec<String> {
        let events = self.events.lock().await;
        events.clone()
    }
}

#[async_trait::async_trait]
impl EventBus for MockEventBus {
    async fn publish(&self, event: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut events = self.events.lock().await;
        events.push(event.to_string());
        Ok(())
    }
}

/// EventBus trait for cross-service communication
#[async_trait::async_trait]
pub trait EventBus: Send + Sync {
    async fn publish(&self, event: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

/// Currency validation helper
pub struct CurrencyValidator;

impl CurrencyValidator {
    pub fn validate_currency(currency: &cpc_wallet::domain::primitives::Currency) -> bool {
        match currency {
            cpc_wallet::domain::primitives::Currency::Dabloons => true,
            cpc_wallet::domain::primitives::Currency::USD => true,
            cpc_wallet::domain::primitives::Currency::EUR => true,
            _ => false,
        }
    }
    
    pub fn is_supported_currency(currency: &cpc_wallet::domain::primitives::Currency) -> bool {
        // Only Dabloons are supported for conversions in the CPC ecosystem
        matches!(currency, cpc_wallet::domain::primitives::Currency::Dabloons)
    }
}

/// LargeDatasetSeeder utility for performance tests
pub struct LargeDatasetSeeder;

impl LargeDatasetSeeder {
    pub async fn seed_volunteer_activities(
        count: usize,
        user_id: Uuid,
    ) -> Result<Vec<volunteer_core::models::VolunteerActivity>, Box<dyn std::error::Error + Send + Sync>> {
        let mut activities = Vec::with_capacity(count);
        
        for i in 0..count {
            let activity = volunteer_core::models::VolunteerActivity::new(
                user_id,
                None,
                format!("Test activity {}", i),
                rust_decimal::Decimal::from(2),
            );
            activities.push(activity);
        }
        
        Ok(activities)
    }
    
    pub async fn seed_skill_listings(
        count: usize,
        provider_id: Uuid,
    ) -> Result<Vec<skill_exchange_core::models::SkillListing>, Box<dyn std::error::Error + Send + Sync>> {
        let mut listings = Vec::with_capacity(count);
        
        for i in 0..count {
            let listing = skill_exchange_core::models::SkillListing::new(
                provider_id,
                format!("Test skill {}", i),
                format!("Test description {}", i),
                "Test category".to_string(),
                Some(rust_decimal::Decimal::from(2)),
            );
            listings.push(listing);
        }
        
        Ok(listings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_currency_validator() {
        // Test supported currency
        let dabloon_currency = cpc_wallet::domain::primitives::Currency::Dabloons;
        assert!(CurrencyValidator::is_supported_currency(&dabloon_currency));
        
        // Test unsupported currency
        let usd_currency = cpc_wallet::domain::primitives::Currency::USD;
        // Dabloons is the only supported currency for conversions
        assert!(CurrencyValidator::is_supported_currency(&usd_currency));
    }
    
    #[tokio::test]
    async fn test_large_dataset_seeder() {
        let user_id = Uuid::new_v4();
        let count = 10;
        
        let result = LargeDatasetSeeder::seed_volunteer_activities(count, user_id).await;
        assert!(result.is_ok());
        let activities = result.unwrap();
        assert_eq!(activities.len(), count);
        
        let provider_id = Uuid::new_v4();
        let result = LargeDatasetSeeder::seed_skill_listings(count, provider_id).await;
        assert!(result.is_ok());
        let listings = result.unwrap();
        assert_eq!(listings.len(), count);
    }
    
    #[tokio::test]
    async fn test_mock_event_bus() {
        let event_bus = MockEventBus::new();
        
        let result = event_bus.publish("test_event").await;
        assert!(result.is_ok());
        
        let events = event_bus.get_events().await;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], "test_event");
    }
    
    #[tokio::test]
    async fn test_transaction_manager() {
        let transaction_manager = MockTransactionManager;
        
        let result = transaction_manager.begin_transaction().await;
        assert!(result.is_ok());
        
        let result = transaction_manager.commit_transaction().await;
        assert!(result.is_ok());
        
        let result = transaction_manager.rollback_transaction().await;
        assert!(result.is_ok());
    }
}