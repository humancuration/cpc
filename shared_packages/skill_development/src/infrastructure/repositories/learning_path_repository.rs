use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::{LearningPath, Milestone};
use tracing::instrument;

#[async_trait]
pub trait LearningPathRepository {
    async fn save(&self, path: &LearningPath) -> Result<(), sqlx::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<LearningPath>, sqlx::Error>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<LearningPath>, sqlx::Error>;
    async fn find_by_skill_id(&self, skill_id: Uuid) -> Result<Vec<LearningPath>, sqlx::Error>;
    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error>;
    async fn save_milestone(&self, milestone: &Milestone, learning_path_id: Uuid) -> Result<(), sqlx::Error>;
    async fn find_milestones_by_learning_path(&self, learning_path_id: Uuid) -> Result<Vec<Milestone>, sqlx::Error>;
    async fn complete_milestone(&self, milestone_id: Uuid) -> Result<(), sqlx::Error>;
}

pub struct PostgresLearningPathRepository {
    pool: PgPool,
}

impl PostgresLearningPathRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LearningPathRepository for PostgresLearningPathRepository {
    #[instrument(skip(self))]
    async fn save(&self, path: &LearningPath) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO learning_paths (id, user_id, skill_id, title, description, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                title = EXCLUDED.title,
                description = EXCLUDED.description,
                updated_at = EXCLUDED.updated_at
            "#,
            path.id,
            path.user_id,
            path.skill_id,
            path.title,
            path.description,
            path.created_at,
            path.updated_at
        )
        .execute(&self.pool)
        .await?;

        // Save milestones
        for milestone in &path.milestones {
            self.save_milestone(milestone, path.id).await?;
        }

