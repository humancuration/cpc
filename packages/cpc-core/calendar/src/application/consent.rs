//! Consent management for cross-module data sharing
//!
//! This module implements the consent checking required by our privacy policies
//! and handles data minimization for cross-module integrations.

use crate::domain::CalendarError;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use async_trait::async_trait;
use tracing::{info, warn};

/// Module identifiers for consent management
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Module {
    Calendar,
    Crm,
    Invoicing,
    Health,
    TaskManager,
    // Add other modules as needed
}

/// Purpose of data sharing for consent management
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConsentPurpose {
    CrmIntegration,
    InvoicingIntegration,
    Analytics,
    // Add other purposes as needed
}

/// Repository for consent records
#[async_trait]
pub trait ConsentRepository: Send + Sync {
    /// Get the latest consent record for a user and purpose
    async fn get_latest_consent(
        &self,
        user_id: Uuid,
        source_module: Module,
        target_module: Module,
        purpose: ConsentPurpose,
    ) -> Result<Option<ConsentRecord>, CalendarError>;
    
    /// Record a new consent decision
    async fn record_consent(
        &self,
        consent: ConsentRecord,
    ) -> Result<(), CalendarError>;
    
    /// Revoke consent for a specific purpose
    async fn revoke_consent(
        &self,
        user_id: Uuid,
        source_module: Module,
        target_module: Module,
        purpose: ConsentPurpose,
    ) -> Result<(), CalendarError>;
}

/// A record of user consent
#[derive(Debug, Clone)]
pub struct ConsentRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub source_module: Module,
    pub target_module: Module,
    pub purpose: ConsentPurpose,
    pub granted: bool,
    pub data_types: Vec<DataType>,
    pub valid_until: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Types of data that can be shared
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataType {
    // CRM data types
    SalesPipelineStages,
    LeadScoringData,
    EmailCampaignData,
    
    // Invoicing data types
    PaymentDueDates,
    PaymentStatus,
    InvoiceAmounts,
    
    // General data types
    EventTimestamps,
    EventDescriptions,
}

/// Service for checking and managing user consent
pub struct ConsentService {
    repo: Arc<dyn ConsentRepository>,
}

impl ConsentService {
    pub fn new(repo: Arc<dyn ConsentRepository>) -> Self {
        Self { repo }
    }
    
    /// Check if user has granted consent for a specific data sharing purpose
    pub async fn has_consent(
        &self,
        user_id: Uuid,
        source_module: Module,
        target_module: Module,
        purpose: ConsentPurpose,
    ) -> Result<bool, CalendarError> {
        let consent = self.repo.get_latest_consent(
            user_id,
            source_module,
            target_module,
            purpose,
        ).await?;
        
        Ok(consent.map(|c| c.granted).unwrap_or(false))
    }
    
    /// Check if specific data type is covered by existing consent
    pub async fn has_consent_for_data_type(
        &self,
        user_id: Uuid,
        source_module: Module,
        target_module: Module,
        purpose: ConsentPurpose,
        data_type: DataType,
    ) -> Result<bool, CalendarError> {
        let consent = self.repo.get_latest_consent(
            user_id,
            source_module,
            target_module,
            purpose,
        ).await?;
        
        Ok(consent.map(|c| 
            c.granted && c.data_types.contains(&data_type)
        ).unwrap_or(false))
    }
    
    /// Record user consent decision
    pub async fn record_consent(
        &self,
        user_id: Uuid,
        source_module: Module,
        target_module: Module,
        purpose: ConsentPurpose,
        granted: bool,
        data_types: Vec<DataType>,
        valid_until: Option<DateTime<Utc>>,
    ) -> Result<ConsentRecord, CalendarError> {
        let now = Utc::now();
        let consent = ConsentRecord {
            id: Uuid::new_v4(),
            user_id,
            source_module,
            target_module,
            purpose,
            granted,
            data_types,
            valid_until,
            created_at: now,
            updated_at: now,
        };
        
        self.repo.record_consent(consent.clone()).await?;
        Ok(consent)
    }
    
    /// Revoke consent for a specific purpose
    pub async fn revoke_consent(
        &self,
        user_id: Uuid,
        source_module: Module,
        target_module: Module,
        purpose: ConsentPurpose,
    ) -> Result<(), CalendarError> {
        self.repo.revoke_consent(
            user_id,
            source_module,
            target_module,
            purpose,
        ).await
    }
}

/// Mock implementation for testing
#[derive(Default)]
pub struct MockConsentRepository;

#[async_trait]
impl ConsentRepository for MockConsentRepository {
    async fn get_latest_consent(
        &self,
        _user_id: Uuid,
        _source_module: Module,
        _target_module: Module,
        _purpose: ConsentPurpose,
    ) -> Result<Option<ConsentRecord>, CalendarError> {
        // For testing, always grant consent
        Ok(Some(ConsentRecord {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            source_module: Module::Crm,
            target_module: Module::Calendar,
            purpose: ConsentPurpose::CrmIntegration,
            granted: true,
            data_types: vec![
                DataType::SalesPipelineStages,
                DataType::LeadScoringData,
            ],
            valid_until: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }))
    }
    
    async fn record_consent(
        &self,
        _consent: ConsentRecord,
    ) -> Result<(), CalendarError> {
        Ok(())
    }
    
    async fn revoke_consent(
        &self,
        _user_id: Uuid,
        _source_module: Module,
        _target_module: Module,
        _purpose: ConsentPurpose,
    ) -> Result<(), CalendarError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    
    #[tokio::test]
    async fn test_consent_service() {
        let repo = Arc::new(MockConsentRepository::default());
        let service = ConsentService::new(repo);
        
        let user_id = Uuid::new_v4();
        
        // Test has_consent
        let has_consent = service.has_consent(
            user_id,
            Module::Crm,
            Module::Calendar,
            ConsentPurpose::CrmIntegration,
        ).await.unwrap();
        assert!(has_consent);
        
        // Test has_consent_for_data_type
        let has_consent_for_data = service.has_consent_for_data_type(
            user_id,
            Module::Crm,
            Module::Calendar,
            ConsentPurpose::CrmIntegration,
            DataType::SalesPipelineStages,
        ).await.unwrap();
        assert!(has_consent_for_data);
        
        let has_consent_for_data = service.has_consent_for_data_type(
            user_id,
            Module::Crm,
            Module::Calendar,
            ConsentPurpose::CrmIntegration,
            DataType::PaymentDueDates,
        ).await.unwrap();
        assert!(!has_consent_for_data);
    }
}