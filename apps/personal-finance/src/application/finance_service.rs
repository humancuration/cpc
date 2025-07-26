//! Main finance service orchestrating all domain services

use std::sync::Arc;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

use crate::domain::{
    models::{Budget, Expense, SavingsGoal, FinanceError},
    budget_service::{BudgetService, UbiService},
    expense_service::{ExpenseService, ReceiptScanner, OcrService},
    savings_service::{SavingsService, TreasuryService},
};

/// Main finance service that orchestrates all financial operations
pub struct FinanceService {
    budget_service: Arc<BudgetService>,
    expense_service: Arc<ExpenseService>,
    savings_service: Arc<SavingsService>,
}

impl FinanceService {
    pub fn new(
        budget_service: Arc<BudgetService>,
        expense_service: Arc<ExpenseService>,
        savings_service: Arc<SavingsService>,
    ) -> Self {
        Self {
            budget_service,
            expense_service,
            savings_service,
        }
    }

    /// Get comprehensive financial overview for a user
    pub async fn get_financial_overview(
        &self,
        user_id: Uuid,
    ) -> Result<FinancialOverview, FinanceError> {
        let monthly_income = self.budget_service
            .get_monthly_ubi_income(user_id)
            .await?;

        let budgets = self.budget_service
            .get_user_budgets(user_id)
            .await?;

        let expenses = self.expense_service
            .get_user_expenses(user_id)
            .await?;

        let savings_goals = self.savings_service
            .get_user_goals(user_id)
            .await?;

        let savings_report = self.savings_service
            .get_savings_report(savings_goals.clone())
            .await?;

        let total_expenses: Decimal = expenses.iter()
            .map(|e| e.amount)
            .sum();

        let total_budgeted: Decimal = budgets.iter()
            .map(|b| b.allocated_amount)
            .sum();

        let remaining_budget = total_budgeted - total_expenses;

        Ok(FinancialOverview {
            monthly_income,
            total_budgeted,
            total_expenses,
            remaining_budget,
            savings_rate: self.savings_service.auto_deduction_percentage,
            savings_progress: savings_report.completion_rate,
            active_goals_count: savings_report.active_goals_count,
            budgets,
            expenses,
            savings_goals,
        })
    }

    /// Process monthly financial cycle
    pub async fn process_monthly_cycle(
        &self,
        user_id: Uuid,
    ) -> Result<MonthlyCycleResult, FinanceError> {
        let monthly_income = self.budget_service
            .get_monthly_ubi_income(user_id)
            .await?;

        let budgets = self.budget_service
            .get_user_budgets(user_id)
            .await?;

        let savings_goals = self.savings_service
            .get_user_goals(user_id)
            .await?;

        // Auto-deduct savings from income
        let deductions = self.savings_service
            .auto_deduct_savings(user_id, monthly_income, savings_goals)
            .await?;

        // Reset monthly budgets
        self.budget_service
            .reset_monthly_budgets(user_id)
            .await?;

        Ok(MonthlyCycleResult {
            monthly_income,
            total_saved: deductions.iter().map(|d| d.amount).sum(),
            deductions,
        })
    }

    /// Add expense with receipt scanning
    pub async fn add_expense_with_receipt(
        &self,
        user_id: Uuid,
        amount: Decimal,
        receipt_image: Vec<u8>,
        category: Option<String>,
    ) -> Result<Expense, FinanceError> {
        let mut expense = Expense::new(
            user_id,
            amount,
            category.unwrap_or_else(|| "Uncategorized".to_string()),
            Utc::now(),
        );

        if !receipt_image.is_empty() {
            self.expense_service
                .categorize_from_receipt(&mut expense, receipt_image)
                .await?;
        }

        self.expense_service
            .add_expense(expense.clone())
            .await?;

        Ok(expense)
    }

    /// Create savings goal
    pub async fn create_savings_goal(
        &self,
        user_id: Uuid,
        name: String,
        target_amount: Decimal,
        target_date: DateTime<Utc>,
        category: String,
    ) -> Result<SavingsGoal, FinanceError> {
        self.savings_service
            .create_savings_goal(user_id, name, target_amount, target_date, category)
            .await
    }

    /// Get budget insights
    pub async fn get_budget_insights(
        &self,
        user_id: Uuid,
    ) -> Result<BudgetInsights, FinanceError> {
        let budgets = self.budget_service
            .get_user_budgets(user_id)
            .await?;

        let expenses = self.expense_service
            .get_user_expenses(user_id)
            .await?;

        let current_month = Utc::now().format("%Y-%m").to_string();
        let monthly_expenses: Decimal = expenses.iter()
            .filter(|e| e.date.format("%Y-%m").to_string() == current_month)
            .map(|e| e.amount)
            .sum();

        let budget_utilization: Vec<CategoryUtilization> = budgets
            .iter()
            .map(|budget| {
                let spent = expenses.iter()
                    .filter(|e| e.category == budget.category)
                    .map(|e| e.amount)
                    .sum::<Decimal>();
                
                CategoryUtilization {
                    category: budget.category.clone(),
                    budgeted: budget.allocated_amount,
                    spent,
                    remaining: budget.allocated_amount - spent,
                    utilization_rate: if budget.allocated_amount > Decimal::ZERO {
                        (spent / budget.allocated_amount) * Decimal::from(100)
                    } else {
                        Decimal::ZERO
                    },
                }
            })
            .collect();

        Ok(BudgetInsights {
            total_monthly_expenses,
            budget_utilization,
        })
    }
}

/// Financial overview for dashboard
#[derive(Debug, Clone)]
pub struct FinancialOverview {
    pub monthly_income: Decimal,
    pub total_budgeted: Decimal,
    pub total_expenses: Decimal,
    pub remaining_budget: Decimal,
    pub savings_rate: Decimal,
    pub savings_progress: Decimal,
    pub active_goals_count: usize,
    pub budgets: Vec<Budget>,
    pub expenses: Vec<Expense>,
    pub savings_goals: Vec<SavingsGoal>,
}

/// Monthly cycle processing result
#[derive(Debug, Clone)]
pub struct MonthlyCycleResult {
    pub monthly_income: Decimal,
    pub total_saved: Decimal,
    pub deductions: Vec<crate::domain::savings_service::SavingsDeduction>,
}

/// Budget insights for analytics
#[derive(Debug, Clone)]
pub struct BudgetInsights {
    pub total_monthly_expenses: Decimal,
    pub budget_utilization: Vec<CategoryUtilization>,
}

/// Category utilization metrics
#[derive(Debug, Clone)]
pub struct CategoryUtilization {
    pub category: String,
    pub budgeted: Decimal,
    pub spent: Decimal,
    pub remaining: Decimal,
    pub utilization_rate: Decimal,
}