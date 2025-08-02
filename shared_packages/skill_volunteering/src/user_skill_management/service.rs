//! Service layer for user skill management.

use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

use super::{
    models::{SkillLevel, UserSkill, UserSkillDetails},
    repository::{UserSkillRepository, UserSkillRepositoryError},
};

#[derive(Error, Debug)]
pub enum UserSkillServiceError {
    #[error("User skill not found")]
    NotFound,
    #[error("User skill combination already exists")]
    AlreadyExists,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("An internal error occurred: {0}")]
    Internal(String),
}

impl From<UserSkillRepositoryError> for UserSkillServiceError {
    fn from(err: UserSkillRepositoryError) -> Self {
        match err {
            UserSkillRepositoryError::NotFound => UserSkillServiceError::NotFound,
            UserSkillRepositoryError::Duplicate => UserSkillServiceError::AlreadyExists,
            UserSkillRepositoryError::DatabaseError(e) => UserSkillServiceError::Internal(e),
            UserSkillRepositoryError::Unexpected => {
                UserSkillServiceError::Internal("An unexpected error occurred".to_string())
            }
        }
    }
}

/// The service for managing user skills.
pub struct UserSkillService {
    repo: Arc<dyn UserSkillRepository>,
}

impl UserSkillService {
    /// Creates a new `UserSkillService`.
    pub fn new(repo: Arc<dyn UserSkillRepository>) -> Self {
        Self { repo }
    }

    /// Adds a skill to a user's profile.
    pub async fn add_user_skill(
        &self,
        user_id: Uuid,
        skill_id: Uuid,
        skill_level_str: &str,
    ) -> Result<UserSkill, UserSkillServiceError> {
        let skill_level = SkillLevel::from_str(skill_level_str).map_err(|_| {
            UserSkillServiceError::InvalidInput(format!("Invalid skill level: {}", skill_level_str))
        })?;

        let user_skill = UserSkill::new(user_id, skill_id, skill_level);

        self.repo.add(&user_skill).await?;

        Ok(user_skill)
    }

    /// Lists all skills for a given user.
    pub async fn list_user_skills(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<UserSkillDetails>, UserSkillServiceError> {
        self.repo.list_by_user_id(user_id).await.map_err(Into::into)
    }

    /// Removes a skill from a user's profile.
    pub async fn remove_user_skill(
        &self,
        user_id: Uuid,
        skill_id: Uuid,
    ) -> Result<(), UserSkillServiceError> {
        self.repo.remove(user_id, skill_id).await.map_err(Into::into)
    }
}