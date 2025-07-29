use crate::domain::models::{Candidate, Resume};
use crate::domain::errors::RecruitmentError;
use crate::infrastructure::repository::RecruitmentRepository;
use uuid::Uuid;
use std::sync::Arc;

pub struct CandidateService {
    repository: Arc<dyn RecruitmentRepository>,
}

impl CandidateService {
    pub fn new(repository: Arc<dyn RecruitmentRepository>) -> Self {
        CandidateService { repository }
    }
    
    pub async fn create_candidate_profile(
        &self,
        user_id: Uuid,
        headline: Option<String>,
        summary: Option<String>,
        location: Option<String>,
        is_open_to_work: bool,
    ) -> Result<Candidate, RecruitmentError> {
        let candidate = Candidate::new(
            user_id,
            headline,
            summary,
            location,
            is_open_to_work,
        );
        
        self.repository.create_candidate(&candidate).await?;
        Ok(candidate)
    }
    
    pub async fn get_candidate(&self, candidate_id: Uuid) -> Result<Candidate, RecruitmentError> {
        self.repository.get_candidate(candidate_id).await
    }
    
    pub async fn get_candidate_by_user_id(&self, user_id: Uuid) -> Result<Candidate, RecruitmentError> {
        self.repository.get_candidate_by_user_id(user_id).await
    }
    
    pub async fn update_candidate_profile(
        &self,
        candidate_id: Uuid,
        user_id: Uuid,
        headline: Option<String>,
        summary: Option<String>,
        location: Option<String>,
        is_open_to_work: bool,
    ) -> Result<Candidate, RecruitmentError> {
        let mut candidate = self.get_candidate(candidate_id).await?;
        
        // Check if the user owns this candidate profile
        if candidate.user_id != user_id {
            return Err(RecruitmentError::AccessDenied);
        }
        
        candidate.update_profile(headline, summary, location, is_open_to_work)?;
        self.repository.update_candidate(&candidate).await?;
        Ok(candidate)
    }
    
    pub async fn toggle_availability(&self, candidate_id: Uuid, user_id: Uuid) -> Result<Candidate, RecruitmentError> {
        let mut candidate = self.get_candidate(candidate_id).await?;
        
        // Check if the user owns this candidate profile
        if candidate.user_id != user_id {
            return Err(RecruitmentError::AccessDenied);
        }
        
        candidate.toggle_availability()?;
        self.repository.update_candidate(&candidate).await?;
        Ok(candidate)
    }
    
    pub async fn search_candidates(
        &self,
        search_term: Option<String>,
        location: Option<String>,
        is_open_to_work: Option<bool>,
    ) -> Result<Vec<Candidate>, RecruitmentError> {
        self.repository.search_candidates(search_term, location, is_open_to_work).await
    }
    
    pub async fn upload_resume(
        &self,
        candidate_id: Uuid,
        user_id: Uuid,
        document_id: Uuid,
        parsed_content: Option<serde_json::Value>,
    ) -> Result<Resume, RecruitmentError> {
        let candidate = self.get_candidate(candidate_id).await?;
        
        // Check if the user owns this candidate profile
        if candidate.user_id != user_id {
            return Err(RecruitmentError::AccessDenied);
        }
        
        let resume = Resume {
            id: Uuid::new_v4(),
            candidate_id,
            document_id,
            parsed_content,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        self.repository.create_resume(&resume).await?;
        Ok(resume)
    }
    
    pub async fn get_candidate_resumes(&self, candidate_id: Uuid, user_id: Uuid) -> Result<Vec<Resume>, RecruitmentError> {
        let candidate = self.get_candidate(candidate_id).await?;
        
        // Check if the user owns this candidate profile
        if candidate.user_id != user_id {
            return Err(RecruitmentError::AccessDenied);
        }
        
        self.repository.get_resumes_by_candidate(candidate_id).await
    }
}