use crate::domain::models::{Job, Employer};
use crate::domain::value_objects::{SalaryRange, EmploymentType, JobStatus};
use crate::domain::errors::RecruitmentError;
use crate::infrastructure::repository::RecruitmentRepository;
use uuid::Uuid;
use std::sync::Arc;

pub struct JobService {
    repository: Arc<dyn RecruitmentRepository>,
}

impl JobService {
    pub fn new(repository: Arc<dyn RecruitmentRepository>) -> Self {
        JobService { repository }
    }
    
    pub async fn create_job(
        &self,
        employer_id: Uuid,
        title: String,
        description: String,
        location: Option<String>,
        is_remote: bool,
        salary_min: Option<String>,
        salary_max: Option<String>,
        employment_type: String,
    ) -> Result<Job, RecruitmentError> {
        // Validate employer exists
        let _employer = self.repository.get_employer_by_user_id(employer_id).await?;
        
        // Parse employment type
        let emp_type = EmploymentType::from_str(&employment_type)
            .map_err(|e| RecruitmentError::InvalidEmploymentType(e))?;
        
        // Parse salary range
        let salary_range = if salary_min.is_some() || salary_max.is_some() {
            let min_decimal = if let Some(min_str) = salary_min {
                Some(min_str.parse::<rust_decimal::Decimal>()
                    .map_err(|e| RecruitmentError::DecimalError(e))?)
            } else {
                None
            };
            
            let max_decimal = if let Some(max_str) = salary_max {
                Some(max_str.parse::<rust_decimal::Decimal>()
                    .map_err(|e| RecruitmentError::DecimalError(e))?)
            } else {
                None
            };
            
            Some(SalaryRange::new(min_decimal, max_decimal)
                .map_err(|e| RecruitmentError::InvalidSalaryRange(e))?)
        } else {
            None
        };
        
        let job = Job::new(
            employer_id,
            title,
            description,
            location,
            is_remote,
            salary_range,
            emp_type,
        );
        
        self.repository.create_job(&job).await?;
        Ok(job)
    }
    
    pub async fn get_job(&self, job_id: Uuid) -> Result<Job, RecruitmentError> {
        self.repository.get_job(job_id).await
    }
    
    pub async fn update_job(
        &self,
        job_id: Uuid,
        employer_user_id: Uuid,
        title: String,
        description: String,
        location: Option<String>,
        is_remote: bool,
        salary_min: Option<String>,
        salary_max: Option<String>,
        employment_type: String,
    ) -> Result<Job, RecruitmentError> {
        let mut job = self.get_job(job_id).await?;
        
        // Check if the employer owns this job
        let employer = self.repository.get_employer(job.employer_id).await?;
        if employer.user_id != employer_user_id {
            return Err(RecruitmentError::AccessDenied);
        }
        
        // Parse employment type
        let emp_type = EmploymentType::from_str(&employment_type)
            .map_err(|e| RecruitmentError::InvalidEmploymentType(e))?;
        
        // Parse salary range
        let salary_range = if salary_min.is_some() || salary_max.is_some() {
            let min_decimal = if let Some(min_str) = salary_min {
                Some(min_str.parse::<rust_decimal::Decimal>()
                    .map_err(|e| RecruitmentError::DecimalError(e))?)
            } else {
                None
            };
            
            let max_decimal = if let Some(max_str) = salary_max {
                Some(max_str.parse::<rust_decimal::Decimal>()
                    .map_err(|e| RecruitmentError::DecimalError(e))?)
            } else {
                None
            };
            
            Some(SalaryRange::new(min_decimal, max_decimal)
                .map_err(|e| RecruitmentError::InvalidSalaryRange(e))?)
        } else {
            None
        };
        
        // Update job fields
        job.title = title;
        job.description = description;
        job.location = location;
        job.is_remote = is_remote;
        job.salary_range = salary_range;
        job.employment_type = emp_type;
        job.updated_at = chrono::Utc::now();
        
        self.repository.update_job(&job).await?;
        Ok(job)
    }
    
    pub async fn publish_job(&self, job_id: Uuid, employer_user_id: Uuid) -> Result<Job, RecruitmentError> {
        let mut job = self.get_job(job_id).await?;
        
        // Check if the employer owns this job
        let employer = self.repository.get_employer(job.employer_id).await?;
        if employer.user_id != employer_user_id {
            return Err(RecruitmentError::AccessDenied);
        }
        
        job.publish()?;
        self.repository.update_job(&job).await?;
        Ok(job)
    }
    
