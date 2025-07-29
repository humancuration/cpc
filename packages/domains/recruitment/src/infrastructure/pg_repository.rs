use crate::domain::models::{Employer, Job, Candidate, Application, Resume};
use crate::domain::value_objects::{EmploymentType, ApplicationStatus, JobStatus};
use crate::domain::errors::RecruitmentError;
use crate::infrastructure::repository::{RecruitmentRepository, RequestContext, Conflict};
use crate::application::interview_service::Interview;
use crate::application::alert_service::{JobAlert, JobAlertMatch, JobAlertCriteria};
use uuid::Uuid;
use sqlx::PgPool;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

pub struct PgRecruitmentRepository {
    pool: PgPool,
}

impl PgRecruitmentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RecruitmentRepository for PgRecruitmentRepository {
    // Employer methods
    async fn create_employer(&self, employer: &Employer, context: RequestContext) -> Result<(), RecruitmentError> {
        // Handle sharing settings based on context
        match context.consistency_level {
            ConsistencyLevel::Strong => {
                // For strong consistency, we might want to replicate to other nodes immediately
                // This is a simplified implementation
            },
            ConsistencyLevel::Eventual => {
                // For eventual consistency, we'll queue for replication
            },
            ConsistencyLevel::Local => {
                // For local consistency, we only write to local storage
            },
        }
        
        sqlx::query!(
            "INSERT INTO employers (id, user_id, company_name, company_description, website, industry, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            employer.id,
            employer.user_id,
            employer.company_name,
            employer.company_description,
            employer.website,
            employer.industry,
            employer.created_at,
            employer.updated_at,
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_employer(&self, id: Uuid, context: RequestContext) -> Result<Employer, RecruitmentError> {
        let row = sqlx::query!(
            "SELECT id, user_id, company_name, company_description, website, industry, created_at, updated_at
             FROM employers WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                // Check if the requesting node can access this data based on sharing settings
                // This is a simplified implementation - in a real system, we would check
                // the actual sharing settings for each field
                Ok(Employer {
                    id: row.id,
                    user_id: row.user_id,
                    company_name: row.company_name,
                    company_description: row.company_description,
                    website: row.website,
                    industry: row.industry,
                    company_name_sharing: Default::default(),
                    company_description_sharing: Default::default(),
                    website_sharing: Default::default(),
                    industry_sharing: Default::default(),
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                })
            },
            None => Err(RecruitmentError::EmployerNotFound(id.to_string())),
        }
    }
    
    async fn get_employer_by_user_id(&self, user_id: Uuid, context: RequestContext) -> Result<Employer, RecruitmentError> {
        let row = sqlx::query!(
            "SELECT id, user_id, company_name, company_description, website, industry, created_at, updated_at
             FROM employers WHERE user_id = $1",
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                // Check if the requesting node can access this data based on sharing settings
                // This is a simplified implementation - in a real system, we would check
                // the actual sharing settings for each field
                Ok(Employer {
                    id: row.id,
                    user_id: row.user_id,
                    company_name: row.company_name,
                    company_description: row.company_description,
                    website: row.website,
                    industry: row.industry,
                    company_name_sharing: Default::default(),
                    company_description_sharing: Default::default(),
                    website_sharing: Default::default(),
                    industry_sharing: Default::default(),
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                })
            },
            None => Err(RecruitmentError::EmployerNotFound(user_id.to_string())),
        }
    }
    
    async fn update_employer(&self, employer: &Employer, context: RequestContext) -> Result<(), RecruitmentError> {
        // Handle sharing settings based on context
        match context.consistency_level {
            ConsistencyLevel::Strong => {
                // For strong consistency, we might want to replicate to other nodes immediately
                // This is a simplified implementation
            },
            ConsistencyLevel::Eventual => {
                // For eventual consistency, we'll queue for replication
            },
            ConsistencyLevel::Local => {
                // For local consistency, we only write to local storage
            },
        }
        
        let rows_affected = sqlx::query!(
            "UPDATE employers
             SET company_name = $1, company_description = $2, website = $3, industry = $4, updated_at = $5
             WHERE id = $6",
            employer.company_name,
            employer.company_description,
            employer.website,
            employer.industry,
            employer.updated_at,
            employer.id,
        )
        .execute(&self.pool)
        .await?
        .rows_affected();
        
        if rows_affected == 0 {
            return Err(RecruitmentError::EmployerNotFound(employer.id.to_string()));
        }
        
        Ok(())
    }
    
