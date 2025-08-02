use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::Tip;
use crate::application::{TipRepository, Result, LearningPlatformError};
use chrono::Utc;

pub struct TipRepositoryImpl {
    pool: PgPool,
}

impl TipRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TipRepository for TipRepositoryImpl {
    async fn create_tip(&self, tip: &Tip) -> Result<Tip> {
        let course_id_str = tip.course_id.as_ref().map(|id| id.to_string());
        
        let result = sqlx::query!(
            "INSERT INTO tips (id, from_user_id, to_user_id, course_id, amount, currency, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id",
            tip.id.to_string(),
            tip.from_user_id.to_string(),
            tip.to_user_id.to_string(),
            course_id_str,
            tip.amount,
            tip.currency,
            tip.created_at
        )
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(_) => Ok(tip.clone()),
            Err(e) => Err(LearningPlatformError::DatabaseError(e.to_string())),
        }
    }
}