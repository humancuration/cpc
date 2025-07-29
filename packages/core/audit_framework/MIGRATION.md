# Migration Guide for Audit Framework

This document provides guidance on migrating existing modules to use the new Audit Framework.

## Overview

The Audit Framework extends the audit capabilities from `consent_manager` into a comprehensive framework for tracking all sensitive operations across the platform, with special attention to regulatory compliance needs.

## Migration Steps

### 1. Update Cargo.toml

Add the audit framework dependency to your module's Cargo.toml:

```toml
[dependencies]
audit_framework = { path = "../audit_framework" }
```

### 2. Replace Direct Audit Implementations

#### Before (Health Module Example)
```rust
// Direct audit implementation
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub accessed_at: DateTime<Utc>,
    pub data_type: String,
    pub data_id: Uuid,
    pub access_type: String,
    pub purpose: String,
    pub source_ip: Option<std::net::IpAddr>,
    pub device_info: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl AuditLog {
    pub fn new(
        user_id: Option<Uuid>,
        data_type: &str,
        data_id: Uuid,
        access_type: &str,
        purpose: &str,
        source_ip: Option<std::net::IpAddr>,
        device_info: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            accessed_at: now,
            data_type: data_type.to_string(),
            data_id,
            access_type: access_type.to_string(),
            purpose: purpose.to_string(),
            source_ip,
            device_info,
            created_at: now,
        }
    }
}
```

#### After (Using Audit Framework)
```rust
// Using Audit Framework
use audit_framework::{AuditService, AuditEvent, AuditAction, PurposeCode};

struct HealthService {
    audit_service: AuditService,
}

impl HealthService {
    async fn access_health_record(&self, doctor_id: &str, patient_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Business logic...
        
        // Record audit event using Audit Framework
        let event = AuditEvent::new_read(
            Some(doctor_id.to_string()),
            "health".to_string(),
            format!("record:{}", patient_id),
            PurposeCode::ProviderAccess,
            serde_json::json!({ "record_type": "vitals" }),
        );
        
        self.audit_service.record_event(event).await?;
        
        Ok(())
    }
}
```

### 3. Update Service Initialization

#### Before (Health Module Example)
```rust
// Direct audit initialization
// No specific initialization needed, just create AuditLog instances directly
```

#### After (Using Audit Framework)
```rust
// Using Audit Framework
use audit_framework::{AuditService, ComplianceEngine};
use storage_abstraction::InMemoryStore;
use std::sync::Arc;

// Create storage backend
let storage = Arc::new(InMemoryStore::new());

// Create compliance engine
let compliance = Arc::new(ComplianceEngine::new(vec![
    audit_framework::Regulation::Hipaa,
    audit_framework::Regulation::Gdpr,
]));

// Create audit service
let audit_service = AuditService::new(storage, compliance);

// Use in service
let service = HealthService::new(audit_service);
```

## Finance Module Integration Example

```rust
// Finance service using Audit Framework
use audit_framework::{AuditService, AuditEvent, AuditAction, PurposeCode};

struct FinanceService {
    audit_service: AuditService,
}

impl FinanceService {
    async fn create_transaction(&self, user_id: &str, transaction_id: &str, amount: f64) -> Result<(), Box<dyn std::error::Error>> {
        // Business logic...
        
        // Record audit event for transaction creation
        let event = AuditEvent::new_create(
            Some(user_id.to_string()),
            "finance".to_string(),
            format!("transaction:{}", transaction_id),
            PurposeCode::UserView,
            serde_json::json!({ "amount": amount, "currency": "USD" }),
        );
        
        self.audit_service.record_event(event).await?;
        
        // Additional audit event for fraud detection on high-value transactions
        if amount > 10000.0 {
            let fraud_event = AuditEvent::new_create(
                Some(user_id.to_string()),
                "finance".to_string(),
                format!("transaction:{}", transaction_id),
                PurposeCode::FraudDetection,
                serde_json::json!({ "risk_score": 95 }),
            );
            
            self.audit_service.record_event(fraud_event).await?;
        }
        
        Ok(())
    }
}
```

## Compliance Verification

The Audit Framework provides built-in compliance verification:

```rust
use audit_framework::{AuditService, Regulation};

struct ComplianceChecker {
    audit_service: AuditService,
}

impl ComplianceChecker {
    fn check_hipaa_compliance(&self) -> audit_framework::domain::policy::ComplianceReport {
        self.audit_service.verify_compliance(Regulation::Hipaa)
    }
    
    fn check_gdpr_compliance(&self) -> audit_framework::domain::policy::ComplianceReport {
        self.audit_service.verify_compliance(Regulation::Gdpr)
    }
}
```

## Testing During Migration

Use the in-memory storage implementation for testing:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use storage_abstraction::InMemoryStore;
    use std::sync::Arc;
    use audit_framework::{AuditService, ComplianceEngine};

    #[tokio::test]
    async fn test_with_in_memory_storage() {
        let storage = Arc::new(InMemoryStore::new());
        let compliance = Arc::new(ComplianceEngine::new(vec![audit_framework::Regulation::Hipaa]));
        let audit_service = AuditService::new(storage, compliance);
        let service = MyService::new(audit_service);
        
        // Test your service logic
        // ...
    }
}
```

## Performance Considerations

1. The Audit Framework uses efficient storage mechanisms through the Storage Abstraction Layer.

2. Audit events are stored asynchronously to minimize impact on primary operations.

3. Compliance checking is done in-memory for fast verification.

## Troubleshooting

### Common Issues

1. **Serialization Errors**: Ensure all audit event metadata is properly serializable to JSON.

2. **Storage Errors**: Check that the storage backend is properly configured and accessible.

3. **Compliance Issues**: Verify that all required audit events are being recorded for your regulations.

### Logging and Monitoring

The Audit Framework uses tracing for logging. Enable tracing in your application to monitor audit operations:

```rust
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // ... rest of your application
}
```

This will provide detailed logs of audit operations, including event recording and compliance checking.