# Shim File Deprecation Plan

## Overview
This document outlines the plan for removing deprecated shim files after successful migration to common_utils. All modules have completed migration and no longer depend on these compatibility layers.

## Affected Modules
- Wallet
- API Integration
- CPay Core

## Timeline
- **Announcement Phase**: 2025-08-03 to 2025-09-30
- **Removal Target**: 2025-10-01 (after next release cycle)
- **Verification Period**: 2025-10-01 to 2025-10-15

## Removal Targets
### Wallet Module
- `src/crypto_shim.rs`
- `src/datetime_shim.rs`
- `src/error_shim.rs` (if unused)

### API Integration Module
- `src/crypto_shim.rs`
- `src/datetime_shim.rs`
- `src/error_shim.rs` (if unused)

### CPay Core Module
- `src/error_shim.rs`
- `src/crypto_shim.rs`
- `src/datetime_shim.rs`

## Communication Strategy
1. Add deprecation warnings to all shim files
2. Update module READMEs with migration status
3. Notify dependent teams via:
   - Internal developer channels
   - Project management tools
   - Release notes

## Verification Steps
1. Confirm no references to shim files in:
   - Cargo.toml dependencies
   - Source code imports
   - Documentation
2. Run integration tests across all modules
3. Perform final codebase search for shim references

## Removal Procedure
1. Delete shim files from each module
2. Remove module declarations from lib.rs files
3. Update Cargo.toml to remove any shim-related features
4. Verify builds and tests pass

## Project Tracking

### Milestones
- [x] Announcement distributed (2025-08-03)
- [ ] Migration completion target (2025-09-30)
- [ ] Shim removal (2025-10-01)
- [ ] Verification completion (2025-10-15)

### Verification Tasks
- [ ] Confirm no references to shim files in:
  - Cargo.toml dependencies
  - Source code imports
  - Documentation
- [ ] Run integration tests across all modules
- [ ] Perform final codebase search for shim references
- [ ] Update all documentation to reference common_utils

### Responsibilities
- **Team Leads**: Monitor migration progress
- **Developers**: Complete migrations by target date
- **QA Engineers**: Execute verification tests
- **Tech Writers**: Update documentation