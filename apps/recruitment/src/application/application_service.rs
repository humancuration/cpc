use crate::domain::models::{Application, Job, Candidate};
use crate::domain::value_objects::ApplicationStatus;
use crate::domain::errors::RecruitmentError;
use crate::infrastructure::repository::RecruitmentRepository;
use uuid::Uuid;
use std::sync::Arc;

pub struct ApplicationService {
    repository: Arc<dyn RecruitmentRepository>,
}

impl ApplicationService {
    pub fn new(repository: Arc<dyn RecruitmentRepository>) -> Self {
        ApplicationService { repository }
    }
    
    pub async fn submit_application(
        &self,
        job_id: Uuid,
        candidate_id: Uuid,
        user_id: Uuid,
        cover_letter: Option<String>,
    ) -> Result<Application, RecruitmentError> {
        // Verify the candidate belongs to the user
        let candidate = self.repository.get_candidate(candidate_id).await?;
        if candidate.user_id != user_id {
            return Err(RecruitmentError::AccessDenied);
        }
        
        // Verify the job exists and is published
        let job = self.repository.get_job(job_id).await?;
        if job.status != crate::domain::models::JobStatus::Published {
            return Err(RecruitmentError::InvalidJobStatus("Cannot apply to unpublished job".to_string()));
        }
        
        // Check if application already exists
        if let Ok(existing_app) = self.repository.get_application_by_job_and_candidate(job_id, candidate_id).await {
            return Err(RecruitmentError::ApplicationNotFound(format!(
                "Application already exists for job {} and candidate {}", 
                job_id, candidate_id
            )));
        }
        
        let application = Application::new(
            job_id,
            candidate_id,
            cover_letter,
        );
        
        self.repository.create_application(&application).await?;
        Ok(application)
    }
    
    pub async fn get_application(&self, application_id: Uuid, user_id: Uuid) -> Result<Application, RecruitmentError> {
        let application = self.repository.get_application(application_id).await?;
        
        // Check if user has access to this application
        let candidate = self.repository.get_candidate(application.candidate_id).await?;
        let job = self.repository.get_job(application.job_id).await?;
        let employer = self.repository.get_employer(job.employer_id).await?;
        
        if candidate.user_id != user_id && employer.user_id != user_id {
            return Err(RecruitmentError::AccessDenied);
        }
        
        Ok(application)
    }
    
    pub async fn get_applications_by_job(&self, job_id: Uuid, employer_user_id: Uuid) -> Result<Vec<Application>, RecruitmentError> {
        // Verify the job belongs to the employer
        let job = self.repository.get_job(job_id).await?;
        let employer = self.repository.get_employer(job.employer_id).await?;
        if employer.user_id != employer_user_id {
            return Err(RecruitmentError::AccessDenied);
        }
        
        self.repository.get_applications_by_job(job_id).await
    }
    
    pub async fn get_applications_by_candidate(&self, candidate_id: Uuid, user_id: Uuid) -> Result<Vec<Application>, RecruitmentError> {
        // Verify the candidate belongs to the user
        let candidate = self.repository.get_candidate(candidate_id).await?;
        if candidate.user_id != user_id {
            return Err(RecruitmentError::AccessDenied);
        }
        
        self.repository.get_applications_by_candidate(candidate_id).await
    }
    
    pub async fn update_application_status(
        &self,
        application_id: Uuid,
        employer_user_id: Uuid,
        status: String,
    ) -> Result<Application, RecruitmentError> {
        let mut application = self.get_application(application_id, employer_user_id).await?;
        
        // Verify the employer has access to this application
        let job = self.repository.get_job(application.job_id).await?;
        let employer = self.repository.get_employer(job.employer_id).await?;
        if employer.user_id != employer_user_id {
            return Err(RecruitmentError::AccessDenied);
        }
        
        let app_status = ApplicationStatus::from_str(&status)
            .map_err(|e| RecruitmentError::InvalidApplicationStatus(e))?;
        
        application.update_status(app_status)?;
        self.repository.update_application(&application).await?;
        Ok(application)
    }
    
    pub async fn withdraw_application(
        &self,
        application_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), RecruitmentError> {
        let application = self.get_application(application_id, user_id).await?;
        
        // Verify the candidate owns this application
        let candidate = self.repository.get_candidate(application.candidate_id).await?;
        if candidate.user_id != user_id {
            return Err(RecruitmentError::AccessDenied);
        }
        
        self.repository.delete_application(application_id).await?;
        Ok(())
    }
}