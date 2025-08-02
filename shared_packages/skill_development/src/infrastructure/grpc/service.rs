use tonic::{Request, Response, Status};
use uuid::Uuid;
use chrono::NaiveDate;

use crate::application::{SkillTrackingService, LearningPathService, CertificationService};
use crate::domain::{CertificationType};

tonic::include_proto!("skill_development");

pub struct SkillDevelopmentServiceImpl {
    skill_tracking_service: SkillTrackingService,
    learning_path_service: LearningPathService,
    certification_service: CertificationService,
}

impl SkillDevelopmentServiceImpl {
    pub fn new(
        skill_tracking_service: SkillTrackingService,
        learning_path_service: LearningPathService,
        certification_service: CertificationService,
    ) -> Self {
        Self {
            skill_tracking_service,
            learning_path_service,
            certification_service,
        }
    }
}

#[tonic::async_trait]
impl skill_development_server::SkillDevelopment for SkillDevelopmentServiceImpl {
    async fn track_skill_progress(
        &self,
        request: Request<TrackSkillProgressRequest>,
    ) -> Result<Response<TrackSkillProgressResponse>, Status> {
        let req = request.into_inner();
        
        let user_id = Uuid::parse_str(&req.user_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid user ID: {}", e)))?;
            
        let skill_id = Uuid::parse_str(&req.skill_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid skill ID: {}", e)))?;

        let progress = self.skill_tracking_service
            .track_skill_progress(skill_id, user_id, req.current_level as u8)
            .await
            .map_err(|e| Status::internal(format!("Failed to track skill progress: {}", e)))?;

        let response = TrackSkillProgressResponse {
            id: progress.id.to_string(),
            skill_name: "TODO: Get skill name".to_string(), // Would need to fetch skill name
            current_level: progress.progress as i32,
            target_level: req.target_level,
            progress_percentage: progress.progress as f32,
            total_hours_invested: 0, // This would need to be calculated
            last_practice_date: chrono::Utc::now().to_rfc3339(),
        };

        Ok(Response::new(response))
    }

    async fn create_learning_path(
        &self,
        request: Request<CreateLearningPathRequest>,
    ) -> Result<Response<CreateLearningPathResponse>, Status> {
        let req = request.into_inner();
        
        let creator_id = Uuid::parse_str(&req.creator_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid creator ID: {}", e)))?;

        let path = self.learning_path_service
            .create_learning_path(
                creator_id,
                Uuid::nil(), // TODO: Parse skill_id from request
                req.title,
                Some(req.description),
            )
            .await
            .map_err(|e| Status::internal(format!("Failed to create learning path: {}", e)))?;

        let response = CreateLearningPathResponse {
            id: path.id.to_string(),
            title: path.title,
            description: path.description.unwrap_or_default(),
            estimated_duration_hours: 0, // This would need to be calculated
            difficulty_level: req.difficulty_level,
            progress_percentage: path.progress_percentage(),
        };

        Ok(Response::new(response))
    }

    async fn issue_certification(
        &self,
        request: Request<IssueCertificationRequest>,
    ) -> Result<Response<IssueCertificationResponse>, Status> {
        let req = request.into_inner();
        
        let user_id = Uuid::parse_str(&req.user_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid user ID: {}", e)))?;
            
        let skill_id = Uuid::parse_str(&req.skill_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid skill ID: {}", e)))?;

        let issued_by = Uuid::parse_str(&req.issued_by)
            .map_err(|e| Status::invalid_argument(format!("Invalid issued by ID: {}", e)))?;

        // Parse certification type
        let certification_type = match req.certification_type {
            0 => CertificationType::CourseCompletion,
            1 => CertificationType::PeerEndorsement,
            2 => CertificationType::SkillAssessment,
            3 => CertificationType::ProjectReview,
            4 => CertificationType::PortfolioReview,
            _ => CertificationType::SkillAssessment,
        };

        let cert = self.certification_service
            .issue_certification(
                "TODO: Get skill name".to_string(), // Would need to fetch skill name
                "TODO: Get organization".to_string(), // Would need to determine issuing org
                chrono::Utc::now().date_naive(),
                user_id,
                Some(skill_id),
                certification_type,
                Some(req.level_achieved as u8),
            )
            .await
            .map_err(|e| Status::internal(format!("Failed to issue certification: {}", e)))?;

        let response = IssueCertificationResponse {
            id: cert.id.to_string(),
            skill_name: "TODO: Get skill name".to_string(), // Would need to fetch skill name
            level_achieved: cert.level_achieved.unwrap_or(0) as i32,
            certification_type: req.certification_type,
            issued_at: chrono::Utc::now().to_rfc3339(),
            verification_code: cert.verification_code,
        };

        Ok(Response::new(response))
    }

    async fn get_user_skill_progress(
        &self,
        request: Request<GetUserSkillProgressRequest>,
    ) -> Result<Response<GetUserSkillProgressResponse>, Status> {
        let req = request.into_inner();
        
        let user_id = Uuid::parse_str(&req.user_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid user ID: {}", e)))?;

        let progress_list = self.skill_tracking_service
            .get_user_skill_progress(user_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to get user skill progress: {}", e)))?;

        let skills: Vec<TrackSkillProgressResponse> = progress_list
            .into_iter()
            .map(|progress| TrackSkillProgressResponse {
                id: progress.id.to_string(),
                skill_name: "TODO: Get skill name".to_string(), // Would need to fetch skill name
                current_level: progress.progress as i32,
                target_level: 100, // Default target
                progress_percentage: progress.progress as f32,
                total_hours_invested: 0, // This would need to be calculated
                last_practice_date: progress.updated_at.to_rfc3339(),
            })
            .collect();

        let response = GetUserSkillProgressResponse { skills };

        Ok(Response::new(response))
    }

    async fn get_user_certifications(
        &self,
        request: Request<GetUserCertificationsRequest>,
    ) -> Result<Response<GetUserCertificationsResponse>, Status> {
        let req = request.into_inner();
        
        let user_id = Uuid::parse_str(&req.user_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid user ID: {}", e)))?;

        let certs = self.certification_service
            .get_user_certifications(user_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to get user certifications: {}", e)))?;

        let certifications: Vec<IssueCertificationResponse> = certs
            .into_iter()
            .map(|cert| IssueCertificationResponse {
                id: cert.id.to_string(),
                skill_name: "TODO: Get skill name".to_string(), // Would need to fetch skill name
                level_achieved: cert.level_achieved.unwrap_or(0) as i32,
                certification_type: match cert.certification_type {
                    CertificationType::CourseCompletion => 0,
                    CertificationType::PeerEndorsement => 1,
                    CertificationType::SkillAssessment => 2,
                    CertificationType::ProjectReview => 3,
                    CertificationType::PortfolioReview => 4,
                },
                issued_at: cert.created_at.to_rfc3339(),
                verification_code: cert.verification_code,
            })
            .collect();

        let response = GetUserCertificationsResponse { certifications };

        Ok(Response::new(response))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;
    use std::env;

    async fn setup_test_db() -> sqlx::PgPool {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://skill_dev_user:secure_password@localhost/skill_dev_test_db".to_string());
        
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to connect to test database")
    }

    #[tokio::test]
    async fn test_track_skill_progress() {
        let pool = setup_test_db().await;
        
        // Run migrations
        sqlx::migrate!("../../../migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        let skill_tracking_service = SkillTrackingService::new(pool.clone());
        let learning_path_service = LearningPathService::new(pool.clone());
        let certification_service = CertificationService::new(pool.clone());

        let service = SkillDevelopmentServiceImpl::new(
            skill_tracking_service,
            learning_path_service,
            certification_service,
        );

        // First, add a skill directly to the database for testing
        let mut skill_repo = crate::infrastructure::repositories::postgres_repo::PostgresRepository::new(pool);
        let skill = skill_repo.add_skill(
            "Rust Programming".to_string(),
            Some("Master Rust programming language".to_string())
        ).await.expect("Failed to add skill");

        // Test track_skill_progress
        let request = Request::new(TrackSkillProgressRequest {
            user_id: Uuid::new_v4().to_string(),
            skill_id: skill.id.to_string(),
            current_level: 75,
            target_level: 100,
        });

        let response = service.track_skill_progress(request).await;
        assert!(response.is_ok());
    }
}