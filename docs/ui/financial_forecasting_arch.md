# Financial Forecasting Dashboard: Architectural Documentation

## 1. Overview

This document specifies the architecture for the **Financial Forecasting Dashboard**, a vertical slice providing advanced business intelligence tools for cooperatives and small businesses. It enables users to project future financial performance based on historical data and defined assumptions.

The dashboard will allow users to:
- Input and manage revenue streams and expense items.
- Generate profit and loss forecasts over user-defined time horizons.
- Visualize financial projections using interactive charts.
- Create and compare different financial scenarios (e.g., "optimistic," "pessimistic," "base case").
- Integrate with cooperative accounting data sources via APIs or manual data entry.

## 2. Architectural Principles

The implementation will strictly adhere to our established architectural principles:

- **Hexagonal Architecture**: The core financial logic in `cpc-core` will be completely independent of the Yew UI, GraphQL API, and any data source integrations. Ports will be defined as Rust service traits.
- **Screaming Architecture**: The project structure will reflect its domain. All new core logic for this feature will reside in `packages/cpc-core/src/financial_forecasting/`.
- **Vertical Slices**: This feature is a self-contained unit. All components, from the data models and forecasting algorithms to the UI components, are developed as a cohesive slice of functionality.
- **Rust-first**: All new business logic, services, and UI components will be implemented in Rust.

## 3. High-Level Dependency Diagram

This diagram shows the flow of data and commands through the system.

```mermaid
graph TD
    subgraph Yew UI (cpc-platform)
        A[ForecastingDashboard Component] -->|Calls Mutation| B(GraphQL Mutation: run_forecast);
        A -->|Subscribes to| C(GraphQL Subscription: forecast_results);
        D[Plotters Charting Library] -->|Renders data in| A;
    end

    subgraph Backend (Axum Server)
        B -->|Invokes| E{FinancialForecastingService};
        C -->|Receives from| F(Broadcast Channel);
    end

    subgraph Core Logic (cpc-core)
        E -->|Uses| G[Forecasting Algorithms];
        E -->|Processes| H[FinancialData Models];
        E -->|Interacts with| I[Accounting Data Adapter];
        E -->|Notifies via| F;
    end
    
    I -->|Fetches/Transforms| J[(Co-op Accounting System)];

```

## 4. Detailed Implementation Guide

### 4.1. Core Domain: Data Models, Algorithms, and Service

**Location**: `packages/cpc-core/src/`

1.  **Create a new module**: `financial_forecasting/mod.rs`.
2.  **Define Data Models** in `packages/cpc-core/src/financial_forecasting/models.rs`. These structs are the canonical representation of our financial data.

    ```rust
    // packages/cpc-core/src/financial_forecasting/models.rs
    use serde::{Deserialize, Serialize};
    use chrono::{DateTime, Utc};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct FinancialStatement {
        pub id: String,
        pub start_date: DateTime<Utc>,
        pub end_date: DateTime<Utc>,
        pub revenue_items: Vec<RevenueItem>,
        pub expense_items: Vec<ExpenseItem>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct RevenueItem {
        pub id: String,
        pub name: String,
        pub amount: f64,
        pub growth_rate_monthly: f64, // e.g., 0.02 for 2%
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ExpenseItem {
        pub id: String,
        pub name: String,
        pub amount: f64,
        pub is_fixed: bool, // Fixed (rent) vs. Variable (cost of goods)
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ForecastScenario {
        pub id: String,
        pub name: String, // "Optimistic", "Pessimistic"
        pub description: String,
        pub initial_statement: FinancialStatement,
        pub forecast_horizon_months: u32,
        pub assumptions: Vec<ForecastAssumption>,
    }
    
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ForecastAssumption {
        pub item_id: String, // ID of the RevenueItem or ExpenseItem
        pub new_growth_rate: Option<f64>,
        pub new_amount: Option<f64>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ForecastResult {
        pub scenario_id: String,
        pub monthly_projections: Vec<ProjectedMonth>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ProjectedMonth {
        pub month: u32,
        pub year: i32,
        pub total_revenue: f64,
        pub total_expenses: f64,
        pub profit_loss: f64,
    }
    ```

3.  **Define Forecasting Algorithms** in `packages/cpc-core/src/financial_forecasting/algorithms.rs`. These are pure functions that take data and return a forecast.

    ```rust
    // packages/cpc-core/src/financial_forecasting/algorithms.rs
    use super::models::{ForecastScenario, ForecastResult, ProjectedMonth};

    pub fn project_profit_and_loss(scenario: &ForecastScenario) -> ForecastResult {
        // Core projection logic will be implemented here.
        // It will loop from 1 to `forecast_horizon_months`, calculating
        // revenue and expenses for each month based on growth rates and assumptions.
        unimplemented!();
    }
    ```

