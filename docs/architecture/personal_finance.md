# Personal Finance Module Architecture

## Module Structure (Hexagonal Architecture with Vertical Slices)

The personal finance module has been moved from `apps/personal-finance/` to `packages/cpc-core/finance/` to follow the screaming architecture principles. All finance domain logic now exists as vertical slices within the core package.

```rust
packages/cpc-core/finance/
├── Cargo.toml
├── MIGRATION_GUIDE.md  # Migration instructions from old personal-finance app
├── README.md           # Module documentation
└── src/
    ├── lib.rs
    ├── domain/          # Pure business models (Budget, SavingsGoal)
    │   ├── budget.rs
    │   ├── savings_goal.rs
    │   ├── primitives.rs
    │   └── mod.rs
    ├── application/     # Service orchestration (BudgetService, SavingsService)
    │   ├── budget_service.rs
    │   ├── savings_service.rs
    │   └── mod.rs
    ├── infrastructure/  # Concrete implementations (repositories, p2p)
    │   ├── database/
    │   │   ├── models.rs
    │   │   ├── repositories.rs
    │   │   └── mod.rs
    │   ├── p2p/
    │   │   ├── data_sharing.rs
    │   │   └── mod.rs
    │   └── mod.rs
    └── presentation/    # UI components (Bevy, Yew)
        ├── bevy/
        │   ├── financial_viz.rs
        │   └── mod.rs
        ├── yew/
        │   ├── components.rs
        │   └── mod.rs
        └── mod.rs
```

## Domain Models

### Budget Model
```rust
pub struct Budget {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category: String,
    pub allocated_amount: Money,  // Using Money type from primitives
    pub spent_amount: Money,
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

### SavingsGoal Model
```rust
pub struct SavingsGoal {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub target_amount: Money,
    pub current_amount: Money,
    pub target_date: DateTime<Utc>,
    pub auto_deduct: bool,
    pub deduction_percentage: Decimal,
    pub description: Option<String>,
    pub category: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

## Service Interfaces

### Budget Service
```rust
pub struct BudgetService {
    repository: Arc<dyn BudgetRepository>,
}

impl BudgetService {
    pub async fn create_budget(
        &self,
        user_id: Uuid,
        category: String,
        allocated_amount: Money,
        period: BudgetPeriod,
    ) -> Result<Budget, FinanceError> {
        let budget = Budget::new(user_id, category, allocated_amount, period, Utc::now(), calculate_end_date(Utc::now(), period));
        self.repository.save(&budget).await?;
        Ok(budget)
    }
}
```

### Savings Service
```rust
pub struct SavingsService {
    repository: Arc<dyn SavingsGoalRepository>,
}

impl SavingsService {
    pub fn calculate_progress(&self, goal: &SavingsGoal) -> SavingsProgress {
        let percentage = (goal.current_amount.amount / goal.target_amount.amount) * dec!(100);
        let days_remaining = (goal.target_date - Utc::now()).num_days();
        
        SavingsProgress {
            current: goal.current_amount.clone(),
            target: goal.target_amount.clone(),
            percentage,
            days_remaining,
            monthly_savings_needed: goal.monthly_savings_needed(),
        }
    }
}
```

## GraphQL API Design

### Mutations
```graphql
input CreateBudgetInput {
    category: String!
    allocatedAmount: MoneyInput!
    period: BudgetPeriod!
}

input RecordExpenseInput {
    amount: MoneyInput!
    category: String
    description: String!
    receiptImage: Upload
}

input CreateSavingsGoalInput {
    name: String!
    targetAmount: MoneyInput!
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

2. p2p Data Sharing:
   - All sensitive financial data is shared exclusively through p2panda peer channels
   - Uses Double Ratchet encryption for privacy-preserving data sharing
   - Implements explicit user consent flows for data sharing

3. Visualization Components:
   - Bevy-based 3D financial visualizations
   - Yew-based web components for dashboard views

4. Treasury Service:
   - Deducts savings automatically when auto_deduct is enabled
   - Creates transactions through `TransactionLedger`

## Migration from Old Structure

If you were previously using the standalone `apps/personal-finance` application, please see [MIGRATION_GUIDE.md](../cpc-core/finance/MIGRATION_GUIDE.md) for detailed instructions on migrating to this module.