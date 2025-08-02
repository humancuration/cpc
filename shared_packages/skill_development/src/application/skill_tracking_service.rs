use crate::domain::{SkillProgress, SkillProgressRepository, PostgresSkillProgressRepository};
use crate::infrastructure::repositories::postgres_repo::PostgresRepository;
use sqlx::PgPool;
use uuid::Uuid;
use tracing::instrument;

pub struct SkillTrackingService {
    skill_progress_repo: PostgresSkillProgressRepository,
    skill_repo: PostgresRepository,
}

impl SkillTrackingService {
    pub fn new(pool: PgPool) -> Self {
        let skill_progress_repo = PostgresSkillProgressRepository::new(pool.clone());
        let skill_repo = PostgresRepository::new(pool);
        Self {
            skill_progress_repo,
            skill_repo,
        }
    }

    #[instrument(skip(self))]
    pub async fn track_skill_progress(
        &self,
        skill_id: Uuid,
        user_id: Uuid,
        progress: u8,
    ) -> Result<SkillProgress, Box<dyn std::error::Error>> {
        // Validate that the skill exists
        let skill = self.skill_repo.get_skill(skill_id).await?;
        if skill.is_none() {
            return Err("Skill not found".into());
        }

        // Create or update skill progress
        let mut skill_progress = match self.skill_progress_repo
            .find_by_user_and_skill(user_id, skill_id).await? {
            Some(mut existing) => {
                existing.update_progress(progress);
                existing
            }
            None => SkillProgress::new(skill_id, user_id, progress),
        };

        self.skill_progress_repo.save(&skill_progress).await?;
        Ok(skill_progress)
    }

    #[instrument(skip(self))]
    pub async fn get_user_skill_progress(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<SkillProgress>, Box<dyn std::error::Error>> {
        let progress = self.skill_progress_repo.find_by_user_id(user_id).await?;
        Ok(progress)
    }

    #[instrument(skip(self))]
    pub async fn get_skill_progress_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<SkillProgress>, Box<dyn std::error::Error>> {
        let progress = self.skill_progress_repo.find_by_id(id).await?;
        Ok(progress)
    }

    #[instrument(skip(self))]
    pub async fn get_user_progress_for_skill(
        &self,
        user_id: Uuid,
        skill_id: Uuid,
    ) -> Result<Option<SkillProgress>, Box<dyn std::error::Error>> {
        let progress = self.skill_progress_repo.find_by_user_and_skill(user_id, skill_id).await?;
        Ok(progress)
    }

    #[instrument(skip(self))]
    pub async fn delete_skill_progress(
        &self,
        id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.skill_progress_repo.delete(id).await?;
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
    async fn test_track_skill_progress() {
        let pool = setup_test_db().await;
        let service = SkillTrackingService::new(pool);
        
        // Run migrations
        sqlx::migrate!("../../migrations")
            .run(&service.skill_progress_repo.pool)
            .await
            .expect("Failed to run migrations");

        // First, add a skill
        let skill = service.skill_repo.add_skill(
            "Rust Programming".to_string(),
            Some("Master Rust programming language".to_string())
        ).await.expect("Failed to add skill");

        let user_id = Uuid::new_v4();
        
        // Track progress
        let progress = service.track_skill_progress(skill.id, user_id, 75)
            .await
            .expect("Failed to track skill progress");

        assert_eq!(progress.skill_id, skill.id);
        assert_eq!(progress.user_id, user_id);
        assert_eq!(progress.progress, 75);

        // Update progress
        let updated_progress = service.track_skill_progress(skill.id, user_id, 85)
            .await
            .expect("Failed to update skill progress");

        assert_eq!(updated_progress.progress, 85);
        assert_eq!(updated_progress.id, progress.id);
    }

    #[tokio::test]
    async fn test_get_user_skill_progress() {
        let pool = setup_test_db().await;
        let service = SkillTrackingService::new(pool);
        
        // Run migrations
        sqlx::migrate!("../../migrations")
            .run(&service.skill_progress_repo.pool)
            .await
            .expect("Failed to run migrations");

        // First, add a skill
        let skill = service.skill_repo.add_skill(
            "Rust Programming".to_string(),
            Some("Master Rust programming language".to_string())
        ).await.expect("Failed to add skill");

        let user_id = Uuid::new_v4();
        
        // Track progress
        service.track_skill_progress(skill.id, user_id, 75)
            .await
            .expect("Failed to track skill progress");

        // Get user progress
        let progress_list = service.get_user_skill_progress(user_id)
            .await
            .expect("Failed to get user skill progress");

        assert_eq!(progress_list.len(), 1);
        assert_eq!(progress_list[0].skill_id, skill.id);
        assert_eq!(progress_list[0].progress, 75);
    }
}