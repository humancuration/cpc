# Expense Tracking System

**Author:** Kai Tanaka  
**Date:** 2025-07-23  
**Status:** Technical Specification  
**Version:** 1.0.0

## Overview

The Expense Tracking System provides comprehensive expense management capabilities for individuals, cooperatives, and businesses within the CPC ecosystem. It enables real-time expense recording, categorization, analysis, and reporting with support for multiple currencies, receipt management, and automated categorization using machine learning.

## Component Specifications

### Frontend Components (Yew)

#### 1. ExpenseDashboard.svelte
**Purpose**: Main container for expense tracking interface
- **Props**: 
  - `userId: string` - User identifier
  - `cooperativeId?: string` - Optional cooperative context
  - `dateRange?: DateRange` - Optional date filtering
- **Features**:
  - Responsive layout with WidgetGrid integration
  - Real-time updates via GraphQL subscriptions
  - Loading states and error handling
  - Export functionality (PDF, CSV, Excel)

#### 2. ExpenseForm.svelte
**Purpose**: Add/edit expense entries
- **Props**:
  - `expense?: Expense` - Optional expense for editing
  - `onSave: (expense: ExpenseInput) => void` - Save callback
- **Features**:
  - Receipt upload with OCR processing
  - Auto-categorization suggestions
  - Multi-currency support with real-time exchange rates
  - Split expense functionality
  - Recurring expense setup

#### 3. ExpenseList.svelte
**Purpose**: Display and manage expense entries
- **Props**:
  - `expenses: Expense[]` - List of expenses
  - `loading: boolean` - Loading state
- **Features**:
  - Sortable columns
  - Inline editing
  - Bulk actions (delete, categorize, export)
  - Advanced filtering and search

#### 4. ExpenseAnalytics.svelte
**Purpose**: Visual expense analytics
- **Components**:
  - `CategoryBreakdown` - Pie chart of expenses by category
  - `MonthlyTrend` - Line chart showing spending trends
  - `MerchantAnalysis` - Bar chart of top merchants
  - `BudgetComparison` - Gauge charts for budget vs actual

### Backend Services (Rust/Axum)

#### 1. Expense Service
**Location**: `apps/backend/src/services/expense_service.rs`

```rust
pub struct ExpenseService {
    db: Arc<Database>,
    ml_service: Arc<MLCategoryService>,
    receipt_service: Arc<ReceiptProcessingService>,
}

impl ExpenseService {
    pub async fn create_expense(&self, input: CreateExpenseInput) -> Result<Expense, Error>;
    pub async fn get_expenses(&self, query: ExpenseQuery) -> Result<Vec<Expense>, Error>;
    pub async fn categorize_expense(&self, expense_id: Uuid) -> Result<Category, Error>;
    pub async fn process_receipt(&self, receipt_data: Vec<u8>) -> Result<ReceiptInfo, Error>;
}
```

#### 2. GraphQL Schema Extensions
```graphql
type Expense {
  id: ID!
  amount: Float!
  currency: String!
  category: ExpenseCategory!
  description: String!
  merchant: Merchant
  receipt: Receipt
  date: DateTime!
  tags: [String!]!
  cooperativeId: ID
  createdBy: User!
  splitDetails: [ExpenseSplit!]
  isRecurring: Boolean!
  recurringRule: RecurringRule
}

type ExpenseCategory {
  id: ID!
  name: String!
  color: String!
  icon: String
  parent: ExpenseCategory
  budgetLimit: Float
}

type Receipt {
  id: ID!
  url: String!
  ocrText: String
  extractedAmount: Float
  extractedDate: DateTime
  extractedMerchant: String
  confidence: Float!
}

input CreateExpenseInput {
  amount: Float!
  currency: String = "USD"
  categoryId: ID
  description: String!
  merchantId: ID
  receiptData: Upload
  date: DateTime
  tags: [String!]
  cooperativeId: ID
  splitWith: [ID!]
  isRecurring: Boolean = false
  recurringRule: RecurringRuleInput
}

type Query {
  expenses(filter: ExpenseFilter, pagination: PaginationInput): ExpenseConnection!
  expenseCategories: [ExpenseCategory!]!
  expenseSummary(filter: ExpenseFilter): ExpenseSummary!
}

type Mutation {
  createExpense(input: CreateExpenseInput!): Expense!
  updateExpense(id: ID!, input: UpdateExpenseInput!): Expense!
  deleteExpense(id: ID!): Boolean!
  categorizeExpense(id: ID!, categoryId: ID!): Expense!
}

type Subscription {
  expenseUpdated(userId: ID!): ExpenseUpdate!
}
```

## Data Models

### Core Entities

