# Android Currency Integration Test Plan

## Overview
This document outlines the testing strategy for the Android currency integration feature, ensuring comprehensive coverage of all components and scenarios.

## Test Categories

### 1. UI Layer Tests (Instrumentation)

#### 1.1 Currency Selection Dialog
- **Test Case**: Verify dropdown shows all currencies
  - **Steps**: 
    1. Open currency preference screen
    2. Check that spinner contains all supported currencies
  - **Expected Result**: All currencies from `Currency::all()` are displayed

- **Test Case**: Verify UI updates instantly when selection changes
  - **Steps**: 
    1. Select a currency from the dropdown
    2. Observe immediate UI feedback
  - **Expected Result**: UI updates without delay

- **Test Case**: Verify accessibility compliance (TalkBack support)
  - **Steps**: 
    1. Enable TalkBack
    2. Navigate through currency preference screen
  - **Expected Result**: All elements are properly announced

#### 1.2 Sync Status Indicator
- **Test Case**: Verify sync status indicator visibility
  - **Steps**: 
    1. Open currency preference screen
    2. Check sync status icon visibility
  - **Expected Result**: Sync status icon is visible and properly positioned

- **Test Case**: Verify sync status updates correctly
  - **Steps**: 
    1. Change currency preference
    2. Observe sync status icon change
  - **Expected Result**: Icon changes to indicate sync status

### 2. FFI Layer Tests (Unit Tests)

#### 2.1 Error Handling
- **Test Case**: Verify error cases are handled properly
  - **Steps**: 
    1. Call FFI functions with invalid parameters
    2. Check error handling
  - **Expected Result**: Proper error codes returned to Kotlin

- **Test Case**: Verify boundary conditions
  - **Steps**: 
    1. Call FFI functions with edge case parameters
    2. Check behavior
  - **Expected Result**: Functions handle edge cases gracefully

#### 2.2 Thread Safety
- **Test Case**: Verify async operations are thread-safe
  - **Steps**: 
    1. Call FFI functions from multiple threads simultaneously
    2. Check for race conditions
  - **Expected Result**: No data corruption or crashes

### 3. Sled Adapter Tests (Integration)

#### 3.1 Basic Operations
- **Test Case**: Verify write/read operations
  - **Steps**: 
    1. Set currency preference
    2. Retrieve currency preference
  - **Expected Result**: Retrieved value matches set value

- **Test Case**: Verify conflict resolution
  - **Steps**: 
    1. Create conflicting updates
    2. Check resolution logic
  - **Expected Result**: Conflicts resolved according to timestamp strategy

#### 3.2 Offline Capability
- **Test Case**: Verify operations work without network
  - **Steps**: 
    1. Disable network
    2. Perform currency preference operations
  - **Expected Result**: Operations succeed using local storage

### 4. gRPC Client Tests (Mock Server)

#### 4.1 Network Failure Scenarios
- **Test Case**: Verify retry logic with exponential backoff
  - **Steps**: 
    1. Configure mock server to fail intermittently
    2. Perform currency preference operations
  - **Expected Result**: Operations retry and eventually succeed

- **Test Case**: Verify authentication handling
  - **Steps**: 
    1. Perform operations with valid/invalid JWT
    2. Check authentication behavior
  - **Expected Result**: Valid tokens work, invalid tokens fail gracefully

#### 4.2 Network Status Monitoring
- **Test Case**: Verify network status detection
  - **Steps**: 
    1. Change network status
    2. Check network monitoring
  - **Expected Result**: Network status changes are detected

### 5. Expense Flow Tests (E2E)

#### 5.1 Import with Multiple Currencies
- **Test Case**: Verify import with USD currency
  - **Steps**: 
    1. Set currency preference to USD
    2. Import expense file
  - **Expected Result**: All expenses processed with USD currency

- **Test Case**: Verify import with EUR currency
  - **Steps**: 
    1. Set currency preference to EUR
    2. Import expense file
  - **Expected Result**: All expenses processed with EUR currency

- **Test Case**: Verify import with JPY currency
  - **Steps**: 
    1. Set currency preference to JPY
    2. Import expense file
  - **Expected Result**: All expenses processed with JPY currency

#### 5.2 Validation
- **Test Case**: Verify currency validation before processing
  - **Steps**: 
    1. Set invalid currency preference
    2. Attempt to import expenses
  - **Expected Result**: Import fails with validation error

#### 5.3 Fallback Behavior
- **Test Case**: Verify fallback when currency unavailable
  - **Steps**: 
    1. Remove currency preference
    2. Attempt to import expenses
  - **Expected Result**: Import uses default currency (USD) with warning

## Test Matrix

| Component | Test Type | Test Cases | Status |
|-----------|-----------|------------|--------|
| UI Layer | Instrumentation | 3 | ✅ |
| FFI Layer | Unit Tests | 2 | ✅ |
| Sled Adapter | Integration | 2 | ✅ |
| gRPC Client | Mock Server | 2 | ✅ |
| Expense Flow | E2E | 3 | ✅ |

## Acceptance Criteria Verification

| Criteria | Status |
|----------|--------|
| ✅ Users can change currency preference in settings | ✅ |
| ✅ Preference changes immediately affect expense import | ✅ |
| ✅ All operations work offline with local storage | ✅ |
| ✅ Changes sync automatically when network available | ⬜ |
| ✅ No data loss during network interruptions | ⬜ |
| ✅ Comprehensive test coverage (80%+) | ⬜ |

## Performance Benchmarks

| Metric | Target | Current |
|--------|--------|---------|
| Cold start impact of Sled initialization | < 100ms | ⬜ |
| Currency preference save time | < 50ms | ⬜ |
| Expense import time (1000 rows) | < 5s | ⬜ |

## Security Validation

| Check | Status |
|-------|--------|
| Currency codes validated against allowed list | ⬜ |
| JWT authentication properly implemented | ⬜ |
| No sensitive data leakage | ⬜ |