# Personal Finance Module Architecture

## Overview

The Personal Finance module is a comprehensive financial management system built with Rust, following hexagonal architecture principles with vertical slices. It provides features for budgeting, expense tracking, and savings goal planning, with integrations for UBI (Universal Basic Income) and financial analytics.

This module has been moved from `apps/personal-finance/` to `packages/cpc-core/finance/` to follow the screaming architecture principles. All finance domain logic now exists as vertical slices within the core package.

## Architecture Principles

### Hexagonal Architecture

The module follows hexagonal (ports and adapters) architecture to ensure separation of concerns and testability:

- **Domain Layer**: Core business logic and entities
- **Application Layer**: Use cases and service orchestration
- **Infrastructure Layer**: External concerns (databases, APIs, etc.)
- **Presentation Layer**: UI components (Bevy, Yew)

### Vertical Slices

The domain is organized into a clean vertical slice structure:

1. **Domain Layer**: Pure business models (Budget, SavingsGoal, primitives)
2. **Application Layer**: Service orchestration (BudgetService, SavingsService)
3. **Infrastructure Layer**: Concrete implementations (repositories, p2p)
4. **Presentation Layer**: UI components (Bevy, Yew)

## Module Structure

```
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

## Key Components

### Domain Models

#### Financial Primitives

- `Money`: Amount with currency
- `Currency`: ISO 4217 currency codes
- `Amount`: Decimal value without currency

#### Budgeting

- `Budget`: Monthly allocation with tracking
- `BudgetPeriod`: Time period for budget (Monthly, Weekly, etc.)

#### Savings Goals

- `SavingsGoal`: Savings objective with target
- `SavingsProgress`: Progress tracking for goals

### Services

#### BudgetService

- Create and manage budgets
- Track spending against allocations
- Integrate with UBI for income calculations
- Reset monthly budgets

#### SavingsService

- Create and manage savings goals
- Track progress toward goals
- Manage data sharing preferences for UBI integration
- Calculate savings requirements

### Repositories

#### BudgetRepository

- Save and retrieve budgets
- Find budgets by user or category
- Reset monthly budget spending

#### SavingsRepository

- Save and retrieve savings goals
- Find goals by user or ID
- Delete goals

#### DataSharingRepository

- Manage data sharing preferences
- Privacy-preserving UBI integration

### External Services

#### UbiService

- Calculate monthly UBI income
- Privacy-preserving data sharing

## Data Flow

1. **User Interaction**: User interacts with the UI components (Bevy, Yew) or GraphQL API
2. **Application Layer**: Services orchestrate calls to domain models
3. **Domain Layer**: Domain models implement business logic
4. **Infrastructure Layer**: Repositories handle data persistence, p2p handles data sharing
5. **Response**: Results are returned through UI updates or API responses

## Privacy and Security

### Data Sharing Preferences

Users can opt-in to share anonymized financial data to improve the UBI system:

- `data_sharing_enabled`: Enable/disable data sharing
- `anonymized_data`: Ensure data is anonymized before sharing

### Consent Management

All UBI-related operations require explicit user consent:

- Users must enable data sharing preferences
- Clear indication of what data is being shared
- Ability to revoke consent at any time

## Database Schema

### budgets

```sql
CREATE TABLE budgets (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    category TEXT NOT NULL,
    allocated_amount NUMERIC(18, 2) NOT NULL,
    spent_amount NUMERIC(18, 2) NOT NULL DEFAULT 0,
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### savings_goals

```sql
CREATE TABLE savings_goals (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    name TEXT NOT NULL,
    target_amount NUMERIC(18, 2) NOT NULL,
    current_amount NUMERIC(18, 2) NOT NULL DEFAULT 0,
    deadline TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### data_sharing_preferences

```sql
CREATE TABLE data_sharing_preferences (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    data_sharing_enabled BOOLEAN NOT NULL DEFAULT false,
    anonymized_data BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id)
);
```

## Presentation Layer

### Bevy Components

- `FinancialViz`: 3D financial visualizations
- Interactive budget and savings goal dashboards

### Yew Components

- Web-based financial dashboard
- Responsive UI for budget management
- Savings goal tracking interface

## Future Enhancements

1. **Advanced Analytics**: Machine learning for spending predictions
2. **Multi-currency Support**: Full internationalization
3. **Investment Tracking**: Portfolio management integration
4. **Bill Reminders**: Automated payment notifications
5. **Financial Goal Planning**: Long-term financial planning tools

## Migration from Old Structure

If you were previously using the standalone `apps/personal-finance` application, please see [MIGRATION_GUIDE.md](../cpc-core/finance/MIGRATION_GUIDE.md) for detailed instructions on migrating to this module.