//! PostgreSQL implementation of the `SkillRepository`.

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::skill_management::{
    use crate::skill_management::{
        models::Skill,
        repository::{PaginatedSkills, SkillRepository, SkillRepositoryError},
    };
    
    /// A PostgreSQL-backed implementation of the `SkillRepository`.
    #[derive(Clone)]
    pub struct PostgresSkillRepository {
        pool: PgPool,
    }
    
    impl PostgresSkillRepository {
        /// Creates a new `PostgresSkillRepository`.
        pub fn new(pool: PgPool) -> Self {
            Self { pool }
        }
    }
    
    #[async_trait]
    impl SkillRepository for PostgresSkillRepository {
        async fn create_skill(&self, skill: &Skill) -> Result<(), SkillRepositoryError> {
            sqlx::query!(
                r#"
                INSERT INTO skills (id, name, category, description, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6)
                "#,
                skill.id,
                skill.name,
                skill.category,
                skill.description,
                skill.created_at,
                skill.updated_at
            )
            .execute(&self.pool)
            .await?;
    
            Ok(())
        }
    
        async fn find_skill_by_id(&self, id: Uuid) -> Result<Option<Skill>, SkillRepositoryError> {
            let skill = sqlx::query_as!(
                Skill,
                r#"
                SELECT id, name, category, description, created_at, updated_at
                FROM skills
                WHERE id = $1
                "#,
                id
            )
            .fetch_optional(&self.pool)
            .await?;
    
            Ok(skill)
        }
    
        async fn update_skill(&self, skill: &Skill) -> Result<(), SkillRepositoryError> {
            let rows_affected = sqlx::query!(
                r#"
                UPDATE skills
                SET name = $2, category = $3, description = $4, updated_at = $5
                WHERE id = $1
                "#,
                skill.id,
                skill.name,
                skill.category,
                skill.description,
                skill.updated_at
            )
            .execute(&self.pool)
            .await?
            .rows_affected();
    
            if rows_affected == 0 {
                return Err(SkillRepositoryError::NotFound);
            }
    
            Ok(())
        }
    
        async fn delete_skill(&self, id: Uuid) -> Result<(), SkillRepositoryError> {
            let rows_affected = sqlx::query!("DELETE FROM skills WHERE id = $1", id)
                .execute(&self.pool)
                .await?
                .rows_affected();
    
            if rows_affected == 0 {
                return Err(SkillRepositoryError::NotFound);
            }
    
            Ok(())
        }
    
        async fn list_skills(
            &self,
            category: Option<String>,
            limit: Option<i32>,
            offset: Option<i32>,
        ) -> Result<PaginatedSkills, SkillRepositoryError> {
            let mut query_builder = sqlx::QueryBuilder::new("SELECT * FROM skills WHERE 1=1");
            let mut count_query_builder =
                sqlx::QueryBuilder::new("SELECT COUNT(*) as count FROM skills WHERE 1=1");
    
            if let Some(cat) = &category {
                query_builder.push(" AND category = ");
                query_builder.push_bind(cat.clone());
                count_query_builder.push(" AND category = ");
                count_query_builder.push_bind(cat.clone());
            }
    
            query_builder.push(" ORDER BY name");
    
            if let Some(lim) = limit {
                query_builder.push(" LIMIT ");
                query_builder.push_bind(lim);
            }
    
            if let Some(off) = offset {
                query_builder.push(" OFFSET ");
                query_builder.push_bind(off);
            }
    
            let skills = query_builder
                .build_query_as::<Skill>()
                .fetch_all(&self.pool)
                .await?;
    
            let total_count: (i64,) = count_query_builder
                .build_query_as()
                .fetch_one(&self.pool)
                .await?;
    
            Ok(PaginatedSkills {
                skills,
                total_count: total_count.0,
            })
        }
    }