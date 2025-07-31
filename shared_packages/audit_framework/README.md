# Unified Audit Framework

Extends the audit capabilities from `consent_manager` into a comprehensive framework for tracking all sensitive operations across the platform, with special attention to regulatory compliance needs.

## Overview

The Unified Audit Framework provides a standardized approach to audit logging and compliance verification across all CPC applications. It extends the HIPAA-compliant patterns from the health module and provides support for multiple regulations including GDPR, PCI DSS, and SOX.

## Features

- **Standardized Audit Events**: Consistent structure for all audit events across domains
- **Multi-Regulation Support**: Built-in support for HIPAA, GDPR, PCI DSS, and other regulations
- **Compliance Verification**: Automated compliance checking and reporting
- **Encryption**: AES-256 encryption for sensitive audit data
- **Export Capabilities**: Export audit logs and compliance reports in multiple formats
- **Storage Abstraction**: Integration with the Storage Abstraction Layer for flexible storage backends
- **Real-time Monitoring**: Integration with the Event Bus System for real-time audit event processing

## Architecture

The module follows hexagonal architecture principles with clear separation of concerns:

```
Domain Layer
├── event.rs        # AuditEvent structure and metadata
├── policy.rs       # Retention/compliance rules
└── errors.rs       # Error types

Application Layer
├── service.rs      # AuditService core
└── compliance.rs   # Regulatory rule engine

Infrastructure Layer
├── storage.rs      # Sled/PostgreSQL adapters
├── encryption.rs   # Audit log encryption
└── export.rs       # Compliance reporting
```

## Usage

### Basic Usage

```rust
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

// Create storage backend
let storage = Arc::new(InMemoryStore::new());

// Create compliance engine
let compliance = Arc::new(ComplianceEngine::new(vec![
    Regulation::Hipaa,
    Regulation::Gdpr,
]));

// Create audit service
let audit_service = AuditService::new(storage, compliance);

// Record an audit event
let event = AuditEvent::new_read(
    Some("user_123".to_string()),
    "health".to_string(),
    "record:patient_456".to_string(),
    PurposeCode::ProviderAccess,
    serde_json::json!({ "record_type": "vitals" }),
);

audit_service.record_event(event).await?;

// Verify compliance
let report = audit_service.verify_compliance(Regulation::Hipaa);
println!("HIPAA Compliance: {} - {}", report.success, report.details);
```

### Health Module Integration

```rust
// Health service using Audit Framework
use audit_framework::{AuditService, AuditEvent, PurposeCode};

struct HealthService {
    audit_service: AuditService,
}

impl HealthService {
    async fn access_health_record(&self, doctor_id: &str, patient_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Business logic...
        
        // Record audit event for HIPAA compliance
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

### Finance Module Integration

```rust
// Finance service using Audit Framework
use audit_framework::{AuditService, AuditEvent, PurposeCode};

struct FinanceService {
    audit_service: AuditService,
}

impl FinanceService {
    async fn monitor_high_value_transaction(&self, user_id: &str, transaction_id: &str, amount: f64) -> Result<(), Box<dyn std::error::Error>> {
        // Record audit event for fraud detection
        let event = AuditEvent::new_create(
            Some(user_id.to_string()),
            "finance".to_string(),
            format!("transaction:{}", transaction_id),
            PurposeCode::FraudDetection,
            serde_json::json!({ "amount": amount, "risk_score": if amount > 10000.0 { 95 } else { 30 } }),
        );
        
        self.audit_service.record_event(event).await?;
        
        Ok(())
    }
}
```

## Integration Examples

See the examples directory for integration examples with:
- Basic usage
- Health module integration
- Finance module integration

Run examples with:
```bash
cargo run --example basic_usage
cargo run --example health_integration
cargo run --example finance_integration
```

## Testing

Run tests with:
```bash
cargo test
```

## Migration

See [MIGRATION.md](MIGRATION.md) for detailed migration guidance for existing modules.

## Dependencies

- **tokio**: Async runtime
- **serde**: Serialization framework
- **rust-crypto**: Cryptographic primitives
- **storage_abstraction**: Storage abstraction layer
- **tracing**: Logging and monitoring

## License

This module is part of the CPC software ecosystem and is licensed under the CPC license.