#### Expense
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expense {
    pub id: Uuid,
    pub user_id: Uuid,
    pub cooperative_id: Option<Uuid>,
    pub amount: Decimal,
    pub currency: String,
    pub category_id: Uuid,
    pub description: String,
    pub merchant_id: Option<Uuid>,
    pub receipt_id: Option<Uuid>,
    pub date: DateTime<Utc>,
    pub tags: Vec<String>,
    pub is_recurring: bool,
    pub recurring_rule: Option<RecurringRule>,
    pub split_details: Vec<ExpenseSplit>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

#### ExpenseCategory
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseCategory {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub color: String,
    pub icon: Option<String>,
    pub parent_id: Option<Uuid>,
    pub budget_limit: Option<Decimal>,
    pub is_system: bool,
}
```

#### ReceiptProcessingResult
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptProcessingResult {
    pub amount: Option<Decimal>,
    pub date: Option<DateTime<Utc>>,
    pub merchant: Option<String>,
    pub category: Option<String>,
    pub items: Vec<ReceiptItem>,
    pub confidence: f32,
}
```

### Database Schema

```sql
-- Expenses table
CREATE TABLE expenses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    cooperative_id UUID REFERENCES cooperatives(id),
    amount DECIMAL(12,2) NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    category_id UUID NOT NULL REFERENCES expense_categories(id),
    description TEXT NOT NULL,
    merchant_id UUID REFERENCES merchants(id),
    receipt_id UUID REFERENCES receipts(id),
    date TIMESTAMPTZ NOT NULL,
    tags TEXT[] DEFAULT '{}',
    is_recurring BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Expense categories
CREATE TABLE expense_categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    name VARCHAR(100) NOT NULL,
    color VARCHAR(7) NOT NULL DEFAULT '#3B82F6',
    icon VARCHAR(50),
    parent_id UUID REFERENCES expense_categories(id),
    budget_limit DECIMAL(12,2),
    is_system BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Receipts
CREATE TABLE receipts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    file_url TEXT NOT NULL,
    ocr_text TEXT,
    extracted_amount DECIMAL(12,2),
    extracted_date TIMESTAMPTZ,
    extracted_merchant VARCHAR(255),
    confidence FLOAT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_expenses_user_date ON expenses(user_id, date DESC);
CREATE INDEX idx_expenses_category ON expenses(category_id);
CREATE INDEX idx_expenses_cooperative ON expenses(cooperative_id);
CREATE INDEX idx_expenses_merchant ON expenses(merchant_id);
```

## API Contracts

### REST Endpoints (Internal)

#### POST /api/v1/expenses
Create a new expense with optional receipt processing.

**Request Body**:
```json
{
  "amount": 45.99,
  "currency": "USD",
  "categoryId": "550e8400-e29b-41d4-a716-446655440000",
  "description": "Team lunch at Italian restaurant",
  "merchantId": "660e8400-e29b-41d4-a716-446655440001",
  "receiptData": "<base64 encoded image>",
  "date": "2025-07-23T12:00:00Z",
  "tags": ["team", "lunch", "client-meeting"],
  "cooperativeId": "770e8400-e29b-41d4-a716-446655440002"
}
```

**Response**:
```json
{
  "id": "880e8400-e29b-41d4-a716-446655440003",
  "amount": 45.99,
  "currency": "USD",
  "category": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "Food & Dining",
    "color": "#3B82F6"
  },
  "description": "Team lunch at Italian restaurant",
  "merchant": {
    "id": "660e8400-e29b-41d4-a716-446655440001",
    "name": "Luigi's Restaurant"
  },
  "receipt": {
    "id": "990e8400-e29b-41d4-a716-446655440004",
    "url": "https://cdn.cpc.com/receipts/990e8400-e29b-41d4-a716-446655440004.jpg",
    "confidence": 0.92
  },
  "date": "2025-07-23T12:00:00Z",
  "tags": ["team", "lunch", "client-meeting"],
  "createdAt": "2025-07-23T12:05:00Z"
}
```

### GraphQL Operations

#### Query: GetExpenses
```graphql
query GetExpenses($filter: ExpenseFilter!, $pagination: PaginationInput) {
  expenses(filter: $filter, pagination: $pagination) {
    edges {
      node {
        id
        amount
        currency
        category {
          name
          color
        }
        description
        merchant {
          name
        }
        date
        tags
      }
    }
    pageInfo {
      hasNextPage
      endCursor
    }
    totalCount
  }
}
```

#### Mutation: CreateExpense
```graphql
mutation CreateExpense($input: CreateExpenseInput!) {
  createExpense(input: $input) {
    id
    amount
    category {
      name
    }
    description
    date
  }
}
```

#### Subscription: ExpenseUpdated
```graphql
subscription ExpenseUpdated($userId: ID!) {
  expenseUpdated(userId: $userId) {
    type
    expense {
      id
      amount
      category {
        name
      }
      description
    }
  }
}
```

## Integration Guidelines

### Frontend Integration

#### 1. Yew State Setup
```javascript
// stores/expenseStore.js
import { writable } from 'svelte/store';
import { graphqlClient } from '$lib/graphql/client';
import { GET_EXPENSES, CREATE_EXPENSE } from '$lib/graphql/queries';

function createExpenseStore() {
  const { subscribe, set, update } = writable({
    expenses: [],
    loading: false,
    error: null
  });

  return {
    subscribe,
    async fetchExpenses(filter) {
      update(state => ({ ...state, loading: true }));
      try {
        const result = await graphqlClient.query({
          query: GET_EXPENSES,
          variables: { filter }
        });
        update(state => ({
          ...state,
          expenses: result.data.expenses.edges.map(e => e.node),
          loading: false
        }));
      } catch (error) {
        update(state => ({ ...state, error, loading: false }));
      }
    },
    async createExpense(input) {
      const result = await graphqlClient.mutate({
        mutation: CREATE_EXPENSE,
        variables: { input }
      });
      update(state => ({
        ...state,
        expenses: [result.data.createExpense, ...state.expenses]
      }));
    }
  };
}

export const expenseStore = createExpenseStore();
```

#### 2. Component Usage Example
```svelte
<script>
  import { onMount } from 'svelte';
  import { expenseStore } from '$lib/stores/expenseStore';
  import ExpenseList from '$lib/components/ExpenseList.svelte';
  import ExpenseForm from '$lib/components/ExpenseForm.svelte';

  let showForm = false;

  onMount(() => {
    expenseStore.fetchExpenses({
      dateRange: {
        start: new Date(2025, 0, 1),
        end: new Date()
      }
    });
  });

  function handleExpenseCreated() {
    showForm = false;
    expenseStore.fetchExpenses();
  }
</script>

<div>
  <button on:click={() => showForm = true}>Add Expense</button>
  
  {#if showForm}
    <ExpenseForm onSave={handleExpenseCreated} />
  {/if}
  
  <ExpenseList 
    expenses={$expenseStore.expenses}
    loading={$expenseStore.loading}
  />
</div>
```

### Backend Integration

#### 1. Service Registration
```rust
// backend/src/main.rs
mod services;
use services::expense_service::ExpenseService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let expense_service = ExpenseService::new(
        db_pool.clone(),
        ml_service.clone(),
        receipt_service.clone(),
    );
    
    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(expense_service)
        .finish();
    
    // ... server setup
}
```

#### 2. GraphQL Resolvers
```rust
// backend/src/graphql/resolvers/expense_resolver.rs
use async_graphql::*;
use crate::services::expense_service::ExpenseService;

pub struct ExpenseQuery;

#[Object]
impl ExpenseQuery {
    async fn expenses(
        &self,
        ctx: &Context<'_>,
        filter: ExpenseFilter,
        pagination: Option<PaginationInput>
    ) -> Result<ExpenseConnection> {
        let service = ctx.data_unchecked::<ExpenseService>();
        service.get_expenses(filter, pagination).await
    }
}

pub struct ExpenseMutation;

#[Object]
impl ExpenseMutation {
    async fn create_expense(
        &self,
        ctx: &Context<'_>,
        input: CreateExpenseInput
    ) -> Result<Expense> {
        let service = ctx.data_unchecked::<ExpenseService>();
        service.create_expense(input).await
    }
}
```

### Third-Party Integrations

#### 1. Banking APIs
- **Plaid Integration**: Connect bank accounts for automatic transaction import
- **Security**: OAuth 2.0 with refresh tokens
- **Data Sync**: Daily batch processing via cpc-node workers

#### 2. Receipt Processing
- **OCR Service**: Tesseract.js for text extraction
- **ML Categorization**: Custom trained models for expense categorization
- **Confidence Scoring**: Threshold-based manual review triggers

#### 3. Currency Exchange
- **Exchange Rate API**: Open Exchange Rates integration
- **Cache Strategy**: Redis-based caching with 1-hour TTL
- **Fallback**: Static rates with daily updates

### Deployment Considerations

#### 1. Database Migration
```bash
# Run migrations
sqlx migrate run

# Verify schema
sqlx migrate info
```

#### 2. Performance Optimization
- **Indexing Strategy**: Composite indexes on (user_id, date, category_id)
- **Query Optimization**: Prepared statements with query plan caching
- **Caching Layer**: Redis for frequently accessed summaries

#### 3. Monitoring Setup
- **Metrics Collection**: Prometheus metrics for API latency and error rates
- **Alerting**: Grafana alerts for failed receipt processing
- **Logging**: Structured JSON logging with correlation IDs

## Security Considerations

### Data Protection
- **Encryption at Rest**: AES-256 encryption for sensitive receipt data
- **Encryption in Transit**: TLS 1.3 for all API communications
- **PII Handling**: Automatic PII detection and masking in OCR results

### Access Control
- **Role-based Permissions**: Cooperative vs individual expense access
- **Audit Trail**: Complete CRUD operation logging
- **Data Retention**: Configurable retention policies per jurisdiction

## Future Enhancements

### Planned Features
1. **Smart Budgeting**: AI-powered budget recommendations
2. **Tax Optimization**: Automatic tax category classification
3. **Collaborative Expenses**: Multi-user expense splitting
4. **Mobile Receipt Capture**: Camera integration with edge processing
5. **Voice Input**: Natural language expense entry

### Scalability Roadmap
1. **Horizontal Scaling**: Database sharding by user_id
2. **Async Processing**: Queue-based receipt processing
3. **CDN Integration**: Global receipt file distribution
4. **Multi-region**: Active-active deployment for low latency