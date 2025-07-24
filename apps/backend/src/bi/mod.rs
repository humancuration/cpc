//! Business Intelligence (BI) toolkit module
//! 
//! This module provides the core functionality for the Business Intelligence
//! toolkit, including impact reporting and future BI features.

pub mod bi_service;
pub mod impact_report;
pub mod models;
pub mod graphql;

/// Re-export commonly used types
pub use bi_service::BIService;
pub use impact_report::ImpactReportService;
pub use models::*;