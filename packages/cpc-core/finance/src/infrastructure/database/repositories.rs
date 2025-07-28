//! Database repositories for the finance module

use uuid::Uuid;
use sqlx::PgPool;
use chrono::Utc;
use crate::{
    application::{
        budget_service::BudgetRepository,
        savings_service::{SavingsRepository, DataSharingRepository},
    },
    domain::{
        budget::Budget,
        savings_goal::SavingsGoal,
        FinanceError,
    },
    infrastructure::database::models::{BudgetDbModel, SavingsGoalDbModel, DataSharingPreferenceDbModel, WalletDbModel, WalletTransactionDbModel},
    application::wallet_service::WalletRepository,
    domain::wallet::{Wallet, WalletTransaction},
};

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
        let budget_db_model = BudgetDbModel::from_domain(budget);
        
        sqlx::query!(
            r#"
            INSERT INTO budgets (id, user_id, category, allocated_amount, spent_amount, dabloons_allocated, dabloons_spent, currency_type, period_start, period_end, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT (id) DO UPDATE SET
                allocated_amount = EXCLUDED.allocated_amount,
                spent_amount = EXCLUDED.spent_amount,
                dabloons_allocated = EXCLUDED.dabloons_allocated,
                dabloons_spent = EXCLUDED.dabloons_spent,
                currency_type = EXCLUDED.currency_type,
                updated_at = EXCLUDED.updated_at
            "#,
            budget_db_model.id,
            budget_db_model.user_id,
            budget_db_model.category,
            budget_db_model.allocated_amount,
            budget_db_model.spent_amount,
            budget_db_model.dabloons_allocated,
            budget_db_model.dabloons_spent,
            budget_db_model.currency_type,
            budget_db_model.period_start,
            budget_db_model.period_end,
            budget_db_model.created_at,
            budget_db_model.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Budget>, FinanceError> {
        // For now, we'll assume a default currency (USD) for the user
        // In a real implementation, this would be retrieved from user preferences
        let currency = crate::domain::primitives::Currency::USD;
        
        let budget_records = sqlx::query_as!(
            BudgetDbModel,
            r#"
            SELECT id, user_id, category, allocated_amount, spent_amount, dabloons_allocated, dabloons_spent, currency_type, period_start, period_end, created_at, updated_at
            FROM budgets
            WHERE user_id = $1
            ORDER BY category
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        let budgets: Vec<Budget> = budget_records
            .into_iter()
            .map(|record| record.to_domain(currency.clone()))
            .collect();

        Ok(budgets)
    }

    async fn find_by_user_and_category(&self, user_id: Uuid, category: &str) -> Result<Option<Budget>, FinanceError> {
        // For now, we'll assume a default currency (USD) for the user
        // In a real implementation, this would be retrieved from user preferences
        let currency = crate::domain::primitives::Currency::USD;
        
        let budget_record = sqlx::query_as!(
            BudgetDbModel,
            r#"
            SELECT id, user_id, category, allocated_amount, spent_amount, dabloons_allocated, dabloons_spent, currency_type, period_start, period_end, created_at, updated_at
            FROM budgets
            WHERE user_id = $1 AND category = $2
            "#,
            user_id,
            category
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        let budget = budget_record.map(|record| record.to_domain(currency));

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
            INSERT INTO savings_goals (id, user_id, name, target_amount, current_amount, target_dabloons, current_dabloons, currency_type, deadline, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                target_amount = EXCLUDED.target_amount,
                current_amount = EXCLUDED.current_amount,
                target_dabloons = EXCLUDED.target_dabloons,
                current_dabloons = EXCLUDED.current_dabloons,
                currency_type = EXCLUDED.currency_type,
                deadline = EXCLUDED.deadline,
                updated_at = EXCLUDED.updated_at
            "#,
            goal_db_model.id,
            goal_db_model.user_id,
            goal_db_model.name,
            goal_db_model.target_amount,
            goal_db_model.current_amount,
            goal_db_model.target_dabloons,
            goal_db_model.current_dabloons,
            goal_db_model.currency_type,
            goal_db_model.deadline,
            goal_db_model.created_at,
            goal_db_model.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<SavingsGoal>, FinanceError> {
        // For now, we'll assume a default currency (USD) for the user
        // In a real implementation, this would be retrieved from user preferences
        let currency = crate::domain::primitives::Currency::USD;
        
        let goal_records = sqlx::query_as!(
            SavingsGoalDbModel,
            r#"
            SELECT id, user_id, name, target_amount, current_amount, target_dabloons, current_dabloons, currency_type, deadline, created_at, updated_at
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
    }

    async fn find_active_by_user_id(&self, user_id: Uuid) -> Result<Vec<SavingsGoal>, FinanceError> {
        // For now, we'll assume a default currency (USD) for the user
        // In a real implementation, this would be retrieved from user preferences
        let currency = crate::domain::primitives::Currency::USD;
        
        let goal_records = sqlx::query_as!(
            SavingsGoalDbModel,
            r#"
            SELECT id, user_id, name, target_amount, current_amount, target_dabloons, current_dabloons, currency_type, deadline, created_at, updated_at
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
        let currency = crate::domain::primitives::Currency::USD;
        
        let goal_record = sqlx::query_as!(
            SavingsGoalDbModel,
            r#"
            SELECT id, user_id, name, target_amount, current_amount, target_dabloons, current_dabloons, currency_type, deadline, created_at, updated_at
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

/// PostgreSQL implementation of DataSharingRepository
pub struct PostgresDataSharingRepository {
    pool: PgPool,
}

impl PostgresDataSharingRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl DataSharingRepository for PostgresDataSharingRepository {
    async fn save(&self, preference: &crate::application::savings_service::DataSharingPreference) -> Result<(), FinanceError> {
        let preference_db_model = DataSharingPreferenceDbModel::from_domain(preference);
        
        sqlx::query!(
            r#"
            INSERT INTO data_sharing_preferences (id, user_id, data_sharing_enabled, anonymized_data, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (user_id) DO UPDATE SET
                data_sharing_enabled = EXCLUDED.data_sharing_enabled,
                anonymized_data = EXCLUDED.anonymized_data,
                updated_at = EXCLUDED.updated_at
            "#,
            preference_db_model.id,
            preference_db_model.user_id,
            preference_db_model.data_sharing_enabled,
            preference_db_model.anonymized_data,
            preference_db_model.created_at,
            preference_db_model.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<crate::application::savings_service::DataSharingPreference>, FinanceError> {
        let preference_record = sqlx::query_as!(
            DataSharingPreferenceDbModel,
            r#"
            SELECT id, user_id, data_sharing_enabled, anonymized_data, created_at, updated_at
            FROM data_sharing_preferences
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        let preference = preference_record.map(|record| record.to_domain());

        Ok(preference)
    }

    async fn create_default(&self, user_id: Uuid) -> Result<crate::application::savings_service::DataSharingPreference, FinanceError> {
        let preference = crate::application::savings_service::DataSharingPreference::new(user_id);
        self.save(&preference).await?;
        Ok(preference)
    }
}

/// PostgreSQL implementation of WalletRepository
pub struct PostgresWalletRepository {
    pool: PgPool,
}

impl PostgresWalletRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl WalletRepository for PostgresWalletRepository {
    async fn save_wallet(&self, wallet: &Wallet) -> Result<(), FinanceError> {
        let wallet_db_model = WalletDbModel::from_domain(wallet);
        
        sqlx::query!(
            r#"
            INSERT INTO wallets (id, user_id, balance, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                balance = EXCLUDED.balance,
                updated_at = EXCLUDED.updated_at
            "#,
            wallet_db_model.id,
            wallet_db_model.user_id,
            wallet_db_model.balance,
            wallet_db_model.created_at,
            wallet_db_model.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_wallet_by_user_id(&self, user_id: Uuid) -> Result<Option<Wallet>, FinanceError> {
        let wallet_record = sqlx::query_as!(
            WalletDbModel,
            r#"
            SELECT id, user_id, balance, created_at, updated_at
            FROM wallets
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        let wallet = wallet_record.map(|record| record.to_domain());

        Ok(wallet)
    }

    async fn save_transaction(&self, transaction: &WalletTransaction) -> Result<(), FinanceError> {
        let transaction_db_model = WalletTransactionDbModel::from_domain(transaction);
        
        sqlx::query!(
            r#"
            INSERT INTO wallet_transactions (id, wallet_id, transaction_type, amount, description, timestamp)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            transaction_db_model.id,
            transaction_db_model.wallet_id,
            transaction_db_model.transaction_type,
            transaction_db_model.amount,
            transaction_db_model.description,
            transaction_db_model.timestamp
        )
        .execute(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_transactions_by_wallet_id(&self, wallet_id: Uuid) -> Result<Vec<WalletTransaction>, FinanceError> {
        let transaction_records = sqlx::query_as!(
            WalletTransactionDbModel,
            r#"
            SELECT id, wallet_id, transaction_type, amount, description, timestamp
            FROM wallet_transactions
            WHERE wallet_id = $1
            ORDER BY timestamp DESC
            "#,
            wallet_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        let transactions: Vec<WalletTransaction> = transaction_records
            .into_iter()
            .map(|record| record.to_domain())
            .collect();

        Ok(transactions)
    }
}