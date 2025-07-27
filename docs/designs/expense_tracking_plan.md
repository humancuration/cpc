# Expense Tracking Feature: Architectural Plan

This document outlines the architecture and implementation plan for the expense tracking feature in the CPC platform. The plan is created based on the feature requirements, as the original implementation in the Android application could not be located.

## 1. Guiding Principles

- **Hexagonal Architecture:** The core domain logic will be independent of external concerns like the database, API, or UI.
- **Screaming Architecture:** The project structure will clearly communicate its purpose (i.e., expense tracking).
- **Vertical Slices:** Features will be organized by business capability, not by technical layer.
- **P2P Compatibility:** Data models will be designed to be compatible with `p2panda` for future synchronization.

## 2. Core Domain Model

The core logic and data structures for expense tracking will reside in `packages/cpc-core/src/expenses/mod.rs`.

```rust
// packages/cpc-core/src/expenses/model.rs

use chrono::{DateTime, Utc};
use uuid::Uuid;
use rust_decimal::Decimal;

pub struct Expense {
    pub id: Uuid,
    pub user_id: Uuid,
    pub project_id: Option<Uuid>,
    pub client_id: Option<Uuid>,
    pub amount: Decimal,
    pub currency: String, // ISO 4217 currency code
    pub description: String,
    pub category: ExpenseCategory,
    pub status: ExpenseStatus,
    pub transaction_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub receipts: Vec<Receipt>,
}

pub struct Receipt {
    pub id: Uuid,
    pub expense_id: Uuid,
    pub file_name: String,
    pub file_path: String, // Path in our storage system
    pub mime_type: String,
    pub uploaded_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExpenseStatus {
    Pending,
    Approved,
    Rejected,
    Reimbursed,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExpenseCategory {
    Travel,
    Meals,
    Software,
    Hardware,
    OfficeSupplies,
    Other(String), // Custom category
}
```

## 3. Database Schema (SQLx)

The following schema will be defined in a new migration file.

```sql
-- Migration file: V_X__create_expense_tables.sql

CREATE TABLE expenses (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    project_id UUID,
    client_id UUID,
    amount DECIMAL(19, 4) NOT NULL,
    currency VARCHAR(3) NOT NULL,
    description TEXT NOT NULL,
    category VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL,
    transaction_date TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE receipts (
    id UUID PRIMARY KEY,
    expense_id UUID NOT NULL REFERENCES expenses(id) ON DELETE CASCADE,
    file_name VARCHAR(255) NOT NULL,
    file_path TEXT NOT NULL,
    mime_type VARCHAR(100) NOT NULL,
    uploaded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_expenses_user_id ON expenses(user_id);
CREATE INDEX idx_expenses_project_id ON expenses(project_id);
CREATE INDEX idx_expenses_status ON expenses(status);
```

## 4. Application Service (Port)

This trait defines the contract for our expense business logic.

```rust
// packages/cpc-core/src/expenses/service.rs

use async_trait::async_trait;
use uuid::Uuid;
use crate::expenses::model::{Expense, ExpenseStatus};

#[async_trait]
pub trait ExpenseService {
    async fn create_expense(&self, user_id: Uuid, input: CreateExpenseInput) -> Result<Expense, anyhow::Error>;
    async fn get_expense_by_id(&self, expense_id: Uuid) -> Result<Option<Expense>, anyhow::Error>;
    async fn get_expenses_for_user(&self, user_id: Uuid) -> Result<Vec<Expense>, anyhow::Error>;
    async fn update_expense_status(&self, expense_id: Uuid, status: ExpenseStatus) -> Result<Expense, anyhow::Error>;
    async fn attach_receipt(&self, expense_id: Uuid, file_data: Vec<u8>, file_name: String, mime_type: String) -> Result<Receipt, anyhow::Error>;
}

// DTOs for service layer
pub struct CreateExpenseInput {
    // ... fields matching the Expense struct, minus generated ones
}
```

## 5. API Layer Definitions

### GraphQL API (Public)

For UI clients (`pds`, `cpc-platform`).

```graphql
# apps/backend/src/graphql/expenses.graphql

type Expense {
  id: ID!
  amount: Float!
  currency: String!
  description: String!
  category: String!
  status: String!
  transactionDate: String!
  receipts: [Receipt!]!
}

type Receipt {
  id: ID!
  fileName: String!
  url: String! # A URL to view/download the receipt
}

input CreateExpenseInput {
  projectId: ID
  amount: Float!
  currency: String!
  description: String!
  category: String!
  transactionDate: String!
}

# Mutations
extend type Mutation {
  createExpense(input: CreateExpenseInput!): Expense
  updateExpenseStatus(id: ID!, status: String!): Expense
  # File upload will be a separate multipart request, which then calls an internal service
  # and the result is associated with an expense.
}

# Subscriptions
extend type Subscription {
  expenseUpdated(id: ID!): Expense
}
```

### gRPC API (Internal)

For communication between the `backend` and `cpc-node` workers, especially for processing receipts (e.g., OCR, thumbnail generation).

```protobuf
// packages/cpc-protos/src/expenses.proto

syntax = "proto3";

package expenses;

service ExpenseProcessing {
  // A worker node can subscribe to a stream of receipt processing jobs
  rpc ProcessReceipts(stream ReceiptJobRequest) returns (stream ReceiptJobResult);
}

message ReceiptJobRequest {
  string receipt_id = 1;
  string file_path = 2; // Location of the uploaded file
}

message ReceiptJobResult {
  string receipt_id = 1;
  bool success = 2;
  string extracted_text = 3; // e.g., from OCR
  string error_message = 4;
}
```

## 6. Vertical Slice Structure

The new feature will be organized as follows:

```
cpc/
├── apps/
│   ├── backend/
│   │   └── src/
│   │       ├── expenses/              # New vertical slice for expenses
│   │       │   ├── mod.rs
│   │       │   ├── graphql.rs         # GraphQL resolvers for expenses
│   │       │   ├── grpc.rs            # gRPC service implementation
│   │       │   └── service.rs         # Implementation of the ExpenseService trait
│   │       └── main.rs                # Register the new services
│   └── cpc-platform/
│       └── src/
│           └── expenses/              # New Yew components for expenses
│               ├── components/
│               │   ├── expense_list.rs
│               │   ├── expense_detail.rs
│               │   └── expense_form.rs
│               └── mod.rs
├── packages/
│   ├── cpc-core/
│   │   └── src/
│   │       ├── expenses/              # Core domain logic
│   │       │   ├── mod.rs
│   │       │   ├── model.rs
│   │       │   └── service.rs
│   │       └── lib.rs                 # Expose the expenses module
│   └── cpc-protos/
│       └── proto/
│           └── expenses.proto         # New protobuf definition
└── docs/
    └── designs/
        └── expense_tracking_plan.md   # This file
```

## 7. Recommended Crates

- **`rust_decimal`**: For precise handling of monetary values, avoiding floating-point inaccuracies.
- **`uuid`**: For generating unique identifiers.
- **`chrono`**: For handling timestamps.
- **`async-graphql`**: For the GraphQL API layer (already in use).
- **`tonic`**: For the gRPC services (already in use).
- **`sqlx`**: For database interaction (already in use).
- **`p2panda`**: For data synchronization (as per architecture).
- **`ffmpeg.wasm` / `av1-encoder`**: If receipts include video snippets.
- **`lopdf` or `pdf-rs`**: If generating PDF expense reports is required.

This plan provides a solid foundation for building the expense tracking feature in a way that is consistent with our existing architecture and principles.