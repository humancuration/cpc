use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::Course;
use crate::application::{CourseRepository, Result, LearningPlatformError};

pub struct CourseRepositoryImpl {
    pool: PgPool,
}

impl CourseRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl CourseRepository for CourseRepositoryImpl {
    async fn create_course(&self, course: &Course) -> Result<Course> {
        // Note: In a real implementation, we would also need to handle modules and lessons
        // For simplicity, we're just storing the course metadata here
        let result = sqlx::query!(
            "INSERT INTO courses (id, title, description, creator_id) VALUES ($1, $2, $3, $4) RETURNING id",
            course.id.to_string(),
            course.title,
            course.description,
            course.creator_id.to_string()
        )
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(_) => Ok(course.clone()),
            Err(e) => Err(LearningPlatformError::DatabaseError(e.to_string())),
        }
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Course> {
        let result = sqlx::query!(
            "SELECT id, title, description, creator_id FROM courses WHERE id = $1",
            id.to_string()
        )
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(row) => {
                let course = Course {
                    id: Uuid::parse_str(&row.id).map_err(|_| LearningPlatformError::DatabaseError("Invalid UUID".to_string()))?,
                    title: row.title,
                    description: row.description,
                    creator_id: Uuid::parse_str(&row.creator_id).map_err(|_| LearningPlatformError::DatabaseError("Invalid UUID".to_string()))?,
                    modules: Vec::new(), // In a full implementation, we would fetch modules and lessons
                };
                Ok(course)
            },
            Err(sqlx::Error::RowNotFound) => Err(LearningPlatformError::CourseNotFound),
            Err(e) => Err(LearningPlatformError::DatabaseError(e.to_string())),
        }
    }
}