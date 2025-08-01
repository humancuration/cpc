# Protobuf Service Refactor

## Summary
Separated combined CpayService into:
- PaymentService (payment processing)
- CauseService (cause management)

## Changes
1. Updated `cpay.proto` with new service definitions
2. Refactored cause_management to implement CauseService
3. Updated cpay_core to implement PaymentService
4. Updated cpay application initialization
5. Maintained backward compatibility with deprecated service

## Migration Notes
- Old clients can still use deprecated CpayService temporarily
- New features should use PaymentService/CauseService
- Full deprecation planned for Q4 2025