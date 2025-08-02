use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use crate::database::models::*;

pub struct DatabaseRepository {
    pool: PgPool,
}

impl DatabaseRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // Course operations
    pub async fn create_course(&self, course: &Course) -> Result<Course, sqlx::Error> {
        let result = sqlx::query_as!(
            Course,
            "INSERT INTO courses (id, title, description, creator_id, created_at) VALUES ($1, $2, $3, $4, $5) RETURNING *",
            course.id,
            course.title,
            course.description,
            course.creator_id,
            course.created_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_course_by_id(&self, id: Uuid) -> Result<Option<Course>, sqlx::Error> {
        let result = sqlx::query_as!(Course, "SELECT * FROM courses WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result)
    }

    pub async fn list_courses(&self) -> Result<Vec<Course>, sqlx::Error> {
        let result = sqlx::query_as!(Course, "SELECT * FROM courses ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;

        Ok(result)
    }

    // Enrollment operations
    pub async fn create_enrollment(&self, enrollment: &Enrollment) -> Result<Enrollment, sqlx::Error> {
        let result = sqlx::query_as!(
            Enrollment,
            "INSERT INTO enrollments (id, user_id, course_id, progress, status, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
            enrollment.id,
            enrollment.user_id,
            enrollment.course_id,
            enrollment.progress,
            enrollment.status,
            enrollment.created_at,
            enrollment.updated_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_enrollment_by_id(&self, id: Uuid) -> Result<Option<Enrollment>, sqlx::Error> {
        let result = sqlx::query_as!(Enrollment, "SELECT * FROM enrollments WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result)
    }

    pub async fn update_enrollment_progress(&self, id: Uuid, progress: f32, status: &str) -> Result<Enrollment, sqlx::Error> {
        let result = sqlx::query_as!(
            Enrollment,
            "UPDATE enrollments SET progress = $1, status = $2, updated_at = $3 WHERE id = $4 RETURNING *",
            progress,
            status,
            Utc::now(),
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_enrollment_by_user_and_course(&self, user_id: Uuid, course_id: Uuid) -> Result<Option<Enrollment>, sqlx::Error> {
        let result = sqlx::query_as!(
            Enrollment,
            "SELECT * FROM enrollments WHERE user_id = $1 AND course_id = $2",
            user_id,
            course_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    // Credential operations
    pub async fn create_credential(&self, credential: &AcademicCredential) -> Result<AcademicCredential, sqlx::Error> {
        let result = sqlx::query_as!(
            AcademicCredential,
            "INSERT INTO academic_credentials (id, user_id, course_id, credential_type, issued_at, verification_code, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
            credential.id,
            credential.user_id,
            credential.course_id,
            credential.credential_type,
            credential.issued_at,
            credential.verification_code,
            credential.created_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    // Tip operations
    pub async fn create_tip(&self, tip: &Tip) -> Result<Tip, sqlx::Error> {
        let result = sqlx::query_as!(
            Tip,
            "INSERT INTO tips (id, from_user_id, to_user_id, course_id, amount, currency, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
            tip.id,
            tip.from_user_id,
            tip.to_user_id,
            tip.course_id,
            tip.amount,
            tip.currency,
            tip.created_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    // User operations
    pub async fn create_user(&self, user: &User) -> Result<User, sqlx::Error> {
        let result = sqlx::query_as!(
            User,
            "INSERT INTO users (id, username, email, password_hash, created_at) VALUES ($1, $2, $3, $4, $5) RETURNING *",
            user.id,
            user.username,
            user.email,
            user.password_hash,
            user.created_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        let result = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result)
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let result = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result)
    }
}