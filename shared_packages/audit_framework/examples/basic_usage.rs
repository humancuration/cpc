//! Basic usage example for the audit framework

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create storage backend
    let storage = Arc::new(InMemoryStore::new());
    
    // Create compliance engine
    let compliance = Arc::new(ComplianceEngine::new(vec![
        Regulation::Hipaa,
        Regulation::Gdpr,
        Regulation::PciDss,
    ]));
    
    // Create audit service
    let audit_service = AuditService::new(storage, compliance);
    
    // Record a health record access event
    let health_event = AuditEvent::new_read(
        Some("doctor_789".to_string()),
        "health".to_string(),
        "record:patient_123".to_string(),
        PurposeCode::ProviderAccess,
        json!({ "record_type": "vitals", "access_reason": "routine_checkup" }),
    );
    
    audit_service.record_event(health_event).await?;
    
    // Record a financial transaction event
    let finance_event = AuditEvent::new_create(
        Some("user_123".to_string()),
        "finance".to_string(),
        "transaction:txn_456".to_string(),
        PurposeCode::UserView,
        json!({ "amount": 150.00, "currency": "USD", "merchant": "Grocery Store" }),
    );
    
    audit_service.record_event(finance_event).await?;
    
    // Verify HIPAA compliance
    let hipaa_report = audit_service.verify_compliance(Regulation::Hipaa);
    println!("HIPAA Compliance Report: {} - {}", hipaa_report.success, hipaa_report.details);
    
    // Verify GDPR compliance
    let gdpr_report = audit_service.verify_compliance(Regulation::Gdpr);
    println!("GDPR Compliance Report: {} - {}", gdpr_report.success, gdpr_report.details);
    
    println!("Audit framework example completed successfully!");
    Ok(())
}