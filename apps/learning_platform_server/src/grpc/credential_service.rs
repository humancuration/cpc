use tonic::{Request, Response, Status};
use uuid::Uuid;
use chrono::Utc;
use crate::database::repository::DatabaseRepository;
use crate::database::models::AcademicCredential as DatabaseCredential;

// Import the generated protobuf types
tonic::include_proto!("cpc.learning_platform");

pub struct CredentialService {
    repository: DatabaseRepository,
}

impl CredentialService {
    pub fn new(repository: DatabaseRepository) -> Self {
        Self { repository }
    }
}

#[tonic::async_trait]
impl credential_service_server::CredentialService for CredentialService {
    async fn issue_credential(
        &self,
        request: Request<IssueCredentialRequest>,
    ) -> Result<Response<IssueCredentialResponse>, Status> {
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
        
        // Check if user is enrolled and has completed the course
        let enrollment = self.repository.get_enrollment_by_user_and_course(user_id, course_id).await
            .map_err(|e| Status::internal(format!("Failed to check enrollment: {}", e)))?
            .ok_or_else(|| Status::not_found("User not enrolled in this course"))?;
        
        if enrollment.status != "COMPLETED" {
            return Err(Status::failed_precondition("User has not completed the course"));
        }
        
        // Create credential
        let credential_id = Uuid::new_v4();
        let verification_code = format!("VC-{}", credential_id.to_string()[..8].to_uppercase());
        
        // Convert protobuf credential type to string
        let credential_type = match req.credential_type {
            0 => "CERTIFICATE",
            1 => "BADGE",
            2 => "MICRO_DEGREE",
            3 => "DEGREE",
            _ => return Err(Status::invalid_argument("Invalid credential type")),
        };
        
        let db_credential = DatabaseCredential {
            id: credential_id,
            user_id,
            course_id,
            credential_type: credential_type.to_string(),
            issued_at: Utc::now(),
            verification_code: verification_code.clone(),
            created_at: Utc::now(),
        };
        
        // Save to database
        let saved_credential = self.repository.create_credential(&db_credential).await
            .map_err(|e| Status::internal(format!("Failed to create credential: {}", e)))?;
        
        // Convert to protobuf credential
        let proto_credential = Credential {
            id: saved_credential.id.to_string(),
            user_id: saved_credential.user_id.to_string(),
            course_id: saved_credential.course_id.to_string(),
            credential_type: req.credential_type,
            verification_code: saved_credential.verification_code,
        };
        
        let response = IssueCredentialResponse {
            credential: Some(proto_credential),
        };
        
        Ok(Response::new(response))
    }
}