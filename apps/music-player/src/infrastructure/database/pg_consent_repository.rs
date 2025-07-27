use sqlx::{Pool, Postgres};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::application::privacy_service::{ConsentType, ConsentStatus};
use crate::domain::errors::{Result, MusicPlayerError};
use super::consent_repository::ConsentRepository;

pub struct PgConsentRepository {
    pool: Pool<Postgres>,
}

impl PgConsentRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ConsentRepository for PgConsentRepository {
    async fn store_consent(
        &self,
        user_id: Uuid,
        consent_type: ConsentType,
        granted: bool,
        expires_at: Option<DateTime<Utc>>
    ) -> Result<()> {
        sqlx::query(
            "INSERT INTO user_consents (user_id, consent_type, granted, expires_at, updated_at)
             VALUES ($1, $2, $3, $4, NOW())
             ON CONFLICT (user_id, consent_type) 
             DO UPDATE SET granted = $3, expires_at = $4, updated_at = NOW()"
        )
        .bind(user_id)
        .bind(consent_type.to_string())
        .bind(granted)
        .bind(expires_at)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_consent(
        &self,
        user_id: Uuid,
        consent_type: ConsentType
    ) -> Result<Option<ConsentStatus>> {
        let row = sqlx::query!(
            "SELECT granted, expires_at 
             FROM user_consents 
             WHERE user_id = $1 AND consent_type = $2",
            user_id,
            consent_type.to_string()
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(row.map(|r| ConsentStatus {
            granted: r.granted,
            expires_at: r.expires_at,
        }))
    }
    
    async fn get_all_consents(
        &self,
        user_id: Uuid
    ) -> Result<Vec<(ConsentType, ConsentStatus)>> {
        let rows = sqlx::query!(
            "SELECT consent_type, granted, expires_at 
             FROM user_consents 
             WHERE user_id = $1",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut results = Vec::new();
        for row in rows {
            if let Ok(consent_type) = row.consent_type.parse::<ConsentType>() {
                results.push((
                    consent_type,
                    ConsentStatus {
                        granted: row.granted,
                        expires_at: row.expires_at,
                    }
                ));
            }
        }
        
        Ok(results)
    }
    
    async fn revoke_all_consents(
        &self,
        user_id: Uuid
    ) -> Result<()> {
        sqlx::query(
            "UPDATE user_consents 
             SET granted = false, expires_at = NOW()
             WHERE user_id = $1 AND granted = true"
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}