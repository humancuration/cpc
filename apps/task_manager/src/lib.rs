pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod web;

use std::sync::Arc;
use sqlx::PgPool;
use crate::application::service::TaskServiceImpl;
use crate::infrastructure::repository::PostgresTaskRepository;
use crate::infrastructure::notification::NotificationService;
use crate::infrastructure::p2p_sync::P2pSyncAdapter;
use crate::web::routes::configure_routes;

pub struct TaskManagerModule {
    pub service: Arc<dyn crate::application::ports::TaskService>,
    pub schema: crate::web::graphql::ServiceSchema,
}

pub async fn initialize(
    pool: PgPool,
    notification_service: Arc<dyn NotificationService>,
    p2p_sync_service: Arc<dyn crate::infrastructure::p2p_sync::P2pSyncService>,
) -> Result<TaskManagerModule, Box<dyn std::error::Error>> {
    let repository = Arc::new(PostgresTaskRepository::new(pool.clone()));
    
    let service = Arc::new(TaskServiceImpl::new(
        repository,
        notification_service,
        p2p_sync_service,
    ));

    let schema = crate::web::graphql::ServiceSchema::build(
        crate::web::graphql::QueryRoot,
        crate::web::graphql::MutationRoot,
        crate::web::graphql::SubscriptionRoot,
    )
    .data(service.clone())
    .finish();

    Ok(TaskManagerModule {
        service,
        schema,
    })
}

pub fn configure_actix_routes(
    cfg: &mut actix_web::web::ServiceConfig,
    service: Arc<dyn crate::application::ports::TaskService>,
) {
    configure_routes(cfg, service);
}

// Re-export key types for integration
pub use crate::domain::models::{Task, Project, Reminder, TaskStatus, TaskPriority};
pub use crate::application::ports::TaskService;
pub use crate::infrastructure::notification::NotificationService;
pub use crate::infrastructure::p2p_sync::P2pSyncService;