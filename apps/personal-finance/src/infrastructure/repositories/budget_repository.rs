//! Repository for budget persistence

use uuid::Uuid;
use rust_decimal::Decimal;
use sqlx::PgPool;
use chrono::{DateTime, Utc};

use crate::domain::{models::{Budget, FinanceError}, budget_service::BudgetRepository};

/// PostgreSQL implementation of BudgetRepository
pub struct PostgresBudgetRepository {
    pool: PgPool,
}

impl PostgresBudgetRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl BudgetRepository for PostgresBudgetRepository {
    async fn save(&self, budget: &Budget) -> Result<(), FinanceError> {
        sqlx::query!(
            r#"
            INSERT INTO budgets (id, user_id, category, allocated_amount, spent_amount, period_start, period_end, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                allocated_amount = EXCLUDED.allocated_amount,
                spent_amount = EXCLUDED.spent_amount,
                updated_at = EXCLUDED.updated_at
            "#,
            budget.id,
            budget.user_id,
            budget.category,
            budget.allocated_amount,
            budget.spent_amount,
            budget.period_start,
            budget.period_end,
            budget.created_at,
            budget.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Budget>, FinanceError> {
        let budgets = sqlx::query_as!(
            Budget,
            r#"
            SELECT id, user_id, category, allocated_amount, spent_amount, period_start, period_end, created_at, updated_at
            FROM budgets
            WHERE user_id = $1
            ORDER BY category
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        Ok(budgets)
    }

    async fn find_by_user_and_category(
        &self,
        user_id: Uuid,
        category: &str,
    ) -> Result<Option<Budget>, FinanceError> {
        let budget = sqlx::query_as!(
            Budget,
            r#"
            SELECT id, user_id, category, allocated_amount, spent_amount, period_start, period_end, created_at, updated_at
            FROM budgets
            WHERE user_id = $1 AND category = $2
            "#,
            user_id,
            category
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        Ok(budget)
    }

    async fn reset_monthly_budgets(&self, user_id: Uuid) -> Result<(), FinanceError> {
        let current_month = Utc::now().format("%Y-%m").to_string();
        
        sqlx::query!(
            r#"
            UPDATE budgets
            SET spent_amount = 0, updated_at = NOW()
            WHERE user_id = $1 
            AND TO_CHAR(period_start, 'YYYY-MM') = $2
            "#,
            user_id,
            current_month
        )
        .execute(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

/// Mock repository for testing
pub struct MockBudgetRepository;

#[async_trait::async_trait]
impl BudgetRepository for MockBudgetRepository {
    async fn save(&self, _budget: &Budget) -> Result<(), FinanceError> {
        Ok(())
    }

    async fn find_by_user_id(&self, _user_id: Uuid) -> Result<Vec<Budget>, FinanceError> {
        Ok(Vec::new())
    }

    async fn find_by_user_and_category(
        &self,
        _user_id: Uuid,
        _category: &str,
    ) -> Result<Option<Budget>, FinanceError> {
        Ok(None)
    }

    async fn reset_monthly_budgets(&self, _user_id: Uuid) -> Result<(), FinanceError> {
        Ok(())
    }
}