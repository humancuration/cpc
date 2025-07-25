use axum::{
    extract::State,
    routing::{get, post},
    Router,
};
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use std::fs;
use tonic::transport::Server;

mod db;
mod routes;
mod services;
mod bi;
mod scheduled_jobs;
mod graphql;
mod invoicing;
mod expenses;
mod repositories;
mod supply_chain;

use crate::db::{DbPool, init_db};
use crate::bi::{BIService, BIConfig};
use crate::scheduled_jobs::start_scheduled_jobs;
use crate::services::asset_storage::AssetStorageService;
use crate::services::asset_preview::AssetPreviewService;
use crate::services::impact::ImpactService;
use crate::services::barcode::BarcodeServiceImpl;
use crate::graphql::schema::{Schema, RootQuery, RootMutation, RootSubscription};
use crate::invoicing::graphql::CustomerLoader;
use crate::expenses::grpc::ExpenseProcessingService;
use async_graphql::dataloader::DataLoader;
use cpc_core::repositories::product_repository::{ProductRepository, PgProductRepository};
use cpc_protos::barcode::v1::barcode_service_server::BarcodeServiceServer;
use cpc_protos::expenses::expense_processing_server::ExpenseProcessingServer;
use crate::supply_chain::service::SupplyChainService;
use cpc_core::financial_forecasting::service::FinancialForecastingService;
use cpc_core::expenses::service::ExpenseService;
use crate::repositories::supply_chain_repository::SupplyChainRepositoryImpl;
use cpc_net::community_repo::CommunityRepo;
use p2panda::prelude::NodeClient;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == "--export-schema-json") {
        export_schema().await;
        return;
    }

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
    let supply_chain_repo = Arc::new(SupplyChainRepositoryImpl::new(db.clone()));
    let supply_chain_service = Arc::new(SupplyChainService::new(supply_chain_repo));
    let financial_forecasting_service = Arc::new(FinancialForecastingService::new());
    let expense_service = expenses::create_service(db.clone());

    // Initialize p2panda client and CommunityRepo
    // In a real application, this endpoint would come from configuration
    let p2panda_client = Arc::new(NodeClient::new("http://localhost:2020", None).expect("Failed to create p2panda client"));
    let community_repo = Arc::new(CommunityRepo::new(p2panda_client));

    // Initialize Dataloaders
    let customer_loader = DataLoader::new(CustomerLoader { pool: db.clone() }, tokio::spawn);

    // Build GraphQL schema
    let schema = Schema::build(RootQuery::default(), RootMutation::default(), RootSubscription::default())
        .data(db.clone())
        .data(customer_loader)
        .data(bi_service.clone())
        .data(asset_storage.clone())
        .data(asset_preview.clone())
        .data(impact_service.clone())
        .data(supply_chain_service.clone())
        .data(financial_forecasting_service.clone())
        .data(expense_service.clone())
        .data(community_repo.clone())
        // Note: SimpleBroker is not a service, it does not need to be Arc-wrapped
        // and it is thread-safe. It's added directly to the schema data.
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
        .add_service(ExpenseProcessingServer::new(ExpenseProcessingService::new(expense_service.clone())))
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

async fn export_schema() {
    let db = init_db().await.expect("Failed to initialize database");
    let asset_storage = Arc::new(AssetStorageService::new(db.clone()).expect("Failed to initialize asset storage"));
    let asset_preview = Arc::new(AssetPreviewService::new(asset_storage.clone()));
    let impact_service = Arc::new(ImpactService::new(db.clone()));
    let bi_config = BIConfig {
        data_sources: vec![],
        processing_timeout: 300,
        cache_duration: 3600,
    };
    let bi_service = Arc::new(BIService::new(bi_config));
    let supply_chain_repo = Arc::new(SupplyChainRepositoryImpl::new(db.clone()));
    let supply_chain_service = Arc::new(SupplyChainService::new(supply_chain_repo));
    let financial_forecasting_service = Arc::new(FinancialForecastingService::new());
    let expense_service = expenses::create_service(db.clone());
    let p2panda_client = Arc::new(NodeClient::new("http://localhost:2020", None).expect("Failed to create p2panda client"));
    let community_repo = Arc::new(CommunityRepo::new(p2panda_client));
    let customer_loader = DataLoader::new(CustomerLoader { pool: db.clone() }, tokio::spawn);

    let schema = Schema::build(RootQuery::default(), RootMutation::default(), RootSubscription::default())
        .data(db.clone())
        .data(customer_loader)
        .data(bi_service.clone())
        .data(asset_storage.clone())
        .data(asset_preview.clone())
        .data(impact_service.clone())
        .data(supply_chain_service.clone())
        .data(financial_forecasting_service.clone())
        .data(expense_service.clone())
        .data(community_repo.clone())
        .finish()
        .expect("Failed to build GraphQL schema");

    let schema_json = serde_json::to_string_pretty(&schema.execute(async_graphql::IntrospectionQuery::new()).await).unwrap();
    fs::write("../../apps/cpc-platform/src/graphql/schema.json", schema_json).expect("Unable to write schema file");

    println!("GraphQL schema exported to ../../apps/cpc-platform/src/graphql/schema.json");
}