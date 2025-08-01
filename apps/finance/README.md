# CPC Finance Module

The CPC Finance Module provides comprehensive personal finance management functionality for the Cooperative Peer Cloud platform.

## Features

- **Budget Management**: Create and track budgets across different categories
- **Savings Goals**: Set and monitor progress toward financial goals
- **Financial Insights**: Analyze spending patterns and financial trends
- **Privacy-Preserving Data Sharing**: Securely share financial data with explicit user consent
- **p2p Data Sharing**: Utilize p2panda with Double Ratchet encryption for secure communication
- **Visualization**: Bevy 3D visualizations and Yew web components for financial data
- **Wallet**: Manage dabloons, the internal currency of the CPC platform

## Architecture

This module follows hexagonal (clean) architecture with vertical slices:

```
apps/finance/
├── src/
│   ├── domain/          # Core business logic and models
│   ├── application/     # Application services orchestrating domain logic
│   ├── infrastructure/  # Database repositories and external service clients
│   └── presentation/    # Visualization components (Bevy and Yew)
├── Cargo.toml           # Package manifest
├── MIGRATION_GUIDE.md   # Migration instructions from old personal-finance app
└── README.md            # This file
```

## Domain Model

### Budget
Represents a financial allocation for a specific category over a time period.

### SavingsGoal
Represents a financial target with progress tracking.

### Money
A monetary amount with currency, supporting arithmetic operations.

### Wallet
Represents a user's wallet for storing dabloons, the internal currency of the CPC platform. Includes functionality for adding, subtracting, and transferring dabloons between users.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cpc-finance = { path = "../apps/finance", features = ["finance"] }
```

## Usage

### Creating a Budget

```rust
use cpc_core::finance::{Budget, BudgetPeriod, Money, Currency};
use uuid::Uuid;
use chrono::Utc;
use rust_decimal::Decimal;

let user_id = Uuid::new_v4();
let budget = Budget::new(
    user_id,
    "Groceries".to_string(),
    Money::new(Decimal::new(500, 0), Currency::USD),
    BudgetPeriod::Monthly,
    Utc::now(),
    Utc::now() + chrono::Duration::days(30),
);
```

### Creating a Savings Goal

```rust
use cpc_core::finance::{SavingsGoal, Money, Currency};
use uuid::Uuid;
use chrono::Utc;
use rust_decimal::Decimal;

let user_id = Uuid::new_v4();
let goal = SavingsGoal::new(
    user_id,
    "Vacation".to_string(),
    Money::new(Decimal::new(2000, 0), Currency::USD),
    Utc::now() + chrono::Duration::days(365),
);
```

### Managing a Wallet

```rust
use cpc_core::finance::{Wallet, Money, Currency};
use uuid::Uuid;
use rust_decimal::Decimal;

let user_id = Uuid::new_v4();
let mut wallet = Wallet::new(user_id);

// Add dabloons to the wallet
wallet.add_dabloons(Money::new(Decimal::new(100, 0), Currency::Dabloons)).unwrap();

// Check balance
assert_eq!(wallet.balance.amount, Decimal::new(100, 0));
```

## Database Migrations

The finance module includes SQL migrations for PostgreSQL:

- `20250728000000_create_budgets_table.sql`
- `20250728000001_create_savings_goals_table.sql`
- `20250728000002_create_data_sharing_preferences_table.sql`
- `20250728000003_create_wallets_table.sql`
- `20250728000004_create_wallet_transactions_table.sql`

## p2p Data Sharing

Financial data is shared securely using p2panda with Double Ratchet encryption:

```rust
// Only share with UBI improvement nodes per user consent
#[cfg(feature = "p2p")]
finance_data_sharing.share_savings_goal(
    goal,
    ubi_node_ids,
    user_consent,
).await?;
```

## Visualization

### Bevy 3D Visualizations

```rust
#[cfg(feature = "visualization")]
use cpc_core::finance::presentation::bevy::financial_viz;

// Create a 3D progress ring for savings goals
let progress_ring = financial_viz::build_savings_progress_ring(&goal);
```

### Yew Web Components

```rust
#[cfg(feature = "web")]
use cpc_core::finance::presentation::yew::components;

// Use the SavingsGoalCard component in your Yew app
html! {
    <components::SavingsGoalCard goal={goal} />
};
```

## Migration from Personal Finance App

If you were previously using the standalone `apps/personal-finance` application, please see [MIGRATION_GUIDE.md](MIGRATION_GUIDE.md) for detailed instructions on migrating to this module.

## License

This project will be licensed under a new type of CoopyLeft license which we will address later. This has no license for now.