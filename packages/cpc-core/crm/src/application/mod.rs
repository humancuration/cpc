//! Application services for the CRM module
//!
//! This module contains the application services that orchestrate domain logic
//! and implement use cases for the CRM functionality.

pub mod contact_service;
pub mod interaction_service;
pub mod pipeline_service;
pub mod deal_service;

// Re-export key types for convenience
pub use contact_service::{ContactService, ContactConsentService};
pub use interaction_service::InteractionService;
pub use pipeline_service::PipelineService;
pub use deal_service::DealService;