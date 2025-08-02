use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::{AcademicCredential, CredentialType};
use crate::application::{CredentialRepository, Result, LearningPlatformError};
use chrono::Utc;
use std::str::FromStr;

pub struct CredentialRepositoryImpl {
    pool: PgPool,
}

impl CredentialRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl CredentialRepository for CredentialRepositoryImpl {
    async fn issue_credential(&self, credential: &AcademicCredential) -> Result<AcademicCredential> {
        let credential_type_str = credential.credential_type.to_string();
        
        let result = sqlx::query!(
            "INSERT INTO academic_credentials (id, user_id, course_id, credential_type, issued_at, verification_code) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
            credential.id.to_string(),
            credential.user_id.to_string(),
            credential.course_id.to_string(),
            credential_type_str,
            credential.issued_at,
            credential.verification_code
        )
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(_) => Ok(credential.clone()),
            Err(e) => Err(LearningPlatformError::DatabaseError(e.to_string())),
        }
    }
}