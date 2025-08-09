//! Audit trail functionality for financial operations
//!
//! This module provides traits and implementations for auditing financial operations
//! using the audit_framework.

use audit_framework::domain::event::{AuditEvent, AuditAction, PurposeCode};
use audit_framework::application::service::AuditService;
use serde_json::Value as JsonValue;
use uuid::Uuid;
use std::sync::Arc;

/// Trait for types that can be audited in financial operations
pub trait FinancialAuditable {
    /// Create an audit event for this operation
    fn create_audit_event(
        &self,
        user_id: Option<String>,
        action: AuditAction,
        purpose: PurposeCode,
        metadata: JsonValue,
    ) -> AuditEvent;
}

/// Audit hook for financial operations
pub struct FinancialAuditHook {
    audit_service: Arc<dyn AuditService>,
}

impl FinancialAuditHook {
    /// Create a new financial audit hook
    pub fn new(audit_service: Arc<dyn AuditService>) -> Self {
        Self { audit_service }
    }
    
    /// Record a financial operation in the audit trail
    pub async fn record_operation(
        &self,
        user_id: Option<String>,
        operation_type: &str,
        input_data: JsonValue,
        parameters: JsonValue,
        result: JsonValue,
    ) -> Result<(), audit_framework::AuditError> {
        let metadata = serde_json::json!({
            "operation_type": operation_type,
            "input_data": input_data,
            "parameters": parameters,
            "result": result
        });
        
        let audit_event = AuditEvent::new(
            user_id,
            "finance".to_string(),
            AuditAction::Update, // Using Update for financial operations
            "financial_operation".to_string(),
            PurposeCode::UserView,
            metadata,
        );
        
        self.audit_service.record_event(audit_event).await
    }
    
    /// Record a monetary calculation in the audit trail
    pub async fn record_calculation(
        &self,
        user_id: Option<String>,
        calculation_type: &str,
        operands: Vec<String>,
        result: String,
    ) -> Result<(), audit_framework::AuditError> {
        let metadata = serde_json::json!({
            "calculation_type": calculation_type,
            "operands": operands,
            "result": result
        });
        
        let audit_event = AuditEvent::new(
            user_id,
            "finance".to_string(),
            AuditAction::Read, // Using Read for calculations
            "financial_calculation".to_string(),
            PurposeCode::UserView,
            metadata,
        );
        
        self.audit_service.record_event(audit_event).await
    }
    
    /// Record a currency conversion in the audit trail
    pub async fn record_conversion(
        &self,
        user_id: Option<String>,
        from_currency: &str,
        to_currency: &str,
        amount: String,
        rate: String,
        result: String,
    ) -> Result<(), audit_framework::AuditError> {
        let metadata = serde_json::json!({
            "from_currency": from_currency,
            "to_currency": to_currency,
            "amount": amount,
            "rate": rate,
            "result": result
        });
        
        let audit_event = AuditEvent::new(
            user_id,
            "finance".to_string(),
            AuditAction::Read,
            "currency_conversion".to_string(),
            PurposeCode::UserView,
            metadata,
        );
        
        self.audit_service.record_event(audit_event).await
    }
}

// Implementation for MonetaryAmount
impl FinancialAuditable for crate::monetary::MonetaryAmount {
    fn create_audit_event(
        &self,
        user_id: Option<String>,
        action: AuditAction,
        purpose: PurposeCode,
        metadata: JsonValue,
    ) -> AuditEvent {
        AuditEvent::new(
            user_id,
            "finance".to_string(),
            action,
            format!("monetary_amount_{}", self.currency()),
            purpose,
            metadata,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    // Note: These tests would require a mock audit service implementation
    // For now, we'll just test that the structures compile correctly
    
    #[test]
    fn test_financial_audit_hook_creation() {
        // This is a placeholder test - in a real implementation we would
        // need to mock the AuditService trait
        // let audit_hook = FinancialAuditHook::new(mock_audit_service);
        // assert!(audit_hook.is_some());
    }
    
    #[test]
    fn test_audit_event_creation() {
        use crate::currency::CurrencyCode;
        use crate::monetary::MonetaryAmount;
        use rust_decimal_macros::dec;
        
        let amount = MonetaryAmount::new(dec!(100.50), CurrencyCode::USD);
        let metadata = json!({"test": "value"});
        let audit_event = amount.create_audit_event(
            Some("user123".to_string()),
            AuditAction::Read,
            PurposeCode::UserView,
            metadata,
        );
        
        assert_eq!(audit_event.domain, "finance");
        assert_eq!(audit_event.target, "monetary_amount_USD");
    }
}