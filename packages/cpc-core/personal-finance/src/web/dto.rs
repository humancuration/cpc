//! Data Transfer Objects for the web layer
//!
//! These structs define the API request/response formats and GraphQL types

use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Financial overview response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialOverviewDto {
    pub user_id: Uuid,
    pub current_balance: Decimal,
    pub monthly_income: Decimal,
    pub monthly_expenses: Decimal,
    pub savings_rate: Decimal,
    pub total_savings: Decimal,
    pub budget_summary: BudgetSummaryDto,
    pub recent_expenses: Vec<ExpenseDto>,
    pub active_savings_goals: Vec<SavingsGoalDto>,
}

/// Budget summary information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetSummaryDto {
    pub total_allocated: Decimal,
    pub total_spent: Decimal,
    pub remaining: Decimal,
    pub category_breakdown: Vec<CategoryBreakdownDto>,
}

/// Category breakdown within budget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryBreakdownDto {
    pub category: String,
    pub allocated: Decimal,
    pub spent: Decimal,
    pub remaining: Decimal,
    pub percentage_used: Decimal,
}

/// Expense DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: Decimal,
    pub category: String,
    pub description: String,
    pub date: NaiveDate,
}

/// Savings goal DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavingsGoalDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub target_amount: Decimal,
    pub current_amount: Decimal,
    pub deadline: NaiveDate,
    pub monthly_contribution: Decimal,
}

/// Create expense request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExpenseRequest {
    pub user_id: Uuid,
    pub amount: Decimal,
    pub category: String,
    pub description: String,
    pub date: NaiveDate,
}

/// Create budget request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBudgetRequest {
    pub user_id: Uuid,
    pub month: NaiveDate,
    pub total_income: Decimal,
    pub categories: Vec<BudgetCategoryRequest>,
}

/// Budget category allocation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetCategoryRequest {
    pub name: String,
    pub allocated_amount: Decimal,
}

/// Create savings goal request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSavingsGoalRequest {
    pub user_id: Uuid,
    pub name: String,
    pub target_amount: Decimal,
    pub deadline: NaiveDate,
    pub monthly_contribution: Decimal,
}

/// Receipt processing request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptProcessingRequest {
    pub user_id: Uuid,
    pub image_data: Option<Vec<u8>>,
    pub image_url: Option<String>,
    pub mime_type: Option<String>,
}

/// Receipt processing response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptProcessingResponse {
    pub merchant_name: String,
    pub total_amount: Decimal,
    pub date: NaiveDate,
    pub category: Option<String>,
    pub items: Vec<ReceiptItemDto>,
}

/// Receipt item DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptItemDto {
    pub name: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub total_price: Decimal,
}

/// Financial insights response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialInsightsDto {
    pub user_id: Uuid,
    pub insights: Vec<String>,
    pub trends: Vec<TrendDataDto>,
}

/// Trend data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendDataDto {
    pub date: NaiveDate,
    pub value: Decimal,
    pub metric: String,
}

/// Update budget request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBudgetRequest {
    pub budget_id: Uuid,
    pub total_income: Option<Decimal>,
    pub categories: Option<Vec<BudgetCategoryRequest>>,
}

/// Update savings goal request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSavingsGoalRequest {
    pub savings_goal_id: Uuid,
    pub current_amount: Option<Decimal>,
    pub monthly_contribution: Option<Decimal>,
}

/// Error response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

/// Success response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ErrorResponse>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error: String, message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ErrorResponse { error, message }),
        }
    }
}