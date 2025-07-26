//! Repository for expense persistence

use uuid::Uuid;
use rust_decimal::Decimal;
use sqlx::PgPool;
use chrono::{DateTime, Utc};

use crate::domain::{models::{Expense, FinanceError}, expense_service::ExpenseRepository};

/// PostgreSQL implementation of ExpenseRepository
pub struct PostgresExpenseRepository {
    pool: PgPool,
}

impl PostgresExpenseRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ExpenseRepository for PostgresExpenseRepository {
    async fn save(&self, expense: &Expense) -> Result<(), FinanceError> {
        sqlx::query!(
            r#"
            INSERT INTO expenses (id, user_id, amount, category, description, date, merchant, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                amount = EXCLUDED.amount,
                category = EXCLUDED.category,
                description = EXCLUDED.description,
                merchant = EXCLUDED.merchant,
                updated_at = EXCLUDED.updated_at
            "#,
            expense.id,
            expense.user_id,
            expense.amount,
            expense.category,
            expense.description,
            expense.date,
            expense.merchant,
            expense.created_at,
            expense.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Expense>, FinanceError> {
        let expenses = sqlx::query_as!(
            Expense,
            r#"
            SELECT id, user_id, amount, category, description, date, merchant, created_at, updated_at
            FROM expenses
            WHERE user_id = $1
            ORDER BY date DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        Ok(expenses)
    }

    async fn find_by_user_and_category(
        &self,
        user_id: Uuid,
        category: &str,
    ) -> Result<Vec<Expense>, FinanceError> {
        let expenses = sqlx::query_as!(
            Expense,
            r#"
            SELECT id, user_id, amount, category, description, date, merchant, created_at, updated_at
            FROM expenses
            WHERE user_id = $1 AND category = $2
            ORDER BY date DESC
            "#,
            user_id,
            category
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        Ok(expenses)
    }

    async fn find_by_date_range(
        &self,
        user_id: Uuid,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<Expense>, FinanceError> {
        let expenses = sqlx::query_as!(
            Expense,
            r#"
            SELECT id, user_id, amount, category, description, date, merchant, created_at, updated_at
            FROM expenses
            WHERE user_id = $1 AND date BETWEEN $2 AND $3
            ORDER BY date DESC
            "#,
            user_id,
            start_date,
            end_date
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        Ok(expenses)
    }

    async fn delete(&self, id: Uuid) -> Result<(), FinanceError> {
        sqlx::query!(
            r#"
            DELETE FROM expenses
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

/// Mock repository for testing
pub struct MockExpenseRepository;

#[async_trait::async_trait]
impl ExpenseRepository for MockExpenseRepository {
    async fn save(&self, _expense: &Expense) -> Result<(), FinanceError> {
        Ok(())
    }

    async fn find_by_user_id(&self, _user_id: Uuid) -> Result<Vec<Expense>, FinanceError> {
        Ok(Vec::new())
    }

    async fn find_by_user_and_category(
        &self,
        _user_id: Uuid,
        _category: &str,
    ) -> Result<Vec<Expense>, FinanceError> {
        Ok(Vec::new())
    }

    async fn find_by_date_range(
        &self,
        _user_id: Uuid,
        _start_date: DateTime<Utc>,
        _end_date: DateTime<Utc>,
    ) -> Result<Vec<Expense>, FinanceError> {
        Ok(Vec::new())
    }

    async fn delete(&self, _id: Uuid) -> Result<(), FinanceError> {
        Ok(())
    }
}