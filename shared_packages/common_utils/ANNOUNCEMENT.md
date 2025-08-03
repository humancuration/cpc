# Shim File Deprecation Announcement

## Deprecation Timeline
- **Announcement Date**: August 3, 2025
- **Removal Date**: October 1, 2025
- **Verification Period**: October 1-15, 2025

## Affected Modules
The following modules contain deprecated shim files that will be removed:

### Wallet Module
- `crypto_shim` → Migrate to `common_utils::crypto`
- `datetime_shim` → Migrate to `common_utils::datetime`
- `error_shim` → Migrate to `common_utils::error`

### API Integration Module
- `crypto_shim` → Migrate to `common_utils::crypto`
- `datetime_shim` → Migrate to `common_utils::datetime`
- `error_shim` → Migrate to `common_utils::error`

### CPay Core Module
- `crypto_shim` → Migrate to `common_utils::crypto`
- `datetime_shim` → Migrate to `common_utils::datetime`
- `error_shim` → Migrate to `common_utils::error`

## Migration Path
All functionality previously provided by shim files is now available in the `common_utils` crate. Update your imports:

```rust
// Replace:
// use wallet::crypto_shim;
// use api_integration::datetime_shim;
// use cpay_core::error_shim;

// With:
use common_utils::{crypto, datetime, error};
```

## Next Steps
1. Update all imports in your codebase to use `common_utils`
2. Remove any remaining references to shim files
3. Verify functionality using integration tests

## Documentation
- [Deprecation Plan](DEPRECATION_PLAN.md)
- [common_utils API Reference](docs/common_utils_api.md)
- [Migration Guide](MIGRATION_GUIDE.md)

Please complete all migrations by **September 30, 2025**. After the removal date, any remaining shim files will be permanently deleted from the codebase.