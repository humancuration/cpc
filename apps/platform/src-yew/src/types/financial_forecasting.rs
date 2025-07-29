use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueItem {
    pub id: String,
    pub name: String,
    pub amount: f64,
    pub growth_rate_monthly: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseItem {
    pub id: String,
    pub name: String,
    pub amount: f64,
    pub is_fixed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialStatement {
    pub id: String,
    pub start_date: String,
    pub end_date: String,
    pub revenue_items: Vec<RevenueItem>,
    pub expense_items: Vec<ExpenseItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastAssumption {
    pub item_id: String,
    pub new_growth_rate: Option<f64>,
    pub new_amount: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastScenario {
    pub id: String,
    pub name: String,
    pub description: String,
    pub initial_statement: FinancialStatement,
    pub forecast_horizon_months: u32,
    pub assumptions: Vec<ForecastAssumption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectedMonth {
    pub month: u32,
    pub year: i32,
    pub total_revenue: f64,
    pub total_expenses: f64,
    pub profit_loss: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastResult {
    pub scenario_id: String,
    pub monthly_projections: Vec<ProjectedMonth>,
}