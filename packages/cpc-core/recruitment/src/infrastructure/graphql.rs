use async_graphql::{Object, Result, Context, ID, SimpleObject, InputObject};
use crate::domain::models::{Job, Candidate, Application, Employer};
use crate::domain::value_objects::{EmploymentType, ApplicationStatus, JobStatus};
use crate::application::job_service::JobService;
use crate::application::candidate_service::CandidateService;
use crate::application::application_service::ApplicationService;
use uuid::Uuid;
use std::sync::Arc;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn job(&self, ctx: &Context<'_>, id: ID) -> Result<Option<JobObject>> {
        let job_service = ctx.data::<Arc<JobService>>().unwrap();
        let job_id = Uuid::parse_str(&id).map_err(|_| "Invalid job ID")?;
        
        match job_service.get_job(job_id).await {
            Ok(job) => Ok(Some(JobObject::from(job))),
            Err(_) => Ok(None), // In a real implementation, you'd want to handle errors properly
        }
    }
    
    async fn jobs(
        &self,
        ctx: &Context<'_>,
        search: Option<String>,
        location: Option<String>,
        employment_type: Option<String>,
        is_remote: Option<bool>,
    ) -> Result<Vec<JobObject>> {
        let job_service = ctx.data::<Arc<JobService>>().unwrap();
        
        let jobs = job_service.search_jobs(search, location, employment_type, is_remote).await
            .map_err(|e| format!("Failed to search jobs: {}", e))?;
        
        Ok(jobs.into_iter().map(JobObject::from).collect())
    }
    
    async fn candidate(&self, ctx: &Context<'_>, id: ID) -> Result<Option<CandidateObject>> {
        let candidate_service = ctx.data::<Arc<CandidateService>>().unwrap();
        let candidate_id = Uuid::parse_str(&id).map_err(|_| "Invalid candidate ID")?;
        
        match candidate_service.get_candidate(candidate_id).await {
            Ok(candidate) => Ok(Some(CandidateObject::from(candidate))),
            Err(_) => Ok(None),
        }
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_job(&self, ctx: &Context<'_>, input: CreateJobInput) -> Result<JobObject> {
        let job_service = ctx.data::<Arc<JobService>>().unwrap();
        
        let job = job_service.create_job(
            Uuid::parse_str(&input.employer_user_id).map_err(|_| "Invalid employer user ID")?,
            input.title,
            input.description,
            input.location,
            input.is_remote,
            input.salary_min,
            input.salary_max,
            input.employment_type,
        ).await.map_err(|e| format!("Failed to create job: {}", e))?;
        
        Ok(JobObject::from(job))
    }
    
    async fn publish_job(&self, ctx: &Context<'_>, job_id: ID, employer_user_id: ID) -> Result<JobObject> {
        let job_service = ctx.data::<Arc<JobService>>().unwrap();
        let job_uuid = Uuid::parse_str(&job_id).map_err(|_| "Invalid job ID")?;
        let employer_uuid = Uuid::parse_str(&employer_user_id).map_err(|_| "Invalid employer user ID")?;
        
        let job = job_service.publish_job(job_uuid, employer_uuid).await
            .map_err(|e| format!("Failed to publish job: {}", e))?;
        
        Ok(JobObject::from(job))
    }
    
    async fn create_candidate_profile(&self, ctx: &Context<'_>, input: CreateCandidateInput) -> Result<CandidateObject> {
        let candidate_service = ctx.data::<Arc<CandidateService>>().unwrap();
        
        let candidate = candidate_service.create_candidate_profile(
            Uuid::parse_str(&input.user_id).map_err(|_| "Invalid user ID")?,
            input.headline,
            input.summary,
            input.location,
            input.is_open_to_work,
        ).await.map_err(|e| format!("Failed to create candidate profile: {}", e))?;
        
        Ok(CandidateObject::from(candidate))
    }
    
    async fn submit_application(&self, ctx: &Context<'_>, input: SubmitApplicationInput) -> Result<ApplicationObject> {
        let application_service = ctx.data::<Arc<ApplicationService>>().unwrap();
        
        let application = application_service.submit_application(
            Uuid::parse_str(&input.job_id).map_err(|_| "Invalid job ID")?,
            Uuid::parse_str(&input.candidate_id).map_err(|_| "Invalid candidate ID")?,
            Uuid::parse_str(&input.user_id).map_err(|_| "Invalid user ID")?,
            input.cover_letter,
        ).await.map_err(|e| format!("Failed to submit application: {}", e))?;
        
        Ok(ApplicationObject::from(application))
    }
}

