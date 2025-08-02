use crate::domain::{LearningPath, Milestone, LearningPathRepository, PostgresLearningPathRepository};
use crate::infrastructure::repositories::postgres_repo::PostgresRepository;
use sqlx::PgPool;
use uuid::Uuid;
use tracing::instrument;

pub struct LearningPathService {
    learning_path_repo: PostgresLearningPathRepository,
    skill_repo: PostgresRepository,
}

impl LearningPathService {
    pub fn new(pool: PgPool) -> Self {
        let learning_path_repo = PostgresLearningPathRepository::new(pool.clone());
        let skill_repo = PostgresRepository::new(pool);
        Self {
            learning_path_repo,
            skill_repo,
        }
    }

    #[instrument(skip(self))]
    pub async fn create_learning_path(
        &self,
        user_id: Uuid,
        skill_id: Uuid,
        title: String,
        description: Option<String>,
    ) -> Result<LearningPath, Box<dyn std::error::Error>> {
        // Validate that the skill exists
        let skill = self.skill_repo.get_skill(skill_id).await?;
        if skill.is_none() {
            return Err("Skill not found".into());
        }

        let path = LearningPath::new(user_id, skill_id, title, description);
        self.learning_path_repo.save(&path).await?;
        Ok(path)
    }

    #[instrument(skip(self))]
    pub async fn get_learning_path(
        &self,
        id: Uuid,
    ) -> Result<Option<LearningPath>, Box<dyn std::error::Error>> {
        let path = self.learning_path_repo.find_by_id(id).await?;
        Ok(path)
    }

    #[instrument(skip(self))]
    pub async fn get_user_learning_paths(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<LearningPath>, Box<dyn std::error::Error>> {
        let paths = self.learning_path_repo.find_by_user_id(user_id).await?;
        Ok(paths)
    }

    #[instrument(skip(self))]
    pub async fn get_skill_learning_paths(
        &self,
        skill_id: Uuid,
    ) -> Result<Vec<LearningPath>, Box<dyn std::error::Error>> {
        let paths = self.learning_path_repo.find_by_skill_id(skill_id).await?;
        Ok(paths)
    }

    #[instrument(skip(self))]
    pub async fn add_milestone_to_path(
        &self,
        learning_path_id: Uuid,
        title: String,
        description: Option<String>,
        order_index: i32,
        estimated_duration_hours: Option<i32>,
    ) -> Result<Milestone, Box<dyn std::error::Error>> {
        // Verify the learning path exists
        let path = self.learning_path_repo.find_by_id(learning_path_id).await?;
        if path.is_none() {
            return Err("Learning path not found".into());
        }

        let milestone = Milestone::new(title, description, order_index, estimated_duration_hours);
        self.learning_path_repo.save_milestone(&milestone, learning_path_id).await?;
        Ok(milestone)
    }

    #[instrument(skip(self))]
    pub async fn complete_milestone(
        &self,
        milestone_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.learning_path_repo.complete_milestone(milestone_id).await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn delete_learning_path(
        &self,
        id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.learning_path_repo.delete(id).await?;
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
    async fn test_create_and_get_learning_path() {
        let pool = setup_test_db().await;
        let service = LearningPathService::new(pool);
        
        // Run migrations
        sqlx::migrate!("../../migrations")
            .run(&service.learning_path_repo.pool)
            .await
            .expect("Failed to run migrations");

        // First, add a skill
        let skill = service.skill_repo.add_skill(
            "Rust Programming".to_string(),
            Some("Master Rust programming language".to_string())
        ).await.expect("Failed to add skill");

        let user_id = Uuid::new_v4();
        
        // Create learning path
        let path = service.create_learning_path(
            user_id,
            skill.id,
            "Rust Programming Path".to_string(),
            Some("Learn Rust from basics to advanced".to_string())
        ).await.expect("Failed to create learning path");

        assert_eq!(path.user_id, user_id);
        assert_eq!(path.skill_id, skill.id);
        assert_eq!(path.title, "Rust Programming Path");

        // Get learning path by ID
        let found = service.get_learning_path(path.id)
            .await
            .expect("Failed to get learning path");
        assert!(found.is_some());
        assert_eq!(found.unwrap().title, "Rust Programming Path");

        // Get user learning paths
        let user_paths = service.get_user_learning_paths(user_id)
            .await
            .expect("Failed to get user learning paths");
        assert_eq!(user_paths.len(), 1);

        // Get skill learning paths
        let skill_paths = service.get_skill_learning_paths(skill.id)
            .await
            .expect("Failed to get skill learning paths");
        assert_eq!(skill_paths.len(), 1);
    }

    #[tokio::test]
    async fn test_add_and_complete_milestone() {
        let pool = setup_test_db().await;
        let service = LearningPathService::new(pool);
        
        // Run migrations
        sqlx::migrate!("../../migrations")
            .run(&service.learning_path_repo.pool)
            .await
            .expect("Failed to run migrations");

        // First, add a skill
        let skill = service.skill_repo.add_skill(
            "Rust Programming".to_string(),
            Some("Master Rust programming language".to_string())
        ).await.expect("Failed to add skill");

        let user_id = Uuid::new_v4();
        
        // Create learning path
        let path = service.create_learning_path(
            user_id,
            skill.id,
            "Rust Programming Path".to_string(),
            None
        ).await.expect("Failed to create learning path");

        // Add milestone
        let milestone = service.add_milestone_to_path(
            path.id,
            "Variables and Data Types".to_string(),
            Some("Learn about variables and data types in Rust".to_string()),
            1,
            Some(2)
        ).await.expect("Failed to add milestone");

        // Verify milestone was added
        let updated_path = service.get_learning_path(path.id)
            .await
            .expect("Failed to get updated learning path")
            .unwrap();
        assert_eq!(updated_path.milestones.len(), 1);
        assert_eq!(updated_path.milestones[0].title, "Variables and Data Types");

        // Complete milestone
        service.complete_milestone(milestone.id)
            .await
            .expect("Failed to complete milestone");

        // Verify milestone is completed
        let final_path = service.get_learning_path(path.id)
            .await
            .expect("Failed to get final learning path")
            .unwrap();
        assert!(final_path.milestones[0].is_completed);
    }
}