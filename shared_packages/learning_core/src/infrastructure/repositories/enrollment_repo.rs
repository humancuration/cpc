use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::{Enrollment, EnrollmentStatus};
use crate::application::{EnrollmentRepository, Result, LearningPlatformError};
use std::str::FromStr;

pub struct EnrollmentRepositoryImpl {
    pool: PgPool,
}

impl EnrollmentRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl EnrollmentRepository for EnrollmentRepositoryImpl {
    async fn enroll_user(&self, enrollment: &Enrollment) -> Result<Enrollment> {
        let status_str = enrollment.status.to_string();
        
        let result = sqlx::query!(
            "INSERT INTO enrollments (id, user_id, course_id, progress, status) VALUES ($1, $2, $3, $4, $5) RETURNING id",
            enrollment.id.to_string(),
            enrollment.user_id.to_string(),
            enrollment.course_id.to_string(),
            enrollment.progress,
            status_str
        )
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(_) => Ok(enrollment.clone()),
            Err(e) => Err(LearningPlatformError::DatabaseError(e.to_string())),
        }
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Enrollment> {
        let result = sqlx::query!(
            "SELECT id, user_id, course_id, progress, status FROM enrollments WHERE id = $1",
            id.to_string()
        )
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(row) => {
                let status = EnrollmentStatus::from_str(&row.status)
                    .map_err(|_| LearningPlatformError::DatabaseError("Invalid status".to_string()))?;
                
                let enrollment = Enrollment {
                    id: Uuid::parse_str(&row.id).map_err(|_| LearningPlatformError::DatabaseError("Invalid UUID".to_string()))?,
                    user_id: Uuid::parse_str(&row.user_id).map_err(|_| LearningPlatformError::DatabaseError("Invalid UUID".to_string()))?,
                    course_id: Uuid::parse_str(&row.course_id).map_err(|_| LearningPlatformError::DatabaseError("Invalid UUID".to_string()))?,
                    progress: row.progress,
                    status,
                };
                Ok(enrollment)
            },
            Err(sqlx::Error::RowNotFound) => Err(LearningPlatformError::EnrollmentNotFound),
            Err(e) => Err(LearningPlatformError::DatabaseError(e.to_string())),
        }
    }

    async fn update_progress(&self, enrollment: &Enrollment) -> Result<Enrollment> {
        let status_str = enrollment.status.to_string();
        
        let result = sqlx::query!(
            "UPDATE enrollments SET progress = $1, status = $2 WHERE id = $3 RETURNING id",
            enrollment.progress,
            status_str,
            enrollment.id.to_string()
        )
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(_) => Ok(enrollment.clone()),
            Err(sqlx::Error::RowNotFound) => Err(LearningPlatformError::EnrollmentNotFound),
            Err(e) => Err(LearningPlatformError::DatabaseError(e.to_string())),
        }
    }

    async fn find_by_user_and_course(&self, user_id: Uuid, course_id: Uuid) -> Result<Enrollment> {
        let result = sqlx::query!(
            "SELECT id, user_id, course_id, progress, status FROM enrollments WHERE user_id = $1 AND course_id = $2",
            user_id.to_string(),
            course_id.to_string()
        )
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(row) => {
                let status = EnrollmentStatus::from_str(&row.status)
                    .map_err(|_| LearningPlatformError::DatabaseError("Invalid status".to_string()))?;
                
                let enrollment = Enrollment {
                    id: Uuid::parse_str(&row.id).map_err(|_| LearningPlatformError::DatabaseError("Invalid UUID".to_string()))?,
                    user_id: Uuid::parse_str(&row.user_id).map_err(|_| LearningPlatformError::DatabaseError("Invalid UUID".to_string()))?,
                    course_id: Uuid::parse_str(&row.course_id).map_err(|_| LearningPlatformError::DatabaseError("Invalid UUID".to_string()))?,
                    progress: row.progress,
                    status,
                };
                Ok(enrollment)
            },
            Err(sqlx::Error::RowNotFound) => Err(LearningPlatformError::EnrollmentNotFound),
            Err(e) => Err(LearningPlatformError::DatabaseError(e.to_string())),
        }
    }
}