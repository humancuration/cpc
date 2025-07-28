use crate::domain::models::Application;
use crate::domain::errors::RecruitmentError;
use crate::infrastructure::repository::{RecruitmentRepository, Interview};
use uuid::Uuid;
use std::sync::Arc;
use chrono::{DateTime, Utc};

pub struct InterviewService {
    repository: Arc<dyn RecruitmentRepository>,
}

impl InterviewService {
    pub fn new(repository: Arc<dyn RecruitmentRepository>) -> Self {
        InterviewService { repository }
    }
    
    pub async fn schedule_interview(
        &self,
        application_id: Uuid,
        employer_user_id: Uuid,
        scheduled_time: DateTime<Utc>,
        location: Option<String>,
        meeting_link: Option<String>,
        notes: Option<String>,
    ) -> Result<Interview, RecruitmentError> {
        // Verify the application exists and employer has access
        let application = self.repository.get_application(application_id).await?;
        let job = self.repository.get_job(application.job_id).await?;
        let employer = self.repository.get_employer(job.employer_id).await?;
        if employer.user_id != employer_user_id {
            return Err(RecruitmentError::AccessDenied);
        }
        
        let interview = Interview::new(
            application_id,
            scheduled_time,
            location,
            meeting_link,
            notes,
        );
        
        self.repository.create_interview(&interview).await?;
        Ok(interview)
    }
    
    pub async fn get_interview(&self, interview_id: Uuid, user_id: Uuid) -> Result<Interview, RecruitmentError> {
        let interview = self.repository.get_interview(interview_id).await?;
        
        // Check if user has access to this interview
        let application = self.repository.get_application(interview.application_id).await?;
        let candidate = self.repository.get_candidate(application.candidate_id).await?;
        let job = self.repository.get_job(application.job_id).await?;
        let employer = self.repository.get_employer(job.employer_id).await?;
        
        if candidate.user_id != user_id && employer.user_id != user_id {
            return Err(RecruitmentError::AccessDenied);
        }
        
        Ok(interview)
    }
    
    pub async fn get_interviews_by_application(&self, application_id: Uuid, user_id: Uuid) -> Result<Vec<Interview>, RecruitmentError> {
        // Verify the user has access to this application
        let application = self.repository.get_application(application_id).await?;
        let candidate = self.repository.get_candidate(application.candidate_id).await?;
        let job = self.repository.get_job(application.job_id).await?;
        let employer = self.repository.get_employer(job.employer_id).await?;
        
        if candidate.user_id != user_id && employer.user_id != user_id {
            return Err(RecruitmentError::AccessDenied);
        }
        
        self.repository.get_interviews_by_application(application_id).await
    }
    
    pub async fn update_interview(
        &self,
        interview_id: Uuid,
        employer_user_id: Uuid,
        scheduled_time: Option<DateTime<Utc>>,
        location: Option<String>,
        meeting_link: Option<String>,
        notes: Option<String>,
    ) -> Result<Interview, RecruitmentError> {
        let mut interview = self.get_interview(interview_id, employer_user_id).await?;
        
        // Verify the employer has access to this interview
        let application = self.repository.get_application(interview.application_id).await?;
        let job = self.repository.get_job(application.job_id).await?;
        let employer = self.repository.get_employer(job.employer_id).await?;
        if employer.user_id != employer_user_id {
            return Err(RecruitmentError::AccessDenied);
        }
        
        if let Some(time) = scheduled_time {
            interview.scheduled_time = time;
        }
        if location.is_some() {
            interview.location = location;
        }
        if meeting_link.is_some() {
            interview.meeting_link = meeting_link;
        }
        if notes.is_some() {
            interview.notes = notes;
        }
        interview.updated_at = chrono::Utc::now();
        
        self.repository.update_interview(&interview).await?;
        Ok(interview)
    }
    
    pub async fn cancel_interview(
        &self,
        interview_id: Uuid,
        employer_user_id: Uuid,
    ) -> Result<(), RecruitmentError> {
        let interview = self.get_interview(interview_id, employer_user_id).await?;
        
        // Verify the employer has access to this interview
        let application = self.repository.get_application(interview.application_id).await?;
        let job = self.repository.get_job(application.job_id).await?;
        let employer = self.repository.get_employer(job.employer_id).await?;
        if employer.user_id != employer_user_id {
            return Err(RecruitmentError::AccessDenied);
        }
        
        self.repository.delete_interview(interview_id).await?;
        Ok(())
    }
}