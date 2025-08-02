use tonic::transport::Channel;
use crate::types::{
    Course, Enrollment, AcademicCredential, Tip,
    CourseRequest, EnrollmentRequest, ProgressUpdateRequest, 
    CredentialRequest, TipRequest
};

// Include the generated gRPC client code
tonic::include_proto!("learning_platform");

pub struct PlatformService {
    client: learning_platform_client::LearningPlatformClient<Channel>,
}

impl PlatformService {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let channel = Channel::from_static("http://localhost:50051")
            .connect()
            .await?;
        Ok(Self {
            client: learning_platform_client::LearningPlatformClient::new(channel),
        })
    }

    pub async fn create_course(&mut self, title: String, description: String, creator_id: String) -> Result<Course, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(CourseRequest {
            title,
            description,
            creator_id,
        });
        
        let response = self.client.create_course(request).await?;
        let proto_course = response.into_inner().course.unwrap();
        
        Ok(Course {
            id: proto_course.id,
            title: proto_course.title,
            description: proto_course.description,
            creator_id: proto_course.creator_id,
            modules: vec![], // In a full implementation, we would convert modules
        })
    }

    pub async fn enroll_user(&mut self, user_id: String, course_id: String) -> Result<Enrollment, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(EnrollmentRequest {
            user_id,
            course_id,
        });
        
        let response = self.client.enroll_user(request).await?;
        let proto_enrollment = response.into_inner().enrollment.unwrap();
        
        Ok(Enrollment {
            id: proto_enrollment.id,
            user_id: proto_enrollment.user_id,
            course_id: proto_enrollment.course_id,
            progress: proto_enrollment.progress,
            status: proto_enrollment.status,
        })
    }

    pub async fn update_progress(&mut self, enrollment_id: String, progress_delta: f32) -> Result<Enrollment, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(ProgressUpdateRequest {
            enrollment_id,
            progress_delta,
        });
        
        let response = self.client.update_progress(request).await?;
        let proto_enrollment = response.into_inner().enrollment.unwrap();
        
        Ok(Enrollment {
            id: proto_enrollment.id,
            user_id: proto_enrollment.user_id,
            course_id: proto_enrollment.course_id,
            progress: proto_enrollment.progress,
            status: proto_enrollment.status,
        })
    }

    pub async fn issue_credential(&mut self, user_id: String, course_id: String, credential_type: i32) -> Result<AcademicCredential, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(CredentialRequest {
            user_id,
            course_id,
            credential_type,
        });
        
        let response = self.client.issue_credential(request).await?;
        let proto_credential = response.into_inner().credential.unwrap();
        
        Ok(AcademicCredential {
            id: proto_credential.id,
            user_id: proto_credential.user_id,
            course_id: proto_credential.course_id,
            credential_type: proto_credential.credential_type,
            issued_at: proto_credential.issued_at.unwrap_or_default(),
            verification_code: proto_credential.verification_code,
        })
    }

    pub async fn tip_educator(&mut self, from_user_id: String, to_user_id: String, amount: f64, currency: String, course_id: Option<String>) -> Result<Tip, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(TipRequest {
            from_user_id,
            to_user_id,
            course_id,
            amount,
            currency,
        });
        
        let response = self.client.tip_educator(request).await?;
        let proto_tip = response.into_inner().tip.unwrap();
        
        Ok(Tip {
            id: proto_tip.id,
            from_user_id: proto_tip.from_user_id,
            to_user_id: proto_tip.to_user_id,
            course_id: proto_tip.course_id,
            amount: proto_tip.amount,
            currency: proto_tip.currency,
            created_at: proto_tip.created_at.unwrap_or_default(),
        })
    }
}