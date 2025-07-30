# Finance-Sheets Currency Internationalization Implementation Plan

## Current State Analysis

After reviewing the codebase, I've identified the following key points:

1. The `Currency` enum in `packages/domains/finance/src/domain/primitives.rs` already supports multiple currencies (USD, EUR, JPY, etc.) plus Dabloons
2. Money operations properly validate currency compatibility (no cross-currency operations allowed)
3. However, Finance-Sheets is hardcoded to USD in two critical locations:
   - `BudgetTemplateService::process_*_template()` methods
   - `ExpenseImportProcessor::process_row()`
4. User currency preferences are not being utilized - several comments in the code indicate:
   ```rust
   // For now, we'll assume a default currency (USD) for the user
   // In a real implementation, would be retrieved from user preferences
   ```

## Implementation Strategy

### 1. User Profile Enhancement

**Problem**: No centralized location for user currency preferences exists.

**Solution**:
- Extend the existing `data_sharing_preferences` table to include currency preferences
- Create a new UserPreferences service to manage these settings

**Implementation Steps**:

1. Add to `packages/domains/finance/src/infrastructure/database/models.rs`:
```rust
/// Database model for data_sharing_preferences table
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DataSharingPreferencesModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub data_sharing_enabled: bool,
    pub anonymized_data: bool,
    // NEW FIELD
    pub preferred_currency: String,  // Will store currency code like "USD", "EUR"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

2. Add to `packages/domains/finance/src/infrastructure/database/repositories.rs`:
```rust
impl DataSharingPreferencesRepository {
    // NEW METHOD
    pub async fn get_user_currency(&self, user_id: Uuid) -> Result<Currency, FinanceError> {
        let preferences = self.find_by_user_id(user_id).await?;
        match preferences {
            Some(p) => {
                // Parse the currency code string back to Currency enum
                match p.preferred_currency.as_str() {
                    "USD" => Ok(Currency::USD),
                    "EUR" => Ok(Currency::EUR),
                    // Add other currency mappings
                    _ => Ok(Currency::USD) // Default fallback
                }
            }
            None => Ok(Currency::USD) // Default fallback
        }
    }

    // NEW METHOD
    pub async fn update_user_currency(&self, user_id: Uuid, currency: Currency) -> Result<(), FinanceError> {
        let mut preferences = self.find_by_user_id(user_id).await?
            .unwrap_or_else(|| DataSharingPreferencesModel::new(user_id));
        
        preferences.preferred_currency = currency.code().to_string();
        preferences.updated_at = Utc::now();
        
        self.save(preferences).await?;
        Ok(())
    }
}
```

3. Create new service at `packages/domains/finance/src/application/user_preferences.rs`:
```rust
//! User preferences service for managing currency and other settings

use crate::domain::primitives::Currency;
use uuid::Uuid;
use std::sync::Arc;

#[async_trait]
pub trait UserPreferences {
    async fn get_preferred_currency(&self, user_id: Uuid) -> Result<Currency, String>;
    async fn set_preferred_currency(&self, user_id: Uuid, currency: Currency) -> Result<(), String>;
}

pub struct UserPreferencesService {
    preferences_repo: Arc<dyn DataSharingPreferencesRepository>,
}

impl UserPreferencesService {
    pub fn new(preferences_repo: Arc<dyn DataSharingPreferencesRepository>) -> Self {
        Self { preferences_repo }
    }
}

#[async_trait]
impl UserPreferences for UserPreferencesService {
    async fn get_preferred_currency(&self, user_id: Uuid) -> Result<Currency, String> {
        self.preferences_repo.get_user_currency(user_id)
            .await
            .map_err(|e| format!("Failed to get user currency: {}", e))
    }