    // Job methods
    async fn create_job(&self, job: &Job, context: RequestContext) -> Result<(), RecruitmentError> {
        // Handle sharing settings based on context
        match context.consistency_level {
            ConsistencyLevel::Strong => {
                // For strong consistency, we might want to replicate to other nodes immediately
                // This is a simplified implementation
            },
            ConsistencyLevel::Eventual => {
                // For eventual consistency, we'll queue for replication
            },
            ConsistencyLevel::Local => {
                // For local consistency, we only write to local storage
            },
        }
        
        let employment_type_str = match job.employment_type {
            EmploymentType::FullTime => "full_time",
            EmploymentType::PartTime => "part_time",
            EmploymentType::Contract => "contract",
            EmploymentType::Internship => "internship",
        };
        
        let status_str = match job.status {
            JobStatus::Draft => "draft",
            JobStatus::Published => "published",
            JobStatus::Filled => "filled",
            JobStatus::Closed => "closed",
        };
        
        let (salary_min, salary_max) = match &job.salary_range {
            Some(range) => (range.min, range.max),
            None => (None, None),
        };
        
        sqlx::query!(
            "INSERT INTO jobs (id, employer_id, title, description, location, is_remote, salary_min, salary_max, employment_type, status, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
            job.id,
            job.employer_id,
            job.title,
            job.description,
            job.location,
            job.is_remote,
            salary_min,
            salary_max,
            employment_type_str,
            status_str,
            job.created_at,
            job.updated_at,
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_job(&self, id: Uuid, context: RequestContext) -> Result<Job, RecruitmentError> {
        let row = sqlx::query!(
            "SELECT id, employer_id, title, description, location, is_remote, salary_min, salary_max, employment_type, status, created_at, updated_at
             FROM jobs WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                let employment_type = match row.employment_type.as_str() {
                    "full_time" => EmploymentType::FullTime,
                    "part_time" => EmploymentType::PartTime,
                    "contract" => EmploymentType::Contract,
                    "internship" => EmploymentType::Internship,
                    _ => return Err(RecruitmentError::InvalidEmploymentType(row.employment_type)),
                };
                
                let status = match row.status.as_str() {
                    "draft" => JobStatus::Draft,
                    "published" => JobStatus::Published,
                    "filled" => JobStatus::Filled,
                    "closed" => JobStatus::Closed,
                    _ => return Err(RecruitmentError::InvalidJobStatus(row.status)),
                };
                
                let salary_range = if row.salary_min.is_some() || row.salary_max.is_some() {
                    Some(crate::domain::value_objects::SalaryRange {
                        min: row.salary_min,
                        max: row.salary_max,
                    })
                } else {
                    None
                };
                
                Ok(Job {
                    id: row.id,
                    employer_id: row.employer_id,
                    title: row.title,
                    description: row.description,
                    location: row.location,
                    is_remote: row.is_remote,
                    salary_range,
                    employment_type,
                    status,
                    title_sharing: Default::default(),
                    description_sharing: Default::default(),
                    location_sharing: Default::default(),
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                })
            },
            None => Err(RecruitmentError::JobNotFound(id.to_string())),
        }
    }
    
    async fn update_job(&self, job: &Job) -> Result<(), RecruitmentError> {
        let employment_type_str = match job.employment_type {
            EmploymentType::FullTime => "full_time",
            EmploymentType::PartTime => "part_time",
            EmploymentType::Contract => "contract",
            EmploymentType::Internship => "internship",
        };
        
        let status_str = match job.status {
            JobStatus::Draft => "draft",
            JobStatus::Published => "published",
            JobStatus::Filled => "filled",
            JobStatus::Closed => "closed",
        };
        
        let (salary_min, salary_max) = match &job.salary_range {
            Some(range) => (range.min, range.max),
            None => (None, None),
        };
        
        let rows_affected = sqlx::query!(
            "UPDATE jobs
             SET title = $1, description = $2, location = $3, is_remote = $4, salary_min = $5, salary_max = $6, employment_type = $7, status = $8, updated_at = $9
             WHERE id = $10",
            job.title,
            job.description,
            job.location,
            job.is_remote,
            salary_min,
            salary_max,
            employment_type_str,
            status_str,
            job.updated_at,
            job.id,
        )
        .execute(&self.pool)
        .await?
        .rows_affected();
        
        if rows_affected == 0 {
            return Err(RecruitmentError::JobNotFound(job.id.to_string()));
        }
        
        Ok(())
    }
    
    async fn get_jobs_by_employer(&self, employer_id: Uuid) -> Result<Vec<Job>, RecruitmentError> {
        let rows = sqlx::query!(
            "SELECT id, employer_id, title, description, location, is_remote, salary_min, salary_max, employment_type, status, created_at, updated_at
             FROM jobs WHERE employer_id = $1
             ORDER BY created_at DESC",
            employer_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut jobs = Vec::new();
        for row in rows {
            let employment_type = match row.employment_type.as_str() {
                "full_time" => EmploymentType::FullTime,
                "part_time" => EmploymentType::PartTime,
                "contract" => EmploymentType::Contract,
                "internship" => EmploymentType::Internship,
                _ => return Err(RecruitmentError::InvalidEmploymentType(row.employment_type)),
            };
            
            let status = match row.status.as_str() {
                "draft" => JobStatus::Draft,
                "published" => JobStatus::Published,
                "filled" => JobStatus::Filled,
                "closed" => JobStatus::Closed,
                _ => return Err(RecruitmentError::InvalidJobStatus(row.status)),
            };
            
            let salary_range = if row.salary_min.is_some() || row.salary_max.is_some() {
                Some(crate::domain::value_objects::SalaryRange {
                    min: row.salary_min,
                    max: row.salary_max,
                })
            } else {
                None
            };
            
            jobs.push(Job {
                id: row.id,
                employer_id: row.employer_id,
                title: row.title,
                description: row.description,
                location: row.location,
                is_remote: row.is_remote,
                salary_range,
                employment_type,
                status,
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }
        
        Ok(jobs)
    }
    
    async fn search_jobs(
        &self,
        search_term: Option<String>,
        location: Option<String>,
        employment_type: Option<String>,
        is_remote: Option<bool>,
    ) -> Result<Vec<Job>, RecruitmentError> {
        // This is a simplified implementation. In a real system, you would use full-text search
        // and more sophisticated filtering.
        let mut query = "SELECT id, employer_id, title, description, location, is_remote, salary_min, salary_max, employment_type, status, created_at, updated_at
                         FROM jobs WHERE status = 'published'".to_string();
        
        let mut params: Vec<Box<dyn sqlx::postgres::PgArg>> = Vec::new();
        let mut param_index = 1;
        
        if let Some(term) = search_term {
            query.push_str(&format!(" AND (title ILIKE $% OR description ILIKE $%)", param_index, param_index));
            params.push(Box::new(format!("%{}%", term)));
            param_index += 1;
        }
        
        if let Some(loc) = location {
            query.push_str(&format!(" AND location ILIKE $%", param_index));
            params.push(Box::new(format!("%{}%", loc)));
            param_index += 1;
        }
        
        if let Some(emp_type) = employment_type {
            query.push_str(&format!(" AND employment_type = $%", param_index));
            params.push(Box::new(emp_type));
            param_index += 1;
        }
        
        if let Some(remote) = is_remote {
            query.push_str(&format!(" AND is_remote = $%", param_index));
            params.push(Box::new(remote));
            param_index += 1;
        }
        
        query.push_str(" ORDER BY created_at DESC LIMIT 100");
        
        // For simplicity, we'll execute a basic query. In a real implementation, you would
        // need to handle the dynamic parameters properly.
        let rows = sqlx::query!(
            "SELECT id, employer_id, title, description, location, is_remote, salary_min, salary_max, employment_type, status, created_at, updated_at
             FROM jobs WHERE status = 'published'
             ORDER BY created_at DESC LIMIT 100"
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut jobs = Vec::new();
        for row in rows {
            let employment_type = match row.employment_type.as_str() {
                "full_time" => EmploymentType::FullTime,
                "part_time" => EmploymentType::PartTime,
                "contract" => EmploymentType::Contract,
                "internship" => EmploymentType::Internship,
                _ => return Err(RecruitmentError::InvalidEmploymentType(row.employment_type)),
            };
            
            let status = match row.status.as_str() {
                "draft" => JobStatus::Draft,
                "published" => JobStatus::Published,
                "filled" => JobStatus::Filled,
                "closed" => JobStatus::Closed,
                _ => return Err(RecruitmentError::InvalidJobStatus(row.status)),
            };
            
            let salary_range = if row.salary_min.is_some() || row.salary_max.is_some() {
                Some(crate::domain::value_objects::SalaryRange {
                    min: row.salary_min,
                    max: row.salary_max,
                })
            } else {
                None
            };
            
            jobs.push(Job {
                id: row.id,
                employer_id: row.employer_id,
                title: row.title,
                description: row.description,
                location: row.location,
                is_remote: row.is_remote,
                salary_range,
                employment_type,
                status,
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }
        
        Ok(jobs)
    }
    
    // Candidate methods
    async fn create_candidate(&self, candidate: &Candidate) -> Result<(), RecruitmentError> {
        sqlx::query!(
            "INSERT INTO candidates (id, user_id, headline, summary, location, is_open_to_work, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            candidate.id,
            candidate.user_id,
            candidate.headline,
            candidate.summary,
            candidate.location,
            candidate.is_open_to_work,
            candidate.created_at,
            candidate.updated_at,
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_candidate(&self, id: Uuid) -> Result<Candidate, RecruitmentError> {
        let row = sqlx::query!(
            "SELECT id, user_id, headline, summary, location, is_open_to_work, created_at, updated_at
             FROM candidates WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => Ok(Candidate {
                id: row.id,
                user_id: row.user_id,
                headline: row.headline,
                summary: row.summary,
                location: row.location,
                is_open_to_work: row.is_open_to_work,
                created_at: row.created_at,
                updated_at: row.updated_at,
            }),
            None => Err(RecruitmentError::CandidateNotFound(id.to_string())),
        }
    }
    
    async fn get_candidate_by_user_id(&self, user_id: Uuid) -> Result<Candidate, RecruitmentError> {
        let row = sqlx::query!(
            "SELECT id, user_id, headline, summary, location, is_open_to_work, created_at, updated_at
             FROM candidates WHERE user_id = $1",
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => Ok(Candidate {
                id: row.id,
                user_id: row.user_id,
                headline: row.headline,
                summary: row.summary,
                location: row.location,
                is_open_to_work: row.is_open_to_work,
                created_at: row.created_at,
                updated_at: row.updated_at,
            }),
            None => Err(RecruitmentError::CandidateNotFound(user_id.to_string())),
        }
    }
    
    async fn update_candidate(&self, candidate: &Candidate) -> Result<(), RecruitmentError> {
        let rows_affected = sqlx::query!(
            "UPDATE candidates
             SET headline = $1, summary = $2, location = $3, is_open_to_work = $4, updated_at = $5
             WHERE id = $6",
            candidate.headline,
            candidate.summary,
            candidate.location,
            candidate.is_open_to_work,
            candidate.updated_at,
            candidate.id,
        )
        .execute(&self.pool)
        .await?
        .rows_affected();
        
        if rows_affected == 0 {
            return Err(RecruitmentError::CandidateNotFound(candidate.id.to_string()));
        }
        
        Ok(())
    }
    
    async fn search_candidates(
        &self,
        search_term: Option<String>,
        location: Option<String>,
        is_open_to_work: Option<bool>,
    ) -> Result<Vec<Candidate>, RecruitmentError> {
        // Simplified implementation
        let rows = sqlx::query!(
            "SELECT id, user_id, headline, summary, location, is_open_to_work, created_at, updated_at
             FROM candidates
             WHERE ($1 IS NULL OR headline ILIKE $1 OR summary ILIKE $1)
             AND ($2 IS NULL OR location ILIKE $2)
             AND ($3 IS NULL OR is_open_to_work = $3)
             ORDER BY created_at DESC LIMIT 100",
            search_term.map(|s| format!("%{}%", s)),
            location.map(|l| format!("%{}%", l)),
            is_open_to_work,
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut candidates = Vec::new();
        for row in rows {
            candidates.push(Candidate {
                id: row.id,
                user_id: row.user_id,
                headline: row.headline,
                summary: row.summary,
                location: row.location,
                is_open_to_work: row.is_open_to_work,
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }
        
        Ok(candidates)
    }
    
    // Application methods
    async fn create_application(&self, application: &Application) -> Result<(), RecruitmentError> {
        let status_str = match application.status {
            ApplicationStatus::Applied => "applied",
            ApplicationStatus::Reviewed => "reviewed",
            ApplicationStatus::Interviewing => "interviewing",
            ApplicationStatus::Offered => "offered",
            ApplicationStatus::Hired => "hired",
            ApplicationStatus::Rejected => "rejected",
        };
        
        sqlx::query!(
            "INSERT INTO applications (id, job_id, candidate_id, status, cover_letter, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
            application.id,
            application.job_id,
            application.candidate_id,
            status_str,
            application.cover_letter,
            application.created_at,
            application.updated_at,
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_application(&self, id: Uuid) -> Result<Application, RecruitmentError> {
        let row = sqlx::query!(
            "SELECT id, job_id, candidate_id, status, cover_letter, created_at, updated_at
             FROM applications WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                let status = match row.status.as_str() {
                    "applied" => ApplicationStatus::Applied,
                    "reviewed" => ApplicationStatus::Reviewed,
                    "interviewing" => ApplicationStatus::Interviewing,
                    "offered" => ApplicationStatus::Offered,
                    "hired" => ApplicationStatus::Hired,
                    "rejected" => ApplicationStatus::Rejected,
                    _ => return Err(RecruitmentError::InvalidApplicationStatus(row.status)),
                };
                
                Ok(Application {
                    id: row.id,
                    job_id: row.job_id,
                    candidate_id: row.candidate_id,
                    status,
                    cover_letter: row.cover_letter,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                })
            },
            None => Err(RecruitmentError::ApplicationNotFound(id.to_string())),
        }
    }
    
    async fn get_application_by_job_and_candidate(&self, job_id: Uuid, candidate_id: Uuid) -> Result<Application, RecruitmentError> {
        let row = sqlx::query!(
            "SELECT id, job_id, candidate_id, status, cover_letter, created_at, updated_at
             FROM applications WHERE job_id = $1 AND candidate_id = $2",
            job_id,
            candidate_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                let status = match row.status.as_str() {
                    "applied" => ApplicationStatus::Applied,
                    "reviewed" => ApplicationStatus::Reviewed,
                    "interviewing" => ApplicationStatus::Interviewing,
                    "offered" => ApplicationStatus::Offered,
                    "hired" => ApplicationStatus::Hired,
                    "rejected" => ApplicationStatus::Rejected,
                    _ => return Err(RecruitmentError::InvalidApplicationStatus(row.status)),
                };
                
                Ok(Application {
                    id: row.id,
                    job_id: row.job_id,
                    candidate_id: row.candidate_id,
                    status,
                    cover_letter: row.cover_letter,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                })
            },
            None => Err(RecruitmentError::ApplicationNotFound(format!("job: {}, candidate: {}", job_id, candidate_id))),
        }
    }
    
    async fn update_application(&self, application: &Application) -> Result<(), RecruitmentError> {
        let status_str = match application.status {
            ApplicationStatus::Applied => "applied",
            ApplicationStatus::Reviewed => "reviewed",
            ApplicationStatus::Interviewing => "interviewing",
            ApplicationStatus::Offered => "offered",
            ApplicationStatus::Hired => "hired",
            ApplicationStatus::Rejected => "rejected",
        };
        
        let rows_affected = sqlx::query!(
            "UPDATE applications
             SET status = $1, cover_letter = $2, updated_at = $3
             WHERE id = $4",
            status_str,
            application.cover_letter,
            application.updated_at,
            application.id,
        )
        .execute(&self.pool)
        .await?
        .rows_affected();
        
        if rows_affected == 0 {
            return Err(RecruitmentError::ApplicationNotFound(application.id.to_string()));
        }
        
        Ok(())
    }
    
    async fn delete_application(&self, id: Uuid) -> Result<(), RecruitmentError> {
        let rows_affected = sqlx::query!(
            "DELETE FROM applications WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();
        
        if rows_affected == 0 {
            return Err(RecruitmentError::ApplicationNotFound(id.to_string()));
        }
        
        Ok(())
    }
    
    async fn get_applications_by_job(&self, job_id: Uuid) -> Result<Vec<Application>, RecruitmentError> {
        let rows = sqlx::query!(
            "SELECT id, job_id, candidate_id, status, cover_letter, created_at, updated_at
             FROM applications WHERE job_id = $1
             ORDER BY created_at DESC",
            job_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut applications = Vec::new();
        for row in rows {
            let status = match row.status.as_str() {
                "applied" => ApplicationStatus::Applied,
                "reviewed" => ApplicationStatus::Reviewed,
                "interviewing" => ApplicationStatus::Interviewing,
                "offered" => ApplicationStatus::Offered,
                "hired" => ApplicationStatus::Hired,
                "rejected" => ApplicationStatus::Rejected,
                _ => return Err(RecruitmentError::InvalidApplicationStatus(row.status)),
            };
            
            applications.push(Application {
                id: row.id,
                job_id: row.job_id,
                candidate_id: row.candidate_id,
                status,
                cover_letter: row.cover_letter,
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }
        
        Ok(applications)
    }
    
    async fn get_applications_by_candidate(&self, candidate_id: Uuid) -> Result<Vec<Application>, RecruitmentError> {
        let rows = sqlx::query!(
            "SELECT id, job_id, candidate_id, status, cover_letter, created_at, updated_at
             FROM applications WHERE candidate_id = $1
             ORDER BY created_at DESC",
            candidate_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut applications = Vec::new();
        for row in rows {
            let status = match row.status.as_str() {
                "applied" => ApplicationStatus::Applied,
                "reviewed" => ApplicationStatus::Reviewed,
                "interviewing" => ApplicationStatus::Interviewing,
                "offered" => ApplicationStatus::Offered,
                "hired" => ApplicationStatus::Hired,
                "rejected" => ApplicationStatus::Rejected,
                _ => return Err(RecruitmentError::InvalidApplicationStatus(row.status)),
            };
            
            applications.push(Application {
                id: row.id,
                job_id: row.job_id,
                candidate_id: row.candidate_id,
                status,
                cover_letter: row.cover_letter,
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }
        
        Ok(applications)
    }
    
    // Resume methods
    async fn create_resume(&self, resume: &Resume) -> Result<(), RecruitmentError> {
        sqlx::query!(
            "INSERT INTO resumes (id, candidate_id, document_id, parsed_content, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6)",
            resume.id,
            resume.candidate_id,
            resume.document_id,
            resume.parsed_content,
            resume.created_at,
            resume.updated_at,
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_resumes_by_candidate(&self, candidate_id: Uuid) -> Result<Vec<Resume>, RecruitmentError> {
        let rows = sqlx::query!(
            "SELECT id, candidate_id, document_id, parsed_content, created_at, updated_at
             FROM resumes WHERE candidate_id = $1
             ORDER BY created_at DESC",
            candidate_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut resumes = Vec::new();
        for row in rows {
            resumes.push(Resume {
                id: row.id,
                candidate_id: row.candidate_id,
                document_id: row.document_id,
                parsed_content: row.parsed_content,
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }
        
        Ok(resumes)
    }
    
    // Interview methods
    async fn create_interview(&self, interview: &Interview) -> Result<(), RecruitmentError> {
        sqlx::query!(
            "INSERT INTO interviews (id, application_id, scheduled_time, location, meeting_link, notes, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            interview.id,
            interview.application_id,
            interview.scheduled_time,
            interview.location,
            interview.meeting_link,
            interview.notes,
            interview.created_at,
            interview.updated_at,
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_interview(&self, id: Uuid) -> Result<Interview, RecruitmentError> {
        let row = sqlx::query!(
            "SELECT id, application_id, scheduled_time, location, meeting_link, notes, created_at, updated_at
             FROM interviews WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => Ok(Interview {
                id: row.id,
                application_id: row.application_id,
                scheduled_time: row.scheduled_time,
                location: row.location,
                meeting_link: row.meeting_link,
                notes: row.notes,
                created_at: row.created_at,
                updated_at: row.updated_at,
            }),
            None => Err(RecruitmentError::ApplicationNotFound(id.to_string())),
        }
    }
    
    async fn get_interviews_by_application(&self, application_id: Uuid) -> Result<Vec<Interview>, RecruitmentError> {
        let rows = sqlx::query!(
            "SELECT id, application_id, scheduled_time, location, meeting_link, notes, created_at, updated_at
             FROM interviews WHERE application_id = $1
             ORDER BY scheduled_time ASC",
            application_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut interviews = Vec::new();
        for row in rows {
            interviews.push(Interview {
                id: row.id,
                application_id: row.application_id,
                scheduled_time: row.scheduled_time,
                location: row.location,
                meeting_link: row.meeting_link,
                notes: row.notes,
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }
        
        Ok(interviews)
    }
    
    async fn update_interview(&self, interview: &Interview) -> Result<(), RecruitmentError> {
        let rows_affected = sqlx::query!(
            "UPDATE interviews
             SET scheduled_time = $1, location = $2, meeting_link = $3, notes = $4, updated_at = $5
             WHERE id = $6",
            interview.scheduled_time,
            interview.location,
            interview.meeting_link,
            interview.notes,
            interview.updated_at,
            interview.id,
        )
        .execute(&self.pool)
        .await?
        .rows_affected();
        
        if rows_affected == 0 {
            return Err(RecruitmentError::ApplicationNotFound(interview.id.to_string()));
        }
        
        Ok(())
    }
    
    async fn delete_interview(&self, id: Uuid) -> Result<(), RecruitmentError> {
        let rows_affected = sqlx::query!(
            "DELETE FROM interviews WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();
        
        if rows_affected == 0 {
            return Err(RecruitmentError::ApplicationNotFound(id.to_string()));
        }
        
        Ok(())
    }
    
    // Job Alert methods
    async fn create_job_alert(&self, alert: &JobAlert) -> Result<(), RecruitmentError> {
        sqlx::query!(
            "INSERT INTO job_alerts (id, candidate_id, keywords, location, employment_type, is_remote, min_salary, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
            alert.id,
            alert.candidate_id,
            alert.search_criteria.keywords.as_ref().map(|v| v.join(",")),
            alert.search_criteria.location,
            alert.search_criteria.employment_type,
            alert.search_criteria.is_remote,
            alert.search_criteria.min_salary,
            alert.created_at,
            alert.updated_at,
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_job_alert(&self, id: Uuid) -> Result<JobAlert, RecruitmentError> {
        let row = sqlx::query!(
            "SELECT id, candidate_id, keywords, location, employment_type, is_remote, min_salary, created_at, updated_at
             FROM job_alerts WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                let keywords = if let Some(kw) = row.keywords {
                    Some(kw.split(',').map(|s| s.to_string()).collect())
                } else {
                    None
                };
                
                Ok(JobAlert {
                    id: row.id,
                    candidate_id: row.candidate_id,
                    search_criteria: JobAlertCriteria {
                        keywords,
                        location: row.location,
                        employment_type: row.employment_type,
                        is_remote: row.is_remote,
                        min_salary: row.min_salary,
                    },
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                })
            },
            None => Err(RecruitmentError::ApplicationNotFound(id.to_string())),
        }
    }
    
    async fn get_job_alerts_by_candidate(&self, candidate_id: Uuid) -> Result<Vec<JobAlert>, RecruitmentError> {
        let rows = sqlx::query!(
            "SELECT id, candidate_id, keywords, location, employment_type, is_remote, min_salary, created_at, updated_at
             FROM job_alerts WHERE candidate_id = $1
             ORDER BY created_at DESC",
            candidate_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut alerts = Vec::new();
        for row in rows {
            let keywords = if let Some(kw) = row.keywords {
                Some(kw.split(',').map(|s| s.to_string()).collect())
            } else {
                None
            };
            
            alerts.push(JobAlert {
                id: row.id,
                candidate_id: row.candidate_id,
                search_criteria: JobAlertCriteria {
                    keywords,
                    location: row.location,
                    employment_type: row.employment_type,
                    is_remote: row.is_remote,
                    min_salary: row.min_salary,
                },
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }
        
        Ok(alerts)
    }
    
    async fn update_job_alert(&self, alert: &JobAlert) -> Result<(), RecruitmentError> {
        let keywords_str = alert.search_criteria.keywords.as_ref().map(|v| v.join(","));
        
        let rows_affected = sqlx::query!(
            "UPDATE job_alerts
             SET keywords = $1, location = $2, employment_type = $3, is_remote = $4, min_salary = $5, updated_at = $6
             WHERE id = $7",
            keywords_str,
            alert.search_criteria.location,
            alert.search_criteria.employment_type,
            alert.search_criteria.is_remote,
            alert.search_criteria.min_salary,
            alert.updated_at,
            alert.id,
        )
        .execute(&self.pool)
        .await?
        .rows_affected();
        
        if rows_affected == 0 {
            return Err(RecruitmentError::ApplicationNotFound(alert.id.to_string()));
        }
        
        Ok(())
    }
    
    async fn delete_job_alert(&self, id: Uuid) -> Result<(), RecruitmentError> {
        let rows_affected = sqlx::query!(
            "DELETE FROM job_alerts WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();
        
        if rows_affected == 0 {
            return Err(RecruitmentError::ApplicationNotFound(id.to_string()));
        }
        
        Ok(())
    }
    
    async fn find_matching_job_alerts(&self, job: &Job) -> Result<Vec<JobAlert>, RecruitmentError> {
        // Simplified implementation - in a real system, this would be more complex
        let rows = sqlx::query!(
            "SELECT id, candidate_id, keywords, location, employment_type, is_remote, min_salary, created_at, updated_at
             FROM job_alerts
             WHERE ($1 IS NULL OR employment_type IS NULL OR employment_type = $1)
             AND ($2 IS NULL OR is_remote IS NULL OR is_remote = $2)
             AND ($3 IS NULL OR min_salary IS NULL OR min_salary <= $3)",
            match job.employment_type {
                EmploymentType::FullTime => Some("full_time"),
                EmploymentType::PartTime => Some("part_time"),
                EmploymentType::Contract => Some("contract"),
                EmploymentType::Internship => Some("internship"),
            },
            Some(job.is_remote),
            job.salary_range.as_ref().and_then(|r| r.min),
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut alerts = Vec::new();
        for row in rows {
            let keywords = if let Some(kw) = row.keywords {
                Some(kw.split(',').map(|s| s.to_string()).collect())
            } else {
                None
            };
            
            alerts.push(JobAlert {
                id: row.id,
                candidate_id: row.candidate_id,
                search_criteria: JobAlertCriteria {
                    keywords,
                    location: row.location,
                    employment_type: row.employment_type,
                    is_remote: row.is_remote,
                    min_salary: row.min_salary,
                },
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }
        
        Ok(alerts)
    }
    
    async fn create_job_alert_match(&self, match_record: &JobAlertMatch) -> Result<(), RecruitmentError> {
        sqlx::query!(
            "INSERT INTO job_alert_matches (id, job_alert_id, job_id, matched_at)
             VALUES ($1, $2, $3, $4)",
            match_record.id,
            match_record.job_alert_id,
            match_record.job_id,
            match_record.matched_at,
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}