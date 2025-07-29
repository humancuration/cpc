//! Migration utility for calendar consent data.

use crate::{
    domain::{
        consent::{ConsentProfile, Domain, DataSharingLevel},
        audit::{AuditEvent, Actor, ConsentAction},
    },
    application::service::ConsentService,
};
use std::sync::Arc;

/// Structure representing existing calendar consent data
#[derive(Debug, Clone)]
pub struct CalendarConsentData {
    /// User ID
    pub user_id: String,
    /// Source module
    pub source_module: Module,
    /// Target module
    pub target_module: Module,
    /// Purpose of consent
    pub purpose: ConsentPurpose,
    /// Type of data being shared
    pub data_type: DataType,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last update timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Module in the system
#[derive(Debug, Clone, PartialEq)]
pub enum Module {
    /// CRM module
    Crm,
    /// Invoicing module
    Invoicing,
    /// Calendar module
    Calendar,
    /// SCM module
    Scm,
}

/// Purpose of consent
#[derive(Debug, Clone, PartialEq)]
pub enum ConsentPurpose {
    /// CRM integration
    CrmIntegration,
    /// Invoicing integration
    InvoicingIntegration,
    /// General purpose
    General,
}

/// Type of data being shared
#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    /// Event details
    EventDetails,
    /// Availability information
    Availability,
    /// Calendar metadata
    Metadata,
}

/// Migrate calendar consent data to the new consent manager format
pub async fn migrate_calendar_consent(
    consent_service: Arc<ConsentService>,
    calendar_data: Vec<CalendarConsentData>,
    actor: Actor,
) -> Result<usize, crate::domain::errors::ConsentError> {
    let mut migrated_count = 0;
    
    for data in calendar_data {
        // Map module pairs to domains
        let domain = match (&data.source_module, &data.target_module) {
            (Module::Crm, Module::Calendar) => Domain::CrmData,
            (Module::Invoicing, Module::Calendar) => Domain::FinancialData,
            (Module::Scm, Module::Calendar) => Domain::ScmData,
            _ => Domain::CalendarData,
        };
        
        // Map purpose to data category
        let category = match &data.purpose {
            ConsentPurpose::CrmIntegration => "crm_integration",
            ConsentPurpose::InvoicingIntegration => "invoicing",
            ConsentPurpose::General => "general",
        };
        
        // Map data type to consent level
        let level = match &data.data_type {
            DataType::Availability => DataSharingLevel::Minimal,
            DataType::EventDetails => DataSharingLevel::Standard,
            DataType::Metadata => DataSharingLevel::Full,
        };
        
        // Update consent level
        consent_service.update_consent_level(&data.user_id, domain.clone(), level.clone(), actor.clone()).await?;
        
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