    async fn set_preferred_currency(&self, user_id: Uuid, currency: Currency) -> Result<(), String> {
        self.preferences_repo.update_user_currency(user_id, currency)
            .await
            .map_err(|e| format!("Failed to update user currency: {}", e))
    }
}
```

### 2. Budget Template Service Modification

**Problem**: Hardcoded USD in template processing

**Changes to `packages/domains/sheets/src/application/budget_templates/template_service.rs`**:

1. Add currency parameter to processing methods:
```rust
fn process_monthly_budget_template(
    &self, 
    sheet: &Sheet, 
    user_id: Uuid,
    currency: Currency  // NEW PARAMETER
) -> Result<(), String> {
    // ... existing code ...
    
    // Create budget entry
    let _budget = self.budget_service.create_budget(
        user_id,
        category,
        Money::new(amount, currency),  // USE PASSED CURRENCY
        BudgetPeriod::Monthly,
        Utc::now(),
        Utc::now() + chrono::Duration::days(30),
    ).map_err(|e| format!("Failed to create budget: {}", e))?;
}
```

2. Update the main apply_template method to fetch user's currency:
```rust
pub fn apply_template(
    &self, 
    sheet: &Sheet, 
    user_id: Uuid,
    user_preferences: &dyn UserPreferences  // NEW PARAMETER
) -> Result<(), String> {
    // Get user's preferred currency
    let currency = user_preferences.get_preferred_currency(user_id)
        .map_err(|e| format!("Failed to get user currency: {}", e))?;
    
    // Identify template type from metadata
    let template_type = self.identify_template(sheet)?;
    
    match template_type.template_type {
        TemplateType::MonthlyBudget => self.process_monthly_budget_template(sheet, user_id, currency),
        TemplateType::WeeklyBudget => self.process_weekly_budget_template(sheet, user_id, currency),
        TemplateType::ProjectBudget => self.process_project_budget_template(sheet, user_id, currency),
        TemplateType::Custom => self.process_custom_template(sheet, user_id, currency),
    }
}
```

### 3. Expense Import Processor Modification

**Problem**: Hardcoded USD in expense processing

**Changes to `packages/domains/sheets/src/application/expense_import/import_processor.rs`**:

1. Add currency parameter to process_row:
```rust
async fn process_row(
    &self, 
    sheet: &Sheet, 
    mapping: &ColumnMapping, 
    row: u32, 
    user_id: Uuid,
    currency: Currency  // NEW PARAMETER
) -> Result<(), String> {
    // ... existing code ...
    
    // Create expense
    let expense = Expense::new(
        user_id,
        Money::new(amount, currency),  // USE PASSED CURRENCY
        category,
        description,
        date,
    );
    
    // ... existing code ...
}
```

2. Update main process method to fetch user's currency:
```rust
pub async fn process(
    &self, 
    sheet: &Sheet, 
    mapping: ColumnMapping, 
    user_id: Uuid,
    user_preferences: &dyn UserPreferences  // NEW PARAMETER
) -> Result<ImportResult, String> {
    // Get user's preferred currency
    let currency = user_preferences.get_preferred_currency(user_id)
        .map_err(|e| format!("Failed to get user currency: {}", e))?;
    
    // ... existing setup code ...
    
    for row in 1..=max_row {
        result.total_rows += 1;
        
        match self.process_row(sheet, &mapping, row, user_id, currency).await {
            Ok(_) => result.successful_imports += 1,
            Err(e) => {
                // ... error handling ...
            }
        }
    }
    
    Ok(result)
}
```

### 4. Testing Strategy

**New Test Files**:

1. `packages/domains/sheets/src/application/budget_templates/currency_integration_test.rs`:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Sheet;
    use crate::application::budget_templates::template_service::BudgetTemplateService;
    use crate::application::budget_templates::template_models::{TemplateType, TemplateIdentification};
    use packages::domains::finance::domain::primitives::{Money, Currency};
    use uuid::Uuid;
    use chrono::Utc;
    use mockall::mock;
    use std::sync::Arc;

    // Mock UserPreferences trait
    mock! {
        UserPreferencesService {}
        #[async_trait]
        impl UserPreferences for UserPreferencesService {
            async fn get_preferred_currency(&self, user_id: Uuid) -> Result<Currency, String>;
            async fn set_preferred_currency(&self, user_id: Uuid, currency: Currency) -> Result<(), String>;
        }
    }

    #[tokio::test]
    async fn test_budget_template_with_eur_currency() {
        // Setup
        let user_id = Uuid::new_v4();
        let mut mock_preferences = MockUserPreferencesService::new();
        mock_preferences
            .expect_get_preferred_currency()
            .returning(|_| Ok(Currency::EUR));
        
        // Create mock budget service
        let mock_budget_service = MockBudgetService::new();
        
        let service = BudgetTemplateService::new(
            Box::new(mock_budget_service),
        );
        
        // Create a simple sheet with test data
        let mut sheet = Sheet::new("Test Sheet".to_string());
        sheet.set_cell_value(0, 0, "Category".into());
        sheet.set_cell_value(0, 1, "Amount".into());
        sheet.set_cell_value(1, 0, "Food".into());
        sheet.set_cell_value(1, 1, 100.50.into());
        
        // Execute
        let result = service.apply_template(
            &sheet, 
            user_id,
            &mock_preferences
        );
        
        // Verify
        assert!(result.is_ok());
        // Would verify that EUR was used in budget creation
    }
    
    #[tokio::test]
    async fn test_budget_template_currency_mismatch() {
        // Setup
        let user_id = Uuid::new_v4();
        let mut mock_preferences = MockUserPreferencesService::new();
        mock_preferences
            .expect_get_preferred_currency()
            .returning(|_| Ok(Currency::USD));
        
        // Create mock budget service that will simulate a currency mismatch
        let mut mock_budget_service = MockBudgetService::new();
        mock_budget_service
            .expect_create_budget()
            .withf(|_, _, amount, _, _, _| amount.currency != Currency::USD)
            .returning(|_, _, _, _, _, _| Err("Currency mismatch".to_string()));
        
        let service = BudgetTemplateService::new(
            Box::new(mock_budget_service),
        );
        
        // Create a sheet
        let mut sheet = Sheet::new("Test Sheet".to_string());
        // ... setup sheet ...
        
        // Execute
        let result = service.apply_template(
            &sheet, 
            user_id,
            &mock_preferences
        );
        
        // Verify error handling
        assert!(result.is_err());
    }
}
```

