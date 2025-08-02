# CPC Payment Service Migration Guide

## Overview
The CPC payment service has been refactored to separate payment processing and cause management into distinct services:
- **PaymentService**: Handles transactions, history, and skill exchange rates
- **CauseService**: Manages cause creation, updates, listings, and features

## Service Structure Comparison

| Old CpayService Method         | New Service         | New Method Name       |
|--------------------------------|---------------------|-----------------------|
| ProcessPayment                 | PaymentService      | ProcessPayment        |
| GetTransactionHistory          | PaymentService      | GetTransactionHistory |
| GetSkillExchangeRates          | PaymentService      | GetSkillExchangeRates |
| GetFeaturedCauses              | CauseService        | GetFeaturedCauses     |
| CreateCause                    | CauseService        | CreateCause           |
| GetCause                       | CauseService        | GetCause              |
| UpdateCause                    | CauseService        | UpdateCause           |
| DeleteCause                    | CauseService        | DeleteCause           |
| ListCauses                     | CauseService        | ListCauses            |

## Migration Steps
1. Update service imports to use PaymentService and CauseService instead of CpayService
2. Replace CpayServiceClient with separate PaymentServiceClient and CauseServiceClient
3. Map method calls to the appropriate new service
4. Handle any currency/skill rate conversions using the new models

## Code Examples

### Before (monolithic service)
```rust
let mut client = CpayServiceClient::connect("http://localhost:50051").await?;
let payment_response = client.process_payment(request).await?;
let causes = client.get_featured_causes(()).await?;
```

### After (separated services)
```rust
let mut payment_client = PaymentServiceClient::connect("http://localhost:50051").await?;
let mut cause_client = CauseServiceClient::connect("http://localhost:50051").await?;

let payment_response = payment_client.process_payment(request).await?;
let causes = cause_client.get_featured_causes(()).await?;
```

## Deprecation Timeline
```mermaid
gantt
    title Service Deprecation Timeline
    dateFormat  YYYY-MM-DD
    section Transition
    CpayService deprecated     :active, 2025-08-01, 60d
    section New Services
    PaymentService available   :2025-08-01, 90d
    CauseService available     :2025-08-01, 90d
    section Removal
    CpayService removed        :2025-10-30, 1d