    pub async fn close_job(&self, job_id: Uuid, employer_user_id: Uuid) -> Result<Job, RecruitmentError> {
        let mut job = self.get_job(job_id).await?;
        
        // Check if the employer owns this job
        let employer = self.repository.get_employer(job.employer_id).await?;
        if employer.user_id != employer_user_id {
            return Err(RecruitmentError::AccessDenied);
        }
        
        job.close()?;
        self.repository.update_job(&job).await?;
        Ok(job)
    }
    
    pub async fn mark_job_as_filled(&self, job_id: Uuid, employer_user_id: Uuid) -> Result<Job, RecruitmentError> {
        let mut job = self.get_job(job_id).await?;
        
        // Check if the employer owns this job
        let employer = self.repository.get_employer(job.employer_id).await?;
        if employer.user_id != employer_user_id {
            return Err(RecruitmentError::AccessDenied);
        }
        
        job.mark_as_filled()?;
        self.repository.update_job(&job).await?;
        Ok(job)
    }
    
    pub async fn search_jobs(
        &self,
        search_term: Option<String>,
        location: Option<String>,
        employment_type: Option<String>,
        is_remote: Option<bool>,
    ) -> Result<Vec<Job>, RecruitmentError> {
        self.repository.search_jobs(search_term, location, employment_type, is_remote).await
    }
    
