# Architectural Plan: Update Expense Functionality

This document outlines the architectural plan for implementing the `updateExpense` feature. The goal is to create a robust and maintainable implementation that follows our established architectural principles.

## 1. Overview

The `updateExpense` functionality will allow users to modify the details of an existing expense. The changes will be initiated through a GraphQL mutation and will propagate down through the service and repository layers to the database.

## 2. Architectural Layers & Changes

The implementation will touch four key areas of our backend, following a vertical slice from the GraphQL API down to the database.

### 2.1. Core Domain Layer (`packages/cpc-core/`)

This layer defines the core business logic and interfaces.

#### `packages/cpc-core/src/expenses/repository.rs`

The `ExpenseRepository` trait defines the contract for our persistence layer. We need to add a method for updating an expense.

- **Action:** Add the `update_expense` method to the `ExpenseRepository` trait.

```rust
// packages/cpc-core/src/expenses/repository.rs

use async_trait::async_trait;
use uuid::Uuid;
use crate::expenses::model::{Expense, ExpenseStatus, Receipt};
use crate::expenses::service::{CreateExpenseInput, UpdateExpenseInput}; // <-- Import UpdateExpenseInput

#[async_trait]
pub trait ExpenseRepository: Send + Sync {
    async fn create_expense(&self, user_id: Uuid, input: &CreateExpenseInput) -> Result<Expense, anyhow::Error>;
    async fn get_expense_by_id(&self, expense_id: Uuid) -> Result<Option<Expense>, anyhow::Error>;
    async fn get_expenses_for_user(&self, user_id: Uuid) -> Result<Vec<Expense>, anyhow::Error>;
    // Add the following method
    async fn update_expense(&self, expense_id: Uuid, input: &UpdateExpenseInput) -> Result<Expense, anyhow::Error>;
    async fn update_expense_status(&self, expense_id: Uuid, status: &ExpenseStatus) -> Result<Expense, anyhow::Error>;
    async fn create_receipt(&self, expense_id: Uuid, file_name: &str, file_path: &str, mime_type: &str) -> Result<Receipt, anyhow::Error>;
}
```

#### `packages/cpc-core/src/expenses/service.rs`

The `ExpenseService` trait defines the application's business logic. We will define the `update_expense` method here. We will also define the `UpdateExpenseInput` struct that will be used across layers.

- **Action:** Define `UpdateExpenseInput` struct.
- **Action:** Add `update_expense` method to the `ExpenseService` trait.

```rust
// packages/cpc-core/src/expenses/service.rs

use async_trait::async_trait;
use uuid::Uuid;
use crate::expenses::model::{Expense, ExpenseStatus, Receipt, ExpenseCategory};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

// ... (CreateExpenseInput)

#[derive(Debug, Clone)]
pub struct UpdateExpenseInput {
    pub project_id: Option<Uuid>,
    pub client_id: Option<Uuid>,
    pub amount: Decimal,
    pub currency: String,
    pub description: String,
    pub category: ExpenseCategory,
    pub transaction_date: DateTime<Utc>,
}


#[async_trait]
pub trait ExpenseService: Send + Sync {
    async fn create_expense(&self, user_id: Uuid, input: CreateExpenseInput) -> Result<Expense, anyhow::Error>;
    async fn get_expense_by_id(&self, expense_id: Uuid) -> Result<Option<Expense>, anyhow::Error>;
    async fn get_expenses_for_user(&self, user_id: Uuid) -> Result<Vec<Expense>, anyhow::Error>;
    // Add the following method
    async fn update_expense(&self, expense_id: Uuid, input: UpdateExpenseInput) -> Result<Expense, anyhow::Error>;
    async fn update_expense_status(&self, expense_id: Uuid, status: ExpenseStatus) -> Result<Expense, anyhow::Error>;
    async fn attach_receipt(
        &self,
        expense_id: Uuid,
        file_data: Vec<u8>,
        file_name: String,
        mime_type: String,
    ) -> Result<Receipt, anyhow::Error>;
}
```

### 2.2. Infrastructure Layer (`apps/backend/src/repositories/`)

This layer provides the concrete implementation of the repository interface.

#### `apps/backend/src/repositories/expense_repository.rs`

We will implement the `update_expense` method in `ExpenseRepositoryImpl`.

- **Action:** Implement `update_expense`.
- **Implementation Details:**
    - Use a SQL `UPDATE` statement to modify the expense record in the `expenses` table.
    - Update the following fields: `project_id`, `client_id`, `amount`, `currency`, `description`, `category`, `transaction_date`, and `updated_at`.
    - The `updated_at` field should be set to the current timestamp (`NOW()`).
    - The query should use the `RETURNING` clause to get the updated expense data.
    - Wrap the database call with `#[instrument]` for tracing.

