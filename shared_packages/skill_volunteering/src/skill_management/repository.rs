//! Repository for skill management.

use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

use super::models::Skill;

/// Represents the paginated result of listing skills.
#[derive(Debug, PartialEq, Eq)]
pub struct PaginatedSkills {
    pub skills: Vec<Skill>,
    pub total_count: i64,
}

#[derive(Error, Debug)]
pub enum SkillRepositoryError {
    #[error("Skill not found")]
    NotFound,
    #[error("A database error occurred: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("An unexpected error occurred")]
    Unexpected,
}

/// A trait for abstracting the storage and retrieval of skills.
#[async_trait]
pub trait SkillRepository: Send + Sync {
    /// Creates a new skill in the repository.
    async fn create_skill(&self, skill: &Skill) -> Result<(), SkillRepositoryError>;

    /// Finds a skill by its ID.
    async fn find_skill_by_id(&self, id: Uuid) -> Result<Option<Skill>, SkillRepositoryError>;

    /// Updates an existing skill.
    async fn update_skill(&self, skill: &Skill) -> Result<(), SkillRepositoryError>;

    /// Deletes a skill by its ID.
    async fn delete_skill(&self, id: Uuid) -> Result<(), SkillRepositoryError>;

    /// Lists skills with pagination and filtering.
    async fn list_skills(
        &self,
        category: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<PaginatedSkills, SkillRepositoryError>;
}