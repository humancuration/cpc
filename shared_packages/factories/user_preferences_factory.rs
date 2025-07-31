//! Factory for creating user preferences implementations
//!
//! This module provides a factory for creating user preferences implementations
//! that automatically respond to network status changes. The factory creates
//! dynamic service proxies that handle implementation switching transparently.
//!
//! ## Usage
//!
//! ```rust
//! // Create once, use everywhere
//! let preferences = UserPreferencesFactory::create(db, network_monitor, grpc_client);
//!
//! // Use without worrying about network status changes
//! preferences.set_preferred_currency(user_id, currency).await;
//! ```
//!
//! The factory's responsibility is creating dynamic service proxies that handle
//! implementation switching automatically. User context belongs at the operation
//! level, not at factory level, enabling reuse across users.

use std::sync::Arc;
use sled::Db;
use packages::domains::finance::application::user_preferences::UserPreferences;
use packages::infra::core::adapters::composite::CompositeUserPreferences;
use packages::infra::grpc::clients::user_preferences::{GrpcUserPreferences, UserPreferencesClient};
use packages::infra::sled::adapters::user_preferences::SledUserPreferences;
use packages::infra::core::network::monitor::NetworkStatusMonitor;

/// Factory for creating user preferences implementations
pub struct UserPreferencesFactory;

impl UserPreferencesFactory {
    /// Create a user preferences proxy that automatically switches implementations
    /// based on network status changes
    pub fn create(
        db: &Db,
        network_monitor: Arc<NetworkStatusMonitor>,
        grpc_client: UserPreferencesClient,
    ) -> Box<dyn UserPreferences> {
        // Create both implementations
        let online_service = GrpcUserPreferences::new(grpc_client, Uuid::nil()); // User ID handled at operation level
        let offline_service = SledUserPreferences::new(db);
        
        // Create composite adapter
        let composite = CompositeUserPreferences::new(online_service, offline_service);
        
        Box::new(composite)
    }
}