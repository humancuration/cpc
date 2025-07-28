//! Migration utility for SCM consent data.

use crate::{
    domain::{
        consent::{ConsentProfile, Domain, DataSharingLevel},
        audit::{AuditEvent, Actor, ConsentAction},
    },
    application::service::ConsentStorage,
};

/// Structure representing existing SCM consent data
#[derive(Debug, Clone)]
pub struct ScmConsentData {
    /// User ID
    pub user_id: String,
    /// Whether data sharing is enabled
    pub data_sharing_enabled: bool,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last update timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Migrate SCM consent data to the new consent manager format
pub async fn migrate_scm_consent<T: ConsentStorage>(
    storage: &T,
    scm_data: Vec<ScmConsentData>,
    actor: Actor,
) -> Result<usize, crate::domain::errors::ConsentError> {
    let mut migrated_count = 0;
    
    for data in scm_data {
        // Convert SCM data sharing preference to consent level
        let level = if data.data_sharing_enabled {
            DataSharingLevel::Standard // SCM typically needs standard level for functionality
        } else {
            DataSharingLevel::None
        };
        
        // Create consent profile
        let profile = ConsentProfile::new(
            data.user_id.clone(),
            Domain::ScmData,
            level.clone(),
        );
        
        // Save the profile
        storage.save_consent_profile(&profile).await?;
        
        // Create audit event for the migration
        let audit_event = AuditEvent::new(
            data.user_id.clone(),
            Domain::ScmData,
            ConsentAction::Granted,
            None,
            level,
            actor.clone(),
        );
        
        storage.save_audit_event(&audit_event).await?;
        
        migrated_count += 1;
    }
    
    Ok(migrated_count)
}

/// Validate SCM consent data before migration
pub fn validate_scm_consent_data(data: &ScmConsentData) -> Result<(), crate::domain::errors::ConsentError> {
    if data.user_id.is_empty() {
        return Err(crate::domain::errors::ConsentError::ValidationError(
            "User ID cannot be empty".to_string()
        ));
    }
    
    // SCM data is generally valid as long as user_id is present
    Ok(())
}