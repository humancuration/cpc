use crate::domain::models::{Employer, Job, Candidate, Application, Resume, DataSharingSettings};
use crate::domain::value_objects::{EmploymentType, ApplicationStatus};
use crate::domain::errors::RecruitmentError;
use uuid::Uuid;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct RequestContext {
    pub requesting_node: Option<Uuid>,
    pub consistency_level: ConsistencyLevel,
}

#[derive(Debug, Clone)]
pub enum ConsistencyLevel {
    Strong,
    Eventual,
    Local,
}

#[async_trait]
pub trait RecruitmentRepository: Send + Sync {
    // Employer methods
    async fn create_employer(&self, employer: &Employer, context: RequestContext) -> Result<(), RecruitmentError>;
    async fn get_employer(&self, id: Uuid, context: RequestContext) -> Result<Employer, RecruitmentError>;
    async fn get_employer_by_user_id(&self, user_id: Uuid, context: RequestContext) -> Result<Employer, RecruitmentError>;
    async fn update_employer(&self, employer: &Employer, context: RequestContext) -> Result<(), RecruitmentError>;
    
    // Job methods
    async fn create_job(&self, job: &Job, context: RequestContext) -> Result<(), RecruitmentError>;
    async fn get_job(&self, id: Uuid, context: RequestContext) -> Result<Job, RecruitmentError>;
    async fn update_job(&self, job: &Job, context: RequestContext) -> Result<(), RecruitmentError>;
    async fn get_jobs_by_employer(&self, employer_id: Uuid, context: RequestContext) -> Result<Vec<Job>, RecruitmentError>;
    async fn search_jobs(
        &self,
        search_term: Option<String>,
        location: Option<String>,
        employment_type: Option<String>,
        is_remote: Option<bool>,
        context: RequestContext,
    ) -> Result<Vec<Job>, RecruitmentError>;
    
    // Candidate methods
    async fn create_candidate(&self, candidate: &Candidate, context: RequestContext) -> Result<(), RecruitmentError>;
    async fn get_candidate(&self, id: Uuid, context: RequestContext) -> Result<Candidate, RecruitmentError>;
    async fn get_candidate_by_user_id(&self, user_id: Uuid, context: RequestContext) -> Result<Candidate, RecruitmentError>;
    async fn update_candidate(&self, candidate: &Candidate, context: RequestContext) -> Result<(), RecruitmentError>;
    async fn search_candidates(
        &self,
        search_term: Option<String>,
        location: Option<String>,
        is_open_to_work: Option<bool>,
        context: RequestContext,
    ) -> Result<Vec<Candidate>, RecruitmentError>;
    
    // Application methods
    async fn create_application(&self, application: &Application, context: RequestContext) -> Result<(), RecruitmentError>;
    async fn get_application(&self, id: Uuid, context: RequestContext) -> Result<Application, RecruitmentError>;
    async fn get_application_by_job_and_candidate(&self, job_id: Uuid, candidate_id: Uuid, context: RequestContext) -> Result<Application, RecruitmentError>;
    async fn update_application(&self, application: &Application, context: RequestContext) -> Result<(), RecruitmentError>;
    async fn delete_application(&self, id: Uuid, context: RequestContext) -> Result<(), RecruitmentError>;
    async fn get_applications_by_job(&self, job_id: Uuid, context: RequestContext) -> Result<Vec<Application>, RecruitmentError>;
    async fn get_applications_by_candidate(&self, candidate_id: Uuid, context: RequestContext) -> Result<Vec<Application>, RecruitmentError>;
    
    // Resume methods
    async fn create_resume(&self, resume: &Resume, context: RequestContext) -> Result<(), RecruitmentError>;
    async fn get_resumes_by_candidate(&self, candidate_id: Uuid, context: RequestContext) -> Result<Vec<Resume>, RecruitmentError>;
    
    // Interview methods
    async fn create_interview(&self, interview: &Interview, context: RequestContext) -> Result<(), RecruitmentError>;
    async fn get_interview(&self, id: Uuid, context: RequestContext) -> Result<Interview, RecruitmentError>;
    async fn get_interviews_by_application(&self, application_id: Uuid, context: RequestContext) -> Result<Vec<Interview>, RecruitmentError>;
    async fn update_interview(&self, interview: &Interview, context: RequestContext) -> Result<(), RecruitmentError>;
    async fn delete_interview(&self, id: Uuid, context: RequestContext) -> Result<(), RecruitmentError>;
    
    // Job Alert methods
    async fn create_job_alert(&self, alert: &JobAlert, context: RequestContext) -> Result<(), RecruitmentError>;
    async fn get_job_alert(&self, id: Uuid, context: RequestContext) -> Result<JobAlert, RecruitmentError>;
    async fn get_job_alerts_by_candidate(&self, candidate_id: Uuid, context: RequestContext) -> Result<Vec<JobAlert>, RecruitmentError>;
    async fn update_job_alert(&self, alert: &JobAlert, context: RequestContext) -> Result<(), RecruitmentError>;
    async fn delete_job_alert(&self, id: Uuid, context: RequestContext) -> Result<(), RecruitmentError>;
    async fn find_matching_job_alerts(&self, job: &Job, context: RequestContext) -> Result<Vec<JobAlert>, RecruitmentError>;
    async fn create_job_alert_match(&self, match_record: &JobAlertMatch, context: RequestContext) -> Result<(), RecruitmentError>;
    
    // P2P methods
    async fn replicate_to_node(&self, node_id: Uuid) -> Result<(), RecruitmentError>;
    async fn resolve_conflicts(&self, conflicts: Vec<Conflict>) -> Result<(), RecruitmentError>;
}

#[derive(Debug, Clone)]
pub struct Interview {
    pub id: Uuid,
    pub application_id: Uuid,
    pub scheduled_time: DateTime<Utc>,
    pub location: Option<String>,
    pub meeting_link: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Interview {
    pub fn new(
        application_id: Uuid,
        scheduled_time: DateTime<Utc>,
        location: Option<String>,
        meeting_link: Option<String>,
        notes: Option<String>,
    ) -> Self {
        let now = chrono::Utc::now();
        Interview {
            id: Uuid::new_v4(),
            application_id,
            scheduled_time,
            location,
            meeting_link,
            notes,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone)]
pub struct JobAlert {
    pub id: Uuid,
    pub candidate_id: Uuid,
    pub search_criteria: JobAlertCriteria,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl JobAlert {
    pub fn new(
        candidate_id: Uuid,
        search_criteria: JobAlertCriteria,
    ) -> Self {
        let now = chrono::Utc::now();
        JobAlert {
            id: Uuid::new_v4(),
            candidate_id,
            search_criteria,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone)]
pub struct JobAlertCriteria {
    pub keywords: Option<Vec<String>>,
    pub location: Option<String>,
    pub employment_type: Option<String>,
    pub is_remote: Option<bool>,
    pub min_salary: Option<Decimal>,
}

#[derive(Debug, Clone)]
pub struct JobAlertMatch {
    pub id: Uuid,
    pub job_alert_id: Uuid,
    pub job_id: Uuid,
    pub matched_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Conflict {
    pub id: Uuid,
    pub entity_type: String,
    pub local_version: serde_json::Value,
    pub remote_version: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}