#[derive(SimpleObject)]
pub struct JobObject {
    pub id: ID,
    pub employer_id: ID,
    pub title: String,
    pub description: String,
    pub location: Option<String>,
    pub is_remote: bool,
    pub salary_min: Option<String>,
    pub salary_max: Option<String>,
    pub employment_type: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Job> for JobObject {
    fn from(job: Job) -> Self {
        JobObject {
            id: ID::from(job.id.to_string()),
            employer_id: ID::from(job.employer_id.to_string()),
            title: job.title,
            description: job.description,
            location: job.location,
            is_remote: job.is_remote,
            salary_min: job.salary_range.as_ref().and_then(|r| r.min.map(|d| d.to_string())),
            salary_max: job.salary_range.as_ref().and_then(|r| r.max.map(|d| d.to_string())),
            employment_type: match job.employment_type {
                EmploymentType::FullTime => "full_time".to_string(),
                EmploymentType::PartTime => "part_time".to_string(),
                EmploymentType::Contract => "contract".to_string(),
                EmploymentType::Internship => "internship".to_string(),
            },
            status: match job.status {
                JobStatus::Draft => "draft".to_string(),
                JobStatus::Published => "published".to_string(),
                JobStatus::Filled => "filled".to_string(),
                JobStatus::Closed => "closed".to_string(),
            },
            created_at: job.created_at.to_rfc3339(),
            updated_at: job.updated_at.to_rfc3339(),
        }
    }
}

#[derive(SimpleObject)]
pub struct CandidateObject {
    pub id: ID,
    pub user_id: ID,
    pub headline: Option<String>,
    pub summary: Option<String>,
    pub location: Option<String>,
    pub is_open_to_work: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Candidate> for CandidateObject {
    fn from(candidate: Candidate) -> Self {
        CandidateObject {
            id: ID::from(candidate.id.to_string()),
            user_id: ID::from(candidate.user_id.to_string()),
            headline: candidate.headline,
            summary: candidate.summary,
            location: candidate.location,
            is_open_to_work: candidate.is_open_to_work,
            created_at: candidate.created_at.to_rfc3339(),
            updated_at: candidate.updated_at.to_rfc3339(),
        }
    }
}

#[derive(SimpleObject)]
pub struct ApplicationObject {
    pub id: ID,
    pub job_id: ID,
    pub candidate_id: ID,
    pub status: String,
    pub cover_letter: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Application> for ApplicationObject {
    fn from(application: Application) -> Self {
        ApplicationObject {
            id: ID::from(application.id.to_string()),
            job_id: ID::from(application.job_id.to_string()),
            candidate_id: ID::from(application.candidate_id.to_string()),
            status: match application.status {
                ApplicationStatus::Applied => "applied".to_string(),
                ApplicationStatus::Reviewed => "reviewed".to_string(),
                ApplicationStatus::Interviewing => "interviewing".to_string(),
                ApplicationStatus::Offered => "offered".to_string(),
                ApplicationStatus::Hired => "hired".to_string(),
                ApplicationStatus::Rejected => "rejected".to_string(),
            },
            cover_letter: application.cover_letter,
            created_at: application.created_at.to_rfc3339(),
            updated_at: application.updated_at.to_rfc3339(),
        }
    }
}

#[derive(SimpleObject)]
pub struct EmployerObject {
    pub id: ID,
    pub user_id: ID,
    pub company_name: String,
    pub company_description: Option<String>,
    pub website: Option<String>,
    pub industry: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Employer> for EmployerObject {
    fn from(employer: Employer) -> Self {
        EmployerObject {
            id: ID::from(employer.id.to_string()),
            user_id: ID::from(employer.user_id.to_string()),
            company_name: employer.company_name,
            company_description: employer.company_description,
            website: employer.website,
            industry: employer.industry,
            created_at: employer.created_at.to_rfc3339(),
            updated_at: employer.updated_at.to_rfc3339(),
        }
    }
}

#[derive(InputObject)]
pub struct CreateJobInput {
    pub employer_user_id: String,
    pub title: String,
    pub description: String,
    pub location: Option<String>,
    pub is_remote: bool,
    pub salary_min: Option<String>,
    pub salary_max: Option<String>,
    pub employment_type: String,
}

#[derive(InputObject)]
pub struct CreateCandidateInput {
    pub user_id: String,
    pub headline: Option<String>,
    pub summary: Option<String>,
    pub location: Option<String>,
    pub is_open_to_work: bool,
}

#[derive(InputObject)]
pub struct SubmitApplicationInput {
    pub job_id: String,
    pub candidate_id: String,
    pub user_id: String,
    pub cover_letter: Option<String>,
}