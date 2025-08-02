use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::*;

pub struct PostgresRepository {
    pool: PgPool,
}

impl PostgresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save_skill(&self, skill: &Skill) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO skills (id, name, description, created_at)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                description = EXCLUDED.description
            "#,
            skill.id,
            skill.name,
            skill.description,
            skill.created_at
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_skill(&self, skill_id: Uuid) -> Result<Option<Skill>, sqlx::Error> {
        let skill = sqlx::query_as!(
            Skill,
            r#"
            SELECT id, name, description, created_at
            FROM skills
            WHERE id = $1
            "#,
            skill_id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(skill)
    }

    pub async fn save_skill_progress(
        &self,
        progress: &SkillProgress,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO skill_progress (id, skill_id, user_id, progress, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                progress = EXCLUDED.progress,
                updated_at = EXCLUDED.updated_at
            "#,
            progress.id,
            progress.skill_id,
            progress.user_id,
            progress.progress as i16,
            progress.updated_at
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_skill_progress(
        &self,
        progress_id: Uuid,
    ) -> Result<SkillProgress, sqlx::Error> {
        let progress = sqlx::query_as!(
            SkillProgress,
            r#"
            SELECT id, skill_id, user_id, progress as "progress: u8", updated_at
            FROM skill_progress
            WHERE id = $1
            "#,
            progress_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(progress)
    }

    pub async fn get_user_skill_progress(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<SkillProgress>, sqlx::Error> {
        let progress_list = sqlx::query_as!(
            SkillProgress,
            r#"
            SELECT id, skill_id, user_id, progress as "progress: u8", updated_at
            FROM skill_progress
            WHERE user_id = $1
            ORDER BY updated_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(progress_list)
    }

    pub async fn get_skill_with_progress(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<SkillWithProgress>, sqlx::Error> {
        let results = sqlx::query!(
            r#"
            SELECT s.id as skill_id, s.name, s.description, s.created_at,
                   sp.id as progress_id, sp.progress, sp.updated_at as progress_updated
            FROM skills s
            LEFT JOIN skill_progress sp ON s.id = sp.skill_id AND sp.user_id = $1
            ORDER BY s.name
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut skills_with_progress = Vec::new();
        for row in results {
            let skill = Skill {
                id: row.skill_id,
                name: row.name,
                description: row.description,
                created_at: row.created_at,
            };

            let progress = if let Some(progress_id) = row.progress_id {
                Some(SkillProgress {
                    id: progress_id,
                    skill_id: row.skill_id,
                    user_id,
                    progress: row.progress.unwrap_or(0) as u8,
                    updated_at: row.progress_updated.unwrap_or(row.created_at),
                })
            } else {
                None
            };

            skills_with_progress.push(SkillWithProgress { skill, progress });
        }

        Ok(skills_with_progress)
    }

    pub async fn save_certification(
        &self,
        certification: &Certification,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO certifications (id, name, issuing_organization, issue_date, user_id)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                issuing_organization = EXCLUDED.issuing_organization,
                issue_date = EXCLUDED.issue_date
            "#,
            certification.id,
            certification.name,
            certification.issuing_organization,
            certification.issue_date,
            certification.user_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_certification(&self, cert_id: Uuid) -> Result<Option<Certification>, sqlx::Error> {
        let cert = sqlx::query_as!(
            Certification,
            r#"
            SELECT id, name, issuing_organization, issue_date, user_id
            FROM certifications
            WHERE id = $1
            "#,
            cert_id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(cert)
    }

    pub async fn get_user_certifications(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Certification>, sqlx::Error> {
        let certifications = sqlx::query_as!(
            Certification,
            r#"
            SELECT id, name, issuing_organization, issue_date, user_id
            FROM certifications
            WHERE user_id = $1
            ORDER BY issue_date DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(certifications)
    }

    pub async fn get_all_skills(&self) -> Result<Vec<Skill>, sqlx::Error> {
        let skills = sqlx::query_as!(
            Skill,
            r#"
            SELECT id, name, description, created_at
            FROM skills
            ORDER BY name
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(skills)
    }

    pub async fn get_all_skill_progress(&self) -> Result<Vec<SkillProgress>, sqlx::Error> {
        let progress_list = sqlx::query_as!(
            SkillProgress,
            r#"
            SELECT id, skill_id, user_id, progress as "progress: u8", updated_at
            FROM skill_progress
            ORDER BY updated_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(progress_list)
    }

    pub async fn get_all_certifications(&self) -> Result<Vec<Certification>, sqlx::Error> {
        let certifications = sqlx::query_as!(
            Certification,
            r#"
            SELECT id, name, issuing_organization, issue_date, user_id
            FROM certifications
            ORDER BY issue_date DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(certifications)
    }

    pub async fn delete_skill_progress(
        &self,
        progress_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM skill_progress
            WHERE id = $1
            "#,
            progress_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_certification(
        &self,
        cert_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM certifications
            WHERE id = $1
            "#,
            cert_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}