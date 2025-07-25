// packages/cpc-core/src/financial_forecasting/models.rs
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use async_graphql::InputObject;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, InputObject)]
#[graphql(name = "FinancialStatementInput")]
pub struct FinancialStatement {
    pub id: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub revenue_items: Vec<RevenueItem>,
    pub expense_items: Vec<ExpenseItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, InputObject)]
#[graphql(name = "RevenueItemInput")]
pub struct RevenueItem {
    pub id: String,
    pub name: String,
    pub amount: f64,
    pub growth_rate_monthly: f64, // e.g., 0.02 for 2%
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, InputObject)]
#[graphql(name = "ExpenseItemInput")]
pub struct ExpenseItem {
    pub id: String,
    pub name: String,
    pub amount: f64,
    pub is_fixed: bool, // Fixed (rent) vs. Variable (cost of goods)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, InputObject)]
pub struct ForecastScenario {
    pub id: String,
    pub name: String, // "Optimistic", "Pessimistic"
    pub description: String,
    pub initial_statement: FinancialStatement,
    pub forecast_horizon_months: u32,
    pub assumptions: Vec<ForecastAssumption>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, InputObject)]
#[graphql(name = "ForecastAssumptionInput")]
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