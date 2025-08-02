use tonic::{Request, Response, Status};
use uuid::Uuid;
use chrono::Utc;
use crate::database::repository::DatabaseRepository;
use crate::database::models::Enrollment as DatabaseEnrollment;

// Import the generated protobuf types
tonic::include_proto!("cpc.learning_platform");

pub struct EnrollmentService {
    repository: DatabaseRepository,
}

impl EnrollmentService {
    pub fn new(repository: DatabaseRepository) -> Self {
        Self { repository }
    }
}

#[tonic::async_trait]
impl enrollment_service_server::EnrollmentService for EnrollmentService {
    async fn enroll(
        &self,
        request: Request<EnrollRequest>,
    ) -> Result<Response<EnrollResponse>, Status> {
        let req = request.into_inner();
        
        let user_id = Uuid::parse_str(&req.user_id)
            .map_err(|_| Status::invalid_argument("Invalid user ID"))?;
            
        let course_id = Uuid::parse_str(&req.course_id)
            .map_err(|_| Status::invalid_argument("Invalid course ID"))?;
        
        // Check if course exists
        if self.repository.get_course_by_id(course_id).await
            .map_err(|e| Status::internal(format!("Failed to check course: {}", e)))?
            .is_none() {
            return Err(Status::not_found("Course not found"));
        }
        
        // Check if user is already enrolled
        if self.repository.get_enrollment_by_user_and_course(user_id, course_id).await
            .map_err(|e| Status::internal(format!("Failed to check enrollment: {}", e)))?
            .is_some() {
            return Err(Status::already_exists("User already enrolled in this course"));
        }
        
        // Create enrollment
        let enrollment_id = Uuid::new_v4();
        let db_enrollment = DatabaseEnrollment {
            id: enrollment_id,
            user_id,
            course_id,
            progress: 0.0,
            status: "ENROLLED".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // Save to database
        let saved_enrollment = self.repository.create_enrollment(&db_enrollment).await
            .map_err(|e| Status::internal(format!("Failed to create enrollment: {}", e)))?;
        
        // Convert to protobuf enrollment
        let proto_enrollment = Enrollment {
            id: saved_enrollment.id.to_string(),
            user_id: saved_enrollment.user_id.to_string(),
            course_id: saved_enrollment.course_id.to_string(),
            status: EnrollmentStatus::Enrolled as i32,
            progress: saved_enrollment.progress,
        };
        
        let response = EnrollResponse {
            enrollment: Some(proto_enrollment),
        };
        
        Ok(Response::new(response))
    }

    async fn update_progress(
        &self,
        request: Request<UpdateProgressRequest>,
    ) -> Result<Response<UpdateProgressResponse>, Status> {
        let req = request.into_inner();
        
        let enrollment_id = Uuid::parse_str(&req.enrollment_id)
            .map_err(|_| Status::invalid_argument("Invalid enrollment ID"))?;
        
        // Fetch existing enrollment
        let mut enrollment = self.repository.get_enrollment_by_id(enrollment_id).await
            .map_err(|e| Status::internal(format!("Failed to fetch enrollment: {}", e)))?
            .ok_or_else(|| Status::not_found("Enrollment not found"))?;
        
        // Update progress
        enrollment.progress = req.progress;
        
        // Determine status based on progress
        enrollment.status = if enrollment.progress >= 100.0 {
            "COMPLETED".to_string()
        } else if enrollment.progress > 0.0 {
            "IN_PROGRESS".to_string()
        } else {
            "ENROLLED".to_string()
        };
        
        enrollment.updated_at = Utc::now();
        
        // Save to database
        let updated_enrollment = self.repository.update_enrollment_progress(
            enrollment.id,
            enrollment.progress,
            &enrollment.status
        ).await
            .map_err(|e| Status::internal(format!("Failed to update enrollment: {}", e)))?;
        
        // Convert to protobuf enrollment
        let status_enum = match updated_enrollment.status.as_str() {
            "ENROLLED" => EnrollmentStatus::Enrolled,
            "IN_PROGRESS" => EnrollmentStatus::InProgress,
            "COMPLETED" => EnrollmentStatus::Completed,
            "DROPPED" => EnrollmentStatus::Dropped,
            _ => EnrollmentStatus::Enrolled,
        };
        
        let proto_enrollment = Enrollment {
            id: updated_enrollment.id.to_string(),
            user_id: updated_enrollment.user_id.to_string(),
            course_id: updated_enrollment.course_id.to_string(),
            status: status_enum as i32,
            progress: updated_enrollment.progress,
        };
        
        let response = UpdateProgressResponse {
            enrollment: Some(proto_enrollment),
        };
        
        Ok(Response::new(response))
    }
}