        Ok(())
    }

    #[instrument(skip(self))]
    async fn find_by_id(&self, id: Uuid) -> Result<Option<LearningPath>, sqlx::Error> {
        let path = sqlx::query!(
            r#"
            SELECT id, user_id, skill_id, title, description, created_at, updated_at
            FROM learning_paths
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        match path {
            Some(p) => {
                let milestones = self.find_milestones_by_learning_path(id).await?;
                Ok(Some(LearningPath {
                    id: p.id,
                    user_id: p.user_id,
                    skill_id: p.skill_id,
                    title: p.title,
                    description: p.description,
                    milestones,
                    created_at: p.created_at,
                    updated_at: p.updated_at,
                }))
            }
            None => Ok(None),
        }
    }

    #[instrument(skip(self))]
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<LearningPath>, sqlx::Error> {
        let paths = sqlx::query!(
            r#"
            SELECT id, user_id, skill_id, title, description, created_at, updated_at
            FROM learning_paths
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut result = Vec::new();
        for p in paths {
            let milestones = self.find_milestones_by_learning_path(p.id).await?;
            result.push(LearningPath {
                id: p.id,
                user_id: p.user_id,
                skill_id: p.skill_id,
                title: p.title,
                description: p.description,
                milestones,
                created_at: p.created_at,
                updated_at: p.updated_at,
            });
        }

        Ok(result)
    }

    #[instrument(skip(self))]
    async fn find_by_skill_id(&self, skill_id: Uuid) -> Result<Vec<LearningPath>, sqlx::Error> {
        let paths = sqlx::query!(
            r#"
            SELECT id, user_id, skill_id, title, description, created_at, updated_at
            FROM learning_paths
            WHERE skill_id = $1
            ORDER BY created_at DESC
            "#,
            skill_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut result = Vec::new();
        for p in paths {
            let milestones = self.find_milestones_by_learning_path(p.id).await?;
            result.push(LearningPath {
                id: p.id,
                user_id: p.user_id,
                skill_id: p.skill_id,
                title: p.title,
                description: p.description,
                milestones,
                created_at: p.created_at,
                updated_at: p.updated_at,
            });
        }

        Ok(result)
    }

    #[instrument(skip(self))]
    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM learning_paths
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    async fn save_milestone(&self, milestone: &Milestone, learning_path_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO milestones (id, learning_path_id, title, description, is_completed, order_index, estimated_duration_hours)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                title = EXCLUDED.title,
                description = EXCLUDED.description,
                is_completed = EXCLUDED.is_completed,
                order_index = EXCLUDED.order_index,
                estimated_duration_hours = EXCLUDED.estimated_duration_hours
            "#,
            milestone.id,
            learning_path_id,
            milestone.title,
            milestone.description,
            milestone.is_completed,
            milestone.order_index,
            milestone.estimated_duration_hours
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    async fn find_milestones_by_learning_path(&self, learning_path_id: Uuid) -> Result<Vec<Milestone>, sqlx::Error> {
        let milestones = sqlx::query!(
            r#"
            SELECT id, learning_path_id, title, description, is_completed, order_index, estimated_duration_hours
            FROM milestones
            WHERE learning_path_id = $1
            ORDER BY order_index
            "#,
            learning_path_id
        )
        .fetch_all(&self.pool)
        .await?;

        let result = milestones.into_iter().map(|m| Milestone {
            id: m.id,
            title: m.title,
            description: m.description,
            is_completed: m.is_completed.unwrap_or(false),
            order_index: m.order_index,
            estimated_duration_hours: m.estimated_duration_hours,
        }).collect();

        Ok(result)
    }

    #[instrument(skip(self))]
    async fn complete_milestone(&self, milestone_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE milestones
            SET is_completed = true
            WHERE id = $1
            "#,
            milestone_id
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
    use chrono::Utc;

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
    async fn test_save_and_find_learning_path() {
        let pool = setup_test_db().await;
        let repo = PostgresLearningPathRepository::new(pool);
        
        // Run migrations
        sqlx::migrate!("../migrations")
            .run(&repo.pool)
            .await
            .expect("Failed to run migrations");

        let user_id = Uuid::new_v4();
        let skill_id = Uuid::new_v4();
        let mut path = LearningPath::new(
            user_id,
            skill_id,
            "Rust Programming".to_string(),
            Some("Learn Rust from basics to advanced".to_string()),
        );

        let milestone = Milestone::new(
            "Variables and Data Types".to_string(),
            Some("Learn about variables and data types in Rust".to_string()),
            1,
            Some(2),
        );

        path.add_milestone(milestone);

        // Save learning path
        repo.save(&path).await.expect("Failed to save learning path");

        // Find by ID
        let found = repo.find_by_id(path.id).await.expect("Failed to find learning path");
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.user_id, user_id);
        assert_eq!(found.skill_id, skill_id);
        assert_eq!(found.title, "Rust Programming");
        assert_eq!(found.milestones.len(), 1);

        // Find by user ID
        let user_paths = repo.find_by_user_id(user_id).await.expect("Failed to find user paths");
        assert!(!user_paths.is_empty());

        // Find by skill ID
        let skill_paths = repo.find_by_skill_id(skill_id).await.expect("Failed to find skill paths");
        assert!(!skill_paths.is_empty());
    }

    #[tokio::test]
    async fn test_complete_milestone() {
        let pool = setup_test_db().await;
        let repo = PostgresLearningPathRepository::new(pool);
        
        // Run migrations
        sqlx::migrate!("../migrations")
            .run(&repo.pool)
            .await
            .expect("Failed to run migrations");

        let user_id = Uuid::new_v4();
        let skill_id = Uuid::new_v4();
        let mut path = LearningPath::new(
            user_id,
            skill_id,
            "Rust Programming".to_string(),
            None,
        );

        let milestone = Milestone::new(
            "Variables and Data Types".to_string(),
            None,
            1,
            Some(2),
        );

        let milestone_id = milestone.id;
        path.add_milestone(milestone);

        // Save learning path
        repo.save(&path).await.expect("Failed to save learning path");

        // Complete milestone
        repo.complete_milestone(milestone_id).await.expect("Failed to complete milestone");

        // Verify milestone is completed
        let found = repo.find_by_id(path.id).await.expect("Failed to find learning path");
        assert!(found.is_some());
        let found = found.unwrap();
        assert!(found.milestones[0].is_completed);
    }
}