    pub async fn get_jobs_by_employer(&self, employer_id: Uuid) -> Result<Vec<Job>, RecruitmentError> {
        self.repository.get_jobs_by_employer(employer_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::Employer;
    use std::collections::HashMap;
    use async_trait::async_trait;
    use chrono::{DateTime, Utc};
    use rust_decimal::Decimal;
    
    // Mock repository for testing
    struct MockRepository {
        employers: HashMap<Uuid, Employer>,
        jobs: HashMap<Uuid, Job>,
    }
    
    impl MockRepository {
        fn new() -> Self {
            MockRepository {
                employers: HashMap::new(),
                jobs: HashMap::new(),
            }
        }
    }
    
    #[async_trait]
    impl RecruitmentRepository for MockRepository {
        async fn create_employer(&self, employer: &Employer) -> Result<(), RecruitmentError> {
            // In a real mock, we would store the employer
            Ok(())
        }
        
        async fn get_employer(&self, id: Uuid) -> Result<Employer, RecruitmentError> {
            self.employers.get(&id).cloned().ok_or(RecruitmentError::EmployerNotFound(id.to_string()))
        }
        
        async fn get_employer_by_user_id(&self, user_id: Uuid) -> Result<Employer, RecruitmentError> {
            for employer in self.employers.values() {
                if employer.user_id == user_id {
                    return Ok(employer.clone());
                }
            }
            Err(RecruitmentError::EmployerNotFound(user_id.to_string()))
        }
        
        async fn update_employer(&self, employer: &Employer) -> Result<(), RecruitmentError> {
            Ok(())
        }
        
        async fn create_job(&self, job: &Job) -> Result<(), RecruitmentError> {
            // In a real mock, we would store the job
            Ok(())
        }
        
        async fn get_job(&self, id: Uuid) -> Result<Job, RecruitmentError> {
            self.jobs.get(&id).cloned().ok_or(RecruitmentError::JobNotFound(id.to_string()))
        }
        
        async fn update_job(&self, job: &Job) -> Result<(), RecruitmentError> {
            Ok(())
        }
        
        async fn get_jobs_by_employer(&self, employer_id: Uuid) -> Result<Vec<Job>, RecruitmentError> {
            Ok(self.jobs.values()
                .filter(|job| job.employer_id == employer_id)
                .cloned()
                .collect())
        }
        
        async fn search_jobs(
            &self,
            search_term: Option<String>,
            location: Option<String>,
            employment_type: Option<String>,
            is_remote: Option<bool>,
        ) -> Result<Vec<Job>, RecruitmentError> {
            Ok(vec![])
        }
        
        // Other methods would be implemented for a complete mock
        async fn create_candidate(&self, candidate: &crate::domain::models::Candidate) -> Result<(), RecruitmentError> {
            todo!()
        }
        
        async fn get_candidate(&self, id: Uuid) -> Result<crate::domain::models::Candidate, RecruitmentError> {
            todo!()
        }
        
        async fn get_candidate_by_user_id(&self, user_id: Uuid) -> Result<crate::domain::models::Candidate, RecruitmentError> {
            todo!()
        }
        
        async fn update_candidate(&self, candidate: &crate::domain::models::Candidate) -> Result<(), RecruitmentError> {
            todo!()
        }
        
        async fn search_candidates(
            &self,
            search_term: Option<String>,
            location: Option<String>,
            is_open_to_work: Option<bool>,
        ) -> Result<Vec<crate::domain::models::Candidate>, RecruitmentError> {
            todo!()
        }
        
        async fn create_application(&self, application: &crate::domain::models::Application) -> Result<(), RecruitmentError> {
            todo!()
        }
        
        async fn get_application(&self, id: Uuid) -> Result<crate::domain::models::Application, RecruitmentError> {
            todo!()
        }
        
        async fn get_application_by_job_and_candidate(&self, job_id: Uuid, candidate_id: Uuid) -> Result<crate::domain::models::Application, RecruitmentError> {
            todo!()
        }
        
        async fn update_application(&self, application: &crate::domain::models::Application) -> Result<(), RecruitmentError> {
            todo!()
        }
        
        async fn delete_application(&self, id: Uuid) -> Result<(), RecruitmentError> {
            todo!()
        }
        
        async fn get_applications_by_job(&self, job_id: Uuid) -> Result<Vec<crate::domain::models::Application>, RecruitmentError> {
            todo!()
        }
        
        async fn get_applications_by_candidate(&self, candidate_id: Uuid) -> Result<Vec<crate::domain::models::Application>, RecruitmentError> {
            todo!()
        }
        
        async fn create_resume(&self, resume: &crate::domain::models::Resume) -> Result<(), RecruitmentError> {
            todo!()
        }
        
        async fn get_resumes_by_candidate(&self, candidate_id: Uuid) -> Result<Vec<crate::domain::models::Resume>, RecruitmentError> {
            todo!()
        }
        
        async fn create_interview(&self, interview: &crate::application::interview_service::Interview) -> Result<(), RecruitmentError> {
            todo!()
        }
        
        async fn get_interview(&self, id: Uuid) -> Result<crate::application::interview_service::Interview, RecruitmentError> {
            todo!()
        }
        
        async fn get_interviews_by_application(&self, application_id: Uuid) -> Result<Vec<crate::application::interview_service::Interview>, RecruitmentError> {
            todo!()
        }
        
        async fn update_interview(&self, interview: &crate::application::interview_service::Interview) -> Result<(), RecruitmentError> {
            todo!()
        }
        
        async fn delete_interview(&self, id: Uuid) -> Result<(), RecruitmentError> {
            todo!()
        }
        
        async fn create_job_alert(&self, alert: &crate::infrastructure::repository::JobAlert) -> Result<(), RecruitmentError> {
            todo!()
        }
        
        async fn get_job_alert(&self, id: Uuid) -> Result<crate::infrastructure::repository::JobAlert, RecruitmentError> {
            todo!()
        }
        
        async fn get_job_alerts_by_candidate(&self, candidate_id: Uuid) -> Result<Vec<crate::infrastructure::repository::JobAlert>, RecruitmentError> {
            todo!()
        }
        
        async fn update_job_alert(&self, alert: &crate::infrastructure::repository::JobAlert) -> Result<(), RecruitmentError> {
            todo!()
        }
        
        async fn delete_job_alert(&self, id: Uuid) -> Result<(), RecruitmentError> {
            todo!()
        }
        
        async fn find_matching_job_alerts(&self, job: &Job) -> Result<Vec<crate::infrastructure::repository::JobAlert>, RecruitmentError> {
            todo!()
        }
        
        async fn create_job_alert_match(&self, match_record: &crate::infrastructure::repository::JobAlertMatch) -> Result<(), RecruitmentError> {
            todo!()
        }
    }
    
    #[tokio::test]
    async fn test_create_job() {
        let mock_repo = Arc::new(MockRepository::new());
        let job_service = JobService::new(mock_repo);
        
        let employer_user_id = Uuid::new_v4();
        let result = job_service.create_job(
            employer_user_id,
            "Software Engineer".to_string(),
            "Develop software applications".to_string(),
            Some("San Francisco".to_string()),
            false,
            Some("50000".to_string()),
            Some("80000".to_string()),
            "full_time".to_string(),
        ).await;
        
        // This will fail because the employer doesn't exist in the mock
        // In a real test, we would set up the mock properly
        assert!(result.is_err());
    }
}