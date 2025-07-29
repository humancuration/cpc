//! Supplier entity and related types

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::primitives::{DomainError, Result};
use super::consent::SupplierConsentSettings;

/// Contact information for a supplier
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContactInformation {
    pub email: String,
    pub phone: String,
    pub address: String,
    pub website: Option<String>,
}

impl ContactInformation {
    pub fn new(email: String, phone: String, address: String) -> Self {
        Self {
            email,
            phone,
            address,
            website: None,
        }
    }

    pub fn with_website(mut self, website: String) -> Self {
        self.website = Some(website);
        self
    }
}

/// Supplier performance metrics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SupplierMetrics {
    pub delivery_time_score: f64, // 0.0 to 1.0, where 1.0 is perfect
    pub quality_score: f64,       // 0.0 to 1.0, where 1.0 is perfect
    pub responsiveness_score: f64, // 0.0 to 1.0, where 1.0 is perfect
    pub last_evaluation_date: DateTime<Utc>,
}

impl SupplierMetrics {
    pub fn new(
        delivery_time_score: f64,
        quality_score: f64,
        responsiveness_score: f64,
    ) -> Self {
        Self {
            delivery_time_score,
            quality_score,
            responsiveness_score,
            last_evaluation_date: Utc::now(),
        }
    }

    /// Calculate overall performance score as weighted average
    pub fn calculate_performance_score(&self) -> f64 {
        // Weighted calculation: 40% delivery time, 40% quality, 20% responsiveness
        (self.delivery_time_score * 0.4) + 
        (self.quality_score * 0.4) + 
        (self.responsiveness_score * 0.2)
    }

    /// Update metrics with new values
    pub fn update_metrics(
        &mut self,
        delivery_time_score: f64,
        quality_score: f64,
        responsiveness_score: f64,
    ) {
        self.delivery_time_score = delivery_time_score;
        self.quality_score = quality_score;
        self.responsiveness_score = responsiveness_score;
        self.last_evaluation_date = Utc::now();
    }
}

/// Contract with a supplier
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Contract {
    pub id: Uuid,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub terms: String,
    pub renewal_required: bool,
    pub special_certifications: Vec<String>,
}

impl Contract {
    pub fn new(
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        terms: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            start_date,
            end_date,
            terms,
            renewal_required: false,
            special_certifications: Vec::new(),
        }
    }

    pub fn with_renewal_required(mut self) -> Self {
        self.renewal_required = true;
        self
    }

    pub fn with_certifications(mut self, certifications: Vec<String>) -> Self {
        self.special_certifications = certifications;
        self
    }

    /// Check if contract is active
    pub fn is_active(&self) -> bool {
        let now = Utc::now();
        self.start_date <= now && self.end_date >= now
    }

    /// Check if contract is expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.end_date
    }

    /// Check if contract is about to expire (within 30 days)
    pub fn is_expiring_soon(&self) -> bool {
        let now = Utc::now();
        let thirty_days = chrono::Duration::days(30);
        now >= (self.end_date - thirty_days) && now <= self.end_date
    }
}

/// Supplier entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Supplier {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub contact_info: ContactInformation,
    pub performance_metrics: SupplierMetrics,
    pub contracts: Vec<Contract>,
    pub is_critical: bool, // Flag for critical suppliers requiring special attention
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub consent_settings: SupplierConsentSettings,
}

