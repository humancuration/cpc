# Protobuf Service Refactor (v1.0)

## Architectural Overview
- **PaymentService**: Handles all payment processing operations
- **CauseService**: Manages cause-related operations and tracking
- **Service Separation**:
  - Enables independent scaling
  - Reduces deployment dependencies
  - Follows domain-driven design principles

## Proto Definitions
```protobuf
// payment_service.proto
service PaymentService {
  rpc ProcessPayment(PaymentRequest) returns (PaymentResponse);
  rpc GetTransactionHistory(HistoryRequest) returns (HistoryResponse);
}

// cause_service.proto
service CauseService {
  rpc CreateCause(CreateCauseRequest) returns (CreateCauseResponse);
  rpc GetCause(GetCauseRequest) returns (GetCauseResponse);
  rpc UpdateCause(UpdateCauseRequest) returns (UpdateCauseResponse);
  rpc DeleteCause(DeleteCauseRequest) returns (DeleteCauseResponse);
  rpc ListCauses(ListCausesRequest) returns (ListCausesResponse);
  rpc GetFeaturedCauses(FeaturedCausesRequest) returns (FeaturedCausesResponse);
}
```

## Migration Steps
1. **Update Dependencies**:
   ```bash
   cargo update cpay_proto
   ```
   
2. **Service Initialization**:
   ```rust
   // Before
   let cpay_service = CpayService::new(repo);
   
   // After
   let payment_service = PaymentService::new(payment_repo);
   let cause_service = CauseService::new(cause_repo);
   ```

3. **Client Usage**:
   ```rust
   // Before
   client.create_cause(request).await?;
   
   // After
   cause_client.create_cause(request).await?;
   ```

## Backward Compatibility
- Deprecated `CpayService` will remain available until 2025-12-31
- Compatibility layer logs warnings when deprecated services are used

## Future Enhancements
- Add streaming endpoints for real-time updates
- Implement service mesh for inter-service communication