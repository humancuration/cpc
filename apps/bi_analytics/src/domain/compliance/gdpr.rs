//! GDPR compliance for the BI & Analytics module

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use thiserror::Error;

/// Error types for GDPR compliance operations
#[derive(Error, Debug)]
pub enum GdprError {
    #[error("Invalid consent data: {0}")]
    InvalidConsent(String),
    
    #[error("Data processing error: {0}")]
    ProcessingError(String),
    
    #[error("Access request not found: {0}")]
    AccessRequestNotFound(String),
}

/// GDPR data processing purposes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProcessingPurpose {
    Analytics,
    Reporting,
    Research,
    Marketing,
}

/// Consent status for data processing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConsentStatus {
    Granted,
    Denied,
    Revoked,
}

/// GDPR consent record
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GdprConsent {
    pub id: Uuid,
    pub user_id: Uuid,
    pub purpose: ProcessingPurpose,
    pub status: ConsentStatus,
    pub granted_at: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl GdprConsent {
    /// Create a new GDPR consent record
    pub fn new(
        user_id: Uuid,
        purpose: ProcessingPurpose,
        status: ConsentStatus,
    ) -> Self {
        let now = Utc::now();
        let (granted_at, revoked_at) = match status {
            ConsentStatus::Granted => (Some(now), None),
            ConsentStatus::Revoked => (None, Some(now)),
            ConsentStatus::Denied => (None, None),
        };
        
        Self {
            id: Uuid::new_v4(),
            user_id,
            purpose,
            status,
            granted_at,
            revoked_at,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Update consent status
    pub fn update_status(&mut self, status: ConsentStatus) {
        self.status = status;
        let now = Utc::now();
        
        match status {
            ConsentStatus::Granted => {
                self.granted_at = Some(now);
                self.revoked_at = None;
            }
            ConsentStatus::Revoked => {
                self.revoked_at = Some(now);
                self.granted_at = None;
            }
            ConsentStatus::Denied => {
                self.granted_at = None;
                self.revoked_at = None;
            }
        }
        
        self.updated_at = now;
    }
    
    /// Check if consent is currently granted
    pub fn is_granted(&self) -> bool {
        matches!(self.status, ConsentStatus::Granted)
    }
}

/// Data anonymization levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnonymizationLevel {
    /// No anonymization
    None,
    
    /// Basic anonymization (e.g., remove names, emails)
    Basic,
    
    /// Strong anonymization (e.g., aggregate data, remove identifiers)
    Strong,
}

/// GDPR data access request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataAccessRequest {
    pub id: Uuid,
    pub user_id: Uuid,
    pub request_type: DataAccessRequestType,
    pub status: DataAccessRequestStatus,
    pub requested_data: Vec<String>, // List of data categories requested
    pub fulfillment_details: Option<String>,
    pub requested_at: DateTime<Utc>,
    pub fulfilled_at: Option<DateTime<Utc>>,
    pub rejected_at: Option<DateTime<Utc>>,
}

/// Types of data access requests
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataAccessRequestType {
    DataExport,
    DataDeletion,
    DataCorrection,
    DataPortability,
}

/// Status of data access requests
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataAccessRequestStatus {
    Pending,
    Processing,
    Fulfilled,
    Rejected,
}

impl DataAccessRequest {
    /// Create a new data access request
    pub fn new(
        user_id: Uuid,
        request_type: DataAccessRequestType,
        requested_data: Vec<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            request_type,
            status: DataAccessRequestStatus::Pending,
            requested_data,
            fulfillment_details: None,
            requested_at: Utc::now(),
            fulfilled_at: None,
            rejected_at: None,
        }
    }
    
    /// Update request status
    pub fn update_status(&mut self, status: DataAccessRequestStatus, details: Option<String>) {
        self.status = status;
        self.fulfillment_details = details;
        let now = Utc::now();
        
        match status {
            DataAccessRequestStatus::Fulfilled => {
                self.fulfilled_at = Some(now);
            }
            DataAccessRequestStatus::Rejected => {
                self.rejected_at = Some(now);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_gdpr_consent() {
        let user_id = Uuid::new_v4();
        let consent = GdprConsent::new(
            user_id,
            ProcessingPurpose::Analytics,
            ConsentStatus::Granted,
        );
        
        assert_eq!(consent.user_id, user_id);
        assert_eq!(consent.purpose, ProcessingPurpose::Analytics);
        assert_eq!(consent.status, ConsentStatus::Granted);
        assert!(consent.granted_at.is_some());
        assert!(consent.is_granted());
    }
    
    #[test]
    fn test_update_consent_status() {
        let user_id = Uuid::new_v4();
        let mut consent = GdprConsent::new(
            user_id,
            ProcessingPurpose::Analytics,
            ConsentStatus::Granted,
        );
        
        consent.update_status(ConsentStatus::Revoked);
        assert_eq!(consent.status, ConsentStatus::Revoked);
        assert!(consent.revoked_at.is_some());
        assert!(!consent.is_granted());
    }
    
    #[test]
    fn test_create_data_access_request() {
        let user_id = Uuid::new_v4();
        let requested_data = vec!["sales_data".to_string(), "user_profile".to_string()];
        
        let request = DataAccessRequest::new(
            user_id,
            DataAccessRequestType::DataExport,
            requested_data.clone(),
        );
        
        assert_eq!(request.user_id, user_id);
        assert_eq!(request.request_type, DataAccessRequestType::DataExport);
        assert_eq!(request.requested_data, requested_data);
        assert_eq!(request.status, DataAccessRequestStatus::Pending);
    }
    
    #[test]
    fn test_update_request_status() {
        let user_id = Uuid::new_v4();
        let requested_data = vec!["sales_data".to_string()];
        
        let mut request = DataAccessRequest::new(
            user_id,
            DataAccessRequestType::DataExport,
            requested_data,
        );
        
        let details = Some("Data exported successfully".to_string());
        request.update_status(DataAccessRequestStatus::Fulfilled, details.clone());
        
        assert_eq!(request.status, DataAccessRequestStatus::Fulfilled);
        assert_eq!(request.fulfillment_details, details);
        assert!(request.fulfilled_at.is_some());
    }
}