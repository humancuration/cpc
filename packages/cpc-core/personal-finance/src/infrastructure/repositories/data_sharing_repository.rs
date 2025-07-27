//! Repository for data sharing preference persistence

use uuid::Uuid;
use sqlx::PgPool;
use chrono::{DateTime, Utc};

use crate::infrastructure::database::models::{DataSharingPreferenceDbModel, DataSharingPreference};
use crate::domain::models::FinanceError;

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
pub trait DataSharingRepository {
    async fn save(&self, preference: &DataSharingPreference) -> Result<(), FinanceError>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<DataSharingPreference>, FinanceError>;
    async fn create_default(&self, user_id: Uuid) -> Result<DataSharingPreference, FinanceError>;
}

#[async_trait::async_trait]
impl DataSharingRepository for PostgresDataSharingRepository {
    async fn save(&self, preference: &DataSharingPreference) -> Result<(), FinanceError> {
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

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<DataSharingPreference>, FinanceError> {
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

    async fn create_default(&self, user_id: Uuid) -> Result<DataSharingPreference, FinanceError> {
        let preference = DataSharingPreference::new(user_id);
        self.save(&preference).await?;
        Ok(preference)
    }
}

/// Mock repository for testing
pub struct MockDataSharingRepository;

#[async_trait::async_trait]
impl DataSharingRepository for MockDataSharingRepository {
    async fn save(&self, _preference: &DataSharingPreference) -> Result<(), FinanceError> {
        Ok(())
    }

    async fn find_by_user_id(&self, _user_id: Uuid) -> Result<Option<DataSharingPreference>, FinanceError> {
        Ok(None)
    }

    async fn create_default(&self, user_id: Uuid) -> Result<DataSharingPreference, FinanceError> {
        Ok(DataSharingPreference::new(user_id))
    }
}