//! Unit tests for audit log functionality
//!
//! This module contains unit tests for the audit log domain model and related functionality.

#[cfg(test)]
mod tests {
    use super::super::audit_log::*;
    use std::net::IpAddr;
    use std::str::FromStr;
    use uuid::Uuid;
    use chrono::Utc;

    #[test]
    fn test_audit_log_creation() {
        let user_id = Some(Uuid::new_v4());
        let data_type = "VitalSign";
        let data_id = Uuid::new_v4();
        let access_type = "Read";
        let purpose = "UserView";
        let source_ip = Some(IpAddr::from_str("192.168.1.1").unwrap());
        let device_info = Some("Test Device".to_string());

        let audit_log = AuditLog::new(
            user_id,
            data_type,
            data_id,
            access_type,
            purpose,
            source_ip,
            device_info.clone(),
        );

        assert_eq!(audit_log.user_id, user_id);
        assert_eq!(audit_log.data_type, data_type);
        assert_eq!(audit_log.data_id, data_id);
        assert_eq!(audit_log.access_type, access_type);
        assert_eq!(audit_log.purpose, purpose);
        assert_eq!(audit_log.source_ip, source_ip);
        assert_eq!(audit_log.device_info, device_info);
        assert!(audit_log.accessed_at <= Utc::now());
        assert!(audit_log.created_at <= Utc::now());
    }

    #[test]
    fn test_audit_log_validation() {
        // Valid audit log
        let valid_audit_log = AuditLog::new(
            Some(Uuid::new_v4()),
            "VitalSign",
            Uuid::new_v4(),
            "Read",
            "UserView",
            None,
            None,
        );
        assert!(valid_audit_log.validate().is_ok());

        // Invalid audit log - empty data type
        let mut invalid_audit_log = valid_audit_log.clone();
        invalid_audit_log.data_type = "".to_string();
        assert!(matches!(
            invalid_audit_log.validate(),
            Err(ValidationError::InvalidDataType)
        ));

        // Invalid audit log - empty access type
        let mut invalid_audit_log = valid_audit_log.clone();
        invalid_audit_log.access_type = "".to_string();
        assert!(matches!(
            invalid_audit_log.validate(),
            Err(ValidationError::InvalidAccessType)
        ));

        // Invalid audit log - empty purpose
        let mut invalid_audit_log = valid_audit_log.clone();
        invalid_audit_log.purpose = "".to_string();
        assert!(matches!(
            invalid_audit_log.validate(),
            Err(ValidationError::InvalidPurpose)
        ));
    }

    #[test]
    fn test_audit_purpose_as_str() {
        assert_eq!(AuditPurpose::UserView.as_str(), "UserView");
        assert_eq!(AuditPurpose::ProviderAccess.as_str(), "ProviderAccess");
        assert_eq!(AuditPurpose::Research.as_str(), "Research");
        assert_eq!(AuditPurpose::DataSync.as_str(), "DataSync");
        assert_eq!(AuditPurpose::Admin.as_str(), "Admin");
        assert_eq!(AuditPurpose::Maintenance.as_str(), "Maintenance");
    }

    #[test]
    fn test_access_type_as_str() {
        assert_eq!(AccessType::Read.as_str(), "Read");
        assert_eq!(AccessType::Write.as_str(), "Write");
        assert_eq!(AccessType::Delete.as_str(), "Delete");
        assert_eq!(AccessType::Export.as_str(), "Export");
    }
    
    #[test]
    fn test_access_attempt_type_as_str() {
        assert_eq!(AccessAttemptType::Success.as_str(), "Success");
        assert_eq!(AccessAttemptType::FailedDualAuth.as_str(), "FailedDualAuth");
        assert_eq!(AccessAttemptType::InvalidCredentials.as_str(), "InvalidCredentials");
        assert_eq!(AccessAttemptType::AccountLocked.as_str(), "AccountLocked");
    }
    
    #[test]
    fn test_audit_log_new_auth_attempt() {
        let user_id = Some(Uuid::new_v4());
        let attempt_type = AccessAttemptType::FailedDualAuth;
        let correlation_id = Uuid::new_v4();
        let risk_score = 25;
        let failure_reason = Some("Test failure reason".to_string());
        let source_ip = Some(IpAddr::from_str("192.168.1.1").unwrap());
        let device_info = Some("Test Device".to_string());
        
        let audit_log = AuditLog::new_auth_attempt(
            user_id,
            attempt_type.clone(),
            correlation_id,
            risk_score,
            failure_reason.clone(),
            source_ip,
            device_info.clone(),
        );
        
        assert_eq!(audit_log.user_id, user_id);
        assert_eq!(audit_log.data_type, "AuthenticationAttempt");
        assert_eq!(audit_log.data_id, Uuid::nil());
        assert_eq!(audit_log.access_type, "Authentication");
        assert_eq!(audit_log.purpose, "SecurityMonitoring");
        assert_eq!(audit_log.attempt_type, attempt_type.as_str());
        assert_eq!(audit_log.attempt_correlation_id, correlation_id);
        assert_eq!(audit_log.risk_score, risk_score);
        assert_eq!(audit_log.failure_reason, failure_reason);
        assert_eq!(audit_log.source_ip, source_ip);
        assert_eq!(audit_log.device_info, device_info);
        assert!(audit_log.accessed_at <= Utc::now());
        assert!(audit_log.created_at <= Utc::now());
    }
    
    #[test]
    fn test_audit_log_validation_with_risk_score() {
        // Valid audit log with risk score
        let mut valid_audit_log = AuditLog::new(
            Some(Uuid::new_v4()),
            "VitalSign",
            Uuid::new_v4(),
            "Read",
            "UserView",
            None,
            None,
        );
        // Set the new fields for validation
        valid_audit_log.attempt_type = "Success".to_string();
        valid_audit_log.attempt_correlation_id = Uuid::new_v4();
        valid_audit_log.risk_score = 50;
        valid_audit_log.failure_reason = None;
        
        assert!(valid_audit_log.validate().is_ok());
        
        // Invalid audit log - risk score too high
        let mut invalid_audit_log = valid_audit_log.clone();
        invalid_audit_log.risk_score = 150;
        assert!(matches!(
            invalid_audit_log.validate(),
            Err(ValidationError::InvalidRiskScore)
        ));
    }
}