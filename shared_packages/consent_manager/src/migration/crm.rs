//! Migration utility for CRM consent data.

use crate::{
    domain::{
        consent::{ConsentProfile, Domain, DataSharingLevel},
        audit::{AuditEvent, Actor, ConsentAction},
    },
    application::service::ConsentStorage,
};

/// Structure representing existing CRM consent data
#[derive(Debug, Clone)]
pub struct CrmConsentData {
    /// User ID
    pub user_id: String,
    /// Current consent level
    pub consent_level: DataSharingLevelOld,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last update timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Legacy data sharing levels in CRM
#[derive(Debug, Clone, PartialEq)]
pub enum DataSharingLevelOld {
    /// No data sharing allowed
    None,
    /// View-only access
    ViewOnly,
    /// Editable access
    Editable,
}

/// Migrate CRM consent data to the new consent manager format
pub async fn migrate_crm_consent<T: ConsentStorage>(
    storage: &T,
    crm_data: Vec<CrmConsentData>,
    actor: Actor,
) -> Result<usize, crate::domain::errors::ConsentError> {
    let mut migrated_count = 0;
    
    for data in crm_data {
        // Convert old CRM consent level to new consent level
        let level = map_crm_level(&data.consent_level);
        
        // Create consent profile
        let profile = ConsentProfile::new(
            data.user_id.clone(),
            Domain::CrmData,
            level.clone(),
        );
        
        // Save the profile
        storage.save_consent_profile(&profile).await?;
        
        // Create audit event for the migration
        let audit_event = AuditEvent::new(
            data.user_id.clone(),
            Domain::CrmData,
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

/// Map old CRM consent level to new consent level
fn map_crm_level(level: &DataSharingLevelOld) -> DataSharingLevel {
    match level {
        DataSharingLevelOld::None => DataSharingLevel::None,
        DataSharingLevelOld::ViewOnly => DataSharingLevel::Minimal,
        DataSharingLevelOld::Editable => DataSharingLevel::Standard,
    }
}

/// Validate CRM consent data before migration
pub fn validate_crm_consent_data(data: &CrmConsentData) -> Result<(), crate::domain::errors::ConsentError> {
    if data.user_id.is_empty() {
        return Err(crate::domain::errors::ConsentError::ValidationError(
            "User ID cannot be empty".to_string()
        ));
    }
    
    // CRM data is generally valid as long as user_id is present
    Ok(())
}