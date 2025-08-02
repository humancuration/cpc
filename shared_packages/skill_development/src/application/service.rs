use uuid::Uuid;

use crate::domain::*;
use crate::infrastructure::repositories::postgres_repo::PostgresRepository;

pub struct SkillDevelopmentService {
    repository: PostgresRepository,
}

impl SkillDevelopmentService {
    pub fn new(repository: PostgresRepository) -> Self {
        Self { repository }
    }

    pub async fn add_skill(&self, name: String, description: Option<String>) -> Result<Skill, sqlx::Error> {
        let skill = Skill {
            id: Uuid::new_v4(),
            name,
            description,
            created_at: chrono::Utc::now(),
        };
        self.repository.save_skill(&skill).await?;
        Ok(skill)
    }

    pub async fn get_skill(&self, skill_id: Uuid) -> Result<Option<Skill>, sqlx::Error> {
        self.repository.get_skill(skill_id).await
    }

    pub async fn list_skills(&self) -> Result<Vec<Skill>, sqlx::Error> {
        self.repository.get_all_skills().await
    }

    pub async fn update_skill_progress(
        &self,
        skill_id: Uuid,
        user_id: Uuid,
        progress: u8,
    ) -> Result<SkillProgress, sqlx::Error> {
        let progress_record = SkillProgress {
            id: Uuid::new_v4(),
            skill_id,
            user_id,
            progress: progress.min(100),
            updated_at: chrono::Utc::now(),
        };
        self.repository.save_skill_progress(&progress_record).await?;
        Ok(progress_record)
    }

    pub async fn get_skill_progress(&self, progress_id: Uuid) -> Result<SkillProgress, sqlx::Error> {
        self.repository.get_skill_progress(progress_id).await
    }

    pub async fn get_user_skill_progress(&self, user_id: Uuid) -> Result<Vec<SkillWithProgress>, sqlx::Error> {
        self.repository.get_skill_with_progress(user_id).await
    }

    pub async fn add_certification(
        &self,
        name: String,
        issuing_organization: String,
        issue_date: chrono::NaiveDate,
        user_id: Uuid,
    ) -> Result<Certification, sqlx::Error> {
        let certification = Certification {
            id: Uuid::new_v4(),
            name,
            issuing_organization,
            issue_date,
            user_id,
        };
        self.repository.save_certification(&certification).await?;
        Ok(certification)
    }

    pub async fn get_user_certifications(&self, user_id: Uuid) -> Result<Vec<Certification>, sqlx::Error> {
        self.repository.get_user_certifications(user_id).await
    }

    pub async fn get_user_skill_profile(&self, user_id: Uuid) -> Result<UserSkillProfile, sqlx::Error> {
        let skills = self.repository.get_skill_with_progress(user_id).await?;
        let certifications = self.repository.get_user_certifications(user_id).await?;
        
        // For now, learning paths are not implemented in the database
        let learning_paths = Vec::new();

        Ok(UserSkillProfile {
            user_id,
            skills,
            certifications,
            learning_paths,
        })
    }

    pub async fn delete_skill_progress(&self, progress_id: Uuid) -> Result<(), sqlx::Error> {
        self.repository.delete_skill_progress(progress_id).await
    }

    pub async fn delete_certification(&self, cert_id: Uuid) -> Result<(), sqlx::Error> {
        self.repository.delete_certification(cert_id).await
    }
}