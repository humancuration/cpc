//! Integration tests for the consent manager
//!
//! These tests verify that the consent manager works correctly with all
//! integrated applications and features.

use consent_manager::{
    domain::{
        consent::{DataSharingLevel, Domain, ConsentProfile},
        audit::{AuditEvent, Actor, ConsentAction},
        errors::ConsentError,
    },
    application::service::{ConsentService, ConsentStorage},
    infrastructure::events::bevy::{ConsentEventChannel, ConsentChangeEvent},
};
use std::collections::HashMap;
use tokio::sync::broadcast::error::RecvError;

/// In-memory storage implementation for testing
struct InMemoryStorage {
    profiles: std::sync::Mutex<HashMap<String, ConsentProfile>>,
    audit_events: std::sync::Mutex<HashMap<String, Vec<AuditEvent>>>,
}

impl InMemoryStorage {
    fn new() -> Self {
        Self {
            profiles: std::sync::Mutex::new(HashMap::new()),
            audit_events: std::sync::Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl ConsentStorage for InMemoryStorage {
    async fn get_consent_profile(&self, user_id: &str, domain: &Domain) -> Result<Option<ConsentProfile>, ConsentError> {
        let key = format!("{}:{:?}", user_id, domain);
        let profiles = self.profiles.lock().unwrap();
        Ok(profiles.get(&key).cloned())
    }

    async fn save_consent_profile(&self, profile: &ConsentProfile) -> Result<(), ConsentError> {
        let key = format!("{}:{:?}", profile.user_id, profile.domain);
        let mut profiles = self.profiles.lock().unwrap();
        profiles.insert(key, profile.clone());
        Ok(())
    }

    async fn revoke_domain(&self, user_id: &str, domain: &Domain) -> Result<(), ConsentError> {
        let key = format!("{}:{:?}", user_id, domain);
        let mut profiles = self.profiles.lock().unwrap();
        profiles.remove(&key);
        Ok(())
    }

    async fn get_audit_events(&self, user_id: &str) -> Result<Vec<AuditEvent>, ConsentError> {
        let audit_events = self.audit_events.lock().unwrap();
        Ok(audit_events.get(user_id).cloned().unwrap_or_default())
    }

    async fn save_audit_event(&self, event: &AuditEvent) -> Result<(), ConsentError> {
        let mut audit_events = self.audit_events.lock().unwrap();
        audit_events.entry(event.user_id.clone()).or_insert_with(Vec::new).push(event.clone());
        Ok(())
    }
}

#[tokio::test]
async fn test_consent_manager_integration() {
    // Create the storage backend
    let storage = Box::new(InMemoryStorage::new());
    
    // Create the consent service
    let consent_service = ConsentService::new(storage);
    
    // Test user and domain
    let user_id = "test_user_123";
    let domain = Domain::FinancialData;
    let actor = Actor::User(user_id.to_string());
    
    // Initially, there should be no consent
    let level = consent_service.get_consent_level(user_id, domain.clone()).await.unwrap();
    assert_eq!(level, DataSharingLevel::None);
    
    // Update consent to Standard level
    consent_service.update_consent_level(user_id, domain.clone(), DataSharingLevel::Standard, actor.clone()).await.unwrap();
    
    // Verify the consent was updated
    let level = consent_service.get_consent_level(user_id, domain.clone()).await.unwrap();
    assert_eq!(level, DataSharingLevel::Standard);
    
    // Check that audit events were created
    let audit_events = consent_service.get_audit_events(user_id).await.unwrap();
    assert_eq!(audit_events.len(), 1);
    assert_eq!(audit_events[0].action, ConsentAction::Granted);
    assert_eq!(audit_events[0].new_level, Some(DataSharingLevel::Standard));
    
    // Update consent to Full level
    consent_service.update_consent_level(user_id, domain.clone(), DataSharingLevel::Full, actor.clone()).await.unwrap();
    
    // Verify the consent was updated
    let level = consent_service.get_consent_level(user_id, domain.clone()).await.unwrap();
    assert_eq!(level, DataSharingLevel::Full);
    
    // Check that audit events were created
    let audit_events = consent_service.get_audit_events(user_id).await.unwrap();
    assert_eq!(audit_events.len(), 2);
    assert_eq!(audit_events[1].action, ConsentAction::Modified);
    assert_eq!(audit_events[1].previous_level, Some(DataSharingLevel::Standard));
    assert_eq!(audit_events[1].new_level, Some(DataSharingLevel::Full));
    
    // Revoke consent
    consent_service.revoke_domain(user_id, domain.clone(), actor).await.unwrap();
    
    // Verify the consent was revoked
    let level = consent_service.get_consent_level(user_id, domain).await.unwrap();
    assert_eq!(level, DataSharingLevel::None);
    
    // Check that audit events were created
    let audit_events = consent_service.get_audit_events(user_id).await.unwrap();
    assert_eq!(audit_events.len(), 3);
    assert_eq!(audit_events[2].action, ConsentAction::Revoked);
    assert_eq!(audit_events[2].previous_level, Some(DataSharingLevel::Full));
    assert_eq!(audit_events[2].new_level, Some(DataSharingLevel::None));
}

#[tokio::test]
async fn test_consent_level_permissions() {
    // Create the storage backend
    let storage = Box::new(InMemoryStorage::new());
    
    // Create the consent service
    let consent_service = ConsentService::new(storage);
    
    // Test user and domain
    let user_id = "test_user_456";
    let domain = Domain::CrmData;
    let actor = Actor::User(user_id.to_string());
    
    // Test that None level denies all access
    consent_service.update_consent_level(user_id, domain.clone(), DataSharingLevel::None, actor.clone()).await.unwrap();
    let level = consent_service.get_consent_level(user_id, domain.clone()).await.unwrap();
    assert!(!level.allows(DataSharingLevel::Minimal));
    assert!(!level.allows(DataSharingLevel::Standard));
    assert!(!level.allows(DataSharingLevel::Full));
    
    // Test that Minimal level allows Minimal access but denies higher levels
    consent_service.update_consent_level(user_id, domain.clone(), DataSharingLevel::Minimal, actor.clone()).await.unwrap();
    let level = consent_service.get_consent_level(user_id, domain.clone()).await.unwrap();
    assert!(level.allows(DataSharingLevel::Minimal));
    assert!(!level.allows(DataSharingLevel::Standard));
    assert!(!level.allows(DataSharingLevel::Full));
    
    // Test that Standard level allows Minimal and Standard access but denies Full access
    consent_service.update_consent_level(user_id, domain.clone(), DataSharingLevel::Standard, actor.clone()).await.unwrap();
    let level = consent_service.get_consent_level(user_id, domain.clone()).await.unwrap();
    assert!(level.allows(DataSharingLevel::Minimal));
    assert!(level.allows(DataSharingLevel::Standard));
    assert!(!level.allows(DataSharingLevel::Full));
    
    // Test that Full level allows all access
    consent_service.update_consent_level(user_id, domain.clone(), DataSharingLevel::Full, actor).await.unwrap();
    let level = consent_service.get_consent_level(user_id, domain).await.unwrap();
    assert!(level.allows(DataSharingLevel::Minimal));
    assert!(level.allows(DataSharingLevel::Standard));
    assert!(level.allows(DataSharingLevel::Full));
}

#[tokio::test]
async fn test_bevy_ecs_event_handling() {
    // Create the storage backend
    let storage = Box::new(InMemoryStorage::new());
    
    // Create the consent service
    let consent_service = ConsentService::new(storage);
    
    // Create the event channel
    let event_channel = ConsentEventChannel::new();
    
    // Subscribe to events
    let mut receiver = event_channel.subscribe();
    
    // Test user and domain
    let user_id = "test_user_events";
    let domain = Domain::CrmData;
    let actor = Actor::User(user_id.to_string());
    
    // Update consent level
    consent_service.update_consent_level(user_id, domain.clone(), DataSharingLevel::Standard, actor).await.unwrap();
    
    // Check that an event was published
    let event = tokio::time::timeout(std::time::Duration::from_millis(100), receiver.recv()).await;
    assert!(event.is_ok());
    
    let event = event.unwrap().unwrap();
    assert_eq!(event.user_id, user_id);
    assert_eq!(event.domain, domain);
    assert_eq!(event.new_level, DataSharingLevel::Standard);
}

#[tokio::test]
async fn test_dual_write_scenarios() {
    // Create the storage backend
    let storage = Box::new(InMemoryStorage::new());
    
    // Create the consent service
    let consent_service = ConsentService::new(storage);
    
    // Test user and domain
    let user_id = "test_user_dual_write";
    let domain = Domain::FinancialData;
    let actor = Actor::User(user_id.to_string());
    
    // Update consent level
    consent_service.update_consent_level(user_id, domain.clone(), DataSharingLevel::Standard, actor.clone()).await.unwrap();
    
    // Verify the consent was updated
    let level = consent_service.get_consent_level(user_id, domain.clone()).await.unwrap();
    assert_eq!(level, DataSharingLevel::Standard);
    
    // Update consent to Full level
    consent_service.update_consent_level(user_id, domain.clone(), DataSharingLevel::Full, actor).await.unwrap();
    
    // Verify the consent was updated
    let level = consent_service.get_consent_level(user_id, domain).await.unwrap();
    assert_eq!(level, DataSharingLevel::Full);
}

#[tokio::test]
async fn test_edge_cases_service_unavailable() {
    // Create a faulty storage backend that simulates service unavailability
    let storage = Box::new(FaultyStorage::new());
    
    // Create the consent service
    let consent_service = ConsentService::new(storage);
    
    // Test user and domain
    let user_id = "test_user_unavailable";
    let domain = Domain::CalendarData;
    let actor = Actor::User(user_id.to_string());
    
    // Try to update consent level - should fail due to storage error
    let result = consent_service.update_consent_level(user_id, domain, DataSharingLevel::Standard, actor).await;
    assert!(result.is_err());
    
    // Check that the error is a storage error
    match result.unwrap_err() {
        ConsentError::StorageError(_) => (),
        _ => panic!("Expected storage error"),
    }
}

/// Faulty storage implementation for testing error cases
struct FaultyStorage;

impl FaultyStorage {
    fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl ConsentStorage for FaultyStorage {
    async fn get_consent_profile(&self, _user_id: &str, _domain: &Domain) -> Result<Option<ConsentProfile>, ConsentError> {
        Err(ConsentError::StorageError("Simulated storage failure".to_string()))
    }

    async fn save_consent_profile(&self, _profile: &ConsentProfile) -> Result<(), ConsentError> {
        Err(ConsentError::StorageError("Simulated storage failure".to_string()))
    }

    async fn revoke_domain(&self, _user_id: &str, _domain: &Domain) -> Result<(), ConsentError> {
        Err(ConsentError::StorageError("Simulated storage failure".to_string()))
    }

    async fn get_audit_events(&self, _user_id: &str) -> Result<Vec<AuditEvent>, ConsentError> {
        Err(ConsentError::StorageError("Simulated storage failure".to_string()))
    }

    async fn save_audit_event(&self, _event: &AuditEvent) -> Result<(), ConsentError> {
        Err(ConsentError::StorageError("Simulated storage failure".to_string()))
    }
}

#[tokio::test]
async fn test_migration_utilities() {
    // Test SCM migration
    use consent_manager::migration::scm::{ScmConsentData, NetworkConsentSettings, InventoryConsentSettings, DataSharingLevelOld};
    
    // Create the storage backend
    let storage = Box::new(InMemoryStorage::new());
    let consent_service = std::sync::Arc::new(ConsentService::new(storage));
    
    // Create test SCM data
    let scm_data = vec![ScmConsentData {
        user_id: "test_user_scm".to_string(),
        network: NetworkConsentSettings {
            share_topology: DataSharingLevelOld::ViewOnly,
            share_node_details: DataSharingLevelOld::Editable,
        },
        inventory: InventoryConsentSettings {
            share_quantities: DataSharingLevelOld::None,
            share_locations: DataSharingLevelOld::FullAccess,
        },
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    }];
    
    let actor = Actor::System;
    
    // Migrate SCM data
    let result = consent_manager::migration::scm::migrate_scm_consent(consent_service.clone(), scm_data, actor).await;
    assert!(result.is_ok());
    
    // Test Calendar migration
    use consent_manager::migration::calendar::{CalendarConsentData, Module, ConsentPurpose, DataType};
    
    // Create test Calendar data
    let calendar_data = vec![CalendarConsentData {
        user_id: "test_user_calendar".to_string(),
        source_module: Module::Crm,
        target_module: Module::Calendar,
        purpose: ConsentPurpose::CrmIntegration,
        data_type: DataType::EventDetails,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    }];
    
    let actor = Actor::System;
    
    // Migrate Calendar data
    let result = consent_manager::migration::calendar::migrate_calendar_consent(consent_service.clone(), calendar_data, actor).await;
    assert!(result.is_ok());
    
    // Test Finance migration
    use consent_manager::migration::finance::FinanceConsentData;
    
    // Create test Finance data
    let finance_data = vec![FinanceConsentData {
        user_id: "test_user_finance".to_string(),
        data_sharing_enabled: true,
        anonymized_data: false,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    }];
    
    let actor = Actor::System;
    
    // Migrate Finance data
    let result = consent_manager::migration::finance::migrate_finance_consent(consent_service.clone(), finance_data, actor).await;
    assert!(result.is_ok());
    
    // Test CRM migration
    use consent_manager::migration::crm::{CrmConsentData, DataSharingLevelOld as CrmDataSharingLevelOld};
    
    // Create test CRM data
    let crm_data = vec![CrmConsentData {
        user_id: "test_user_crm".to_string(),
        consent_level: CrmDataSharingLevelOld::Editable,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    }];
    
    let actor = Actor::System;
    
    // Migrate CRM data
    let result = consent_manager::migration::crm::migrate_crm_consent(consent_service.clone(), crm_data, actor).await;
    assert!(result.is_ok());
}