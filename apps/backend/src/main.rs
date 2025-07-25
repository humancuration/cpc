use axum::{
    extract::State,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use tonic::transport::Server;

mod db;
mod routes;
mod services;
mod bi;
mod scheduled_jobs;
mod graphql;

use crate::db::{DbPool, init_db};
use crate::bi::{BIService, BIConfig};
use crate::scheduled_jobs::start_scheduled_jobs;
use crate::services::asset_storage::AssetStorageService;
use crate::services::asset_preview::AssetPreviewService;
use crate::services::impact::ImpactService;
use crate::services::barcode::BarcodeServiceImpl;
use crate::graphql::schema::{Schema, RootQuery, RootMutation, RootSubscription};
use cpc_core::repositories::product_repository::{ProductRepository, PgProductRepository};
use cpc_protos::barcode::v1::barcode_service_server::BarcodeServiceServer;
use cpc_core::supply_chain::service::SupplyChainService;
use cpc_core::financial_forecasting::service::FinancialForecastingService;
use cpc_core::expenses::service::ExpenseService;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Initialize database
    let db = init_db()
        .await
        .expect("Failed to initialize database");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Failed to run database migrations");

    // Initialize asset services
    let asset_storage = Arc::new(AssetStorageService::new(db.clone()).expect("Failed to initialize asset storage"));
    let asset_preview = Arc::new(AssetPreviewService::new(asset_storage.clone()));

    // Initialize impact service
    let impact_service = Arc::new(ImpactService::new(db.clone()));
    
    // Initialize barcode service
    let product_repository = PgProductRepository::new(db.clone());
    let barcode_service = BarcodeServiceImpl::new(product_repository);

    // Start scheduled jobs
    start_scheduled_jobs(db.clone()).await;

    // Initialize BI service
    let bi_config = BIConfig {
        data_sources: vec![],
        processing_timeout: 300,
        cache_duration: 3600,
    };
    let bi_service = Arc::new(BIService::new(bi_config));

    // Initialize core services
    let supply_chain_service = Arc::new(SupplyChainService::new());
    let financial_forecasting_service = Arc::new(FinancialForecastingService::new());
    let expense_service = Arc::new(ExpenseService::new());

    // Build GraphQL schema
    let schema = Schema::build(RootQuery::default(), RootMutation::default(), RootSubscription::default())
        .data(db.clone())
        .data(bi_service.clone())
        .data(asset_storage.clone())
        .data(asset_preview.clone())
        .data(impact_service.clone())
        .data(supply_chain_service.clone())
        .data(financial_forecasting_service.clone())
        .data(expense_service.clone())
        .finish()
        .expect("Failed to build GraphQL schema");

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(routes::health_check))
        .route("/graphql", post(graphql_handler))
        .route("/graphql", get(graphql_playground))
        .nest("/api", create_api_router())
        .nest("/bi", BIService::router(bi_service.clone()))
        .with_state(schema)
        .layer(CorsLayer::permissive());

    // Create gRPC router
    let grpc_router = Server::builder()
        .add_service(BarcodeServiceServer::new(barcode_service))
        .into_router();

    // Combine Axum and gRPC routers
    let app = app
        .merge(grpc_router);

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn create_api_router() -> Router {
    Router::new()
        .route("/upload", post(routes::upload::upload_image))
        .merge(routes::publish::router())
        .merge(routes::update::router())
        .merge(routes::impact::router())
}

async fn graphql_handler(
    State(schema): State<Schema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl axum::response::IntoResponse {
    async_graphql::http::GraphQLPlayground::build(
        async_graphql::http::GraphQLPlaygroundSource::GraphQL,
    )
    .endpoint("/graphql")
    .subscription_endpoint("/graphql")
    .finish()
}