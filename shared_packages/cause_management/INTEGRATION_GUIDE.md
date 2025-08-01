# Cause Management Service Integration Guide

## Overview

This guide explains how to integrate the Cause Management Service with the existing CPay Core service and other components of the CPC platform.

## Service Architecture

The Cause Management Service is designed as a separate microservice that works alongside the CPay Core service. Both services share the same proto definitions but handle different aspects of the platform:

- **CPay Core**: Handles payment processing, transaction history, and financial operations
- **Cause Management**: Handles cause creation, management, and donation tracking

## Integration Points

### 1. Shared Protocol Buffers

Both services use the same `cpay.proto` definition file, which allows them to:
- Share message definitions (Cause, PaymentRequest, etc.)
- Maintain consistent API interfaces
- Enable service-to-service communication

### 2. Database Integration

While each service manages its own database tables, they can reference each other's data:
- CPay Core transactions can reference cause IDs
- Cause Management service can track donations from transaction data

### 3. gRPC Communication

Services communicate through gRPC calls:
- CPay Core can call Cause Management to retrieve cause details
- Cause Management can call CPay Core to get transaction data for donation calculations

## Implementation Example

### Using the Cause Management Service

```rust
use cause_management::{CauseManagementServiceImpl, service::CauseServiceImpl, repository::PostgresCauseRepository};
use sqlx::PgPool;
use std::sync::Arc;

// Database setup
let pool = PgPool::connect("postgresql://localhost:5432/cpc").await?;

// Repository setup
let cause_repository = Arc::new(PostgresCauseRepository::new(pool));

// Service setup
let cause_service = Arc::new(CauseServiceImpl::new(cause_repository));
let cause_management_service = CauseManagementServiceImpl::new(cause_service);

// Start the gRPC server
let addr = "0.0.0.0:50051".parse()?;
cause_management_service.start_grpc_server(addr).await?;
```

### Calling from CPay Core

```rust
// In cpay_core, you can call the cause management service
use cause_management::proto::cpay_service_client::CpayServiceClient;
use cause_management::proto::{GetCauseRequest, Cause};

let mut client = CpayServiceClient::connect("http://localhost:50051").await?;
let request = tonic::Request::new(GetCauseRequest {
    cause_id: "some-cause-id".to_string(),
});

let response = client.get_cause(request).await?;
let cause: Cause = response.into_inner().cause;
```

## Database Schema Integration

### CPay Core Tables
- `traditional_currency_transactions` - Stores payment transaction data
- References to `cause_id` for donations

### Cause Management Tables
- `causes` - Stores cause information and donation tracking
- Can be joined with transaction data for reporting

## Error Handling

Both services follow consistent error handling patterns:
- Use of `thiserror` for structured error types
- Proper gRPC status codes for service-to-service communication
- Detailed error messages for debugging

## Deployment Considerations

### Separate Deployments
Each service should be deployed independently:
- CPay Core: Handles high-volume payment processing
- Cause Management: Handles cause-related operations
- Allows for independent scaling and maintenance

### Shared Dependencies
- Both services use the same proto definitions
- Both services connect to the same database cluster
- Both services follow the same logging and monitoring patterns

## Testing Integration

### Unit Tests
Each service has independent unit tests that don't require the other service to be running.

### Integration Tests
For end-to-end testing, both services need to be running:
1. Start CPay Core service
2. Start Cause Management service
3. Run integration tests that verify cross-service communication

## Monitoring and Observability

Both services use the same tracing infrastructure:
- Consistent logging formats
- Distributed tracing support
- Metrics collection