# Expense Tracker Module Architecture Design

## Overview

This document outlines the architecture design for the Expense Tracker module, which enables users to log daily expenses with dual-currency support (traditional + Dabloons) and receipt scanning capabilities. The design follows our screaming architecture principles and is implemented as a vertical slice within the finance domain.

## Architecture Principles

### Hexagonal Architecture
- Clear separation between domain, application, and infrastructure layers
- Domain layer contains pure business logic with no external dependencies
- Application layer orchestrates domain logic and defines service interfaces
- Infrastructure layer provides concrete implementations for repositories and external services

### Screaming Architecture
- Module structure clearly expresses its purpose through naming and organization
- Focused on business capability (expense tracking) rather than technical concerns
- All related functionality is co-located in a single vertical slice

### Vertical Slice Implementation
- Implemented as `packages/cpc-core/finance/src/expense_tracker/`
- Self-contained unit providing complete expense tracking functionality
- Can be developed, tested, and deployed independently

## Domain Model

```rust
// packages/cpc-core/finance/src/domain/expense_tracker/mod.rs

use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::domain::primitives::{Money, Currency};

/// Primary expense categories
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum ExpenseCategory {
    Food,
    Transportation,
    Housing,
    Utilities,
    Entertainment,
    Healthcare,
    Education,
    PersonalCare,
    Shopping,
    Travel,
    Business,
    Other(String), // Custom category with user-provided name
}

/// Expense status (for tracking processing state)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum ExpenseStatus {
    Draft,
    Processed,
    Verified,
    Rejected,
    Archived,
}

/// Represents a single expense transaction
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Expense {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: Money, // Supports dual-currency
    pub category: ExpenseCategory,
    pub date: DateTime<Utc>,
    pub description: String,
    pub status: ExpenseStatus,
    pub receipt_id: Option<Uuid>,
    pub is_recurring: bool,
    pub recurrence_pattern: Option<String>, // CRON-like pattern for recurring expenses
    pub linked_budget_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Expense {
    /// Create a new expense with minimum required fields
    pub fn new(
        user_id: Uuid,
        amount: Money,
        category: ExpenseCategory,
        date: DateTime<Utc>,
        description: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            amount,
            category,
            date,
            description,
            status: ExpenseStatus::Processed,
            receipt_id: None,
            is_recurring: false,
            recurrence_pattern: None,
            linked_budget_id: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Create a new draft expense (for receipt scanning flow)
    pub fn new_draft(
        user_id: Uuid,
        receipt_id: Uuid,
        description: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            amount: Money::new_zero(Currency::USD), // Will be populated from receipt
            category: ExpenseCategory::Other("Unprocessed".to_string()),
            date: now,
            description,
            status: ExpenseStatus::Draft,
            receipt_id: Some(receipt_id),
            is_recurring: false,
            recurrence_pattern: None,
            linked_budget_id: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Update expense amount (handles validation)
    pub fn update_amount(&mut self, amount: Money) -> Result<(), FinanceError> {
        self.amount = amount;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Link to a budget category
    pub fn link_to_budget(&mut self, budget_id: Uuid) {
        self.linked_budget_id = Some(budget_id);
        self.updated_at = Utc::now();
    }

    /// Mark as recurring with specified pattern
    pub fn mark_as_recurring(&mut self, pattern: String) {
        self.is_recurring = true;
        self.recurrence_pattern = Some(pattern);
        self.updated_at = Utc::now();
    }
}

/// Receipt data model for scanned receipts
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Receipt {
    pub id: Uuid,
    pub user_id: Uuid,
    pub image_data: ReceiptImageData,
    pub extracted_text: String,
    pub merchant_name: Option<String>,
    pub transaction_date: Option<DateTime<Utc>>,
    pub total_amount: Option<Money>,
    pub processing_status: ReceiptProcessingStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ReceiptProcessingStatus {
    Uploaded,
    Processing,
    Processed,
    Failed(String), // Error message
    Verified,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ReceiptImageData {
    LocalPath(String),  // For mobile/desktop apps with access to local storage
    Base64Data(String), // For web applications
    ReferenceId(Uuid),  // For cloud storage references
}
```

## Application Layer

### Service Interface Definitions

