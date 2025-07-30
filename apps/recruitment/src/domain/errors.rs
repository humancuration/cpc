use thiserror::Error;
use rust_decimal::Error as DecimalError;

#[derive(Error, Debug)]
pub enum RecruitmentError {
    #[error("Job not found: {0}")]
    JobNotFound(String),
    
    #[error("Candidate not found: {0}")]
    CandidateNotFound(String),
    
    #[error("Employer not found: {0}")]
    EmployerNotFound(String),
    
    #[error("Application not found: {0}")]
    ApplicationNotFound(String),
    
    #[error("Access denied")]
    AccessDenied,
    
    #[error("Invalid job status: {0}")]
    InvalidJobStatus(String),
    
    #[error("Invalid application status: {0}")]
    InvalidApplicationStatus(String),
    
    #[error("Invalid employment type: {0}")]
    InvalidEmploymentType(String),
    
    #[error("Salary range error: {0}")]
    InvalidSalaryRange(String),
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Decimal conversion error: {0}")]
    DecimalError(#[from] DecimalError),
    
    #[error("Resume processing error: {0}")]
    ResumeProcessingError(String),
}