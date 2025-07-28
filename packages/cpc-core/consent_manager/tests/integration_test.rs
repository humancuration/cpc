//! Integration tests for the consent manager.

use consent_manager::{
    domain::{
        consent::{DataSharingLevel, Domain},
        audit::Actor,
    },
    application::service::ConsentService,
    infrastructure::storage::sled_adapter::SledAdapter,
};
use sled::Config;

#[tokio::test]
async fn test_consent_lifecycle() {
    // Create a temporary sled database
    let config = Config::new().temporary(true);
    let db = config.open().expect("Failed to open sled database");
    
    // Create the sled adapter
    let sled_adapter = SledAdapter::new(db);
    
    // Create the consent service
    let consent_service = ConsentService::new(Box::new(sled_adapter));
    
    let user_id = "test_user";
    let domain = Domain::FinancialData;
    let actor = Actor::User(user_id.to_string());
    
    // Initially, consent should be None
    let initial_level = consent_service
        .get_consent_level(user_id, domain.clone())
        .await
        .expect("Failed to get initial consent level");
        
    assert_eq!(initial_level, DataSharingLevel::None);
    
    // Update consent to Standard
    consent_service
        .update_consent_level(user_id, domain.clone(), DataSharingLevel::Standard, actor.clone())
        .await
        .expect("Failed to update consent level");
    
    // Verify the update
    let updated_level = consent_service
        .get_consent_level(user_id, domain.clone())
        .await
        .expect("Failed to get updated consent level");
        
    assert_eq!(updated_level, DataSharingLevel::Standard);
    
    // Revoke consent
    consent_service
        .revoke_domain(user_id, domain.clone(), actor)
        .await
        .expect("Failed to revoke domain consent");
    
    // Verify revocation
    let revoked_level = consent_service
        .get_consent_level(user_id, domain)
        .await
        .expect("Failed to get revoked consent level");
        
    assert_eq!(revoked_level, DataSharingLevel::None);
}

#[tokio::test]
async fn test_audit_trail() {
    // Create a temporary sled database
    let config = Config::new().temporary(true);
    let db = config.open().expect("Failed to open sled database");
    
    // Create the sled adapter
    let sled_adapter = SledAdapter::new(db);
    
    // Create the consent service
    let consent_service = ConsentService::new(Box::new(sled_adapter));
    
    let user_id = "test_user";
    let domain = Domain::HealthData;
    let actor = Actor::User(user_id.to_string());
    
    // Perform consent operations
    consent_service
        .update_consent_level(user_id, domain.clone(), DataSharingLevel::Minimal, actor.clone())
        .await
        .expect("Failed to update consent level");
    
    consent_service
        .update_consent_level(user_id, domain.clone(), DataSharingLevel::Standard, actor.clone())
        .await
        .expect("Failed to update consent level");
    
    // Get audit events
    let audit_events = consent_service
        .get_audit_events(user_id)
        .await
        .expect("Failed to get audit events");
    
    // Should have 2 events
    assert_eq!(audit_events.len(), 2);
    
    // Verify first event (Granted)
    let first_event = &audit_events[0];
    assert_eq!(first_event.domain, domain);
    assert_eq!(first_event.action, consent_manager::domain::audit::ConsentAction::Granted);
    assert_eq!(first_event.new_level, DataSharingLevel::Minimal);
    assert!(first_event.previous_level.is_none());
    
    // Verify second event (Modified)
    let second_event = &audit_events[1];
    assert_eq!(second_event.domain, domain);
    assert_eq!(second_event.action, consent_manager::domain::audit::ConsentAction::Modified);
    assert_eq!(second_event.new_level, DataSharingLevel::Standard);
    assert_eq!(second_event.previous_level, Some(DataSharingLevel::Minimal));
}