use learning_core::application::LearningPlatformService;
use learning_core::infrastructure::repositories::{CourseRepositoryImpl, EnrollmentRepositoryImpl, CredentialRepositoryImpl, TipRepositoryImpl};
use learning_core::domain::{EnrollmentStatus, CredentialType};
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This example assumes you have a database set up
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://learning_user:secure_password@localhost/learning_db".to_string());
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Set up repositories
    let course_repo = Box::new(CourseRepositoryImpl::new(pool.clone()));
    let enrollment_repo = Box::new(EnrollmentRepositoryImpl::new(pool.clone()));
    let credential_repo = Box::new(CredentialRepositoryImpl::new(pool.clone()));
    let tip_repo = Box::new(TipRepositoryImpl::new(pool.clone()));
    
    // Set up service
    let service = LearningPlatformService::new(
        course_repo,
        enrollment_repo,
        credential_repo,
        tip_repo,
    );
    
    // Create a course
    let creator_id = Uuid::new_v4();
    let course = service.create_course(
        "Rust Programming".to_string(),
        "Learn Rust programming language".to_string(),
        creator_id
    ).await?;
    
    println!("Created course: {} - {}", course.id, course.title);
    
    // Enroll a user
    let user_id = Uuid::new_v4();
    let enrollment = service.enroll_user(user_id, course.id).await?;
    
    println!("Enrolled user: {} in course: {}", enrollment.user_id, enrollment.course_id);
    
    // Update progress
    let updated_enrollment = service.update_progress(enrollment.id, 75.0).await?;
    
    println!("Updated progress: {}% - Status: {:?}", updated_enrollment.progress, updated_enrollment.status);
    
    // Complete course
    let completed_enrollment = service.update_progress(enrollment.id, 25.0).await?;
    
    println!("Completed course: {}% - Status: {:?}", completed_enrollment.progress, completed_enrollment.status);
    
    // Issue credential
    let credential = service.issue_credential(user_id, course.id, CredentialType::Certificate).await?;
    
    println!("Issued credential: {} - Verification code: {}", credential.id, credential.verification_code);
    
    // Tip educator
    let tip = service.tip_educator(
        user_id,
        creator_id,
        15.0,
        "USD".to_string(),
        Some(course.id)
    ).await?;
    
    println!("Sent tip: {} {} from {} to {}", tip.amount, tip.currency, tip.from_user_id, tip.to_user_id);
    
    Ok(())
}