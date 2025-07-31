//! Composite adapter for user preferences with network-aware switching
//!
//! This module implements the composite adapter pattern for runtime selection
//! between online/offline implementations of UserPreferences. It automatically
//! switches between implementations based on network status changes.
//!
//! ## Design Philosophy
//!
//! The composite adapter prioritizes network responsiveness over strict consistency:
//! - When online, operations are attempted against the backend first
//! - On failure, operations fall back to local storage
//! - When offline, operations go directly to local storage
//! - Network status changes automatically trigger implementation switching
//!
//! This approach ensures user actions aren't lost during connectivity issues
//! while providing immediate feedback when possible.

use async_trait::async_trait;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use packages::domains::finance::domain::primitives::Currency;
use packages::domains::finance::application::user_preferences::UserPreferences;
use packages::infra::grpc::clients::user_preferences::GrpcUserPreferences;
use packages::infra::sled::adapters::user_preferences::SledUserPreferences;
use packages::infra::core::network::monitor::{NetworkStatusObserver, NetworkStatusMonitor};
use thiserror::Error;

/// Error types for user preferences operations
#[derive(Error, Debug)]
pub enum PreferencesError {
    #[error("Online service failure while setting currency: {0}")]
    OnlineFailure(String),
    
    #[error("Offline storage failure while setting currency: {0}")]
    OfflineFailure(String),
    
    #[error("Both services failed - online: {online}, offline: {offline}")]
    DualFailure {
        online: String,
        offline: String,
    },
}

/// Composite implementation that can switch between online and offline modes
#[derive(Clone)]
pub struct CompositeUserPreferences {
    /// Current implementation (online or offline)
    current_impl: Arc<RwLock<UserPreferencesImpl>>,
    
    /// Online implementation
    online_service: GrpcUserPreferences,
    
    /// Offline implementation
    offline_service: SledUserPreferences,
}

/// Implementation variants
#[derive(Clone)]
enum UserPreferencesImpl {
    Online(GrpcUserPreferences),
    Offline(SledUserPreferences),
}

/// Implementation variants
enum UserPreferencesImpl {
    Online(GrpcUserPreferences),
    Offline(SledUserPreferences),
}

impl CompositeUserPreferences {
    /// Create a new composite user preferences adapter
    pub fn new(
        online_service: GrpcUserPreferences,
        offline_service: SledUserPreferences
    ) -> Self {
        Self {
            current_impl: Arc::new(RwLock::new(UserPreferencesImpl::Online(online_service.clone()))),
            online_service,
            offline_service,
        }
    }
    
    /// Switch implementation based on network status
    fn switch_implementation(&self, is_connected: bool) {
        let mut current_impl = self.current_impl.write().unwrap();
        *current_impl = if is_connected {
            UserPreferencesImpl::Online(self.online_service.clone())
        } else {
            UserPreferencesImpl::Offline(self.offline_service.clone())
        };
    }
    
    /// Create a new online service with the specified user ID
    fn online_service_with_user(&self, user_id: Uuid) -> GrpcUserPreferences {
        GrpcUserPreferences::new(self.online_service.client.clone(), user_id)
    }
}

#[async_trait]
impl UserPreferences for CompositeUserPreferences {
    async fn get_preferred_currency(&self, user_id: Uuid) -> Result<Currency, String> {
        let current_impl = self.current_impl.read().unwrap();
        match &*current_impl {
            UserPreferencesImpl::Online(_) => {
                let service = self.online_service_with_user(user_id);
                service.get_preferred_currency(user_id).await
                .map_err(|e| PreferencesError::OnlineFailure(e).to_string())
            },
            UserPreferencesImpl::Offline(service) =>
                service.get_preferred_currency(user_id).await
                .map_err(|e| PreferencesError::OfflineFailure(e).to_string()),
        }
    }
    
    async fn set_preferred_currency(
        &self,
        user_id: Uuid,
        currency: Currency
    ) -> Result<(), String> {
        let current_impl = self.current_impl.read().unwrap();
        match &*current_impl {
            UserPreferencesImpl::Online(_) => {
                // Try online service first
                let service = self.online_service_with_user(user_id);
                match service.set_preferred_currency(user_id, currency.clone()).await {
                    Ok(_) => Ok(()),
                    Err(online_error) => {
                        // Fallback to offline storage
                        match self.offline_service.set_preferred_currency(user_id, currency).await {
                            Ok(_) => Ok(()),
                            Err(offline_error) => {
                                Err(PreferencesError::DualFailure {
                                    online: online_error,
                                    offline: offline_error,
                                }.to_string())
                            }
                        }
                    }
                }
            }
            UserPreferencesImpl::Offline(service) => {
                service.set_preferred_currency(user_id, currency).await
                .map_err(|e| PreferencesError::OfflineFailure(e).to_string())
            }
        }
    }
}

impl NetworkStatusObserver for CompositeUserPreferences {
    /// Called when network status changes
    fn on_network_status_changed(&self, is_connected: bool) {
        self.switch_implementation(is_connected);
    }
}