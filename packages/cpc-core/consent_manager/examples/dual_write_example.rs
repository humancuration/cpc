//! Example of dual-write implementation for consent management during migration
//!
//! This example shows how applications can implement a dual-write strategy
//! during the migration from legacy consent systems to the new centralized
//! Consent Manager.

use consent_manager::{
    domain::{
        consent::{DataSharingLevel, Domain},
        audit::Actor,
    },
    application::service::ConsentService,
};
use std::sync::Arc;

/// Wrapper service that handles dual-write during migration
pub struct DualWriteConsentService {
    /// The new centralized consent service
    new_service: Arc<ConsentService>,
    /// The legacy consent service (if still needed during migration)
    #[cfg(feature = "migration-mode")]
    legacy_service: Option<Arc<dyn LegacyConsentService>>,
}

impl DualWriteConsentService {
    /// Create a new dual-write consent service
    pub fn new(
        new_service: Arc<ConsentService>,
        #[cfg(feature = "migration-mode")] legacy_service: Option<Arc<dyn LegacyConsentService>>,
    ) -> Self {
        Self {
            new_service,
            #[cfg(feature = "migration-mode")]
            legacy_service,
        }
    }

    /// Get the consent level for a user and domain
    ///
    /// This method tries to get the consent level from the new service first,
    /// and falls back to the legacy service if configured and the new service
    /// is unavailable.
    pub async fn get_consent_level(&self, user_id: &str, domain: Domain) -> Result<DataSharingLevel, Box<dyn std::error::Error>> {
        // Try the new service first
        match self.new_service.get_consent_level(user_id, domain.clone()).await {
            Ok(level) => Ok(level),
            Err(e) => {
                // If the new service fails, try the legacy service if available
                #[cfg(feature = "migration-mode")]
                {
                    if let Some(legacy_service) = &self.legacy_service {
                        return legacy_service.get_consent_level(user_id, domain).await;
                    }
                }
                
                // If no legacy service or it also fails, return the error
                Err(Box::new(e) as Box<dyn std::error::Error>)
            }
        }
    }

    /// Update the consent level for a user and domain
    ///
    /// This method writes to both the new and legacy services (if configured)
    /// to ensure data consistency during migration.
    pub async fn update_consent_level(
        &self,
        user_id: &str,
        domain: Domain,
        level: DataSharingLevel,
        actor: Actor,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Write to the new service
        self.new_service.update_consent_level(user_id, domain.clone(), level.clone(), actor.clone()).await?;
        
        // Also write to the legacy service during migration
        #[cfg(feature = "migration-mode")]
        {
            if let Some(legacy_service) = &self.legacy_service {
                if let Err(e) = legacy_service.update_consent_level(user_id, domain, level, actor).await {
                    // Log the error but don't fail the operation
                    eprintln!("Warning: Failed to update legacy consent service: {:?}", e);
                }
            }
        }
        
        Ok(())
    }

    /// Revoke consent for a specific domain
    ///
    /// This method revokes consent in both the new and legacy services (if configured)
    /// to ensure data consistency during migration.
    pub async fn revoke_domain(
        &self,
        user_id: &str,
        domain: Domain,
        actor: Actor,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Revoke in the new service
        self.new_service.revoke_domain(user_id, domain.clone(), actor.clone()).await?;
        
        // Also revoke in the legacy service during migration
        #[cfg(feature = "migration-mode")]
        {
            if let Some(legacy_service) = &self.legacy_service {
                if let Err(e) = legacy_service.revoke_domain(user_id, domain, actor).await {
                    // Log the error but don't fail the operation
                    eprintln!("Warning: Failed to revoke consent in legacy service: {:?}", e);
                }
            }
        }
        
        Ok(())
    }
}

/// Trait for legacy consent services (for migration purposes)
#[cfg(feature = "migration-mode")]
#[async_trait::async_trait]
pub trait LegacyConsentService: Send + Sync {
    /// Get the consent level for a user and domain
    async fn get_consent_level(&self, user_id: &str, domain: Domain) -> Result<DataSharingLevel, Box<dyn std::error::Error>>;
    