impl Supplier {
    /// Create a new supplier
    pub fn new(
        name: String,
        contact_info: ContactInformation,
        performance_metrics: SupplierMetrics,
        contracts: Vec<Contract>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            contact_info,
            performance_metrics,
            contracts,
            is_critical: false,
            created_at: now,
            updated_at: now,
            consent_settings: SupplierConsentSettings::default(),
        }
    }

    /// Set description for the supplier
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Mark supplier as critical
    pub fn as_critical(mut self) -> Self {
        self.is_critical = true;
        self
    }

    /// Set consent settings for the supplier
    pub fn set_consent_settings(&mut self, settings: SupplierConsentSettings) {
        self.consent_settings = settings;
        self.updated_at = Utc::now();
    }

    /// Calculate performance score
    pub fn calculate_performance_score(&self) -> f64 {
        self.performance_metrics.calculate_performance_score()
    }

    /// Validate contract
    pub fn validate_contract(&self, contract: &Contract) -> Result<()> {
        // Ensure contract terms valid for this supplier
        if contract.start_date >= contract.end_date {
            return Err(DomainError::ValidationError {
                message: "Contract start date must be before end date".to_string(),
            });
        }
        
        // Ensure contract is not in the past
        if contract.end_date < Utc::now() {
            return Err(DomainError::ValidationError {
                message: "Contract end date must be in the future".to_string(),
            });
        }
        
        Ok(())
    }

    /// Add a contract to the supplier
    pub fn add_contract(&mut self, contract: Contract) -> Result<()> {
        self.validate_contract(&contract)?;
        self.contracts.push(contract);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Remove a contract from the supplier
    pub fn remove_contract(&mut self, contract_id: Uuid) -> Result<()> {
        let initial_count = self.contracts.len();
        self.contracts.retain(|c| c.id != contract_id);
        
        if self.contracts.len() == initial_count {
            return Err(DomainError::NotFound);
        }
        
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Get active contracts
    pub fn active_contracts(&self) -> Vec<&Contract> {
        self.contracts.iter().filter(|c| c.is_active()).collect()
    }

    /// Check if supplier has any active contracts
    pub fn has_active_contracts(&self) -> bool {
        self.contracts.iter().any(|c| c.is_active())
    }

    /// Get contracts expiring soon
    pub fn expiring_contracts(&self) -> Vec<&Contract> {
        self.contracts.iter().filter(|c| c.is_expiring_soon()).collect()
    }

    /// Validate the supplier
    pub fn validate(&self) -> Result<()> {
        // Must have valid contact information
        if self.contact_info.email.is_empty() || 
           self.contact_info.phone.is_empty() || 
           self.contact_info.address.is_empty() {
            return Err(DomainError::ValidationError {
                message: "Supplier must have valid contact information".to_string(),
            });
        }
        
        // Performance metrics must be between 0.0 and 1.0
        if self.performance_metrics.delivery_time_score < 0.0 || 
           self.performance_metrics.delivery_time_score > 1.0 {
            return Err(DomainError::ValidationError {
                message: "Delivery time score must be between 0.0 and 1.0".to_string(),
            });
        }
        
        if self.performance_metrics.quality_score < 0.0 || 
           self.performance_metrics.quality_score > 1.0 {
            return Err(DomainError::ValidationError {
                message: "Quality score must be between 0.0 and 1.0".to_string(),
            });
        }
        
        if self.performance_metrics.responsiveness_score < 0.0 || 
           self.performance_metrics.responsiveness_score > 1.0 {
            return Err(DomainError::ValidationError {
                message: "Responsiveness score must be between 0.0 and 1.0".to_string(),
            });
        }
        
        // Validate all contracts
        for contract in &self.contracts {
            self.validate_contract(contract)?;
        }
        
        Ok(())
    }

    /// Check if contact info can be shared
    pub fn can_share_contact_info(&self) -> bool {
        matches!(
            self.consent_settings.share_contact_info,
            super::primitives::DataSharingLevel::ViewOnly | 
            super::primitives::DataSharingLevel::Editable | 
            super::primitives::DataSharingLevel::FullAccess
        )
    }

    /// Check if performance metrics can be shared
    pub fn can_share_performance_metrics(&self) -> bool {
        matches!(
            self.consent_settings.share_performance_metrics,
            super::primitives::DataSharingLevel::ViewOnly | 
            super::primitives::DataSharingLevel::Editable | 
            super::primitives::DataSharingLevel::FullAccess
        )
    }

    /// Check if contract details can be shared
    pub fn can_share_contract_details(&self) -> bool {
        matches!(
            self.consent_settings.share_contract_details,
            super::primitives::DataSharingLevel::ViewOnly | 
            super::primitives::DataSharingLevel::Editable | 
            super::primitives::DataSharingLevel::FullAccess
        )
    }

    /// Check if certification data can be shared
    pub fn can_share_certification_data(&self) -> bool {
        matches!(
            self.consent_settings.share_certification_data,
            super::primitives::DataSharingLevel::ViewOnly | 
            super::primitives::DataSharingLevel::Editable | 
            super::primitives::DataSharingLevel::FullAccess
        )
    }
}