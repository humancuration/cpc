//! Compliance management service for the BI & Analytics module

use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::{
    compliance::gdpr::{GdprConsent, ProcessingPurpose, ConsentStatus, DataAccessRequest, DataAccessRequestType, DataAccessRequestStatus},
    compliance::hipaa::{AccessPermission, AccessRole, PhiCategory, AuditLogEntry, AuditAction, HipaaConfig},
};
use thiserror::Error;

/// Error types for compliance management operations
#[derive(Error, Debug)]
pub enum ComplianceManagementError {
    #[error("GDPR error: {0}")]
    GdprError(String),
    
    #[error("HIPAA error: {0}")]
    HipaaError(String),
    
    #[error("Access control error: {0}")]
    AccessControlError(String),
    
    #[error("Audit error: {0}")]
    AuditError(String),
}

/// Compliance management service
pub struct ComplianceManagementService<G: GdprRepository, H: HipaaRepository> {
    gdpr_repository: G,
    hipaa_repository: H,
}

impl<G: GdprRepository, H: HipaaRepository> ComplianceManagementService<G, H> {
    /// Create a new compliance management service
    pub fn new(gdpr_repository: G, hipaa_repository: H) -> Self {
        Self {
            gdpr_repository,
            hipaa_repository,
        }
    }
    
    /// Record GDPR consent
    pub async fn record_gdpr_consent(
        &self,
        user_id: Uuid,
        purpose: ProcessingPurpose,
        status: ConsentStatus,
    ) -> Result<GdprConsent, ComplianceManagementError> {
        let consent = GdprConsent::new(user_id, purpose, status);
        
        self.gdpr_repository.save_consent(&consent)
            .await
            .map_err(|e| ComplianceManagementError::GdprError(e.to_string()))?;
        
        Ok(consent)
    }
    
    /// Update GDPR consent status
    pub async fn update_gdpr_consent(
        &self,
        consent_id: Uuid,
        status: ConsentStatus,
    ) -> Result<GdprConsent, ComplianceManagementError> {
        let mut consent = self.gdpr_repository.get_consent(consent_id)
            .await
            .map_err(|e| ComplianceManagementError::GdprError(e.to_string()))?;
        
        consent.update_status(status);
        
        self.gdpr_repository.save_consent(&consent)
            .await
            .map_err(|e| ComplianceManagementError::GdprError(e.to_string()))?;
        
        Ok(consent)
    }
    
