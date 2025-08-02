use tonic::{Request, Response, Status};
use uuid::Uuid;
use chrono::Utc;
use crate::database::repository::DatabaseRepository;
use crate::database::models::Enrollment as DatabaseEnrollment;
use crate::error::AppError;
use crate::utils::parse_uuid;

// Import the generated protobuf types
tonic::include_proto!("cpc.learning_platform");
tonic::include_proto!("cpc.learning_platform_server");

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
        
        let user_id = parse_uuid(&req.user_id)?;
        let course_id = parse_uuid(&req.course_id)?;
        
        // Check if course exists
        if self.repository.get_course_by_id(course_id).await
            .map_err(AppError::from)?
            .is_none() {
            return Err(AppError::NotFound("Course not found".to_string()).into());
        }
        
        // Check if user is already enrolled
        if self.repository.get_enrollment_by_user_and_course(user_id, course_id).await
            .map_err(AppError::from)?
            .is_some() {
            return Err(AppError::AlreadyExists("User already enrolled in this course".to_string()).into());
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
            .map_err(AppError::from)?;
        
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
        
        let enrollment_id = parse_uuid(&req.enrollment_id)?;
        
        // Validate progress
        if req.progress < 0.0 || req.progress > 100.0 {
            return Err(AppError::Validation("Progress must be between 0 and 100".to_string()).into());
        }
        
        // Fetch existing enrollment
        let mut enrollment = self.repository.get_enrollment_by_id(enrollment_id).await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::NotFound("Enrollment not found".to_string()))?;
        
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
            .map_err(AppError::from)?;
        
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