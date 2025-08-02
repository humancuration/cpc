use tonic::{Request, Response, Status};
use uuid::Uuid;
use crate::application::LearningPlatformService;
use crate::domain::{EnrollmentStatus, CredentialType};

tonic::include_proto!("learning_platform");

pub struct LearningPlatformServiceImpl {
    learning_service: LearningPlatformService,
}

impl LearningPlatformServiceImpl {
    pub fn new(learning_service: LearningPlatformService) -> Self {
        Self { learning_service }
    }
}

#[tonic::async_trait]
impl learning_platform_server::LearningPlatform for LearningPlatformServiceImpl {
    async fn create_course(
        &self,
        request: Request<CourseRequest>,
    ) -> Result<Response<CourseResponse>, Status> {
        let req = request.into_inner();
        
        let creator_id = Uuid::parse_str(&req.creator_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid creator ID: {}", e)))?;
        
        let course = self.learning_service
            .create_course(req.title, req.description, creator_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to create course: {:?}", e)))?;

        // Convert domain Course to proto Course
        let proto_course = Course {
            id: course.id.to_string(),
            title: course.title,
            description: course.description,
            creator_id: course.creator_id.to_string(),
            modules: vec![], // In a full implementation, we would convert modules
        };

        let response = CourseResponse {
            course: Some(proto_course),
        };

        Ok(Response::new(response))
    }

    async fn enroll_user(
        &self,
        request: Request<EnrollmentRequest>,
    ) -> Result<Response<EnrollmentResponse>, Status> {
        let req = request.into_inner();
        
        let user_id = Uuid::parse_str(&req.user_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid user ID: {}", e)))?;
            
        let course_id = Uuid::parse_str(&req.course_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid course ID: {}", e)))?;

        let enrollment = self.learning_service
            .enroll_user(user_id, course_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to enroll user: {:?}", e)))?;

        // Convert domain Enrollment to proto Enrollment
        let status = match enrollment.status {
            EnrollmentStatus::Enrolled => enrollment::Status::Enrolled as i32,
            EnrollmentStatus::InProgress => enrollment::Status::InProgress as i32,
            EnrollmentStatus::Completed => enrollment::Status::Completed as i32,
            EnrollmentStatus::Dropped => enrollment::Status::Dropped as i32,
        };

        let proto_enrollment = Enrollment {
            id: enrollment.id.to_string(),
            user_id: enrollment.user_id.to_string(),
            course_id: enrollment.course_id.to_string(),
            progress: enrollment.progress,
            status,
        };

        let response = EnrollmentResponse {
            enrollment: Some(proto_enrollment),
        };

        Ok(Response::new(response))
    }

    async fn update_progress(
        &self,
        request: Request<ProgressUpdateRequest>,
    ) -> Result<Response<ProgressResponse>, Status> {
        let req = request.into_inner();
        
        let enrollment_id = Uuid::parse_str(&req.enrollment_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid enrollment ID: {}", e)))?;

        let enrollment = self.learning_service
            .update_progress(enrollment_id, req.progress_delta)
            .await
            .map_err(|e| Status::internal(format!("Failed to update progress: {:?}", e)))?;

        // Convert domain Enrollment to proto Enrollment
        let status = match enrollment.status {
            EnrollmentStatus::Enrolled => enrollment::Status::Enrolled as i32,
            EnrollmentStatus::InProgress => enrollment::Status::InProgress as i32,
            EnrollmentStatus::Completed => enrollment::Status::Completed as i32,
            EnrollmentStatus::Dropped => enrollment::Status::Dropped as i32,
        };

        let proto_enrollment = Enrollment {
            id: enrollment.id.to_string(),
            user_id: enrollment.user_id.to_string(),
            course_id: enrollment.course_id.to_string(),
            progress: enrollment.progress,
            status,
        };

        let response = ProgressResponse {
            enrollment: Some(proto_enrollment),
        };

        Ok(Response::new(response))
    }

    async fn issue_credential(
        &self,
        request: Request<CredentialRequest>,
    ) -> Result<Response<CredentialResponse>, Status> {
        let req = request.into_inner();
        
        let user_id = Uuid::parse_str(&req.user_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid user ID: {}", e)))?;
            
        let course_id = Uuid::parse_str(&req.course_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid course ID: {}", e)))?;

        // Convert proto CredentialType to domain CredentialType
        let credential_type = match req.credential_type {
            0 => CredentialType::Certificate,
            1 => CredentialType::MicroDegree,
            2 => CredentialType::Degree,
            3 => CredentialType::Badge,
            _ => return Err(Status::invalid_argument("Invalid credential type")),
        };

        let credential = self.learning_service
            .issue_credential(user_id, course_id, credential_type)
            .await
            .map_err(|e| Status::internal(format!("Failed to issue credential: {:?}", e)))?;

        // Convert domain AcademicCredential to proto AcademicCredential
        let credential_type = match credential.credential_type {
            CredentialType::Certificate => academic_credential::CredentialType::Certificate as i32,
            CredentialType::MicroDegree => academic_credential::CredentialType::MicroDegree as i32,
            CredentialType::Degree => academic_credential::CredentialType::Degree as i32,
            CredentialType::Badge => academic_credential::CredentialType::Badge as i32,
        };

        let proto_credential = AcademicCredential {
            id: credential.id.to_string(),
            user_id: credential.user_id.to_string(),
            course_id: credential.course_id.to_string(),
            credential_type,
            issued_at: Some(prost_types::Timestamp {
                seconds: credential.issued_at.timestamp(),
                nanos: credential.issued_at.timestamp_subsec_nanos() as i32,
            }),
            verification_code: credential.verification_code,
        };

        let response = CredentialResponse {
            credential: Some(proto_credential),
        };

        Ok(Response::new(response))
    }

    async fn tip_educator(
        &self,
        request: Request<TipRequest>,
    ) -> Result<Response<TipResponse>, Status> {
        let req = request.into_inner();
        
        let from_user_id = Uuid::parse_str(&req.from_user_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid from user ID: {}", e)))?;
            
        let to_user_id = Uuid::parse_str(&req.to_user_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid to user ID: {}", e)))?;

        let course_id = req.course_id.as_ref().map(|id| {
            Uuid::parse_str(id).map_err(|e| Status::invalid_argument(format!("Invalid course ID: {}", e)))
        }).transpose()?;

        let tip = self.learning_service
            .tip_educator(from_user_id, to_user_id, req.amount, req.currency, course_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to process tip: {:?}", e)))?;

        // Convert domain Tip to proto Tip
        let proto_tip = Tip {
            id: tip.id.to_string(),
            from_user_id: tip.from_user_id.to_string(),
            to_user_id: tip.to_user_id.to_string(),
            course_id: tip.course_id.map(|id| id.to_string()),
            amount: tip.amount,
            currency: tip.currency,
            created_at: Some(prost_types::Timestamp {
                seconds: tip.created_at.timestamp(),
                nanos: tip.created_at.timestamp_subsec_nanos() as i32,
            }),
        };

        let response = TipResponse {
            tip: Some(proto_tip),
        };

        Ok(Response::new(response))
    }
}