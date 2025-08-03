//! Shared test utilities for integration tests

use uuid::Uuid;
use async_graphql::{Context, EmptyMutation, EmptySubscription, Schema};
use std::sync::Arc;

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