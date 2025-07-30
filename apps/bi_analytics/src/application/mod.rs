//! Application services for the BI & Analytics module

pub mod data_ingestion;
pub mod report_generation;
pub mod dashboard_management;
pub mod compliance_management;
pub mod visualization_service;

// Re-export key types
pub use data_ingestion::DataIngestionService;
pub use report_generation::ReportGenerationService;
pub use dashboard_management::DashboardManagementService;
pub use compliance_management::ComplianceManagementService;
pub use visualization_service::{
    VisualizationService, BevyVisualizationService, VisualizationPayload, Base64Image,
    VisualizationError, NavigationHint,
};