```rust
// packages/cpc-core/finance/src/application/expense_tracker/expense_service.rs

use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::domain::{
    expense_tracker::{Expense, ExpenseCategory, Receipt},
    primitives::Money,
    FinanceError,
};

/// Repository trait for expense persistence
#[async_trait]
pub trait ExpenseRepository {
    async fn save(&self, expense: &Expense) -> Result<(), FinanceError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Expense>, FinanceError>;
    async fn find_by_user_id(
        &self,
        user_id: Uuid,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
    ) -> Result<Vec<Expense>, FinanceError>;
    async fn delete(&self, id: Uuid) -> Result<(), FinanceError>;
    async fn update(&self, expense: &Expense) -> Result<(), FinanceError>;
}

/// Repository trait for receipt persistence
#[async_trait]
pub trait ReceiptRepository {
    async fn save(&self, receipt: &Receipt) -> Result<(), FinanceError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Receipt>, FinanceError>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Receipt>, FinanceError>;
    async fn delete(&self, id: Uuid) -> Result<(), FinanceError>;
}

/// Service trait for expense operations
#[async_trait]
pub trait ExpenseService {
    /// Create a new expense
    async fn create_expense(
        &self,
        user_id: Uuid,
        amount: Money,
        category: ExpenseCategory,
        date: DateTime<Utc>,
        description: String,
    ) -> Result<Expense, FinanceError>;

    /// Create a draft expense from a receipt (for processing flow)
    async fn create_draft_from_receipt(
        &self,
        user_id: Uuid,
        receipt_id: Uuid,
        description: String,
    ) -> Result<Expense, FinanceError>;

    /// Get all expenses for a user within a date range
    async fn get_user_expenses(
        &self,
        user_id: Uuid,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
    ) -> Result<Vec<Expense>, FinanceError>;

    /// Update an existing expense
    async fn update_expense(
        &self,
        expense_id: Uuid,
        amount: Option<Money>,
        category: Option<ExpenseCategory>,
        date: Option<DateTime<Utc>>,
        description: Option<String>,
        status: Option<ExpenseStatus>,
        linked_budget_id: Option<Uuid>,
    ) -> Result<Expense, FinanceError>;

    /// Delete an expense
    async fn delete_expense(&self, expense_id: Uuid) -> Result<(), FinanceError>;

    /// Link an expense to a budget category
    async fn link_to_budget(&self, expense_id: Uuid, budget_id: Uuid) -> Result<(), FinanceError>;

    /// Process a receipt to extract expense data
    async fn process_receipt(&self, receipt_id: Uuid) -> Result<Expense, FinanceError>;

    /// Get receipt details
    async fn get_receipt(&self, receipt_id: Uuid) -> Result<Option<Receipt>, FinanceError>;

    /// Save a new receipt
    async fn save_receipt(
        &self,
        user_id: Uuid,
        image_data: ReceiptImageData,
    ) -> Result<Receipt, FinanceError>;
}
```

### Implementation Strategy

The implementation will follow our existing pattern seen in wallet_service.rs and savings_service.rs:

1. Create `ExpenseServiceImpl` that implements `ExpenseService`
2. Accept repository dependencies through constructor
3. Implement business logic with proper error handling
4. Integrate with existing services (Wallet, Budget)

## Infrastructure Layer

### Repository Implementations

We'll implement:
- `PostgresExpenseRepository` - for database operations
- `PostgresReceiptRepository` - for receipt storage
- `BevyReceiptScanner` - for camera access and UI (using Bevy)
- `OCRService` - for text extraction from receipts
- `CategoryClassifier` - for automatic categorization
- `P2PExpenseSharing` - for secure sharing with other users

### Database Schema Updates

```sql
-- packages/cpc-core/migrations/20250728000015_add_expense_tracker_tables.sql

CREATE TABLE expenses (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    amount NUMERIC(20, 10) NOT NULL,
    currency VARCHAR(10) NOT NULL,
    dabloons_amount NUMERIC(20, 10) DEFAULT 0.0,
    category VARCHAR(50) NOT NULL,
    custom_category VARCHAR(50),
    date TIMESTAMP WITH TIME ZONE NOT NULL,
    description TEXT NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'Processed',
    receipt_id UUID,
    is_recurring BOOLEAN NOT NULL DEFAULT false,
    recurrence_pattern VARCHAR(100),
    linked_budget_id UUID,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (receipt_id) REFERENCES receipts(id) ON DELETE SET NULL,
    FOREIGN KEY (linked_budget_id) REFERENCES budgets(id) ON DELETE SET NULL
);

CREATE TABLE receipts (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    image_data BYTEA NOT NULL,
    image_format VARCHAR(10) NOT NULL,
    extracted_text TEXT NOT NULL DEFAULT '',
    merchant_name VARCHAR(100),
    transaction_date TIMESTAMP WITH TIME ZONE,
    total_amount NUMERIC(20, 10),
    currency VARCHAR(10),
    dabloons_amount NUMERIC(20, 10) DEFAULT 0.0,
    processing_status VARCHAR(20) NOT NULL DEFAULT 'Uploaded',
    processing_error TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE expense_sharing_preferences (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL UNIQUE,
    sharing_enabled BOOLEAN NOT NULL DEFAULT false,
    anonymized BOOLEAN NOT NULL DEFAULT false,
    shared_categories JSONB NOT NULL DEFAULT '[]'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Indexes for performance
CREATE INDEX idx_expenses_user_id ON expenses(user_id);
CREATE INDEX idx_expenses_date ON expenses(date);
CREATE INDEX idx_receipts_user_id ON receipts(user_id);
CREATE INDEX idx_receipts_processing_status ON receipts(processing_status);
```

