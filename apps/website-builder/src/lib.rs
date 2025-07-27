//! Website Builder Module
//!
//! This module provides functionality for creating both full websites and link-in-bio sites.
//! It follows a hexagonal architecture pattern with clear separation of concerns.

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod web;

// Re-export key components for easier access
pub use domain::models::Site;
pub use domain::models::SiteType;
pub use application::site_service::SiteService;
pub use application::template_service::TemplateService;
pub use application::analytics_service::AnalyticsService;
pub use infrastructure::repository::SiteRepository;
pub use web::module::WebsiteBuilderModule;
pub use web::modular_module::ModularWebsiteBuilder;