# Personal Finance Module Architecture

## Module Structure (Hexagonal Architecture)

```rust
apps/personal-finance/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── domain/
    │   ├── models.rs
    │   ├── budget_service.rs
    │   ├── expense_service.rs
    │   ├── savings_service.rs
    │   └── mod.rs
    ├── application/
    │   ├── finance_service.rs
    │   └── mod.rs
    ├── infrastructure/
    │   ├── repository.rs
    │   └── mod.rs
    └── web/
        ├── graphql/
        │   ├── mutations.rs
        │   ├── queries.rs
        │   ├── subscriptions.rs
        │   └── mod.rs
        ├── routes.rs
        └── mod.rs
```

## Domain Models

### Budget Model
```rust
pub struct Budget {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category: String,
    pub allocated_amount: Decimal,
    pub spent_amount: Decimal,
    pub period: BudgetPeriod, // Monthly, Weekly, etc.
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

pub enum BudgetPeriod {
    Monthly,
    Weekly,
    BiWeekly,
    Custom,
}
```

### Expense Model
```rust
pub struct Expense {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: Decimal,
    pub currency: String,
    pub category: String,
    pub description: String,
    pub date: DateTime<Utc>,
    pub receipt_id: Option<Uuid>, // For scanned receipts
    pub payment_method: String,
}
```

### SavingsGoal Model
```rust
pub struct SavingsGoal {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub target_amount: Decimal,
    pub current_amount: Decimal,
    pub target_date: DateTime<Utc>,
    pub auto_deduct: bool,
    pub deduction_percentage: Decimal,
}
```

## Service Interfaces

### Monthly Budget Calculation (UBI Integration)
```rust
pub struct BudgetService {
    ubi_service: Arc<dyn UbiServiceInterface>,
}

impl BudgetService {
    pub async fn calculate_monthly_budget(
        &self, 
        user_id: Uuid
    ) -> Result<Budget, FinanceError> {
        let ubi_balance = self.ubi_service.get_ubi_balance(user_id).await?;
        // Calculate budget allocation based on UBI income
        // ... business logic ...
    }
}
```

### Expense Categorization (Royalty Service Pattern)
```rust
pub struct ExpenseService {
    categorization_rules: Arc<RwLock<HashMap<String, CategoryRule>>>,
}

impl ExpenseService {
    pub async fn categorize_expense(
        &self, 
        expense: &mut Expense,
        receipt_data: Option<Vec<u8>>
    ) -> Result<(), FinanceError> {
        // Use royalty service pattern for rule-based categorization
        if let Some(data) = receipt_data {
            let ocr_text = self.scan_receipt(data).await?;
            expense.category = self.apply_categorization_rules(&ocr_text);
        }
        // ... additional processing ...
    }
}
```

### Savings Progress Tracking
```rust
pub struct SavingsService;

impl SavingsService {
    pub fn calculate_progress(&self, goal: &SavingsGoal) -> SavingsProgress {
        let percentage = (goal.current_amount / goal.target_amount) * dec!(100);
        let days_remaining = (goal.target_date - Utc::now()).num_days();
        
        SavingsProgress {
            current: goal.current_amount,
            target: goal.target_amount,
            percentage,
            days_remaining,
        }
    }
}
```

## GraphQL API Design

### Mutations
```graphql
input CreateBudgetInput {
    category: String!
    allocatedAmount: Decimal!
    period: BudgetPeriod!
}

input RecordExpenseInput {
    amount: Decimal!
    currency: String!
    category: String
    description: String!
    receiptImage: Upload
}

input CreateSavingsGoalInput {
    name: String!
    targetAmount: Decimal!
    targetDate: DateTime!
    autoDeduct: Boolean
    deductionPercentage: Decimal
}

type Mutation {
    createBudget(input: CreateBudgetInput!): Budget!
    updateBudget(id: ID!, input: CreateBudgetInput!): Budget!
    recordExpense(input: RecordExpenseInput!): Expense!
    createSavingsGoal(input: CreateSavingsGoalInput!): SavingsGoal!
    updateSavingsGoal(id: ID!, input: CreateSavingsGoalInput!): SavingsGoal!
}
```

### Queries
```graphql
type Query {
    getBudget(id: ID!): Budget
    listBudgets: [Budget!]!
    getExpense(id: ID!): Expense
    listExpenses(category: String, startDate: DateTime, endDate: DateTime): [Expense!]!
    getSavingsGoal(id: ID!): SavingsGoal
    listSavingsGoals: [SavingsGoal!]!
    getSavingsProgress(id: ID!): SavingsProgress!
}
```

### Subscriptions
```graphql
type Subscription {
    budgetUpdated(userId: ID!): Budget!
    expenseRecorded(userId: ID!): Expense!
    savingsProgressUpdated(goalId: ID!): SavingsProgress!
}
```

## Integration Points

1. UBI Service Integration:
   - Monthly budget calculation incorporates UBI income
   - Uses `UbiService` from `cpc-core` via trait interface

2. Receipt Scanning:
   - Uses ffmpeg.wasm for image processing
   - Integrates with OCR services for text extraction

3. Royalty Service Patterns:
   - Rule-based expense categorization similar to royalty distribution rules
   - Dynamic rule management interface

4. Treasury Service:
   - Deducts savings automatically when auto_deduct is enabled
   - Creates transactions through `TransactionLedger`