use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;
use learning_core::*;
use chrono::Utc;

#[tokio::test]
async fn test_learning_platform_integration() -> Result<(), Box<dyn std::error::Error>> {
    // This test assumes you have a test database set up
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://learning_user:secure_password@localhost/learning_test_db".to_string());
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    // Run migrations
    sqlx::migrate!("../migrations")
        .run(&pool)
        .await?;
    
    // Set up repositories
    let course_repo = Box::new(infrastructure::repositories::CourseRepositoryImpl::new(pool.clone()));
    let enrollment_repo = Box::new(infrastructure::repositories::EnrollmentRepositoryImpl::new(pool.clone()));
    let credential_repo = Box::new(infrastructure::repositories::CredentialRepositoryImpl::new(pool.clone()));
    let tip_repo = Box::new(infrastructure::repositories::TipRepositoryImpl::new(pool.clone()));
    
    // Set up service
    let service = application::LearningPlatformService::new(
        course_repo,
        enrollment_repo,
        credential_repo,
        tip_repo,
    );
    
    // Test creating a course
    let creator_id = Uuid::new_v4();
    let course = service.create_course(
        "Rust Programming".to_string(),
        "Learn Rust programming language".to_string(),
        creator_id
    ).await?;
    
    assert_eq!(course.title, "Rust Programming");
    assert_eq!(course.creator_id, creator_id);
    
    // Test enrolling a user
    let user_id = Uuid::new_v4();
    let enrollment = service.enroll_user(user_id, course.id).await?;
    
    assert_eq!(enrollment.user_id, user_id);
    assert_eq!(enrollment.course_id, course.id);
    assert_eq!(enrollment.progress, 0.0);
    
    // Test updating progress
    let updated_enrollment = service.update_progress(enrollment.id, 50.0).await?;
    
    assert_eq!(updated_enrollment.progress, 50.0);
    assert_eq!(updated_enrollment.status, domain::EnrollmentStatus::InProgress);
    
    // Test completing course
    let completed_enrollment = service.update_progress(enrollment.id, 50.0).await?;
    
    assert_eq!(completed_enrollment.progress, 100.0);
    assert_eq!(completed_enrollment.status, domain::EnrollmentStatus::Completed);
    
    // Test issuing credential
    let credential = service.issue_credential(
        user_id, 
        course.id, 
        domain::CredentialType::Certificate
    ).await?;
    
    assert_eq!(credential.user_id, user_id);
    assert_eq!(credential.course_id, course.id);
    assert_eq!(credential.credential_type, domain::CredentialType::Certificate);
    
    // Test tipping educator
    let tip = service.tip_educator(
        user_id,
        creator_id,
        10.0,
        "USD".to_string(),
        Some(course.id)
    ).await?;
    
    assert_eq!(tip.from_user_id, user_id);
    assert_eq!(tip.to_user_id, creator_id);
    assert_eq!(tip.amount, 10.0);
    assert_eq!(tip.currency, "USD");
    assert_eq!(tip.course_id, Some(course.id));
    
    Ok(())
}