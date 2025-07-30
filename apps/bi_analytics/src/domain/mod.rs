//! Domain entities and business logic for the BI & Analytics module

pub mod dataset;
pub mod report;
pub mod dashboard;
pub mod compliance;

// Re-export key types
pub use dataset::Dataset;
pub use report::Report;
pub use dashboard::Dashboard;