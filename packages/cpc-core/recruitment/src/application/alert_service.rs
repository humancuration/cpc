use crate::domain::models::{Job, Candidate};
use crate::domain::errors::RecruitmentError;
use crate::infrastructure::repository::{RecruitmentRepository, JobAlert, JobAlertMatch, JobAlertCriteria};
use uuid::Uuid;
use std::sync::Arc;
use chrono;

pub struct AlertService {
    repository: Arc<dyn RecruitmentRepository>,
}

impl AlertService {
    pub fn new(repository: Arc<dyn RecruitmentRepository>) -> Self {
        AlertService { repository }
    }
    
    pub async fn create_job_alert(
        &self,
        candidate_id: Uuid,
        user_id: Uuid,
        search_criteria: JobAlertCriteria,
    ) -> Result<JobAlert, RecruitmentError> {
        // Verify the candidate belongs to the user
        let candidate = self.repository.get_candidate(candidate_id).await?;
        if candidate.user_id != user_id {
            return Err(RecruitmentError::AccessDenied);
        }
        
        let alert = JobAlert::new(
            candidate_id,
            search_criteria,
        );
        
        self.repository.create_job_alert(&alert).await?;
        Ok(alert)
    }
    
    pub async fn get_job_alerts_for_candidate(&self, candidate_id: Uuid, user_id: Uuid) -> Result<Vec<JobAlert>, RecruitmentError> {
        // Verify the candidate belongs to the user
        let candidate = self.repository.get_candidate(candidate_id).await?;
        if candidate.user_id != user_id {
            return Err(RecruitmentError::AccessDenied);
        }
        
        self.repository.get_job_alerts_by_candidate(candidate_id).await
    }
    
    pub async fn update_job_alert(
        &self,
        alert_id: Uuid,
        user_id: Uuid,
        search_criteria: JobAlertCriteria,
    ) -> Result<JobAlert, RecruitmentError> {
        let mut alert = self.repository.get_job_alert(alert_id).await?;
        
        // Verify the candidate belongs to the user
        let candidate = self.repository.get_candidate(alert.candidate_id).await?;
        if candidate.user_id != user_id {
            return Err(RecruitmentError::AccessDenied);
        }
        
        alert.search_criteria = search_criteria;
        alert.updated_at = chrono::Utc::now();
        
        self.repository.update_job_alert(&alert).await?;
        Ok(alert)
    }
    
    pub async fn delete_job_alert(
        &self,
        alert_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), RecruitmentError> {
        let alert = self.repository.get_job_alert(alert_id).await?;
        
        // Verify the candidate belongs to the user
        let candidate = self.repository.get_candidate(alert.candidate_id).await?;
        if candidate.user_id != user_id {
            return Err(RecruitmentError::AccessDenied);
        }
        
        self.repository.delete_job_alert(alert_id).await?;
        Ok(())
    }
    
    pub async fn check_and_send_job_alerts(&self, new_job: &Job) -> Result<Vec<JobAlertMatch>, RecruitmentError> {
        // Find matching job alerts
        let matching_alerts = self.repository.find_matching_job_alerts(new_job).await?;
        
        let mut matches = Vec::new();
        
        for alert in matching_alerts {
            // Create match record
            let match_record = JobAlertMatch {
                id: Uuid::new_v4(),
                job_alert_id: alert.id,
                job_id: new_job.id,
                matched_at: chrono::Utc::now(),
            };
            
            self.repository.create_job_alert_match(&match_record).await?;
            matches.push(match_record);
            
            // TODO: Send notification to candidate about the matching job
            // This would integrate with the notification system
        }
        
        Ok(matches)
    }
}