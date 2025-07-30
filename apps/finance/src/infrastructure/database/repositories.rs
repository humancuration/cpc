//! Database repositories for the finance module

use uuid::Uuid;
use sqlx::PgPool;
use chrono::Utc;
use tracing::{info, warn};
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
    infrastructure::database::models::{BudgetDbModel, SavingsGoalDbModel, DataSharingPreferenceDbModel, WalletDbModel, WalletTransactionDbModel, UIConfigDbModel, UIDistributionDbModel},
    application::wallet_service::WalletRepository,
    domain::wallet::{Wallet, WalletTransaction},
    application::rewards_service::{UIConfigRepository, UIDistributionRepository},
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
async fn get_user_currency(&self, user_id: Uuid) -> Result<crate::domain::primitives::Currency, FinanceError> {
    let currency_code: Option<String> = sqlx::query_scalar!(
        r#"
        SELECT preferred_currency
        FROM data_sharing_preferences
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_optional(&self.pool)
    .await
    .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

    // Default to USD if not found
    let code = currency_code.unwrap_or_else(|| "USD".to_string());
    
    // Validate currency code and convert to Currency enum
    let currency = match code.as_str() {
        "USD" => crate::domain::primitives::Currency::USD,
        "EUR" => crate::domain::primitives::Currency::EUR,
        "GBP" => crate::domain::primitives::Currency::GBP,
        "JPY" => crate::domain::primitives::Currency::JPY,
        "CAD" => crate::domain::primitives::Currency::CAD,
        "AUD" => crate::domain::primitives::Currency::AUD,
        "CHF" => crate::domain::primitives::Currency::CHF,
        "CNY" => crate::domain::primitives::Currency::CNY,
        "SEK" => crate::domain::primitives::Currency::SEK,
        "NZD" => crate::domain::primitives::Currency::NZD,
        "MXN" => crate::domain::primitives::Currency::MXN,
        "SGD" => crate::domain::primitives::Currency::SGD,
        "HKD" => crate::domain::primitives::Currency::HKD,
        "NOK" => crate::domain::primitives::Currency::NOK,
        "KRW" => crate::domain::primitives::Currency::KRW,
        "TRY" => crate::domain::primitives::Currency::TRY,
        "RUB" => crate::domain::primitives::Currency::RUB,
        "INR" => crate::domain::primitives::Currency::INR,
        "BRL" => crate::domain::primitives::Currency::BRL,
        "ZAR" => crate::domain::primitives::Currency::ZAR,
        "DABLOONS" => crate::domain::primitives::Currency::Dabloons,
        _ => {
            // Log warning for invalid currency code and default to USD
            warn!("Invalid currency code '{}' for user {}, defaulting to USD", code, user_id);
            crate::domain::primitives::Currency::USD
        }
    };
    
    info!("Retrieved currency {:?} for user {}", currency, user_id);
    Ok(currency)
}
    }

    async fn update_user_currency(&self, user_id: Uuid, currency: crate::domain::primitives::Currency) -> Result<(), FinanceError> {
        info!("Updating currency for user {} to {:?}", user_id, currency);
        
        sqlx::query!(
            r#"
            UPDATE data_sharing_preferences
            SET preferred_currency = $1, updated_at = NOW()
            WHERE user_id = $2
            "#,
            currency.code(),
            user_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        info!("Successfully updated currency for user {} to {:?}", user_id, currency);
        Ok(())
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

/// PostgreSQL implementation of UIConfigRepository
pub struct PostgresUIConfigRepository {
    pool: PgPool,
}

impl PostgresUIConfigRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UIConfigRepository for PostgresUIConfigRepository {
    async fn get_config(&self) -> Result<Option<crate::domain::rewards::UniversalIncomeConfig>, FinanceError> {
        let config_record = sqlx::query_as!(
            UIConfigDbModel,
            r#"
            SELECT id, daily_amount, start_date, is_active, created_at, updated_at
            FROM ui_config
            ORDER BY created_at DESC
            LIMIT 1
            "#
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        let config = config_record.map(|record| record.to_domain());

        Ok(config)
    }

    async fn save_config(&self, config: &crate::domain::rewards::UniversalIncomeConfig) -> Result<(), FinanceError> {
        let config_db_model = UIConfigDbModel::from_domain(config);
        
        sqlx::query!(
            r#"
            INSERT INTO ui_config (id, daily_amount, start_date, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id) DO UPDATE SET
                daily_amount = EXCLUDED.daily_amount,
                start_date = EXCLUDED.start_date,
                is_active = EXCLUDED.is_active,
                updated_at = EXCLUDED.updated_at
            "#,
            Uuid::new_v4(), // Generate new ID for inserts
            config_db_model.daily_amount,
            config_db_model.start_date,
            config_db_model.is_active,
            config_db_model.created_at,
            config_db_model.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

/// PostgreSQL implementation of UIDistributionRepository
pub struct PostgresUIDistributionRepository {
    pool: PgPool,
}

impl PostgresUIDistributionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UIDistributionRepository for PostgresUIDistributionRepository {
    async fn save_distribution(&self, distribution: &crate::domain::rewards::UIDistribution) -> Result<(), FinanceError> {
        let distribution_db_model = UIDistributionDbModel::from_domain(distribution);
        
        sqlx::query!(
            r#"
            INSERT INTO ui_distributions (id, user_id, amount, distribution_date, created_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            distribution_db_model.id,
            distribution_db_model.user_id,
            distribution_db_model.amount,
            distribution_db_model.distribution_date,
            distribution_db_model.created_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn has_received(&self, user_id: Uuid, date: chrono::NaiveDate) -> Result<bool, FinanceError> {
        let count: i64 = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) as "count!"
            FROM ui_distributions
            WHERE user_id = $1 AND distribution_date = $2
            "#,
            user_id,
            date
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        Ok(count > 0)
    }

    async fn get_distributions_for_user(&self, user_id: Uuid) -> Result<Vec<crate::domain::rewards::UIDistribution>, FinanceError> {
        let distribution_records = sqlx::query_as!(
            UIDistributionDbModel,
            r#"
            SELECT id, user_id, amount, distribution_date, created_at
            FROM ui_distributions
            WHERE user_id = $1
            ORDER BY distribution_date DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;

        let distributions: Vec<crate::domain::rewards::UIDistribution> = distribution_records
            .into_iter()
            .map(|record| record.to_domain())
            .collect();

        Ok(distributions)
    }
}