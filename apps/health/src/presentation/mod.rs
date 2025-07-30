//! Presentation layer for the health module
//!
//! This module contains UI components for both Bevy (3D) and Yew (web) interfaces.

pub mod bevy;
pub mod yew;

/// Module initialization structure
pub struct HealthModule {
    pub query: HealthQuery,
    pub mutation: HealthMutation,
    pub subscription: HealthSubscription,
    pub router: axum::Router,
}

/// GraphQL query root for health module
pub struct HealthQuery;

/// GraphQL mutation root for health module
pub struct HealthMutation;

/// GraphQL subscription root for health module
pub struct HealthSubscription;

impl HealthModule {
    /// Initialize the health module
    pub fn initialize(db_pool: sqlx::PgPool) -> Self {
        // In a real implementation, this would:
        // 1. Initialize repositories with the database pool
        // 2. Initialize services with repositories
        // 3. Set up GraphQL schema components
        // 4. Create API routes
        
        Self {
            query: HealthQuery,
            mutation: HealthMutation,
            subscription: HealthSubscription,
            router: axum::Router::new(),
        }
    }
}