```rust
// apps/backend/src/repositories/expense_repository.rs

// ... imports
use cpc_core::expenses::service::UpdateExpenseInput;

// ...

#[async_trait]
impl ExpenseRepository for ExpenseRepositoryImpl {
    // ... other methods

    #[instrument(skip(self, input))]
    async fn update_expense(&self, expense_id: Uuid, input: &UpdateExpenseInput) -> anyhow::Result<Expense> {
        let expense_db = sqlx::query_as!(
            ExpenseDb,
            r#"
            UPDATE expenses
            SET
                project_id = $1,
                client_id = $2,
                amount = $3,
                currency = $4,
                description = $5,
                category = $6,
                transaction_date = $7,
                updated_at = NOW()
            WHERE id = $8
            RETURNING id, user_id, project_id, client_id, amount, currency, description, category, status, transaction_date, created_at, updated_at
            "#,
            input.project_id,
            input.client_id,
            input.amount,
            input.currency,
            input.description,
            input.category.to_string(),
            input.transaction_date,
            expense_id
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to update expense in database")?;

        let mut expense: Expense = expense_db.try_into()?;
        expense.receipts = self.get_receipts_for_expense(expense.id).await?;
        Ok(expense)
    }

    // ... other methods
}
```

### 2.3. Application Layer (`apps/backend/src/expenses/`)

This layer implements the business logic defined in the core service trait.

#### `apps/backend/src/expenses/service.rs`

- **Action:** Implement the `update_expense` method in `ExpenseServiceImpl`.
- **Implementation Details:**
    - This method will orchestrate the update process.
    - It will simply delegate the call to the `expense_repo.update_expense` method.
    - Add `UpdateExpenseInput` to the `use` statement.

```rust
// apps/backend/src/expenses/service.rs

// ... imports
use cpc_core::expenses::service::{CreateExpenseInput, ExpenseService, UpdateExpenseInput};

// ...

#[async_trait]
impl ExpenseService for ExpenseServiceImpl {
    // ... other methods

    async fn update_expense(&self, expense_id: Uuid, input: UpdateExpenseInput) -> Result<Expense, anyhow::Error> {
        self.expense_repo.update_expense(expense_id, &input).await
    }

    // ... other methods
}
```

### 2.4. GraphQL Layer (`apps/backend/src/graphql/`)

This layer exposes our functionality via the GraphQL API.

#### `apps/backend/src/graphql/expenses.rs`

- **Action:** Refactor the `update_expense` mutation to use the new service method.
- **Implementation Details:**
    - The mutation will now accept `UpdateExpenseInput` from `cpc_core`.
    - It will call `service.update_expense` and return the result.
    - The `UpdateExpenseInput` from `async-graphql` needs to be converted to the one from `cpc-core`.

First, let's define the GraphQL input object. Note that `cpc_core::expenses::model::ExpenseInput` will be replaced with a new `UpdateExpenseInput` in `async_graphql`.

```rust
// apps/backend/src/graphql/expenses.rs

// ... imports
use cpc_core::expenses::model::{ExpenseCategory};

// ... (ExpenseObject)

#[derive(InputObject)]
pub struct UpdateExpenseInput {
    pub project_id: Option<Uuid>,
    pub client_id: Option<Uuid>,
    pub amount: f64,
    pub currency: String,
    pub description: String,
    pub category: String, // We'll parse this into an enum
    pub transaction_date: chrono::DateTime<chrono::Utc>,
}

impl From<UpdateExpenseInput> for cpc_core::expenses::service::UpdateExpenseInput {
    fn from(input: UpdateExpenseInput) -> Self {
        Self {
            project_id: input.project_id,
            client_id: input.client_id,
            amount: rust_decimal::Decimal::from_f64(input.amount).unwrap_or_default(),
            currency: input.currency,
            description: input.description,
            category: input.category.parse().unwrap_or(ExpenseCategory::Other("".to_string())),
            transaction_date: input.transaction_date,
        }
    }
}


// ... (ExpensesQueryRoot)

#[Object]
impl ExpensesMutationRoot {
    // ... create_expense

    /// Update an existing expense
    #[graphql(name = "updateExpense")]
    async fn update_expense(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        input: UpdateExpenseInput,
    ) -> Result<ExpenseObject> {
        let service = ctx.data_unchecked::<Arc<dyn ExpenseService>>();
        
        let updated_expense = service.update_expense(id, input.into()).await
            .map_err(|e| Error::new(format!("Failed to update expense: {}", e)))?;
        
        Ok(updated_expense.into())
    }

    // ... delete_expense
}
```

## 3. Summary of Changes

1.  **`packages/cpc-core/src/expenses/service.rs`**:
    - Define `UpdateExpenseInput` struct.
    - Add `update_expense` to `ExpenseService` trait.
2.  **`packages/cpc-core/src/expenses/repository.rs`**:
    - Add `update_expense` to `ExpenseRepository` trait.
3.  **`apps/backend/src/repositories/expense_repository.rs`**:
    - Implement `update_expense` with `SQL UPDATE` query.
4.  **`apps/backend/src/expenses/service.rs`**:
    - Implement `update_expense` in `ExpenseServiceImpl`.
5.  **`apps/backend/src/graphql/expenses.rs`**:
    - Define `UpdateExpenseInput` for GraphQL.
    - Implement `From<UpdateExpenseInput>` for the core input type.
    - Refactor `update_expense` mutation to use the service.

This plan provides a clear path for implementation, ensuring consistency with our existing architecture.