use actix_web::{web, HttpResponse};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use std::sync::Arc;

use crate::application::ports::TaskService;
use crate::web::graphql::{QueryRoot, MutationRoot, SubscriptionRoot, ServiceSchema};

pub fn configure_routes(
    cfg: &mut web::ServiceConfig,
    service: Arc<dyn TaskService>,
) {
    let schema = ServiceSchema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(service)
        .finish();

    cfg.app_data(web::Data::new(schema.clone()))
        .service(
            web::resource("/graphql")
                .route(web::post().to(graphql_handler))
                .route(web::get().to(graphql_handler)),
        )
        .service(
            web::resource("/graphql/subscriptions")
                .route(web::get().to(GraphQLSubscription::new(schema))),
        )
        .service(
            web::resource("/health")
                .route(web::get().to(health_check)),
        );
}

async fn graphql_handler(
    schema: web::Data<ServiceSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "task_manager"
    }))
}