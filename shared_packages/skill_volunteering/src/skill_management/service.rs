//! Service layer for skill management.

use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

use super::{
    use super::{
        models::Skill,
        repository::{PaginatedSkills, SkillRepository, SkillRepositoryError},
    };
    
    #[derive(Error, Debug)]
    pub enum SkillServiceError {
        #[error("Skill not found")]
        NotFound,
        #[error("Invalid input: {0}")]
        InvalidInput(String),
        #[error("An internal error occurred: {0}")]
        Internal(String),
    }
    
    impl From<SkillRepositoryError> for SkillServiceError {
        fn from(err: SkillRepositoryError) -> Self {
            match err {
                SkillRepositoryError::NotFound => SkillServiceError::NotFound,
                SkillRepositoryError::DatabaseError(e) => SkillServiceError::Internal(e.to_string()),
                SkillRepositoryError::Unexpected => {
                    SkillServiceError::Internal("An unexpected error occurred".to_string())
                }
            }
        }
    }
    
    /// The service for managing skills.
    pub struct SkillService {
        repo: Arc<dyn SkillRepository>,
    }
    
    impl SkillService {
        /// Creates a new `SkillService`.
        pub fn new(repo: Arc<dyn SkillRepository>) -> Self {
            Self { repo }
        }
    
        /// Creates a new skill.
        pub async fn create_skill(
            &self,
            name: String,
            category: String,
            description: Option<String>,
        ) -> Result<Skill, SkillServiceError> {
            if name.is_empty() {
                return Err(SkillServiceError::InvalidInput(
                    "Skill name cannot be empty".to_string(),
                ));
            }
            if category.is_empty() {
                return Err(SkillServiceError::InvalidInput(
                    "Skill category cannot be empty".to_string(),
                ));
            }
    
            let skill = Skill::new(name, category, description);
            self.repo.create_skill(&skill).await?;
            Ok(skill)
        }
    
        /// Gets a skill by its ID.
        pub async fn get_skill(&self, id: Uuid) -> Result<Skill, SkillServiceError> {
            self.repo
                .find_skill_by_id(id)
                .await?
                .ok_or(SkillServiceError::NotFound)
        }
    
        /// Lists skills with pagination and filtering.
        pub async fn list_skills(
            &self,
            category: Option<String>,
            limit: Option<i32>,
            offset: Option<i32>,
        ) -> Result<PaginatedSkills, SkillServiceError> {
            self.repo
                .list_skills(category, limit, offset)
                .await
                .map_err(Into::into)
        }
    }