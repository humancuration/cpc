//! PostgreSQL implementation of the `UserSkillRepository`.

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::user_skill_management::{
    models::{UserSkill, UserSkillDetails},
    repository::{UserSkillRepository, UserSkillRepositoryError},
};

/// A PostgreSQL-backed implementation of the `UserSkillRepository`.
#[derive(Clone)]
pub struct PostgresUserSkillRepository {
    pool: PgPool,
}

impl PostgresUserSkillRepository {
    /// Creates a new `PostgresUserSkillRepository`.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserSkillRepository for PostgresUserSkillRepository {
    async fn add(&self, user_skill: &UserSkill) -> Result<(), UserSkillRepositoryError> {
        sqlx::query!(
            r#"
            INSERT INTO user_skills (user_id, skill_id, skill_level, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            user_skill.user_id,
            user_skill.skill_id,
            user_skill.skill_level as _,
            user_skill.created_at,
            user_skill.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn list_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<UserSkillDetails>, UserSkillRepositoryError> {
        let details = sqlx::query_as!(
            UserSkillDetails,
            r#"
            SELECT
                us.user_id,
                us.skill_level as "skill_level!: _",
                us.created_at,
                s.id as "skill_id!",
                s.name as "skill_name!",
                s.category as "skill_category!",
                s.description as "skill_description!",
                s.created_at as "skill_created_at!",
                s.updated_at as "skill_updated_at!"
            FROM user_skills us
            JOIN skills s ON us.skill_id = s.id
            WHERE us.user_id = $1
            ORDER BY s.name
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(details)
    }

    async fn remove(&self, user_id: Uuid, skill_id: Uuid) -> Result<(), UserSkillRepositoryError> {
        let result = sqlx::query!(
            "DELETE FROM user_skills WHERE user_id = $1 AND skill_id = $2",
            user_id,
            skill_id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(UserSkillRepositoryError::NotFound);
        }

        Ok(())
    }
}