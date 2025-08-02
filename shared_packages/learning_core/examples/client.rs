use learning_core::learning_platform_client::LearningPlatformClient;
use learning_core::{CourseRequest, EnrollmentRequest, ProgressUpdateRequest, CredentialRequest, TipRequest};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the gRPC server
    let mut client = LearningPlatformClient::connect("http://[::1]:50052").await?;

    // Create a course
    let course_request = tonic::Request::new(CourseRequest {
        title: "Rust Programming".to_string(),
        description: "Learn Rust programming language".to_string(),
        creator_id: Uuid::new_v4().to_string(),
    });

    let course_response = client.create_course(course_request).await?;
    let course = course_response.into_inner().course.unwrap();
    println!("Created course: {} - {}", course.id, course.title);

    // Enroll a user
    let enrollment_request = tonic::Request::new(EnrollmentRequest {
        user_id: Uuid::new_v4().to_string(),
        course_id: course.id.clone(),
    });

    let enrollment_response = client.enroll_user(enrollment_request).await?;
    let enrollment = enrollment_response.into_inner().enrollment.unwrap();
    println!("Enrolled user: {} in course: {}", enrollment.user_id, enrollment.course_id);

    // Update progress
    let progress_request = tonic::Request::new(ProgressUpdateRequest {
        enrollment_id: enrollment.id.clone(),
        progress_delta: 50.0,
    });

    let progress_response = client.update_progress(progress_request).await?;
    let updated_enrollment = progress_response.into_inner().enrollment.unwrap();
    println!("Updated progress: {}% - Status: {:?}", updated_enrollment.progress, updated_enrollment.status);

    // Complete course
    let complete_request = tonic::Request::new(ProgressUpdateRequest {
        enrollment_id: enrollment.id.clone(),
        progress_delta: 50.0,
    });

    let complete_response = client.update_progress(complete_request).await?;
    let completed_enrollment = complete_response.into_inner().enrollment.unwrap();
    println!("Completed course: {}% - Status: {:?}", completed_enrollment.progress, completed_enrollment.status);

    // Issue credential
    let credential_request = tonic::Request::new(CredentialRequest {
        user_id: enrollment.user_id.clone(),
        course_id: course.id.clone(),
        credential_type: 0, // Certificate
    });

    let credential_response = client.issue_credential(credential_request).await?;
    let credential = credential_response.into_inner().credential.unwrap();
    println!("Issued credential: {} - Verification code: {}", credential.id, credential.verification_code);

    // Tip educator
    let tip_request = tonic::Request::new(TipRequest {
        from_user_id: enrollment.user_id.clone(),
        to_user_id: course.creator_id.clone(),
        course_id: Some(course.id.clone()),
        amount: 10.0,
        currency: "USD".to_string(),
    });

    let tip_response = client.tip_educator(tip_request).await?;
    let tip = tip_response.into_inner().tip.unwrap();
    println!("Sent tip: {} {} from {} to {}", tip.amount, tip.currency, tip.from_user_id, tip.to_user_id);

    Ok(())
}