//! Axum routes for the website builder module

use axum::{
    routing::{get, post},
    Router,
};

use crate::application::site_service::SiteService;
use crate::application::template_service::TemplateService;
use crate::application::analytics_service::AnalyticsService;

// Placeholder for the actual route handlers
// In a real implementation, these would be actual async functions
// that handle the HTTP requests and interact with the services

pub async fn create_site_handler() {}
pub async fn update_site_handler() {}
pub async fn publish_site_handler() {}
pub async fn get_site_handler() {}
pub async fn list_sites_handler() {}
pub async fn create_template_handler() {}
pub async fn apply_template_handler() {}
pub async fn list_templates_handler() {}
pub async fn track_link_click_handler() {}
pub async fn get_analytics_handler() {}

pub fn create_website_builder_router(
    _site_service: SiteService,
    _template_service: TemplateService,
    _analytics_service: AnalyticsService,
) -> Router {
    Router::new()
        .route("/sites", post(create_site_handler))
        .route("/sites/:id", get(get_site_handler).put(update_site_handler))
        .route("/sites/:id/publish", post(publish_site_handler))
        .route("/sites", get(list_sites_handler))
        .route("/templates", post(create_template_handler))
        .route("/templates", get(list_templates_handler))
        .route("/sites/:id/template/:template_id", post(apply_template_handler))
        .route("/links/:id/click", post(track_link_click_handler))
        .route("/sites/:id/analytics", get(get_analytics_handler))
}