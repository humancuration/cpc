//! PostgreSQL implementation of ExpenseSharingPreferenceRepository

use async_trait::async_trait;
use uuid::Uuid;
use sqlx::PgPool;
use chrono::{DateTime, Utc};
use crate::{
    domain::{
        expense_tracker::ExpenseSharingPreferences,
        expense_tracker::ExpenseCategory,
        FinanceError,
    },
    expense_tracker::application::expense_service::ExpenseSharingPreferenceRepository,
};

/// Database model for expense_sharing_preferences table
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct ExpenseSharingPreferenceDbModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub sharing_enabled: bool,
    pub anonymized: bool,
    pub shared_categories: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ExpenseSharingPreferenceDbModel {
    /// Convert domain ExpenseSharingPreferences to database model
    pub fn from_domain(preference: &ExpenseSharingPreferences) -> Self {
        Self {
            id: preference.id,
            user_id: preference.user_id,
            sharing_enabled: preference.sharing_enabled,
            anonymized: preference.anonymized,
            shared_categories: serde_json::to_value(&preference.shared_categories)
                .unwrap_or(serde_json::Value::Array(vec![])),
            created_at: preference.created_at,
            updated_at: preference.updated_at,
        }
    }
    
    /// Convert database model to domain ExpenseSharingPreferences
    pub fn to_domain(&self) -> ExpenseSharingPreferences {
        let shared_categories: Vec<ExpenseCategory> = serde_json::from_value(self.shared_categories.clone())
            .unwrap_or_else(|_| vec![]);
        
        ExpenseSharingPreferences {
            id: self.id,
            user_id: self.user_id,
            sharing_enabled: self.sharing_enabled,
            anonymized: self.anonymized,
            shared_categories,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

/// PostgreSQL implementation of ExpenseSharingPreferenceRepository
pub struct PostgresExpenseSharingPreferenceRepository {
    pool: PgPool,
}

impl PostgresExpenseSharingPreferenceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ExpenseSharingPreferenceRepository for PostgresExpenseSharingPreferenceRepository {
    async fn save(&self, preference: &ExpenseSharingPreferences) -> Result<(), FinanceError> {
        let preference_db_model = ExpenseSharingPreferenceDbModel::from_domain(preference);
        
        sqlx::query!(
            r#"
            INSERT INTO expense_sharing_preferences (id, user_id, sharing_enabled, anonymized, shared_categories, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (user_id) DO UPDATE SET
                sharing_enabled = EXCLUDED.sharing_enabled,
                anonymized = EXCLUDED.anonymized,
                shared_categories = EXCLUDED.shared_categories,
                updated_at = EXCLUDED.updated_at
            "#,
            preference_db_model.id,
            preference_db_model.user_id,
            preference_db_model.sharing_enabled,
            preference_db_model.anonymized,
            preference_db_model.shared_categories,
            preference_db_model.created_at,
            preference_db_model.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| FinanceError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<ExpenseSharingPreferences>, FinanceError> {
        let preference_record = sqlx::query_as!(
            ExpenseSharingPreferenceDbModel,
            r#"
            SELECT id, user_id, sharing_enabled, anonymized, shared_categories, created_at, updated_at
            FROM expense_sharing_preferences
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
    
    async fn create_default(&self, user_id: Uuid) -> Result<ExpenseSharingPreferences, FinanceError> {
        let preference = ExpenseSharingPreferences::new(user_id);
        self.save(&preference).await?;
        Ok(preference)
    }
}