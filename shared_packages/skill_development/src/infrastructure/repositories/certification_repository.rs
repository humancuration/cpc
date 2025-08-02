use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::{Certification, CertificationType};
use tracing::instrument;

#[async_trait]
pub trait CertificationRepository {
    async fn save(&self, certification: &Certification) -> Result<(), sqlx::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Certification>, sqlx::Error>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Certification>, sqlx::Error>;
    async fn find_by_skill_id(&self, skill_id: Uuid) -> Result<Vec<Certification>, sqlx::Error>;
    async fn find_by_verification_code(&self, code: &str) -> Result<Option<Certification>, sqlx::Error>;
    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error>;
}

pub struct PostgresCertificationRepository {
    pool: PgPool,
}

impl PostgresCertificationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CertificationRepository for PostgresCertificationRepository {
    #[instrument(skip(self))]
    async fn save(&self, certification: &Certification) -> Result<(), sqlx::Error> {
        let certification_type: i16 = match certification.certification_type {
            CertificationType::CourseCompletion => 0,
            CertificationType::PeerEndorsement => 1,
            CertificationType::SkillAssessment => 2,
            CertificationType::ProjectReview => 3,
            CertificationType::PortfolioReview => 4,
        };

        sqlx::query!(
            r#"
            INSERT INTO certifications (id, name, issuing_organization, issue_date, user_id, skill_id, certification_type, level_achieved, verification_code, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                issuing_organization = EXCLUDED.issuing_organization,
                issue_date = EXCLUDED.issue_date,
                skill_id = EXCLUDED.skill_id,
                certification_type = EXCLUDED.certification_type,
                level_achieved = EXCLUDED.level_achieved,
                verification_code = EXCLUDED.verification_code
            "#,
            certification.id,
            certification.name,
            certification.issuing_organization,
            certification.issue_date,
            certification.user_id,
            certification.skill_id,
            certification_type,
            certification.level_achieved.map(|l| l as i16),
            certification.verification_code,
            certification.created_at
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    #[instrument(skip(self))]
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Certification>, sqlx::Error> {
        let cert = sqlx::query!(
            r#"
            SELECT id, name, issuing_organization, issue_date, user_id, skill_id, certification_type, level_achieved, verification_code, created_at
            FROM certifications
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        match cert {
            Some(c) => {
                let certification_type = match c.certification_type {
                    0 => CertificationType::CourseCompletion,
                    1 => CertificationType::PeerEndorsement,
                    2 => CertificationType::SkillAssessment,
                    3 => CertificationType::ProjectReview,
                    4 => CertificationType::PortfolioReview,
                    _ => CertificationType::SkillAssessment, // default
                };

                Ok(Some(Certification {
                    id: c.id,
                    name: c.name,
                    issuing_organization: c.issuing_organization,
                    issue_date: c.issue_date,
                    user_id: c.user_id,
                    skill_id: c.skill_id,
                    certification_type,
                    level_achieved: c.level_achieved.map(|l| l as u8),
                    verification_code: c.verification_code,
                    created_at: c.created_at,
                }))
            }
            None => Ok(None),
        }
    }

    #[instrument(skip(self))]
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Certification>, sqlx::Error> {
        let certs = sqlx::query!(
            r#"
            SELECT id, name, issuing_organization, issue_date, user_id, skill_id, certification_type, level_achieved, verification_code, created_at
            FROM certifications
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        let result = certs.into_iter().map(|c| {
            let certification_type = match c.certification_type {
                0 => CertificationType::CourseCompletion,
                1 => CertificationType::PeerEndorsement,
                2 => CertificationType::SkillAssessment,
                3 => CertificationType::ProjectReview,
                4 => CertificationType::PortfolioReview,
                _ => CertificationType::SkillAssessment, // default
            };

            Certification {
                id: c.id,
                name: c.name,
                issuing_organization: c.issuing_organization,
                issue_date: c.issue_date,
                user_id: c.user_id,
                skill_id: c.skill_id,
                certification_type,
                level_achieved: c.level_achieved.map(|l| l as u8),
                verification_code: c.verification_code,
                created_at: c.created_at,
            }
        }).collect();

        Ok(result)
    }

    #[instrument(skip(self))]
    async fn find_by_skill_id(&self, skill_id: Uuid) -> Result<Vec<Certification>, sqlx::Error> {
        let certs = sqlx::query!(
            r#"
            SELECT id, name, issuing_organization, issue_date, user_id, skill_id, certification_type, level_achieved, verification_code, created_at
            FROM certifications
            WHERE skill_id = $1
            ORDER BY created_at DESC
            "#,
            skill_id
        )
        .fetch_all(&self.pool)
        .await?;

        let result = certs.into_iter().map(|c| {
            let certification_type = match c.certification_type {
                0 => CertificationType::CourseCompletion,
                1 => CertificationType::PeerEndorsement,
                2 => CertificationType::SkillAssessment,
                3 => CertificationType::ProjectReview,
                4 => CertificationType::PortfolioReview,
                _ => CertificationType::SkillAssessment, // default
            };

            Certification {
                id: c.id,
                name: c.name,
                issuing_organization: c.issuing_organization,
                issue_date: c.issue_date,
                user_id: c.user_id,
                skill_id: c.skill_id,
                certification_type,
                level_achieved: c.level_achieved.map(|l| l as u8),
                verification_code: c.verification_code,
                created_at: c.created_at,
            }
        }).collect();

        Ok(result)
    }

    #[instrument(skip(self))]
    async fn find_by_verification_code(&self, code: &str) -> Result<Option<Certification>, sqlx::Error> {
        let cert = sqlx::query!(
            r#"
            SELECT id, name, issuing_organization, issue_date, user_id, skill_id, certification_type, level_achieved, verification_code, created_at
            FROM certifications
            WHERE verification_code = $1
            "#,
            code
        )
        .fetch_optional(&self.pool)
        .await?;

        match cert {
            Some(c) => {
                let certification_type = match c.certification_type {
                    0 => CertificationType::CourseCompletion,
                    1 => CertificationType::PeerEndorsement,
                    2 => CertificationType::SkillAssessment,
                    3 => CertificationType::ProjectReview,
                    4 => CertificationType::PortfolioReview,
                    _ => CertificationType::SkillAssessment, // default
                };

                Ok(Some(Certification {
                    id: c.id,
                    name: c.name,
                    issuing_organization: c.issuing_organization,
                    issue_date: c.issue_date,
                    user_id: c.user_id,
                    skill_id: c.skill_id,
                    certification_type,
                    level_achieved: c.level_achieved.map(|l| l as u8),
                    verification_code: c.verification_code,
                    created_at: c.created_at,
                }))
            }
            None => Ok(None),
        }
    }

    #[instrument(skip(self))]
    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM certifications
            WHERE id = $1
            "#,
            id
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
    use chrono::NaiveDate;

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
    async fn test_save_and_find_certification() {
        let pool = setup_test_db().await;
        let repo = PostgresCertificationRepository::new(pool);
        
        // Run migrations
        sqlx::migrate!("../migrations")
            .run(&repo.pool)
            .await
            .expect("Failed to run migrations");

        let issue_date = NaiveDate::from_ymd_opt(2024, 7, 15).unwrap();
        let user_id = Uuid::new_v4();
        let skill_id = Uuid::new_v4();

        let cert = Certification::new(
            "Rust Programming Certification".to_string(),
            "CPC Cooperative".to_string(),
            issue_date,
            user_id,
            Some(skill_id),
            CertificationType::SkillAssessment,
            Some(2), // Advanced
        );

        // Save certification
        repo.save(&cert).await.expect("Failed to save certification");

        // Find by ID
        let found = repo.find_by_id(cert.id).await.expect("Failed to find certification");
        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.name, "Rust Programming Certification");
        assert_eq!(found.issuing_organization, "CPC Cooperative");
        assert_eq!(found.issue_date, issue_date);
        assert_eq!(found.user_id, user_id);
        assert_eq!(found.skill_id, Some(skill_id));
        assert_eq!(found.certification_type, CertificationType::SkillAssessment);
        assert_eq!(found.level_achieved, Some(2));

        // Find by user ID
        let user_certs = repo.find_by_user_id(user_id).await.expect("Failed to find user certifications");
        assert!(!user_certs.is_empty());

        // Find by skill ID
        let skill_certs = repo.find_by_skill_id(skill_id).await.expect("Failed to find skill certifications");
        assert!(!skill_certs.is_empty());

        // Find by verification code
        let code_cert = repo.find_by_verification_code(&cert.verification_code).await.expect("Failed to find certification by code");
        assert!(code_cert.is_some());
        assert_eq!(code_cert.unwrap().id, cert.id);
    }
}