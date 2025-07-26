//! Repository for savings goal persistence

use uuid::Uuid;
use rust_decimal::Decimal;
use sqlx::PgPool;
use chrono::{DateTime, Utc};

use crate::domain::{models::{SavingsGoal, FinanceError}, savings_service::SavingsRepository};

/// PostgreSQL implementation of SavingsRepository
pub struct PostgresSavingsRepository {
    pool: PgPool,
}

impl PostgresSavingsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl SavingsRepository for PostgresSavingsRepository {
    async fn save(&self, goal: &SavingsGoal) -> Result<(), FinanceError> {
        sqlx::query!(
            r#"
            INSERT INTO savings_goals (id, user_id, name, target_amount, current_amount, target_date, category, is_completed, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                target_amount = EXCLUDED.target_amount,
                current_amount = EXCLUDED.current_amount,
                target_date = EXCLUDED.target_date,
                category = EXCLUDED.category,
                is_completed = EXCLUDED.is_completed,
                updated_at = EXCLUDED.updated_at
            "#,
            goal.id,
            goal.user_id,
            goal.name,
            goal.target_amount,
            goal.current_amount,
            goal.target_date,
            goal.category,
            goal.is_completed,
            goal.created_at,
            goal.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<SavingsGoal>, FinanceError> {
        let goals = sqlx::query_as!(
            SavingsGoal,
            r#"
            SELECT id, user_id, name, target_amount, current_amount, target_date, category, is_completed, created_at, updated_at
            FROM savings_goals
            WHERE user_id = $1
            ORDER BY target_date
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        Ok(goals)
    }

    async fn find_active_by_user_id(&self, user_id: Uuid) -> Result<Vec<SavingsGoal>, FinanceError> {
        let goals = sqlx::query_as!(
            SavingsGoal,
            r#"
            SELECT id, user_id, name, target_amount, current_amount, target_date, category, is_completed, created_at, updated_at
            FROM savings_goals
            WHERE user_id = $1 AND is_completed = false
            ORDER BY target_date
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        Ok(goals)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<SavingsGoal>, FinanceError> {
        let goal = sqlx::query_as!(
            SavingsGoal,
            r#"
            SELECT id, user_id, name, target_amount, current_amount, target_date, category, is_completed, created_at, updated_at
            FROM savings_goals
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        Ok(goal)
    }

    async fn delete(&self, id: Uuid) -> Result<(), FinanceError> {
        sqlx::query!(
            r#"
            DELETE FROM savings_goals
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
pub struct MockSavingsRepository;

#[async_trait::async_trait]
impl SavingsRepository for MockSavingsRepository {
    async fn save(&self, _goal: &SavingsGoal) -> Result<(), FinanceError> {
        Ok(())
    }

    async fn find_by_user_id(&self, _user_id: Uuid) -> Result<Vec<SavingsGoal>, FinanceError> {
        Ok(Vec::new())
    }

    async fn find_active_by_user_id(&self, _user_id: Uuid) -> Result<Vec<SavingsGoal>, FinanceError> {
        Ok(Vec::new())
    }

    async fn find_by_id(&self, _id: Uuid) -> Result<Option<SavingsGoal>, FinanceError> {
        Ok(None)
    }

    async fn delete(&self, _id: Uuid) -> Result<(), FinanceError> {
        Ok(())
    }
}