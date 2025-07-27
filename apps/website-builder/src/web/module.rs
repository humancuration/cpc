//! Module initialization and wiring for the website builder

use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;

use crate::application::site_service::SiteService;
use crate::application::template_service::TemplateService;
use crate::application::analytics_service::AnalyticsService;
use crate::infrastructure::repository::SiteRepository;
use crate::infrastructure::p2p_store::P2pandaClient;
use crate::infrastructure::media_processor::MediaProcessor;
use crate::web::graphql::{WebsiteBuilderQuery, WebsiteBuilderMutation, WebsiteBuilderSubscription};
use crate::web::routes::create_website_builder_router;

// This struct holds all the pieces the backend needs from this module
pub struct WebsiteBuilderModule {
    pub router: Router,
    pub query: WebsiteBuilderQuery,
    pub mutation: WebsiteBuilderMutation,
    pub subscription: WebsiteBuilderSubscription,
}

// This function initializes the module and its dependencies
pub fn initialize(db_pool: PgPool) -> WebsiteBuilderModule {
    // Initialize infrastructure components
    let db_pool = Arc::new(db_pool);
    let site_repository = Arc::new(SiteRepository::new(db_pool.clone()));
    let _p2p_client = P2pandaClient::new();
    let _media_processor = MediaProcessor::new();

    // Initialize application services
    let template_service = Arc::new(TemplateService::new(site_repository.clone()));
    let site_service = Arc::new(SiteService::new(
        site_repository.clone(),
        template_service.clone(),
    ));
    let analytics_service = Arc::new(AnalyticsService::new(site_repository.clone()));

    // Initialize web components
    let router = create_website_builder_router(
        (*site_service).clone(),
        (*template_service).clone(),
        (*analytics_service).clone(),
    );

    let query = WebsiteBuilderQuery;
    let mutation = WebsiteBuilderMutation;
    let subscription = WebsiteBuilderSubscription;

    WebsiteBuilderModule {
        router,
        query,
        mutation,
        subscription,
    }
}