    /// Check if user has granted consent for a purpose
    pub async fn has_gdpr_consent(
        &self,
        user_id: Uuid,
        purpose: ProcessingPurpose,
    ) -> Result<bool, ComplianceManagementError> {
        let consents = self.gdpr_repository.get_consents_by_user_and_purpose(user_id, &purpose)
            .await
            .map_err(|e| ComplianceManagementError::GdprError(e.to_string()))?;
        
        // Check if any consent is currently granted
        for consent in consents {
            if consent.is_granted() {
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    /// Create a data access request
    pub async fn create_data_access_request(
        &self,
        user_id: Uuid,
        request_type: DataAccessRequestType,
        requested_data: Vec<String>,
    ) -> Result<DataAccessRequest, ComplianceManagementError> {
        let request = DataAccessRequest::new(user_id, request_type, requested_data);
        
        self.gdpr_repository.save_data_access_request(&request)
            .await
            .map_err(|e| ComplianceManagementError::GdprError(e.to_string()))?;
        
        Ok(request)
    }
    
    /// Update data access request status
    pub async fn update_data_access_request(
        &self,
        request_id: Uuid,
        status: DataAccessRequestStatus,
        details: Option<String>,
    ) -> Result<DataAccessRequest, ComplianceManagementError> {
        let mut request = self.gdpr_repository.get_data_access_request(request_id)
            .await
            .map_err(|e| ComplianceManagementError::GdprError(e.to_string()))?;
        
        request.update_status(status, details);
        
        self.gdpr_repository.save_data_access_request(&request)
            .await
            .map_err(|e| ComplianceManagementError::GdprError(e.to_string()))?;
        
        Ok(request)
    }
    
    /// Grant HIPAA access permission
    pub async fn grant_hipaa_access(
        &self,
        user_id: Uuid,
        role: AccessRole,
        phi_categories: Vec<PhiCategory>,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<AccessPermission, ComplianceManagementError> {
        let permission = AccessPermission::new(user_id, role, phi_categories, expires_at);
        
        self.hipaa_repository.save_access_permission(&permission)
            .await
            .map_err(|e| ComplianceManagementError::HipaaError(e.to_string()))?;
        
        Ok(permission)
    }
    
    /// Check if user has access to a PHI category
    pub async fn has_hipaa_access(
        &self,
        user_id: Uuid,
        category: &PhiCategory,
    ) -> Result<bool, ComplianceManagementError> {
        let permissions = self.hipaa_repository.get_permissions_by_user(user_id)
            .await
            .map_err(|e| ComplianceManagementError::HipaaError(e.to_string()))?;
        
        // Check if any permission grants access to this category
        for permission in permissions {
            if permission.has_access_to(category) {
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    /// Revoke HIPAA access permission
    pub async fn revoke_hipaa_access(
        &self,
        permission_id: Uuid,
    ) -> Result<(), ComplianceManagementError> {
        let mut permission = self.hipaa_repository.get_permission(permission_id)
            .await
            .map_err(|e| ComplianceManagementError::HipaaError(e.to_string()))?;
        
        permission.revoke();
        
        self.hipaa_repository.save_access_permission(&permission)
            .await
            .map_err(|e| ComplianceManagementError::HipaaError(e.to_string()))
    }
    
    /// Log a HIPAA audit entry
    pub async fn log_hipaa_audit(
        &self,
        user_id: Uuid,
        action: AuditAction,
        phi_category: Option<PhiCategory>,
        dataset_id: Option<Uuid>,
        report_id: Option<Uuid>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<AuditLogEntry, ComplianceManagementError> {
        let entry = AuditLogEntry::new(
            user_id,
            action,
            phi_category,
            dataset_id,
            report_id,
            ip_address,
            user_agent,
        );
        
        self.hipaa_repository.save_audit_log_entry(&entry)
            .await
            .map_err(|e| ComplianceManagementError::AuditError(e.to_string()))?;
        
        Ok(entry)
    }
    
    /// Get HIPAA configuration
    pub async fn get_hipaa_config(&self) -> Result<HipaaConfig, ComplianceManagementError> {
        self.hipaa_repository.get_config()
            .await
            .map_err(|e| ComplianceManagementError::HipaaError(e.to_string()))
    }
    
    /// Update HIPAA configuration
    pub async fn update_hipaa_config(
        &self,
        encryption_enabled: Option<bool>,
        audit_logging_enabled: Option<bool>,
        access_control_enabled: Option<bool>,
        data_retention_days: Option<u32>,
    ) -> Result<HipaaConfig, ComplianceManagementError> {
        let mut config = self.hipaa_repository.get_config()
            .await
            .map_err(|e| ComplianceManagementError::HipaaError(e.to_string()))?;
        
        config.update(
            encryption_enabled,
            audit_logging_enabled,
            access_control_enabled,
            data_retention_days,
        );
        
        self.hipaa_repository.save_config(&config)
            .await
            .map_err(|e| ComplianceManagementError::HipaaError(e.to_string()))?;
        
        Ok(config)
    }
}

/// Repository trait for GDPR storage
#[async_trait]
pub trait GdprRepository: Send + Sync {
    /// Save a GDPR consent record
    async fn save_consent(&self, consent: &GdprConsent) -> Result<(), ComplianceManagementError>;
    
    /// Get a GDPR consent record by ID
    async fn get_consent(&self, id: Uuid) -> Result<GdprConsent, ComplianceManagementError>;
    
    /// Get GDPR consents by user and purpose
    async fn get_consents_by_user_and_purpose(&self, user_id: Uuid, purpose: &ProcessingPurpose) -> Result<Vec<GdprConsent>, ComplianceManagementError>;
    
    /// Save a data access request
    async fn save_data_access_request(&self, request: &DataAccessRequest) -> Result<(), ComplianceManagementError>;
    
    /// Get a data access request by ID
    async fn get_data_access_request(&self, id: Uuid) -> Result<DataAccessRequest, ComplianceManagementError>;
    
    /// Get data access requests by user
    async fn get_data_access_requests_by_user(&self, user_id: Uuid) -> Result<Vec<DataAccessRequest>, ComplianceManagementError>;
}

/// Repository trait for HIPAA storage
#[async_trait]
pub trait HipaaRepository: Send + Sync {
    /// Save a HIPAA access permission
    async fn save_access_permission(&self, permission: &AccessPermission) -> Result<(), ComplianceManagementError>;
    
    /// Get a HIPAA access permission by ID
    async fn get_permission(&self, id: Uuid) -> Result<AccessPermission, ComplianceManagementError>;
    
    /// Get HIPAA access permissions by user
    async fn get_permissions_by_user(&self, user_id: Uuid) -> Result<Vec<AccessPermission>, ComplianceManagementError>;
    
    /// Save a HIPAA audit log entry
    async fn save_audit_log_entry(&self, entry: &AuditLogEntry) -> Result<(), ComplianceManagementError>;
    
    /// Get HIPAA configuration
    async fn get_config(&self) -> Result<HipaaConfig, ComplianceManagementError>;
    
    /// Save HIPAA configuration
    async fn save_config(&self, config: &HipaaConfig) -> Result<(), ComplianceManagementError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::compliance::gdpr::{ProcessingPurpose, ConsentStatus, DataAccessRequestType, DataAccessRequestStatus};
    use crate::domain::compliance::hipaa::{AccessRole, PhiCategory, AuditAction, HipaaConfig};
    
    // Mock GDPR repository for testing
    struct MockGdprRepository;
    
    #[async_trait]
    impl GdprRepository for MockGdprRepository {
        async fn save_consent(&self, _consent: &GdprConsent) -> Result<(), ComplianceManagementError> {
            Ok(())
        }
        
        async fn get_consent(&self, _id: Uuid) -> Result<GdprConsent, ComplianceManagementError> {
            Ok(GdprConsent::new(
                Uuid::new_v4(),
                ProcessingPurpose::Analytics,
                ConsentStatus::Granted,
            ))
        }
        
        async fn get_consents_by_user_and_purpose(&self, _user_id: Uuid, _purpose: &ProcessingPurpose) -> Result<Vec<GdprConsent>, ComplianceManagementError> {
            Ok(vec![GdprConsent::new(
                Uuid::new_v4(),
                ProcessingPurpose::Analytics,
                ConsentStatus::Granted,
            )])
        }
        
        async fn save_data_access_request(&self, _request: &DataAccessRequest) -> Result<(), ComplianceManagementError> {
            Ok(())
        }
        
        async fn get_data_access_request(&self, _id: Uuid) -> Result<DataAccessRequest, ComplianceManagementError> {
            Ok(DataAccessRequest::new(
                Uuid::new_v4(),
                DataAccessRequestType::DataExport,
                vec!["test_data".to_string()],
            ))
        }
        
        async fn get_data_access_requests_by_user(&self, _user_id: Uuid) -> Result<Vec<DataAccessRequest>, ComplianceManagementError> {
            Ok(vec![DataAccessRequest::new(
                Uuid::new_v4(),
                DataAccessRequestType::DataExport,
                vec!["test_data".to_string()],
            )])
        }
    }
    
    // Mock HIPAA repository for testing
    struct MockHipaaRepository;
    
    #[async_trait]
    impl HipaaRepository for MockHipaaRepository {
        async fn save_access_permission(&self, _permission: &AccessPermission) -> Result<(), ComplianceManagementError> {
            Ok(())
        }
        
        async fn get_permission(&self, _id: Uuid) -> Result<AccessPermission, ComplianceManagementError> {
            Ok(AccessPermission::new(
                Uuid::new_v4(),
                AccessRole::Analyst,
                vec![PhiCategory::Demographic],
                None,
            ))
        }
        
        async fn get_permissions_by_user(&self, _user_id: Uuid) -> Result<Vec<AccessPermission>, ComplianceManagementError> {
            Ok(vec![AccessPermission::new(
                Uuid::new_v4(),
                AccessRole::Analyst,
                vec![PhiCategory::Demographic],
                None,
            )])
        }
        
        async fn save_audit_log_entry(&self, _entry: &AuditLogEntry) -> Result<(), ComplianceManagementError> {
            Ok(())
        }
        
        async fn get_config(&self) -> Result<HipaaConfig, ComplianceManagementError> {
            Ok(HipaaConfig::default())
        }
        
        async fn save_config(&self, _config: &HipaaConfig) -> Result<(), ComplianceManagementError> {
            Ok(())
        }
    }
    
    #[tokio::test]
    async fn test_record_gdpr_consent() {
        let gdpr_repository = MockGdprRepository;
        let hipaa_repository = MockHipaaRepository;
        let service = ComplianceManagementService::new(gdpr_repository, hipaa_repository);
        
        let consent = service.record_gdpr_consent(
            Uuid::new_v4(),
            ProcessingPurpose::Analytics,
            ConsentStatus::Granted,
        ).await.unwrap();
        
        assert_eq!(consent.purpose, ProcessingPurpose::Analytics);
        assert_eq!(consent.status, ConsentStatus::Granted);
        assert!(consent.is_granted());
    }
    
    #[tokio::test]
    async fn test_update_gdpr_consent() {
        let gdpr_repository = MockGdprRepository;
        let hipaa_repository = MockHipaaRepository;
        let service = ComplianceManagementService::new(gdpr_repository, hipaa_repository);
        
        let consent = service.update_gdpr_consent(
            Uuid::new_v4(),
            ConsentStatus::Revoked,
        ).await.unwrap();
        
        assert_eq!(consent.status, ConsentStatus::Revoked);
        assert!(!consent.is_granted());
    }
    
    #[tokio::test]
    async fn test_has_gdpr_consent() {
        let gdpr_repository = MockGdprRepository;
        let hipaa_repository = MockHipaaRepository;
        let service = ComplianceManagementService::new(gdpr_repository, hipaa_repository);
        
        let has_consent = service.has_gdpr_consent(
            Uuid::new_v4(),
            ProcessingPurpose::Analytics,
        ).await.unwrap();
        
        assert!(has_consent);
    }
    
    #[tokio::test]
    async fn test_create_data_access_request() {
        let gdpr_repository = MockGdprRepository;
        let hipaa_repository = MockHipaaRepository;
        let service = ComplianceManagementService::new(gdpr_repository, hipaa_repository);
        
        let request = service.create_data_access_request(
            Uuid::new_v4(),
            DataAccessRequestType::DataExport,
            vec!["test_data".to_string()],
        ).await.unwrap();
        
        assert_eq!(request.request_type, DataAccessRequestType::DataExport);
        assert_eq!(request.status, DataAccessRequestStatus::Pending);
    }
    
    #[tokio::test]
    async fn test_grant_hipaa_access() {
        let gdpr_repository = MockGdprRepository;
        let hipaa_repository = MockHipaaRepository;
        let service = ComplianceManagementService::new(gdpr_repository, hipaa_repository);
        
        let permission = service.grant_hipaa_access(
            Uuid::new_v4(),
            AccessRole::Analyst,
            vec![PhiCategory::Demographic],
            None,
        ).await.unwrap();
        
        assert_eq!(permission.role, AccessRole::Analyst);
        assert!(permission.has_access_to(&PhiCategory::Demographic));
    }
    
    #[tokio::test]
    async fn test_has_hipaa_access() {
        let gdpr_repository = MockGdprRepository;
        let hipaa_repository = MockHipaaRepository;
        let service = ComplianceManagementService::new(gdpr_repository, hipaa_repository);
        
        let has_access = service.has_hipaa_access(
            Uuid::new_v4(),
            &PhiCategory::Demographic,
        ).await.unwrap();
        
        assert!(has_access);
    }
    
    #[tokio::test]
    async fn test_log_hipaa_audit() {
        let gdpr_repository = MockGdprRepository;
        let hipaa_repository = MockHipaaRepository;
        let service = ComplianceManagementService::new(gdpr_repository, hipaa_repository);
        
        let entry = service.log_hipaa_audit(
            Uuid::new_v4(),
            AuditAction::View,
            Some(PhiCategory::Demographic),
            Some(Uuid::new_v4()),
            None,
            Some("192.168.1.1".to_string()),
            Some("Mozilla/5.0".to_string()),
        ).await.unwrap();
        
        assert_eq!(entry.action, AuditAction::View);
        assert_eq!(entry.phi_category, Some(PhiCategory::Demographic));
        assert_eq!(entry.ip_address, Some("192.168.1.1".to_string()));
    }
    
    #[tokio::test]
    async fn test_get_and_update_hipaa_config() {
        let gdpr_repository = MockGdprRepository;
        let hipaa_repository = MockHipaaRepository;
        let service = ComplianceManagementService::new(gdpr_repository, hipaa_repository);
        
        let config = service.get_hipaa_config().await.unwrap();
        assert!(config.encryption_enabled);
        
        let updated_config = service.update_hipaa_config(
            Some(false),
            None,
            None,
            Some(365),
        ).await.unwrap();
        
        assert!(!updated_config.encryption_enabled);
        assert_eq!(updated_config.data_retention_days, 365);
    }
}