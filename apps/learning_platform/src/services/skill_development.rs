use tonic::transport::Channel;
use crate::types::{SkillProgress, Skill, Certification};
use uuid::Uuid;

// Include the generated gRPC client code
tonic::include_proto!("skill_development");

pub struct SkillDevelopmentService {
    client: skill_development_client::SkillDevelopmentClient<Channel>,
}

impl SkillDevelopmentService {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let channel = Channel::from_static("http://localhost:50052")
            .connect()
            .await?;
        Ok(Self {
            client: skill_development_client::SkillDevelopmentClient::new(channel),
        })
    }

    pub async fn track_skill_progress(&mut self, user_id: String, skill_id: String, current_level: u8, target_level: u8) -> Result<SkillProgress, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(TrackSkillProgressRequest {
            user_id,
            skill_id,
            current_level: current_level as i32,
            target_level: target_level as i32,
        });
        
        let response = self.client.track_skill_progress(request).await?;
        let proto_progress = response.into_inner();
        
        Ok(SkillProgress {
            id: proto_progress.id,
            skill_name: proto_progress.skill_name,
            current_level: proto_progress.current_level as u8,
            target_level: proto_progress.target_level as u8,
            progress_percentage: proto_progress.progress_percentage,
            total_hours_invested: proto_progress.total_hours_invested,
            last_practice_date: proto_progress.last_practice_date,
        })
    }

    pub async fn get_user_skill_progress(&mut self, user_id: String) -> Result<Vec<SkillProgress>, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(GetUserSkillProgressRequest {
            user_id,
        });
        
        let response = self.client.get_user_skill_progress(request).await?;
        let proto_response = response.into_inner();
        
        let skills: Vec<SkillProgress> = proto_response.skills
            .into_iter()
            .map(|proto_skill| SkillProgress {
                id: proto_skill.id,
                skill_name: proto_skill.skill_name,
                current_level: proto_skill.current_level as u8,
                target_level: proto_skill.target_level as u8,
                progress_percentage: proto_skill.progress_percentage,
                total_hours_invested: proto_skill.total_hours_invested,
                last_practice_date: proto_skill.last_practice_date,
            })
            .collect();
        
        Ok(skills)
    }

    pub async fn get_user_certifications(&mut self, user_id: String) -> Result<Vec<Certification>, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(GetUserCertificationsRequest {
            user_id,
        });
        
        let response = self.client.get_user_certifications(request).await?;
        let proto_response = response.into_inner();
        
        let certifications: Vec<Certification> = proto_response.certifications
            .into_iter()
            .map(|proto_cert| Certification {
                id: proto_cert.id,
                skill_name: proto_cert.skill_name,
                level_achieved: proto_cert.level_achieved as u8,
                certification_type: proto_cert.certification_type,
                issued_at: proto_cert.issued_at,
                verification_code: proto_cert.verification_code,
            })
            .collect();
        
        Ok(certifications)
    }
}