//! Budget service for managing budgets and allocations

use async_trait::async_trait;
use uuid::Uuid;
use crate::budgeting::domain::models::{Budget, BudgetPeriod};
use crate::domain::models::FinanceError;
use cpc_core::finance::Money;

#[async_trait]
pub trait BudgetRepository {
    async fn save(&self, budget: &Budget) -> Result<(), FinanceError>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Budget>, FinanceError>;
    async fn find_by_user_and_category(&self, user_id: Uuid, category: &str) -> Result<Option<Budget>, FinanceError>;
    async fn reset_monthly_budgets(&self, user_id: Uuid) -> Result<(), FinanceError>;
}

#[async_trait]
pub trait UbiService {
    async fn get_monthly_ubi_income(&self, user_id: Uuid) -> Result<Money, FinanceError>;
}

#[async_trait]
pub trait BudgetService {
    async fn create_budget(&self, user_id: Uuid, category: String, allocated_amount: Money, period: BudgetPeriod, start_date: chrono::DateTime<chrono::Utc>, end_date: chrono::DateTime<chrono::Utc>) -> Result<Budget, FinanceError>;
    async fn get_user_budgets(&self, user_id: Uuid) -> Result<Vec<Budget>, FinanceError>;
    async fn get_budget_by_category(&self, user_id: Uuid, category: &str) -> Result<Option<Budget>, FinanceError>;
    async fn update_spent_amount(&self, user_id: Uuid, category: &str, amount: Money) -> Result<Budget, FinanceError>;
    async fn get_monthly_ubi_income(&self, user_id: Uuid) -> Result<Money, FinanceError>;
    async fn reset_monthly_budgets(&self, user_id: Uuid) -> Result<(), FinanceError>;
}

pub struct BudgetServiceImpl {
    budget_repo: std::sync::Arc<dyn BudgetRepository>,
    ubi_service: std::sync::Arc<dyn UbiService>,
}

impl BudgetServiceImpl {
    pub fn new(
        budget_repo: std::sync::Arc<dyn BudgetRepository>,
        ubi_service: std::sync::Arc<dyn UbiService>,
    ) -> Self {
        Self {
            budget_repo,
            ubi_service,
        }
    }
}

#[async_trait]
impl BudgetService for BudgetServiceImpl {
    async fn create_budget(&self, user_id: Uuid, category: String, allocated_amount: Money, period: BudgetPeriod, start_date: chrono::DateTime<chrono::Utc>, end_date: chrono::DateTime<chrono::Utc>) -> Result<Budget, FinanceError> {
        let budget = Budget::new(user_id, category, allocated_amount, period, start_date, end_date);
        self.budget_repo.save(&budget).await?;
        Ok(budget)
    }

    async fn get_user_budgets(&self, user_id: Uuid) -> Result<Vec<Budget>, FinanceError> {
        self.budget_repo.find_by_user_id(user_id).await
    }

    async fn get_budget_by_category(&self, user_id: Uuid, category: &str) -> Result<Option<Budget>, FinanceError> {
        self.budget_repo.find_by_user_and_category(user_id, category).await
    }

    async fn update_spent_amount(&self, user_id: Uuid, category: &str, amount: Money) -> Result<Budget, FinanceError> {
        let mut budget = self.budget_repo.find_by_user_and_category(user_id, category).await?
            .ok_or_else(|| FinanceError::BudgetNotFound(Uuid::nil()))?; // TODO: Fix this error handling
        
        let new_spent = budget.spent_amount.add(&amount)
            .map_err(|e| FinanceError::InvalidAmount(e.to_string()))?;
        budget.spent_amount = new_spent;
        
        self.budget_repo.save(&budget).await?;
        Ok(budget)
    }

    async fn get_monthly_ubi_income(&self, user_id: Uuid) -> Result<Money, FinanceError> {
        self.ubi_service.get_monthly_ubi_income(user_id).await
    }

    async fn reset_monthly_budgets(&self, user_id: Uuid) -> Result<(), FinanceError> {
        self.budget_repo.reset_monthly_budgets(user_id).await
    }
}