//! HIPAA compliance for the BI & Analytics module

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use thiserror::Error;

/// Error types for HIPAA compliance operations
#[derive(Error, Debug)]
pub enum HipaaError {
    #[error("Invalid PHI handling: {0}")]
    InvalidPhiHandling(String),
    
    #[error("Access control error: {0}")]
    AccessControlError(String),
    
    #[error("Audit log error: {0}")]
    AuditLogError(String),
}

/// PHI (Protected Health Information) data categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PhiCategory {
    Demographic,
    MedicalHistory,
    TreatmentInfo,
    PaymentInfo,
    DeviceData,
}

/// HIPAA security levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SecurityLevel {
    /// Public data (no PHI)
    Public,
    
    /// Limited dataset (some identifiers removed)
    Limited,
    
    /// Full PHI (requires strict access controls)
    Full,
}

/// PHI handling requirements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PhiHandling {
    pub category: PhiCategory,
    pub security_level: SecurityLevel,
    pub encryption_required: bool,
    pub access_control_required: bool,
    pub audit_logging_required: bool,
}

/// HIPAA access control roles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccessRole {
    Admin,
    Analyst,
    Researcher,
    Auditor,
}

/// HIPAA access permissions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccessPermission {
    pub id: Uuid,
    pub user_id: Uuid,
    pub role: AccessRole,
    pub phi_categories: Vec<PhiCategory>,
    pub granted_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
}

impl AccessPermission {
    /// Create a new access permission
    pub fn new(
        user_id: Uuid,
        role: AccessRole,
        phi_categories: Vec<PhiCategory>,
        expires_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            role,
            phi_categories,
            granted_at: Utc::now(),
            expires_at,
            is_active: true,
        }
    }
    
    /// Check if permission is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Utc::now() > expires_at
        } else {
            false
        }
    }
    
    /// Check if user has access to a specific PHI category
    pub fn has_access_to(&self, category: &PhiCategory) -> bool {
        self.is_active && 
        !self.is_expired() && 
        self.phi_categories.contains(category)
    }
    
    /// Revoke permission
    pub fn revoke(&mut self) {
        self.is_active = false;
    }
}

/// HIPAA audit log entry
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuditLogEntry {
    pub id: Uuid,
    pub user_id: Uuid,
    pub action: AuditAction,
    pub phi_category: Option<PhiCategory>,
    pub dataset_id: Option<Uuid>,
    pub report_id: Option<Uuid>,
    pub timestamp: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

/// HIPAA audit actions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditAction {
    View,
    Export,
    Modify,
    Delete,
    Query,
}

impl AuditLogEntry {
    /// Create a new audit log entry
    pub fn new(
        user_id: Uuid,
        action: AuditAction,
        phi_category: Option<PhiCategory>,
        dataset_id: Option<Uuid>,
        report_id: Option<Uuid>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            action,
            phi_category,
            dataset_id,
            report_id,
            timestamp: Utc::now(),
            ip_address,
            user_agent,
        }
    }
}

/// HIPAA compliance configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HipaaConfig {
    pub encryption_enabled: bool,
    pub audit_logging_enabled: bool,
    pub access_control_enabled: bool,
    pub data_retention_days: u32,
    pub last_updated: DateTime<Utc>,
}

impl HipaaConfig {
    /// Create default HIPAA configuration
    pub fn default() -> Self {
        Self {
            encryption_enabled: true,
            audit_logging_enabled: true,
            access_control_enabled: true,
            data_retention_days: 365 * 6, // 6 years
            last_updated: Utc::now(),
        }
    }
    
    /// Update configuration
    pub fn update(&mut self, 
        encryption_enabled: Option<bool>,
        audit_logging_enabled: Option<bool>,
        access_control_enabled: Option<bool>,
        data_retention_days: Option<u32>,
    ) {
        if let Some(val) = encryption_enabled {
            self.encryption_enabled = val;
        }
        if let Some(val) = audit_logging_enabled {
            self.audit_logging_enabled = val;
        }
        if let Some(val) = access_control_enabled {
            self.access_control_enabled = val;
        }
        if let Some(val) = data_retention_days {
            self.data_retention_days = val;
        }
        self.last_updated = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_access_permission() {
        let user_id = Uuid::new_v4();
        let phi_categories = vec![PhiCategory::Demographic, PhiCategory::MedicalHistory];
        
        let permission = AccessPermission::new(
            user_id,
            AccessRole::Analyst,
            phi_categories.clone(),
            None,
        );
        
        assert_eq!(permission.user_id, user_id);
        assert_eq!(permission.role, AccessRole::Analyst);
        assert_eq!(permission.phi_categories, phi_categories);
        assert!(permission.is_active);
        assert!(!permission.is_expired());
    }
    
    #[test]
    fn test_check_access() {
        let user_id = Uuid::new_v4();
        let phi_categories = vec![PhiCategory::Demographic];
        
        let permission = AccessPermission::new(
            user_id,
            AccessRole::Analyst,
            phi_categories.clone(),
            None,
        );
        
        assert!(permission.has_access_to(&PhiCategory::Demographic));
        assert!(!permission.has_access_to(&PhiCategory::MedicalHistory));
    }
    
    #[test]
    fn test_revoke_permission() {
        let user_id = Uuid::new_v4();
        let phi_categories = vec![PhiCategory::Demographic];
        
        let mut permission = AccessPermission::new(
            user_id,
            AccessRole::Analyst,
            phi_categories,
            None,
        );
        
        permission.revoke();
        assert!(!permission.is_active);
    }
    
    #[test]
    fn test_create_audit_log_entry() {
        let user_id = Uuid::new_v4();
        let dataset_id = Uuid::new_v4();
        
        let entry = AuditLogEntry::new(
            user_id,
            AuditAction::View,
            Some(PhiCategory::Demographic),
            Some(dataset_id),
            None,
            Some("192.168.1.1".to_string()),
            Some("Mozilla/5.0".to_string()),
        );
        
        assert_eq!(entry.user_id, user_id);
        assert_eq!(entry.action, AuditAction::View);
        assert_eq!(entry.phi_category, Some(PhiCategory::Demographic));
        assert_eq!(entry.dataset_id, Some(dataset_id));
        assert_eq!(entry.ip_address, Some("192.168.1.1".to_string()));
        assert_eq!(entry.user_agent, Some("Mozilla/5.0".to_string()));
    }
    
    #[test]
    fn test_hipaa_config() {
        let mut config = HipaaConfig::default();
        assert!(config.encryption_enabled);
        assert!(config.audit_logging_enabled);
        assert!(config.access_control_enabled);
        assert_eq!(config.data_retention_days, 365 * 6);
        
        config.update(
            Some(false),
            None,
            None,
            Some(365),
        );
        
        assert!(!config.encryption_enabled);
        assert_eq!(config.data_retention_days, 365);
    }
}