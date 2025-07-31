//! Migration utility for finance consent data.

use crate::{
    domain::{
        consent::{ConsentProfile, Domain, DataSharingLevel},
        audit::{AuditEvent, Actor, ConsentAction},
    },
    application::service::ConsentService,
};
use std::sync::Arc;

/// Structure representing existing finance consent data
#[derive(Debug, Clone)]
pub struct FinanceConsentData {
    /// User ID
    pub user_id: String,
    /// Whether financial data sharing is enabled
    pub data_sharing_enabled: bool,
    /// Whether data should be anonymized
    pub anonymized_data: bool,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last update timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Migrate finance consent data to the new consent manager format
pub async fn migrate_finance_consent(
    consent_service: Arc<ConsentService>,
    finance_data: Vec<FinanceConsentData>,
    actor: Actor,
) -> Result<usize, crate::domain::errors::ConsentError> {
    let mut migrated_count = 0;
    
    for data in finance_data {
        // Convert finance data sharing preference to consent level
        let level = map_finance_preference(&data);
        
        // Update consent level
        consent_service.update_consent_level(&data.user_id, Domain::FinancialData, level.clone(), actor.clone()).await?;
        
        migrated_count += 1;
    }
    
    Ok(migrated_count)
}

/// Map finance preference to consent level
fn map_finance_preference(preference: &FinanceConsentData) -> DataSharingLevel {
    if !preference.data_sharing_enabled {
        DataSharingLevel::None
    } else if preference.anonymized_data {
        DataSharingLevel::Minimal
    } else {
        DataSharingLevel::Standard
    }
}

/// Validate finance consent data before migration
pub fn validate_finance_consent_data(data: &FinanceConsentData) -> Result<(), crate::domain::errors::ConsentError> {
    if data.user_id.is_empty() {
        return Err(crate::domain::errors::ConsentError::ValidationError(
            "User ID cannot be empty".to_string()
        ));
    }
    
    // Finance data is generally valid as long as user_id is present
    Ok(())
}