2. `packages/domains/sheets/src/application/expense_import/currency_integration_test.rs`:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Sheet, CellValue};
    use packages::domains::finance::domain::primitives::Currency;
    use uuid::Uuid;
    use mockall::mock;
    use std::sync::Arc;

    mock! {
        UserPreferencesService {}
        #[async_trait]
        impl UserPreferences for UserPreferencesService {
            async fn get_preferred_currency(&self, user_id: Uuid) -> Result<Currency, String>;
            async fn set_preferred_currency(&self, user_id: Uuid, currency: Currency) -> Result<(), String>;
        }
    }

    #[tokio::test]
    async fn test_import_with_jpy_currency() {
        // Setup
        let user_id = Uuid::new_v4();
        let mut mock_preferences = MockUserPreferencesService::new();
        mock_preferences
            .expect_get_preferred_currency()
            .returning(|_| Ok(Currency::JPY));
        
        // Create test sheet
        let mut sheet = Sheet::new("Test Import".to_string());
        sheet.set_cell_value(0, 0, "Date".into());
        sheet.set_cell_value(0, 1, "Amount".into());
        sheet.set_cell_value(0, 2, "Category".into());
        sheet.set_cell_value(1, 0, "2023-07-01".into());
        sheet.set_cell_value(1, 1, 1500.0.into()); // 1500 JPY
        sheet.set_cell_value(1, 2, "Food".into());
        
        // Create mapping
        let mapping = ColumnMapping {
            date_column: "A".to_string(),
            amount_column: "B".to_string(),
            category_column: "C".to_string(),
            description_column: None,
        };
        
        // Create mock expense service
        let mock_expense_service = MockExpenseService::new();
        
        let processor = ExpenseImportProcessor::new(
            Box::new(mock_expense_service),
        );
        
        // Execute
        let result = processor.process(
            &sheet,
            mapping,
            user_id,
            &mock_preferences
        ).await;
        
        // Verify
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.successful_imports, 1);
        // Would verify JPY was used in expense creation
    }
}
```

## Implementation Plan Summary

1. **Phase 1: Infrastructure Setup (1 day)**
   - Extend database schema for user preferences
   - Implement UserPreferences service
   - Add necessary dependency injections

2. **Phase 2: Core Logic Modification (2 days)**
   - Update BudgetTemplateService to use dynamic currency
   - Update ExpenseImportProcessor to use dynamic currency
   - Add necessary error handling

3. **Phase 3: Testing (1.5 days)**
   - Implement comprehensive test suite
   - Verify edge cases (currency mismatch, invalid currencies)
   - Add integration tests with multiple currency scenarios

4. **Phase 4: UI Integration (0.5 days)**
   - Add currency selection UI component
   - Connect UI to UserPreferences service

## Priority Justification

This is correctly prioritized as #1 because:
1. International users cannot properly use the application with hardcoded USD
2. Currency handling is foundational to financial applications
3. Fixing this early prevents technical debt from accumulating
4. It enables proper testing of financial calculations in different currencies

## Mobile Optimization Plan (Brief Overview)

For Mobile Optimization (#3 priority), we'll:
1. Create responsive breakpoints for all sheet UI components
2. Implement touch-friendly controls:
   - Larger touch targets for cell selection
   - Swipe gestures for row/column navigation
   - Long-press context menus
3. Optimize formula input with mobile-friendly keyboard
4. Create mobile-specific error dialogs with clear actions

## Dashboard Integration Plan (Brief Overview)

For Dashboard Integration (#4 priority), we'll:
1. Implement FinanceAggregator service to collect financial data
2. Create DashboardService with event handlers
3. Add real-time data updates between Sheets and Dashboard
4. Implement data request and cell value update protocols

## Test Implementation Plan (Brief Overview)

For Test Implementation (#2 priority), we'll:
1. Create comprehensive test files for all currency scenarios
2. Include mobile interaction tests using touch simulation
3. Add responsive layout tests for different screen sizes
4. Implement financial calculation edge cases (rounding, precision)

<new_task>
<mode>ougcode</mode>
<message>
We need to implement the Currency Internationalization feature for Finance-Sheets as the top priority. Here are the detailed implementation instructions:

# Task: Currency Internationalization Implementation

## Goal
Replace hardcoded USD with dynamic user-preferred currency in all Finance-Sheets operations.

## Files to Modify

### 1. Database Schema Extension
- Target file: `packages/domains/finance/src/infrastructure/database/models.rs`
- Changes: Add `preferred_currency` field to `DataSharingPreferencesModel`

### 2. Repository Updates
- Target file: `packages/domains/finance/src/infrastructure/database/repositories.rs`
- Changes: Add methods `get_user_currency` and `update_user_currency`

### 3. User Preferences Service
- Target file: `packages/domains/finance/src/application/user_preferences.rs` (NEW FILE)
- Changes: Implement full UserPreferences service

### 4. Budget Template Service
- Target file: `packages/domains/sheets/src/application/budget_templates/template_service.rs`
- Changes: 
  - Modify all `process_*_template` methods to accept currency parameter
  - Update `apply_template` to fetch user's currency

### 5. Expense Import Processor
- Target file: `packages/domains/sheets/src/application/expense_import/import_processor.rs`
- Changes:
  - Modify `process_row` to accept currency parameter
  - Update `process` to fetch user's currency

### 6. Test Implementation
- New files:
  - `packages/domains/sheets/src/application/budget_templates/currency_integration_test.rs`
  - `packages/domains/sheets/src/application/expense_import/currency_integration_test.rs`
- Changes:
  - Add comprehensive tests for multiple currency scenarios
  - Include error handling tests for currency mismatches

## Implementation Guidelines

1. **User Preferences**:
   - Extend existing data_sharing_preferences table (don't create new tables)
   - Store currency as string code (e.g., "USD", "EUR") but convert to Currency enum when used
   - Provide proper error handling and default to USD when preferences not found

2. **Budget Template Service**:
   - Maintain all existing functionality
   - Only change currency handling
   - Add proper error messages for currency-related issues

3. **Expense Import Processor**:
   - Same as Budget Template Service - only change currency handling
   - Ensure date parsing and other functionality remains intact

4. **Testing**:
   - Implement at least 5 test cases per component
   - Include tests for:
     - Different currency formats (USD, EUR, JPY)
     - Currency mismatch scenarios
     - Edge cases (zero amounts, negative amounts)
     - Database errors

## Acceptance Criteria

1. Users can set and retrieve their preferred currency
2. Budget templates correctly process amounts in user's preferred currency
3. Expense imports correctly process amounts in user's preferred currency
4. Comprehensive test coverage (80%+)
5. No regression in existing functionality

## Priority
This is the highest priority task as it's foundational for international users.

## References
- Review `packages/domains/finance/src/domain/primitives.rs` for Currency enum implementation
- Check existing comments about user preferences in database repositories
- Use the hardcoded USD instances as reference points for changes

Please implement this with care for backward compatibility and thorough testing.
</message>
</new_task>