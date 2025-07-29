//! Finance module integration example for the audit framework

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

/// Finance service that uses the audit framework
struct FinanceService {
    audit_service: AuditService,
}

impl FinanceService {
    /// Create a new finance service
    pub fn new(audit_service: AuditService) -> Self {
        Self { audit_service }
    }
    
    /// Create a new transaction
    pub async fn create_transaction(
        &self,
        user_id: &str,
        transaction_id: &str,
        amount: f64,
        merchant: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Business logic would go here
        println!("Creating transaction for user {}: ${} at {}", user_id, amount, merchant);
        
        // Record audit event
        let event = AuditEvent::new_create(
            Some(user_id.to_string()),
            "finance".to_string(),
            format!("transaction:{}", transaction_id),
            PurposeCode::UserView,
            json!({ "amount": amount, "currency": "USD", "merchant": merchant }),
        );
        
        self.audit_service.record_event(event).await?;
        
        Ok(())
    }
    
    /// Monitor high-value transactions for fraud detection
    pub async fn monitor_high_value_transaction(
        &self,
        user_id: &str,
        transaction_id: &str,
        amount: f64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Record audit event for fraud detection
        let event = AuditEvent::new_create(
            Some(user_id.to_string()),
            "finance".to_string(),
            format!("transaction:{}", transaction_id),
            PurposeCode::FraudDetection,
            json!({ "amount": amount, "risk_score": if amount > 10000.0 { 95 } else { 30 } }),
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
        Regulation::PciDss,
    ]));
    
    // Create audit service
    let audit_service = AuditService::new(storage, compliance);
    
    // Create finance service
    let finance_service = FinanceService::new(audit_service.clone());
    
    // Create a regular transaction
    finance_service.create_transaction("user_123", "txn_456", 25.50, "Coffee Shop").await?;
    
    // Create a high-value transaction
    finance_service.create_transaction("user_123", "txn_789", 15000.00, "Car Dealer").await?;
    finance_service.monitor_high_value_transaction("user_123", "txn_789", 15000.00).await?;
    
    // Verify PCI DSS compliance
    let report = audit_service.verify_compliance(Regulation::PciDss);
    println!("PCI DSS Compliance: {} - {}", report.success, report.details);
    
    println!("Finance integration example completed successfully!");
    Ok(())
}