## Integration Points

### 1. Wallet Service Integration

```rust
// Example integration in ExpenseServiceImpl

async fn create_expense(
    &self,
    user_id: Uuid,
    amount: Money,
    category: ExpenseCategory,
    date: DateTime<Utc>,
    description: String,
) -> Result<Expense, FinanceError> {
    // Create the expense
    let mut expense = Expense::new(user_id, amount.clone(), category, date, description);
    
    // Deduct from wallet using WalletService
    // This will handle both traditional currency and Dabloons
    match &amount.currency {
        Currency::Dabloons => {
            self.wallet_service
                .subtract_dabloons(user_id, amount.clone(), Some(format!("Expense: {}", expense.description)))
                .await?;
        }
        _ => {
            // For traditional currency, we'd use a different method
            // This would be implemented in a future iteration
            return Err(FinanceError::FinancialError(FinancialError::CurrencyNotSupported));
        }
    }
    
    // Save the expense
    self.expense_repo.save(&expense).await?;
    
    // If linked to a budget, update budget spent amount
    if let Some(budget_id) = expense.linked_budget_id {
        self.budget_service
            .update_spent_with_dabloons(budget_id, amount)
            .await?;
    }
    
    Ok(expense)
}
```

### 2. Budget Service Integration

The integration with the budget service follows the pattern established in `WALLET_INTEGRATION_IMPLEMENTATION_SUMMARY.md`:

- Use `link_to_budget` method to connect expenses to budget categories
- Update budget spent amounts when expenses are created/modified
- Support dual-currency tracking in budget calculations

### 3. p2panda Integration for Expense Sharing

Following the pattern from `UNIVERSAL_INCOME_IMPLEMENTATION_SUMMARY.md`:

- Implement `P2PExpenseSharing` struct in infrastructure layer
- Use Double Ratchet encryption for secure sharing
- Implement user consent verification
- Support both direct sharing and federation-wide sharing

## Sequence Diagrams

### 1. Expense Creation with Receipt Scanning

```
User -> Mobile App: Open camera to scan receipt
Mobile App -> BevyReceiptScanner: Access camera
BevyReceiptScanner -> Mobile App: Return image
Mobile App -> ExpenseService: save_receipt(image_data)
ExpenseService -> PostgresReceiptRepository: Save receipt record
PostgresReceiptRepository -> ExpenseService: Receipt ID
ExpenseService -> OCRService: process_receipt(receipt_id)
OCRService -> ExpenseService: Extracted text data
ExpenseService -> CategoryClassifier: classify_expense(extracted_text)
CategoryClassifier -> ExpenseService: Suggested category & amount
ExpenseService -> Mobile App: Display draft expense
Mobile App -> User: Review/edit expense details
User -> Mobile App: Confirm expense
Mobile App -> ExpenseService: create_expense(confirmed_data)
ExpenseService -> WalletService: subtract_dabloons(amount)
WalletService -> ExpenseService: Updated wallet
ExpenseService -> PostgresExpenseRepository: Save expense
PostgresExpenseRepository -> ExpenseService: Success
ExpenseService -> BudgetService: update_spent_with_dabloons(budget_id, amount)
BudgetService -> ExpenseService: Success
ExpenseService -> Mobile App: Created expense
```

### 2. p2p Expense Sharing

