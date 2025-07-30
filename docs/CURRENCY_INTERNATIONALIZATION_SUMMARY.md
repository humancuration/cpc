# Currency Internationalization Implementation Summary

## Overview
This implementation adds currency internationalization support to the Finance-Sheets application, allowing users to set their preferred currency for all financial operations.

## Changes Made

### 1. Database Changes
- **File**: `packages/domains/finance/src/infrastructure/database/models.rs`
  - Added `preferred_currency: String` field to `DataSharingPreferenceDbModel`
  - Updated `from_domain` and `to_domain` methods to handle the new field

- **File**: `packages/domains/finance/src/infrastructure/database/repositories.rs`
  - Added `get_user_currency` method to fetch user's preferred currency
  - Added `update_user_currency` method to update user's preferred currency
  - Implementation defaults to USD when currency is not found

- **Migration**: `migrations/20250729000001_add_data_sharing_preferences_table.sql`
  - Created new migration to add `data_sharing_preferences` table with `preferred_currency` column

### 2. Domain Model Changes
- **File**: `packages/domains/finance/src/application/savings_service.rs`
  - Added `preferred_currency: String` field to `DataSharingPreference` domain model
  - Updated `new` constructor to initialize with default "USD" currency
  - Added `get_user_currency` and `update_user_currency` methods to `DataSharingRepository` trait

### 3. Application Layer Changes
- **File**: `packages/domains/finance/src/application/user_preferences.rs` (New)
  - Created new module with `UserPreferences` trait
  - Implemented `UserPreferencesService` for managing user currency preferences

- **File**: `packages/domains/finance/src/application/mod.rs`
  - Added `user_preferences` module export

### 4. Sheets Application Changes
- **File**: `packages/domains/sheets/src/application/budget_templates/template_service.rs`
  - Modified `apply_template` method to accept `UserPreferences` parameter
  - Updated all template processing methods to accept and use currency parameter
  - Replaced hardcoded "USD" with dynamic currency from user preferences

- **File**: `packages/domains/sheets/src/application/expense_import/import_processor.rs`
  - Modified `process` method to accept `UserPreferences` parameter
  - Updated `process_row` method to accept and use currency parameter
  - Replaced hardcoded "USD" with dynamic currency from user preferences

- **File**: `packages/domains/sheets/src/application/expense_import/import_service.rs`
  - Updated `process` method signature to pass `UserPreferences` to processor

### 5. Test Files
- **File**: `packages/domains/sheets/src/application/budget_templates/currency_integration_test.rs` (New)
  - Created comprehensive tests for budget template processing with different currencies
  - Tests cover USD, EUR, JPY, zero amounts, and edge cases

- **File**: `packages/domains/sheets/src/application/expense_import/currency_integration_test.rs` (New)
  - Created comprehensive tests for expense import processing with different currencies
  - Tests cover USD, EUR, JPY, zero amounts, negative amounts, and multiple currencies

## Key Features
1. **Dynamic Currency Support**: All financial operations now use the user's preferred currency
2. **Backward Compatibility**: Defaults to USD for users without explicit currency preferences
3. **Comprehensive Testing**: Tests cover multiple currencies and edge cases
4. **Database Migration**: Properly handles database schema changes
5. **Error Handling**: Graceful handling of currency-related errors

## Usage
Users can now set their preferred currency through the `UserPreferencesService`, and all budget templates and expense imports will automatically use that currency for financial operations.

## Supported Currencies
- USD (US Dollar)
- EUR (Euro)
- GBP (British Pound)
- JPY (Japanese Yen)
- CAD (Canadian Dollar)
- AUD (Australian Dollar)
- CHF (Swiss Franc)
- CNY (Chinese Yuan)
- SEK (Swedish Krona)
- NZD (New Zealand Dollar)
- MXN (Mexican Peso)
- SGD (Singapore Dollar)
- HKD (Hong Kong Dollar)
- NOK (Norwegian Krone)
- KRW (South Korean Won)
- TRY (Turkish Lira)
- RUB (Russian Ruble)
- INR (Indian Rupee)
- BRL (Brazilian Real)
- ZAR (South African Rand)
- DABLOONS (Internal platform currency)