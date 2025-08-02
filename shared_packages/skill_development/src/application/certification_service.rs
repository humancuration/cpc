use crate::domain::{Certification, CertificationType, CertificationRepository, PostgresCertificationRepository};
use crate::infrastructure::repositories::postgres_repo::PostgresRepository;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::NaiveDate;
use tracing::instrument;

pub struct CertificationService {
    certification_repo: PostgresCertificationRepository,
    skill_repo: PostgresRepository,
}

impl CertificationService {
    pub fn new(pool: PgPool) -> Self {
        let certification_repo = PostgresCertificationRepository::new(pool.clone());
        let skill_repo = PostgresRepository::new(pool);
        Self {
            certification_repo,
            skill_repo,
        }
    }

    #[instrument(skip(self))]
    pub async fn issue_certification(
        &self,
        name: String,
        issuing_organization: String,
        issue_date: NaiveDate,
        user_id: Uuid,
        skill_id: Option<Uuid>,
        certification_type: CertificationType,
        level_achieved: Option<u8>,
    ) -> Result<Certification, Box<dyn std::error::Error>> {
        // Validate that the skill exists if provided
        if let Some(skill_id) = skill_id {
            let skill = self.skill_repo.get_skill(skill_id).await?;
            if skill.is_none() {
                return Err("Skill not found".into());
            }
        }

        let cert = Certification::new(
            name,
            issuing_organization,
            issue_date,
            user_id,
            skill_id,
            certification_type,
            level_achieved,
        );

        self.certification_repo.save(&cert).await?;
        Ok(cert)
    }

    #[instrument(skip(self))]
    pub async fn get_certification(
        &self,
        id: Uuid,
    ) -> Result<Option<Certification>, Box<dyn std::error::Error>> {
        let cert = self.certification_repo.find_by_id(id).await?;
        Ok(cert)
    }

    #[instrument(skip(self))]
    pub async fn get_user_certifications(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Certification>, Box<dyn std::error::Error>> {
        let certs = self.certification_repo.find_by_user_id(user_id).await?;
        Ok(certs)
    }

    #[instrument(skip(self))]
    pub async fn get_skill_certifications(
        &self,
        skill_id: Uuid,
    ) -> Result<Vec<Certification>, Box<dyn std::error::Error>> {
        let certs = self.certification_repo.find_by_skill_id(skill_id).await?;
        Ok(certs)
    }

    #[instrument(skip(self))]
    pub async fn verify_certification(
        &self,
        verification_code: String,
    ) -> Result<Option<Certification>, Box<dyn std::error::Error>> {
        let cert = self.certification_repo.find_by_verification_code(&verification_code).await?;
        Ok(cert)
    }

    #[instrument(skip(self))]
    pub async fn delete_certification(
        &self,
        id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.certification_repo.delete(id).await?;
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
    async fn test_issue_and_get_certification() {
        let pool = setup_test_db().await;
        let service = CertificationService::new(pool);
        
        // Run migrations
        sqlx::migrate!("../../migrations")
            .run(&service.certification_repo.pool)
            .await
            .expect("Failed to run migrations");

        let issue_date = NaiveDate::from_ymd_opt(2024, 7, 15).unwrap();
        let user_id = Uuid::new_v4();

        // Issue certification without skill
        let cert = service.issue_certification(
            "General Programming Certification".to_string(),
            "CPC Cooperative".to_string(),
            issue_date,
            user_id,
            None,
            CertificationType::CourseCompletion,
            None,
        ).await.expect("Failed to issue certification");

        assert_eq!(cert.name, "General Programming Certification");
        assert_eq!(cert.issuing_organization, "CPC Cooperative");
        assert_eq!(cert.issue_date, issue_date);
        assert_eq!(cert.user_id, user_id);
        assert!(cert.skill_id.is_none());

        // Get certification by ID
        let found = service.get_certification(cert.id)
            .await
            .expect("Failed to get certification");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "General Programming Certification");

        // Get user certifications
        let user_certs = service.get_user_certifications(user_id)
            .await
            .expect("Failed to get user certifications");
        assert_eq!(user_certs.len(), 1);
    }

    #[tokio::test]
    async fn test_issue_certification_with_skill() {
        let pool = setup_test_db().await;
        let service = CertificationService::new(pool);
        
        // Run migrations
        sqlx::migrate!("../../migrations")
            .run(&service.certification_repo.pool)
            .await
            .expect("Failed to run migrations");

        // First, add a skill
        let skill = service.skill_repo.add_skill(
            "Rust Programming".to_string(),
            Some("Master Rust programming language".to_string())
        ).await.expect("Failed to add skill");

        let issue_date = NaiveDate::from_ymd_opt(2024, 7, 15).unwrap();
        let user_id = Uuid::new_v4();
        let skill_id = skill.id;

        // Issue certification with skill
        let cert = service.issue_certification(
            "Rust Programming Certification".to_string(),
            "CPC Cooperative".to_string(),
            issue_date,
            user_id,
            Some(skill_id),
            CertificationType::SkillAssessment,
            Some(2), // Advanced
        ).await.expect("Failed to issue certification with skill");

        assert_eq!(cert.name, "Rust Programming Certification");
        assert_eq!(cert.skill_id, Some(skill_id));
        assert_eq!(cert.certification_type, CertificationType::SkillAssessment);
        assert_eq!(cert.level_achieved, Some(2));

        // Get skill certifications
        let skill_certs = service.get_skill_certifications(skill_id)
            .await
            .expect("Failed to get skill certifications");
        assert_eq!(skill_certs.len(), 1);
        assert_eq!(skill_certs[0].name, "Rust Programming Certification");
    }

    #[tokio::test]
    async fn test_verify_certification() {
        let pool = setup_test_db().await;
        let service = CertificationService::new(pool);
        
        // Run migrations
        sqlx::migrate!("../../migrations")
            .run(&service.certification_repo.pool)
            .await
            .expect("Failed to run migrations");

        let issue_date = NaiveDate::from_ymd_opt(2024, 7, 15).unwrap();
        let user_id = Uuid::new_v4();

        // Issue certification
        let cert = service.issue_certification(
            "Test Certification".to_string(),
            "CPC Cooperative".to_string(),
            issue_date,
            user_id,
            None,
            CertificationType::CourseCompletion,
            None,
        ).await.expect("Failed to issue certification");

        // Verify certification
        let verified = service.verify_certification(cert.verification_code.clone())
            .await
            .expect("Failed to verify certification");
        assert!(verified.is_some());
        assert_eq!(verified.unwrap().id, cert.id);

        // Try to verify with invalid code
        let invalid = service.verify_certification("INVALID-CODE".to_string())
            .await
            .expect("Failed to verify with invalid code");
        assert!(invalid.is_none());
    }
}