//! Health module integration example for the audit framework

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

/// Health service that uses the audit framework
struct HealthService {
    audit_service: AuditService,
}

impl HealthService {
    /// Create a new health service
    pub fn new(audit_service: AuditService) -> Self {
        Self { audit_service }
    }
    
    /// Access a health record
    pub async fn access_health_record(
        &self,
        doctor_id: &str,
        patient_id: &str,
        record_type: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Business logic would go here
        println!("Doctor {} accessing health record for patient {}", doctor_id, patient_id);
        
        // Record audit event for HIPAA compliance
        let event = AuditEvent::new_read(
            Some(doctor_id.to_string()),
            "health".to_string(),
            format!("record:{}", patient_id),
            PurposeCode::ProviderAccess,
            json!({ "record_type": record_type }),
        );
        
        self.audit_service.record_event(event).await?;
        
        Ok(())
    }
    
    /// Verify HIPAA compliance
    pub fn verify_hipaa_compliance(&self) -> audit_framework::domain::policy::ComplianceReport {
        self.audit_service.verify_compliance(Regulation::Hipaa)
    }
}

/// Fraud detection service
struct FraudDetectionService {
    audit_service: AuditService,
}

impl FraudDetectionService {
    /// Create a new fraud detection service
    pub fn new(audit_service: AuditService) -> Self {
        Self { audit_service }
    }
    
    /// Monitor high-value transactions
    pub async fn monitor_high_value_transaction(
        &self,
        user_id: &str,
        transaction_id: &str,
        risk_score: f64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Record audit event for fraud detection
        let event = AuditEvent::new_create(
            Some(user_id.to_string()),
            "health".to_string(),
            format!("transaction:{}", transaction_id),
            PurposeCode::FraudDetection,
            json!({ "risk_score": risk_score }),
        );
        
        self.audit_service.record_event(event).await?;
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create storage backend
    let storage = Arc::new(InMemoryStore::new());
    
    // Create compliance engine
    let compliance = Arc::new(ComplianceEngine::new(vec![
        Regulation::Hipaa,
        Regulation::Gdpr,
    ]));
    
    // Create audit service
    let audit_service = AuditService::new(storage, compliance);
    
    // Create health service
    let health_service = HealthService::new(audit_service.clone());
    
    // Create fraud detection service
    let fraud_service = FraudDetectionService::new(audit_service.clone());
    
    // Access a health record
    health_service.access_health_record("doctor_789", "patient_123", "vitals").await?;
    
    // Monitor a high-risk transaction
    fraud_service.monitor_high_value_transaction("user_123", "txn_456", 85.5).await?;
    
    // Verify HIPAA compliance
    let report = health_service.verify_hipaa_compliance();
    println!("HIPAA Compliance: {} - {}", report.success, report.details);
    
    println!("Health integration example completed successfully!");
    Ok(())
}