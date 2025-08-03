# Release v1.5.0 - August 2025

## Features
- Added new transaction batching system to CPay Core
- Implemented enhanced crypto utilities in common_utils
- Improved API integration error handling

## Improvements
- Reduced API response times by 25%
- Optimized database queries for wallet operations
- Updated documentation across all modules

## Deprecation Notice
Shim files in wallet, api_integration, and cpay_core modules are deprecated and will be removed on 2025-10-01. Migrate to common_utils equivalents:
- `crypto_shim` → `common_utils::crypto`
- `datetime_shim` → `common_utils::datetime`
- `error_shim` → `common_utils::error`

## Bug Fixes
- Fixed transaction validation edge case in CPay Core
- Resolved intermittent API timeout issue
- Corrected wallet balance calculation rounding error

## Upgrade Instructions
1. Update all imports to use common_utils instead of shim files
2. Run integration tests before deployment
3. Verify database migrations complete successfully

For full details, see the [Deprecation Plan](shared_packages/common_utils/DEPRECATION_PLAN.md)