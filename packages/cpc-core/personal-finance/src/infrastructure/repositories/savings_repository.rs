//! Repository for savings goal persistence

use uuid::Uuid;
use rust_decimal::Decimal;
use sqlx::PgPool;
use chrono::{DateTime, Utc};
use cpc_core::finance::Currency;

use crate::savings_goals::domain::models::{SavingsGoal, SavingsProgress};
use crate::infrastructure::database::models::SavingsGoalDbModel;
use crate::domain::models::FinanceError;
use crate::savings_goals::domain::savings_service::SavingsRepository;

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
        let goal_db_model = SavingsGoalDbModel::from_domain(goal);
        
        sqlx::query!(
            r#"
            INSERT INTO savings_goals (id, user_id, name, target_amount, current_amount, deadline, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                target_amount = EXCLUDED.target_amount,
                current_amount = EXCLUDED.current_amount,
                deadline = EXCLUDED.deadline,
                updated_at = EXCLUDED.updated_at
            "#,
            goal_db_model.id,
            goal_db_model.user_id,
            goal_db_model.name,
            goal_db_model.target_amount,
            goal_db_model.current_amount,
            goal_db_model.deadline,
            goal_db_model.created_at,
            goal_db_model.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        Ok(())
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<SavingsGoal>, FinanceError> {
        // For now, we'll assume a default currency (USD) for the user
        // In a real implementation, this would be retrieved from user preferences
        let currency = Currency::USD;
        
        let goal_records = sqlx::query_as!(
            SavingsGoalDbModel,
            r#"
            SELECT id, user_id, name, target_amount, current_amount, deadline, created_at, updated_at
            FROM savings_goals
            WHERE user_id = $1
            ORDER BY deadline
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        let goals: Vec<SavingsGoal> = goal_records
            .into_iter()
            .map(|record| record.to_domain(currency.clone()))
            .collect();

        Ok(goals)

        Ok(goals)
    }

    async fn find_active_by_user_id(&self, user_id: Uuid) -> Result<Vec<SavingsGoal>, FinanceError> {
        // For now, we'll assume a default currency (USD) for the user
        // In a real implementation, this would be retrieved from user preferences
        let currency = Currency::USD;
        
        let goal_records = sqlx::query_as!(
            SavingsGoalDbModel,
            r#"
            SELECT id, user_id, name, target_amount, current_amount, deadline, created_at, updated_at
            FROM savings_goals
            WHERE user_id = $1
            ORDER BY deadline
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        // Filter for active (incomplete) goals in memory since we don't have an is_completed field anymore
        let goals: Vec<SavingsGoal> = goal_records
            .into_iter()
            .map(|record| record.to_domain(currency.clone()))
            .filter(|goal| !goal.is_complete())
            .collect();

        Ok(goals)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<SavingsGoal>, FinanceError> {
        // For now, we'll assume a default currency (USD) for the user
        // In a real implementation, this would be retrieved from user preferences
        let currency = Currency::USD;
        
        let goal_record = sqlx::query_as!(
            SavingsGoalDbModel,
            r#"
            SELECT id, user_id, name, target_amount, current_amount, deadline, created_at, updated_at
            FROM savings_goals
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        let goal = goal_record.map(|record| record.to_domain(currency));

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