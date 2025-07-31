//! Migration utility for SCM consent data.

use crate::{
    domain::{
        consent::{ConsentProfile, Domain, DataSharingLevel},
        audit::{AuditEvent, Actor, ConsentAction},
    },
    application::service::ConsentService,
};
use std::sync::Arc;

/// Structure representing existing SCM consent data
#[derive(Debug, Clone)]
pub struct ScmConsentData {
    /// User ID
    pub user_id: String,
    /// Network consent settings
    pub network: NetworkConsentSettings,
    /// Inventory consent settings
    pub inventory: InventoryConsentSettings,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last update timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Network consent settings
#[derive(Debug, Clone)]
pub struct NetworkConsentSettings {
    /// Whether to share network topology
    pub share_topology: DataSharingLevelOld,
    /// Whether to share node details
    pub share_node_details: DataSharingLevelOld,
}

/// Inventory consent settings
#[derive(Debug, Clone)]
pub struct InventoryConsentSettings {
    /// Whether to share inventory quantities
    pub share_quantities: DataSharingLevelOld,
    /// Whether to share inventory locations
    pub share_locations: DataSharingLevelOld,
}

/// Legacy data sharing levels in SCM
#[derive(Debug, Clone, PartialEq)]
pub enum DataSharingLevelOld {
    /// No data sharing allowed
    None,
    /// View-only access
    ViewOnly,
    /// Editable access
    Editable,
    /// Full access
    FullAccess,
}

/// Migrate SCM consent data to the new consent manager format
pub async fn migrate_scm_consent(
    consent_service: Arc<ConsentService>,
    scm_data: Vec<ScmConsentData>,
    actor: Actor,
) -> Result<usize, crate::domain::errors::ConsentError> {
    let mut migrated_count = 0;
    
    for data in scm_data {
        // Network settings
        migrate_domain(
            &consent_service,
            data.user_id.clone(),
            Domain::ScmData,
            "network_topology",
            data.network.share_topology,
            actor.clone(),
        ).await?;
        
        migrate_domain(
            &consent_service,
            data.user_id.clone(),
            Domain::ScmData,
            "node_details",
            data.network.share_node_details,
            actor.clone(),
        ).await?;
        
        // Inventory settings
        migrate_domain(
            &consent_service,
            data.user_id.clone(),
            Domain::ScmData,
            "inventory_quantities",
            data.inventory.share_quantities,
            actor.clone(),
        ).await?;
        
        migrate_domain(
            &consent_service,
            data.user_id.clone(),
            Domain::ScmData,
            "inventory_locations",
            data.inventory.share_locations,
            actor.clone(),
        ).await?;
        
        migrated_count += 1;
    }
    
    Ok(migrated_count)
}

/// Migrate a specific domain setting
async fn migrate_domain(
    consent_service: &ConsentService,
    user_id: String,
    domain: Domain,
    category: &str,
    old_level: DataSharingLevelOld,
    actor: Actor,
) -> Result<(), crate::domain::errors::ConsentError> {
    let new_level = map_scm_level(&old_level);
    
    // Update consent level
    consent_service.update_consent_level(&user_id, domain.clone(), new_level.clone(), actor.clone()).await?;
    
    Ok(())
}

/// Map old SCM consent level to new consent level
fn map_scm_level(level: &DataSharingLevelOld) -> DataSharingLevel {
    match level {
        DataSharingLevelOld::None => DataSharingLevel::None,
        DataSharingLevelOld::ViewOnly => DataSharingLevel::Minimal,
        DataSharingLevelOld::Editable |
        DataSharingLevelOld::FullAccess => DataSharingLevel::Standard,
    }
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