//! Main finance service orchestrating all domain services

use std::sync::Arc;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use cpc_core::finance::{Money, Currency};

use crate::budgeting::domain::{
    models::{Budget, BudgetPeriod},
    budget_service::{BudgetService, UbiService, BudgetRepository},
};
use crate::expense_tracking::domain::{
    models::{Expense, Receipt, ExpenseCategory},
    expense_service::{ExpenseService, ExpenseRepository, TreasuryService, OcrService},
};
use crate::savings_goals::domain::{
    models::{SavingsGoal, SavingsProgress},
    savings_service::{SavingsService, SavingsRepository, DataSharingRepository},
};
use crate::domain::models::FinanceError;
use crate::infrastructure::database::models::DataSharingPreference;

/// Main finance service that orchestrates all financial operations
pub struct FinanceService {
    budget_service: std::sync::Arc<dyn BudgetService>,
    expense_service: std::sync::Arc<dyn ExpenseService>,
    savings_service: std::sync::Arc<dyn SavingsService>,
}

impl FinanceService {
    pub fn new(
        budget_service: std::sync::Arc<dyn BudgetService>,
        expense_service: std::sync::Arc<dyn ExpenseService>,
        savings_service: std::sync::Arc<dyn SavingsService>,
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

        // For analysis, we'll convert to a common currency (USD) if needed
        // In a real implementation, we'd need proper currency conversion
        let mut total_expenses = Money::zero(Currency::USD);
        for expense in &expenses {
            total_expenses = total_expenses.add(&expense.amount)
                .unwrap_or_else(|_| total_expenses.clone());
        }

        let mut total_budgeted = Money::zero(Currency::USD);
        for budget in &budgets {
            total_budgeted = total_budgeted.add(&budget.allocated_amount)
                .unwrap_or_else(|_| total_budgeted.clone());
        }

        let remaining_budget = total_budgeted.subtract(&total_expenses)
            .unwrap_or_else(|_| Money::zero(Currency::USD));

        // Calculate savings progress
        let mut total_target = Money::zero(Currency::USD);
        let mut total_current = Money::zero(Currency::USD);
        for goal in &savings_goals {
            total_target = total_target.add(&goal.target_amount)
                .unwrap_or_else(|_| total_target.clone());
            total_current = total_current.add(&goal.current_amount)
                .unwrap_or_else(|_| total_current.clone());
        }
        
        let savings_progress = if total_target.is_zero() {
            Decimal::ZERO
        } else {
            (total_current.amount / total_target.amount) * Decimal::from(100)
        };

        Ok(FinancialOverview {
            monthly_income,
            total_budgeted,
            total_expenses,
            remaining_budget,
            savings_progress,
            active_goals_count: savings_goals.len(),
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

        // Reset monthly budgets
        self.budget_service
            .reset_monthly_budgets(user_id)
            .await?;

        Ok(MonthlyCycleResult {
            monthly_income,
            budgets_reset: budgets.len(),
            savings_goals_count: savings_goals.len(),
        })
    }

    /// Add expense with receipt scanning
    pub async fn add_expense_with_receipt(
        &self,
        user_id: Uuid,
        amount: Money,
        receipt_image: Vec<u8>,
        category: Option<String>,
    ) -> Result<Expense, FinanceError> {
        if !receipt_image.is_empty() {
            self.expense_service
                .process_receipt_image(user_id, &receipt_image)
                .await
        } else {
            let description = category.unwrap_or_else(|| "Uncategorized".to_string());
            self.expense_service
                .create_expense(user_id, amount, description, "Unknown".to_string())
                .await
        }
    }

    /// Create savings goal
    pub async fn create_savings_goal(
        &self,
        user_id: Uuid,
        name: String,
        target_amount: Money,
        target_date: DateTime<Utc>,
    ) -> Result<SavingsGoal, FinanceError> {
        self.savings_service
            .create_goal(user_id, name, target_amount, target_date)
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
        
        // Calculate monthly expenses
        let mut monthly_expenses = Money::zero(Currency::USD);
        for expense in &expenses {
            if expense.date.format("%Y-%m").to_string() == current_month {
                monthly_expenses = monthly_expenses.add(&expense.amount)
                    .unwrap_or_else(|_| monthly_expenses.clone());
            }
        }

        let budget_utilization: Vec<CategoryUtilization> = budgets
            .iter()
            .map(|budget| {
                // Calculate spent amount for this budget category
                let mut spent = Money::zero(Currency::USD);
                for expense in &expenses {
                    if expense.category == budget.category {
                        spent = spent.add(&expense.amount)
                            .unwrap_or_else(|_| spent.clone());
                    }
                }
                
                // Calculate remaining budget
                let remaining = budget.remaining_amount();
                
                // Calculate utilization rate
                let utilization_rate = budget.utilization_percentage();
                
                CategoryUtilization {
                    category: budget.category.clone(),
                    budgeted: budget.allocated_amount.clone(),
                    spent,
                    remaining,
                    utilization_rate,
                }
            })
            .collect();

        Ok(BudgetInsights {
            total_monthly_expenses: monthly_expenses,
            budget_utilization,
        })
    }

    /// Update expense category
    pub async fn update_expense_category(
        &self,
        expense_id: Uuid,
        category: String,
    ) -> Result<Expense, FinanceError> {
        self.expense_service
            .update_expense_category(expense_id, category)
            .await
    }

    /// Get active savings goals
    pub async fn get_active_savings_goals(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<SavingsGoal>, FinanceError> {
        self.savings_service
            .get_active_goals(user_id)
            .await
    }

    /// Get savings goal progress
    pub async fn get_savings_goal_progress(
        &self,
        goal_id: Uuid,
    ) -> Result<SavingsProgress, FinanceError> {
        self.savings_service
            .get_goal_progress(goal_id)
            .await
    }

    /// Update savings goal progress
    pub async fn update_savings_goal_progress(
        &self,
        goal_id: Uuid,
        amount: Money,
    ) -> Result<SavingsGoal, FinanceError> {
        self.savings_service
            .update_goal_progress(goal_id, amount)
            .await
    }

    /// Update budget spent amount
    pub async fn update_budget_spent(
        &self,
        user_id: Uuid,
        category: &str,
        amount: Money,
    ) -> Result<Budget, FinanceError> {
        self.budget_service
            .update_spent_amount(user_id, category, amount)
            .await
    }

    /// Get data sharing preference
    pub async fn get_data_sharing_preference(
        &self,
        user_id: Uuid,
    ) -> Result<DataSharingPreference, FinanceError> {
        self.savings_service
            .get_data_sharing_preference(user_id)
            .await
    }

    /// Update data sharing preference
    pub async fn update_data_sharing_preference(
        &self,
        user_id: Uuid,
        enabled: bool,
        anonymized: bool,
    ) -> Result<DataSharingPreference, FinanceError> {
        self.savings_service
            .update_data_sharing_preference(user_id, enabled, anonymized)
            .await
    }
}

/// Financial overview for dashboard
#[derive(Debug, Clone)]
pub struct FinancialOverview {
    pub monthly_income: Money,
    pub total_budgeted: Money,
    pub total_expenses: Money,
    pub remaining_budget: Money,
    pub savings_progress: Decimal,
    pub active_goals_count: usize,
    pub budgets: Vec<Budget>,
    pub expenses: Vec<Expense>,
    pub savings_goals: Vec<SavingsGoal>,
}

/// Monthly cycle processing result
#[derive(Debug, Clone)]
pub struct MonthlyCycleResult {
    pub monthly_income: Money,
    pub budgets_reset: usize,
    pub savings_goals_count: usize,
}

/// Budget insights for analytics
#[derive(Debug, Clone)]
pub struct BudgetInsights {
    pub total_monthly_expenses: Money,
    pub budget_utilization: Vec<CategoryUtilization>,
}

/// Category utilization metrics
#[derive(Debug, Clone)]
pub struct CategoryUtilization {
    pub category: String,
    pub budgeted: Money,
    pub spent: Money,
    pub remaining: Money,
    pub utilization_rate: Decimal,
}