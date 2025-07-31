//! Integration tests for the audit framework

use audit_framework::{
    AuditService,
    AuditEvent,
    AuditAction,
    PurposeCode,
    Regulation,
    ComplianceEngine,
};
use storage_abstraction::InMemoryStore;
use std::sync::Arc;
use serde_json::json;

struct TestAuditStorage {
    inner: InMemoryStore,
}

impl TestAuditStorage {
    fn new() -> Self {
        Self {
            inner: InMemoryStore::new(),
        }
    }
}

#[tokio::test]
async fn test_audit_event_recording() {
    // Create audit service
    let storage = Arc::new(TestAuditStorage::new());
    let compliance = Arc::new(ComplianceEngine::new(vec![Regulation::Hipaa]));
    let audit_service = AuditService::new(storage, compliance);
    
    // Create audit event
    let event = AuditEvent::new_read(
        Some("user_123".to_string()),
        "health".to_string(),
        "record:patient_456".to_string(),
        PurposeCode::ProviderAccess,
        json!({ "record_type": "vitals" }),
    );
    
    // Record the event
    let result = audit_service.record_event(event).await;
    assert!(result.is_ok());
}

#[tokio::test]
fn test_compliance_verification() {
    // Create audit service
    let storage = Arc::new(TestAuditStorage::new());
    let compliance = Arc::new(ComplianceEngine::new(vec![Regulation::Hipaa, Regulation::Gdpr]));
    let audit_service = AuditService::new(storage, compliance);
    
    // Verify HIPAA compliance
    let report = audit_service.verify_compliance(Regulation::Hipaa);
    assert!(report.success);
    assert_eq!(report.regulation, Regulation::Hipaa);
    
    // Verify GDPR compliance
    let report = audit_service.verify_compliance(Regulation::Gdpr);
    assert!(report.success);
    assert_eq!(report.regulation, Regulation::Gdpr);
    
    // Verify unsupported regulation
    let report = audit_service.verify_compliance(Regulation::PciDss);
    assert!(!report.success);
}