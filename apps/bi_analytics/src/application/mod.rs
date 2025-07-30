//! Application services for the BI & Analytics module

pub mod data_ingestion;
pub mod report_generation;
pub mod dashboard_management;
pub mod compliance_management;

// Re-export key types
pub use data_ingestion::DataIngestionService;
pub use report_generation::ReportGenerationService;
pub use dashboard_management::DashboardManagementService;
pub use compliance_management::ComplianceManagementService;