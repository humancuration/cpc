# Plan to Fix Compilation Issues in Shared Packages

## Overview
This document outlines the plan to fix compilation issues in the shared packages to ensure apps can compile successfully. The issues identified are:

1. Invalid version specification for bb8-redis in redis_utils
2. Duplicated content in db_pool/src/lib.rs
3. Duplicated content in redis_utils/src/lib.rs
4. Placeholder implementations in db_abstraction/src/repositories.rs

## Issue 1: Fix bb8-redis version in redis_utils/Cargo.toml

### Problem
In `shared_packages/redis_utils/Cargo.toml`, line 10 has an invalid version specification "0..0" for bb8-redis dependency.

### Solution
Update the version to "0.24.0" to match the workspace dependencies defined in the root Cargo.toml.

### Files to modify
- shared_packages/redis_utils/Cargo.toml

## Issue 2: Remove duplicated content in db_pool/src/lib.rs

### Problem
The content in `shared_packages/db_pool/src/lib.rs` is duplicated. Lines 1-79 are repeated in lines 80-158.

### Solution
Remove the duplicated content (lines 80-158) while preserving the module structure and tests.

### Files to modify
- shared_packages/db_pool/src/lib.rs

## Issue 3: Remove duplicated content in redis_utils/src/lib.rs

### Problem
The content in `shared_packages/redis_utils/src/lib.rs` is duplicated. Lines 1-57 are repeated in lines 58-114.

### Solution
Remove the duplicated content (lines 58-114) while preserving the module structure and tests.

### Files to modify
- shared_packages/redis_utils/src/lib.rs

## Issue 4: Implement proper database connection handling in db_abstraction/src/repositories.rs

### Problem
The implementations in `shared_packages/db_abstraction/src/repositories.rs` are placeholders that return "Not implemented" errors.

### Solution
Implement proper database connection handling using the db_pool package:
1. Implement the ConnectionPool trait for both PostgreSQL and SQLite pool wrappers
2. Implement the DatabaseConnection trait for database connections
3. Implement the Transaction functionality
4. Implement the CRUD operations for User and Entity repositories

### Files to modify
- shared_packages/db_abstraction/src/repositories.rs

## Implementation Order
1. Fix the bb8-redis version in redis_utils/Cargo.toml
2. Remove duplicated content in redis_utils/src/lib.rs
3. Remove duplicated content in db_pool/src/lib.rs
4. Implement proper database connection handling in db_abstraction/src/repositories.rs

## Testing
After implementing these fixes, verify that the finance, sheets, and crm apps can compile successfully with their newly added dependencies.