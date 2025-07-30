//! Recruitment System Module
//!
//! This module provides comprehensive job market functionality with focus on:
//! - Job Market Creation & Management
//! - Career Development
//! - Recruitment Workflow
//! - Network Building
//!
//! Follows hexagonal architecture with clean separation of domain/application/infrastructure

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;

// Re-export key types
pub use domain::models::{Job, Candidate, Employer, Application};
pub use domain::value_objects::{SalaryRange, EmploymentType, ApplicationStatus};
pub use domain::errors::RecruitmentError;

pub use application::job_service::JobService;
pub use application::candidate_service::CandidateService;
pub use application::application_service::ApplicationService;
pub use application::interview_service::InterviewService;
pub use application::alert_service::AlertService;

pub use infrastructure::repository::RecruitmentRepository;
pub use infrastructure::pg_repository::PgRecruitmentRepository;
pub use infrastructure::resume_parser::ResumeParser;
pub use infrastructure::matching_engine::MatchingEngine;
pub use infrastructure::calendar_integration::CalendarIntegration;
pub use infrastructure::notification::NotificationService;

// Re-export presentation components when web feature is enabled
#[cfg(feature = "web")]
pub use presentation::*;