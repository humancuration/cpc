//! Migration utility for finance consent data.

use crate::{
    domain::{
        consent::{ConsentProfile, Domain, DataSharingLevel},
        audit::{AuditEvent, Actor, ConsentAction},
    },
    application::service::ConsentStorage,
};

/// Structure representing existing finance consent data
#[derive(Debug, Clone)]
pub struct FinanceConsentData {
    /// User ID
    pub user_id: String,
    /// Whether financial data sharing is enabled
    pub data_sharing_enabled: bool,
    /// Types of financial data shared
    pub shared_data_types: Vec<FinanceDataType>,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last update timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Types of financial data that can be shared
#[derive(Debug, Clone, PartialEq)]
pub enum FinanceDataType {
    /// Account balances
    AccountBalances,
    /// Transaction history
    TransactionHistory,
    /// Investment portfolio
    InvestmentPortfolio,
    /// Credit score
    CreditScore,
    /// Tax information
    TaxInformation,
}

/// Migrate finance consent data to the new consent manager format
pub async fn migrate_finance_consent<T: ConsentStorage>(
    storage: &T,
    finance_data: Vec<FinanceConsentData>,
    actor: Actor,
) -> Result<usize, crate::domain::errors::ConsentError> {
    let mut migrated_count = 0;
    
    for data in finance_data {
        // Convert finance data sharing preference to consent level
        let level = if data.data_sharing_enabled {
            // Determine level based on data types shared
            if data.shared_data_types.contains(&FinanceDataType::TaxInformation) ||
               data.shared_data_types.contains(&FinanceDataType::CreditScore) {
                // Highly sensitive data requires full consent
                DataSharingLevel::Full
            } else if data.shared_data_types.contains(&FinanceDataType::InvestmentPortfolio) ||
                      data.shared_data_types.contains(&FinanceDataType::TransactionHistory) {
                // Detailed financial data requires standard consent
                DataSharingLevel::Standard
            } else {
                // Basic financial data requires minimal consent
                DataSharingLevel::Minimal
            }
        } else {
            DataSharingLevel::None
        };
        
        // Create consent profile
        let profile = ConsentProfile::new(
            data.user_id.clone(),
            Domain::FinancialData,
            level.clone(),
        );
        
        // Save the profile
        storage.save_consent_profile(&profile).await?;
        
        // Create audit event for the migration
        let audit_event = AuditEvent::new(
            data.user_id.clone(),
            Domain::FinancialData,
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