```
User A -> Mobile App: Select expense to share
Mobile App -> ExpenseService: get_sharing_preferences(user_id)
ExpenseService -> PostgresExpenseSharingRepository: Fetch preferences
PostgresExpenseSharingRepository -> ExpenseService: Preferences
ExpenseService -> Mobile App: Display sharing options
User A -> Mobile App: Select recipient & confirm sharing
Mobile App -> ExpenseService: share_expense(expense_id, recipient_id)
ExpenseService -> ExpenseSharingPreferenceValidator: validate_sharing(expense, preferences)
ExpenseSharingPreferenceValidator -> ExpenseService: Validation result
ExpenseService -> P2PExpenseSharing: encrypt_and_send(expense, recipient_id)
P2PExpenseSharing -> p2panda: Send encrypted data
p2panda -> Recipient Node: Relay encrypted data
Recipient Node -> User B's App: Deliver encrypted expense
User B's App -> P2PExpenseSharing: decrypt_received_data()
P2PExpenseSharing -> User B's App: Decrypted expense
User B's App -> User B: Display shared expense
```

## Privacy Considerations

### Granular Consent Controls

Following the pattern in `savings_service.rs`, we'll implement:

```rust
// packages/cpc-core/finance/src/application/expense_tracker/sharing_preferences.rs

#[async_trait]
pub trait ExpenseSharingPreferenceService {
    async fn get_preferences(&self, user_id: Uuid) -> Result<ExpenseSharingPreferences, FinanceError>;
    async fn update_preferences(
        &self,
        user_id: Uuid,
        enabled: bool,
        anonymized: bool,
        categories: Vec<ExpenseCategory>,
    ) -> Result<ExpenseSharingPreferences, FinanceError>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct ExpenseSharingPreferences {
    pub id: Uuid,
    pub user_id: Uuid,
    pub sharing_enabled: bool,
    pub anonymized: bool,
    pub shared_categories: Vec<ExpenseCategory>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ExpenseSharingPreferences {
    pub fn new(user_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            sharing_enabled: false,
            anonymized: false,
            shared_categories: vec![],
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn enable_sharing(&mut self) {
        self.sharing_enabled = true;
        self.updated_at = Utc::now();
    }
    
    pub fn disable_sharing(&mut self) {
        self.sharing_enabled = false;
        self.updated_at = Utc::now();
    }
    
    pub fn toggle_anonymization(&mut self) {
        self.anonymized = !self.anonymized;
        self.updated_at = Utc::now();
    }
    
    pub fn set_shared_categories(&mut self, categories: Vec<ExpenseCategory>) {
        self.shared_categories = categories;
        self.updated_at = Utc::now();
    }
}
```

### Key Privacy Features

1. **Category-level sharing controls**: Users can choose which expense categories to share
2. **Anonymization options**: Share without revealing personal details
3. **Recipient-specific permissions**: Different sharing rules for different users
4. **Time-limited sharing**: Expenses can be shared for specific durations
5. **Opt-out registry**: Respect federation-wide data sharing preferences

## Testing Strategy

### Unit Tests
- Domain model validation
- Expense creation and modification
- Receipt processing logic
- Category classification accuracy

### Integration Tests
- Wallet service integration
- Budget service integration
- Receipt scanning flow
- p2p sharing functionality

### End-to-End Tests
- Full expense tracking workflow
- Cross-device synchronization
- Security and privacy controls

## Future Expansion

### Short-term
- Machine learning for automatic categorization
- Recurring expense detection
- Multi-currency conversion for traditional currencies
- Expense analysis and insights

### Medium-term
- Integration with Universal Income for expense-based rewards
- Cooperative-wide spending pattern analysis (with consent)
- Budget forecasting based on historical expenses

### Long-term
- Integration with cooperative marketplace for Dabloon spending
- Expense-based social gifting features
- Advanced financial planning using expense history

## Implementation Roadmap

### Phase 1 (MVP)
- Basic expense tracking with manual entry
- Dual-currency support (Dabloons + primary currency)
- Integration with Wallet service
- Simple budget linking

### Phase 2
- Receipt scanning functionality
- Automatic categorization
- Basic expense sharing with consent controls

### Phase 3
- Advanced analytics and insights
- Recurring expense management
- Federation-wide expense patterns (with consent)

## Conclusion

This design provides a comprehensive architecture for the Expense Tracker module that follows our screaming architecture principles and integrates seamlessly with existing finance systems. The vertical slice implementation ensures cohesive functionality while maintaining clear separation of concerns. The design supports all key requirements including dual-currency tracking, receipt scanning, automatic categorization, and secure p2p sharing with granular consent controls.
