use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::{SkillProgress};
use tracing::instrument;

#[async_trait]
pub trait SkillProgressRepository {
    async fn save(&self, progress: &SkillProgress) -> Result<(), sqlx::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<SkillProgress>, sqlx::Error>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<SkillProgress>, sqlx::Error>;
    async fn find_by_skill_id(&self, skill_id: Uuid) -> Result<Vec<SkillProgress>, sqlx::Error>;
    async fn find_by_user_and_skill(
        &self,
        user_id: Uuid,
        skill_id: Uuid,
    ) -> Result<Option<SkillProgress>, sqlx::Error>;
    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error>;
}

pub struct PostgresSkillProgressRepository {
    pool: PgPool,
}

impl PostgresSkillProgressRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SkillProgressRepository for PostgresSkillProgressRepository {
    #[instrument(skip(self))]
    async fn save(&self, progress: &SkillProgress) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO skill_progress (id, skill_id, user_id, progress, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                progress = EXCLUDED.progress,
                updated_at = EXCLUDED.updated_at
            "#,
            progress.id,
            progress.skill_id,
            progress.user_id,
            progress.progress as i16,
            progress.updated_at
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    async fn find_by_id(&self, id: Uuid) -> Result<Option<SkillProgress>, sqlx::Error> {
        let progress = sqlx::query_as!(
            SkillProgress,
            r#"
            SELECT id, skill_id, user_id, progress as "progress: u8", updated_at
            FROM skill_progress
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(progress)
    }

    #[instrument(skip(self))]
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<SkillProgress>, sqlx::Error> {
        let progress_list = sqlx::query_as!(
            SkillProgress,
            r#"
            SELECT id, skill_id, user_id, progress as "progress: u8", updated_at
            FROM skill_progress
            WHERE user_id = $1
            ORDER BY updated_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(progress_list)
    }

    #[instrument(skip(self))]
    async fn find_by_skill_id(&self, skill_id: Uuid) -> Result<Vec<SkillProgress>, sqlx::Error> {
        let progress_list = sqlx::query_as!(
            SkillProgress,
            r#"
            SELECT id, skill_id, user_id, progress as "progress: u8", updated_at
            FROM skill_progress
            WHERE skill_id = $1
            ORDER BY updated_at DESC
            "#,
            skill_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(progress_list)
    }

    #[instrument(skip(self))]
    async fn find_by_user_and_skill(
        &self,
        user_id: Uuid,
        skill_id: Uuid,
    ) -> Result<Option<SkillProgress>, sqlx::Error> {
        let progress = sqlx::query_as!(
            SkillProgress,
            r#"
            SELECT id, skill_id, user_id, progress as "progress: u8", updated_at
            FROM skill_progress
            WHERE user_id = $1 AND skill_id = $2
            ORDER BY updated_at DESC
            LIMIT 1
            "#,
            user_id,
            skill_id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(progress)
    }

    #[instrument(skip(self))]
    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM skill_progress
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;
    use std::env;

    async fn setup_test_db() -> PgPool {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://skill_dev_user:secure_password@localhost/skill_dev_test_db".to_string());
        
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to connect to test database")
    }

    #[tokio::test]
    async fn test_save_and_find_skill_progress() {
        let pool = setup_test_db().await;
        let repo = PostgresSkillProgressRepository::new(pool);
        
        // Run migrations
        sqlx::migrate!("../migrations")
            .run(&repo.pool)
            .await
            .expect("Failed to run migrations");

        let skill_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let progress = SkillProgress::new(skill_id, user_id, 75);

        // Save progress
        repo.save(&progress).await.expect("Failed to save progress");

        // Find by ID
        let found = repo.find_by_id(progress.id).await.expect("Failed to find progress");
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.skill_id, skill_id);
        assert_eq!(found.user_id, user_id);
        assert_eq!(found.progress, 75);

        // Find by user ID
        let user_progress = repo.find_by_user_id(user_id).await.expect("Failed to find user progress");
        assert!(!user_progress.is_empty());

        // Find by skill ID
        let skill_progress = repo.find_by_skill_id(skill_id).await.expect("Failed to find skill progress");
        assert!(!skill_progress.is_empty());

        // Find by user and skill
        let user_skill_progress = repo.find_by_user_and_skill(user_id, skill_id).await.expect("Failed to find user skill progress");
        assert!(user_skill_progress.is_some());
    }
}