    /// Update the consent level for a user and domain
    async fn update_consent_level(
        &self,
        user_id: &str,
        domain: Domain,
        level: DataSharingLevel,
        actor: Actor,
    ) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Revoke consent for a specific domain
    async fn revoke_domain(
        &self,
        user_id: &str,
        domain: Domain,
        actor: Actor,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use consent_manager::domain::consent::Domain;
    use consent_manager::domain::audit::Actor;
    use std::collections::HashMap;
    use tokio;

    // Mock implementation of LegacyConsentService for testing
    #[cfg(feature = "migration-mode")]
    struct MockLegacyConsentService {
        consent_levels: std::sync::Mutex<HashMap<String, DataSharingLevel>>,
    }

    #[cfg(feature = "migration-mode")]
    #[async_trait::async_trait]
    impl LegacyConsentService for MockLegacyConsentService {
        async fn get_consent_level(&self, user_id: &str, _domain: Domain) -> Result<DataSharingLevel, Box<dyn std::error::Error>> {
            let levels = self.consent_levels.lock().unwrap();
            Ok(levels.get(user_id).cloned().unwrap_or(DataSharingLevel::None))
        }

        async fn update_consent_level(
            &self,
            user_id: &str,
            _domain: Domain,
            level: DataSharingLevel,
            _actor: Actor,
        ) -> Result<(), Box<dyn std::error::Error>> {
            let mut levels = self.consent_levels.lock().unwrap();
            levels.insert(user_id.to_string(), level);
            Ok(())
        }

        async fn revoke_domain(
            &self,
            user_id: &str,
            _domain: Domain,
            _actor: Actor,
        ) -> Result<(), Box<dyn std::error::Error>> {
            let mut levels = self.consent_levels.lock().unwrap();
            levels.insert(user_id.to_string(), DataSharingLevel::None);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_dual_write_consent_service() {
        // Create a mock storage for the new consent service
        struct MockStorage;
        
        #[async_trait::async_trait]
        impl consent_manager::application::service::ConsentStorage for MockStorage {
            async fn get_consent_profile(
                &self,
                _user_id: &str,
                _domain: &Domain,
            ) -> Result<Option<consent_manager::domain::consent::ConsentProfile>, consent_manager::domain::errors::ConsentError> {
                Ok(None)
            }

            async fn save_consent_profile(
                &self,
                _profile: &consent_manager::domain::consent::ConsentProfile,
            ) -> Result<(), consent_manager::domain::errors::ConsentError> {
                Ok(())
            }

            async fn revoke_domain(
                &self,
                _user_id: &str,
                _domain: &Domain,
            ) -> Result<(), consent_manager::domain::errors::ConsentError> {
                Ok(())
            }

            async fn get_audit_events(
                &self,
                _user_id: &str,
            ) -> Result<Vec<consent_manager::domain::audit::AuditEvent>, consent_manager::domain::errors::ConsentError> {
                Ok(vec![])
            }

            async fn save_audit_event(
                &self,
                _event: &consent_manager::domain::audit::AuditEvent,
            ) -> Result<(), consent_manager::domain::errors::ConsentError> {
                Ok(())
            }
        }

        // Create the new consent service
        let storage = Box::new(MockStorage);
        let new_service = Arc::new(ConsentService::new(storage));

        // Create the dual-write service
        let dual_write_service = DualWriteConsentService::new(
            new_service,
            #[cfg(feature = "migration-mode")]
            None,
        );

        // Test updating consent level
        let user_id = "test_user";
        let domain = Domain::FinancialData;
        let level = DataSharingLevel::Standard;
        let actor = Actor::User(user_id.to_string());

        let result = dual_write_service.update_consent_level(user_id, domain.clone(), level.clone(), actor).await;
        assert!(result.is_ok());

        // Test getting consent level
        let result = dual_write_service.get_consent_level(user_id, domain).await;
        assert!(result.is_ok());
    }
}