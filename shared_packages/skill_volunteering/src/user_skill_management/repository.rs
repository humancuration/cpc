//! Repository for user skill management.

use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

use super::models::{UserSkill, UserSkillDetails};

#[derive(Error, Debug, PartialEq)]
pub enum UserSkillRepositoryError {
    #[error("User skill combination already exists")]
    Duplicate,
    #[error("User skill not found")]
    NotFound,
    #[error("A database error occurred: {0}")]
    DatabaseError(String),
    #[error("An unexpected error occurred")]
    Unexpected,
}

impl From<sqlx::Error> for UserSkillRepositoryError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                UserSkillRepositoryError::Duplicate
            }
            sqlx::Error::RowNotFound => UserSkillRepositoryError::NotFound,
            _ => UserSkillRepositoryError::DatabaseError(err.to_string()),
        }
    }
}

/// A trait for abstracting the storage and retrieval of user skills.
#[async_trait]
pub trait UserSkillRepository: Send + Sync {
    /// Adds a new skill to a user's profile.
    async fn add(&self, user_skill: &UserSkill) -> Result<(), UserSkillRepositoryError>;

    /// Lists all skills for a given user, with full skill details.
    async fn list_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<UserSkillDetails>, UserSkillRepositoryError>;

    /// Removes a skill from a user's profile.
    async fn remove(&self, user_id: Uuid, skill_id: Uuid) -> Result<(), UserSkillRepositoryError>;
}