4.  **Define the Service** in `packages/cpc-core/src/financial_forecasting/service.rs`. This service orchestrates the forecasting process.

    ```rust
    // packages/cpc-core/src/financial_forecasting/service.rs
    use super::models::{ForecastScenario, ForecastResult};
    use super::algorithms;
    use anyhow::Result;
    use tokio::sync::broadcast;
    use std::sync::Arc;

    // A port for fetching accounting data from an external source
    pub trait AccountingAdapter: Send + Sync {
        fn get_latest_financials(&self) -> Result<FinancialStatement>;
    }
    
    pub struct FinancialForecastingService {
        // For long-running jobs, we notify on completion
        result_notifier: broadcast::Sender<Arc<ForecastResult>>,
        // Potentially connect to a real accounting system in the future
        accounting_adapter: Option<Arc<dyn AccountingAdapter>>,
    }

    impl FinancialForecastingService {
        pub fn new() -> Self {
            let (tx, _rx) = broadcast::channel(100);
            Self { result_notifier: tx, accounting_adapter: None }
        }

        /// Runs a new forecast scenario. This is an async task.
        pub async fn run_forecast(&self, scenario: ForecastScenario) {
            let notifier = self.result_notifier.clone();
            tokio::spawn(async move {
                let result = algorithms::project_profit_and_loss(&scenario);
                // The result is broadcast to any active subscribers (e.g., GraphQL subscriptions)
                let _ = notifier.send(Arc::new(result));
            });
        }

        /// Subscribes to the results of completed forecast jobs.
        pub fn get_result_stream(&self) -> broadcast::Receiver<Arc<ForecastResult>> {
            self.result_notifier.subscribe()
        }
    }
    ```

### 4.2. GraphQL API Layer

**Location**: `apps/backend/src/graphql/`

1.  **Create `financial_forecasting.rs`**: This file will define the GraphQL schema for forecasting.

    ```rust
    // apps/backend/src/graphql/financial_forecasting.rs
    use async_graphql::*;
    use cpc_core::financial_forecasting::{models, service::FinancialForecastingService};
    use futures::Stream;
    use async_stream::stream;
    use std::sync::Arc;

    // Expose core models via GraphQL types (likely using SimpleObject derive)
    
    #[derive(Default)]
    pub struct FinancialForecastingQueryRoot;

    #[Object]
    impl FinancialForecastingQueryRoot {
        // Queries could be for fetching historical data or saved scenarios
        async fn get_saved_scenarios(&self, _ctx: &Context<'_>) -> Result<Vec<models::ForecastScenario>> {
            unimplemented!()
        }
    }
    
    #[derive(Default)]
    pub struct FinancialForecastingMutationRoot;

    #[Object]
    impl FinancialForecastingMutationRoot {
        /// Kicks off a financial forecast. The result will be sent via subscription.
        async fn run_forecast_scenario(&self, ctx: &Context<'_>, scenario: models::ForecastScenario) -> Result<bool> {
            let service = ctx.data_unchecked::<FinancialForecastingService>();
            service.run_forecast(scenario).await;
            Ok(true)
        }
    }

    #[derive(Default)]
    pub struct FinancialForecastingSubscriptionRoot;
    
    #[Subscription]
    impl FinancialForecastingSubscriptionRoot {
        async fn forecast_result(&self, ctx: &Context<'_>, scenario_id: String) -> impl Stream<Item = Arc<models::ForecastResult>> {
            let service = ctx.data_unchecked::<FinancialForecastingService>();
            let mut rx = service.get_result_stream();

            stream! {
                while let Ok(result) = rx.recv().await {
                    if result.scenario_id == scenario_id {
                        yield result;
                    }
                }
            }
        }
    }
    ```

2.  **Integrate into `schema.rs`**: Merge the new roots into the main schema.

### 4.3. Frontend Yew Component

**Location**: `apps/cpc-platform/src-yew/src/components/financials/`

The frontend will consist of:
1.  **A form component**: For creating and editing `ForecastScenario` objects (defining revenue/expense items, growth rates, etc.).
2.  **A control panel**: To trigger the `run_forecast_scenario` mutation.
3.  **A display component**: This component will subscribe to `forecast_result` and, upon receiving data, use `plotters-rs` and `plotters-canvas` to render the `ForecastResult`. It will draw line charts for Total Revenue, Total Expenses, and Profit/Loss over the projected timeline.

### 5. List of File Modifications and Creations

**New Files:**
- `docs/ui/financial_forecasting_arch.md`
- `packages/cpc-core/src/financial_forecasting/mod.rs`
- `packages/cpc-core/src/financial_forecasting/models.rs`
- `packages/cpc-core/src/financial_forecasting/service.rs`
- `packages/cpc-core/src/financial_forecasting/algorithms.rs`
- `apps/backend/src/graphql/financial_forecasting.rs`
- `apps/cpc-platform/src-yew/src/components/financials/dashboard.rs`
- `apps/cpc-platform/src-yew/src/components/financials/chart.rs`

**Modified Files:**
- `packages/cpc-core/src/services/mod.rs`: To add `pub mod financial_forecasting;`.
- `apps/backend/src/graphql/schema.rs`: To merge the new financial forecasting GraphQL roots.
- `apps/backend/src/main.rs`: To initialize and manage `FinancialForecastingService` in the GraphQL context.