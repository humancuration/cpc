#[cfg(test)]
mod tests {
    use sqlx::PgPool;
    use uuid::Uuid;
    use chrono::Utc;
    use learning_platform_server::database::{models::*, repository::DatabaseRepository};

    // Helper function to create a test database pool
    async fn create_test_pool() -> PgPool {
        let db_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://localhost/learning_platform".to_string());
        PgPool::connect(&db_url).await.unwrap()
    }

    #[tokio::test]
    async fn test_create_and_get_user() {
        let pool = create_test_pool().await;
        let repo = DatabaseRepository::new(pool);
        
        // Create a user
        let user_id = Uuid::new_v4();
        let user = User {
            id: user_id,
            username: format!("testuser_{}", Uuid::new_v4()),
            email: format!("test_{}@example.com", Uuid::new_v4()),
            password_hash: "hashed_password".to_string(),
            created_at: Utc::now(),
        };
        
        // Save user
        let saved_user = repo.create_user(&user).await.unwrap();
        assert_eq!(saved_user.id, user_id);
        
        // Retrieve user by ID
        let retrieved_user = repo.get_user_by_id(user_id).await.unwrap();
        assert!(retrieved_user.is_some());
        assert_eq!(retrieved_user.unwrap().username, user.username);
        
        // Retrieve user by username
        let retrieved_user = repo.get_user_by_username(&user.username).await.unwrap();
        assert!(retrieved_user.is_some());
        assert_eq!(retrieved_user.unwrap().id, user_id);
    }

    #[tokio::test]
    async fn test_create_and_get_course() {
        let pool = create_test_pool().await;
        let repo = DatabaseRepository::new(pool);
        
        // Create a course
        let course_id = Uuid::new_v4();
        let creator_id = Uuid::new_v4();
        let course = Course {
            id: course_id,
            title: "Test Course".to_string(),
            description: "Test Description".to_string(),
            creator_id,
            created_at: Utc::now(),
        };
        
        // Save course
        let saved_course = repo.create_course(&course).await.unwrap();
        assert_eq!(saved_course.id, course_id);
        
        // Retrieve course by ID
        let retrieved_course = repo.get_course_by_id(course_id).await.unwrap();
        assert!(retrieved_course.is_some());
        assert_eq!(retrieved_course.unwrap().title, course.title);
    }
}