//! Migration utility for calendar consent data.

use crate::{
    domain::{
        consent::{ConsentProfile, Domain, DataSharingLevel},
        audit::{AuditEvent, Actor, ConsentAction},
    },
    application::service::ConsentStorage,
};

/// Structure representing existing calendar consent data
#[derive(Debug, Clone)]
pub struct CalendarConsentData {
    /// User ID
    pub user_id: String,
    /// Whether calendar sharing is enabled
    pub sharing_enabled: bool,
    /// Level of detail shared (if sharing is enabled)
    pub detail_level: CalendarDetailLevel,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last update timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Level of calendar detail shared
#[derive(Debug, Clone, PartialEq)]
pub enum CalendarDetailLevel {
    /// Only availability/free-busy information
    AvailabilityOnly,
    /// Basic event details (titles, times)
    BasicDetails,
    /// Full event details (including descriptions, locations)
    FullDetails,
}

/// Migrate calendar consent data to the new consent manager format
pub async fn migrate_calendar_consent<T: ConsentStorage>(
    storage: &T,
    calendar_data: Vec<CalendarConsentData>,
    actor: Actor,
) -> Result<usize, crate::domain::errors::ConsentError> {
    let mut migrated_count = 0;
    
    for data in calendar_data {
        // Convert calendar detail level to consent level
        let level = match (&data.sharing_enabled, &data.detail_level) {
            (false, _) => DataSharingLevel::None,
            (true, CalendarDetailLevel::AvailabilityOnly) => DataSharingLevel::Minimal,
            (true, CalendarDetailLevel::BasicDetails) => DataSharingLevel::Standard,
            (true, CalendarDetailLevel::FullDetails) => DataSharingLevel::Full,
        };
        
        // Create consent profile
        let profile = ConsentProfile::new(
            data.user_id.clone(),
            Domain::CalendarData,
            level.clone(),
        );
        
        // Save the profile
        storage.save_consent_profile(&profile).await?;
        
        // Create audit event for the migration
        let audit_event = AuditEvent::new(
            data.user_id.clone(),
            Domain::CalendarData,
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

/// Validate calendar consent data before migration
pub fn validate_calendar_consent_data(data: &CalendarConsentData) -> Result<(), crate::domain::errors::ConsentError> {
    if data.user_id.is_empty() {
        return Err(crate::domain::errors::ConsentError::ValidationError(
            "User ID cannot be empty".to_string()
        ));
    }
    
    // Calendar data is generally valid as long as user_id is present
    Ok(())
}