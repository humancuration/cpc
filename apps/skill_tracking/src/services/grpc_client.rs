use tonic::transport::Channel;
use uuid::Uuid;

use crate::types::{SkillProgressData, LearningPathData, CertificationData, SkillLevel, DifficultyLevel, CertificationType};

tonic::include_proto!("skill_development");

pub struct SkillDevelopmentClient {
    client: skill_development_client::SkillDevelopmentClient<Channel>,
}

impl SkillDevelopmentClient {
    pub async fn new(base_url: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = skill_development_client::SkillDevelopmentClient::connect(base_url).await?;
        Ok(Self { client })
    }

    pub async fn track_skill_progress(
        &mut self,
        user_id: Uuid,
        skill_id: Uuid,
        current_level: i32,
        target_level: i32,
    ) -> Result<SkillProgressData, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(TrackSkillProgressRequest {
            user_id: user_id.to_string(),
            skill_id: skill_id.to_string(),
            current_level,
            target_level,
        });

        let response = self.client.track_skill_progress(request).await?;
        let progress = response.into_inner();

        // Convert current_level to SkillLevel
        let current_level_enum = match progress.current_level {
            0..=19 => SkillLevel::Beginner,
            20..=39 => SkillLevel::Intermediate,
            40..=59 => SkillLevel::Advanced,
            60..=79 => SkillLevel::Expert,
            _ => SkillLevel::Master,
        };

        // Convert target_level to SkillLevel
        let target_level_enum = match progress.target_level {
            0..=19 => SkillLevel::Beginner,
            20..=39 => SkillLevel::Intermediate,
            40..=59 => SkillLevel::Advanced,
            60..=79 => SkillLevel::Expert,
            _ => SkillLevel::Master,
        };

        Ok(SkillProgressData {
            id: Uuid::parse_str(&progress.id)?,
            skill_name: progress.skill_name,
            current_level: current_level_enum,
            target_level: target_level_enum,
            progress_percentage: progress.progress_percentage,
            total_hours_invested: progress.total_hours_invested as u32,
            last_practice_date: Some(progress.last_practice_date),
        })
    }

    pub async fn create_learning_path(
        &mut self,
        title: String,
        description: String,
        creator_id: Uuid,
        difficulty_level: i32,
    ) -> Result<LearningPathData, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(CreateLearningPathRequest {
            title,
            description,
            creator_id: creator_id.to_string(),
            difficulty_level,
        });

        let response = self.client.create_learning_path(request).await?;
        let path = response.into_inner();

        // Convert difficulty_level to DifficultyLevel
        let difficulty_enum = match path.difficulty_level {
            0 => DifficultyLevel::Beginner,
            1 => DifficultyLevel::Intermediate,
            _ => DifficultyLevel::Advanced,
        };

        Ok(LearningPathData {
            id: Uuid::parse_str(&path.id)?,
            title: path.title,
            description: path.description,
            estimated_duration_hours: path.estimated_duration_hours as u32,
            difficulty_level: difficulty_enum,
            progress: path.progress_percentage,
        })
    }

    pub async fn issue_certification(
        &mut self,
        user_id: Uuid,
        skill_id: Uuid,
        certification_type: i32,
        level_achieved: i32,
        issued_by: Uuid,
    ) -> Result<CertificationData, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(IssueCertificationRequest {
            user_id: user_id.to_string(),
            skill_id: skill_id.to_string(),
            certification_type,
            level_achieved,
            issued_by: issued_by.to_string(),
        });

        let response = self.client.issue_certification(request).await?;
        let cert = response.into_inner();

        // Convert level_achieved to SkillLevel
        let level_enum = match cert.level_achieved {
            0 => SkillLevel::Beginner,
            1 => SkillLevel::Intermediate,
            2 => SkillLevel::Advanced,
            3 => SkillLevel::Expert,
            _ => SkillLevel::Master,
        };

        // Convert certification_type to CertificationType
        let cert_type_enum = match cert.certification_type {
            0 => CertificationType::CourseCompletion,
            1 => CertificationType::PeerEndorsement,
            2 => CertificationType::SkillAssessment,
            3 => CertificationType::ProjectReview,
            _ => CertificationType::PortfolioReview,
        };

        Ok(CertificationData {
            id: Uuid::parse_str(&cert.id)?,
            skill_name: cert.skill_name,
            level_achieved: level_enum,
            certification_type: cert_type_enum,
            issued_at: cert.issued_at,
            verification_code: cert.verification_code,
        })
    }

    pub async fn get_user_skill_progress(
        &mut self,
        user_id: Uuid,
    ) -> Result<Vec<SkillProgressData>, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(GetUserSkillProgressRequest {
            user_id: user_id.to_string(),
        });

        let response = self.client.get_user_skill_progress(request).await?;
        let progress_response = response.into_inner();

        let mut progress_list = Vec::new();
        for progress in progress_response.skills {
            // Convert current_level to SkillLevel
            let current_level_enum = match progress.current_level {
                0..=19 => SkillLevel::Beginner,
                20..=39 => SkillLevel::Intermediate,
                40..=59 => SkillLevel::Advanced,
                60..=79 => SkillLevel::Expert,
                _ => SkillLevel::Master,
            };

            // Convert target_level to SkillLevel
            let target_level_enum = match progress.target_level {
                0..=19 => SkillLevel::Beginner,
                20..=39 => SkillLevel::Intermediate,
                40..=59 => SkillLevel::Advanced,
                60..=79 => SkillLevel::Expert,
                _ => SkillLevel::Master,
            };

            progress_list.push(SkillProgressData {
                id: Uuid::parse_str(&progress.id)?,
                skill_name: progress.skill_name,
                current_level: current_level_enum,
                target_level: target_level_enum,
                progress_percentage: progress.progress_percentage,
                total_hours_invested: progress.total_hours_invested as u32,
                last_practice_date: Some(progress.last_practice_date),
            });
        }

        Ok(progress_list)
    }

    pub async fn get_user_certifications(
        &mut self,
        user_id: Uuid,
    ) -> Result<Vec<CertificationData>, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(GetUserCertificationsRequest {
            user_id: user_id.to_string(),
        });

        let response = self.client.get_user_certifications(request).await?;
        let cert_response = response.into_inner();

        let mut cert_list = Vec::new();
        for cert in cert_response.certifications {
            // Convert level_achieved to SkillLevel
            let level_enum = match cert.level_achieved {
                0 => SkillLevel::Beginner,
                1 => SkillLevel::Intermediate,
                2 => SkillLevel::Advanced,
                3 => SkillLevel::Expert,
                _ => SkillLevel::Master,
            };

            // Convert certification_type to CertificationType
            let cert_type_enum = match cert.certification_type {
                0 => CertificationType::CourseCompletion,
                1 => CertificationType::PeerEndorsement,
                2 => CertificationType::SkillAssessment,
                3 => CertificationType::ProjectReview,
                _ => CertificationType::PortfolioReview,
            };

            cert_list.push(CertificationData {
                id: Uuid::parse_str(&cert.id)?,
                skill_name: cert.skill_name,
                level_achieved: level_enum,
                certification_type: cert_type_enum,
                issued_at: cert.issued_at,
                verification_code: cert.verification_code,
            });
        }

        Ok(cert_list)
    }
}