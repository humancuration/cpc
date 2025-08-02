#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use tonic::transport::Server;
    use sqlx::PgPool;
    use uuid::Uuid;
    use chrono::Utc;
    use learning_platform_server::{
        database::{models::*, repository::DatabaseRepository},
        grpc::{
            course_service::CourseService,
            enrollment_service::EnrollmentService,
            credential_service::CredentialService,
            tip_service::TipService,
            auth_service::AuthService,
            user_service::UserService,
            health_service::HealthService,
        },
    };

    // Helper function to create a test database pool
    async fn create_test_pool() -> PgPool {
        let db_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://localhost/learning_platform".to_string());
        PgPool::connect(&db_url).await.unwrap()
    }

    #[tokio::test]
    async fn test_health_check() {
        let health_service = HealthService::new();
        
        // Create a health check request
        let request = tonic::Request::new(learning_platform_server::HealthCheckRequest {
            service: String::new(),
        });
        
        // Call the check method
        let response = health_service.check(request).await.unwrap();
        
        // Verify the response
        assert_eq!(response.into_inner().status, 1); // SERVING status
    }

    #[tokio::test]
    async fn test_user_registration() {
        let pool = create_test_pool().await;
        let repository = DatabaseRepository::new(pool);
        let user_service = UserService::new(repository);
        
        // Create a register user request
        let username = format!("testuser_{}", Uuid::new_v4());
        let request = tonic::Request::new(learning_platform_server::RegisterUserRequest {
            username: username.clone(),
            email: format!("test_{}@example.com", Uuid::new_v4()),
            password: "password123".to_string(),
        });
        
        // Call the register_user method
        let response = user_service.register_user(request).await.unwrap();
        
        // Verify the response
        assert!(!response.into_inner().user_id.is_empty());
    }

    #[tokio::test]
    async fn test_course_creation() {
        let pool = create_test_pool().await;
        let repository = DatabaseRepository::new(pool);
        let course_service = CourseService::new(repository);
        
        // Create a course
        let course = learning_platform_server::Course {
            id: String::new(),
            title: "Test Course".to_string(),
            description: "Test Description".to_string(),
            creator_id: Uuid::new_v4().to_string(),
            modules: vec![],
        };
        
        let request = tonic::Request::new(learning_platform_server::CreateCourseRequest {
            course: Some(course),
        });
        
        // Call the create_course method
        let response = course_service.create_course(request).await.unwrap();
        
        // Verify the response
        assert!(!response.into_inner().course_id.is_empty());
    }
}