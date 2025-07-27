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
mod auth;
mod config;
mod middleware;
mod integration_docs;
pub mod project;
// Import the website builder module
use cpc_website_builder::web::module as website_builder;

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
use crate::auth::{AuthState, auth_middleware, optional_auth_middleware, create_rate_limiter, create_security_middleware};
use crate::config::Config;
use crate::middleware::{SecurityMiddleware, SecurityConfig, security_middleware, request_id_middleware, logging_middleware};
use async_graphql::dataloader::DataLoader;
use cpc_core::repositories::product_repository::{ProductRepository, PgProductRepository};
use cpc_core::repositories::user_repository::{UserRepository, PgUserRepository};
use cpc_core::repositories::social_repository::{SocialRepository, PgSocialRepository};
use cpc_core::repositories::forum_repository::{ForumRepository, PgForumRepository};
use cpc_core::repositories::governance_repository::{GovernanceRepository, PgGovernanceRepository};
use cpc_core::repositories::project_repository::ProjectRepository;
use cpc_core::business::impact::DefaultImpactCalculator;
use cpc_core::services::identity::IdentityService;
use cpc_core::services::social::SocialService;
use cpc_core::services::forum::ForumService;
use cpc_core::services::governance::GovernanceService;
use cpc_protos::barcode::v1::barcode_service_server::BarcodeServiceServer;
use cpc_protos::expenses::expense_processing_server::ExpenseProcessingServer;
use crate::supply_chain::service::SupplyChainService;
use cpc_core::financial_forecasting::service::FinancialForecastingService;
use cpc_core::expenses::service::ExpenseService;
use cpc_core::finance::royalty_service::RoyaltyService;
use cpc_core::finance::transactions::TransactionLedger;
use crate::repositories::supply_chain_repository::SupplyChainRepositoryImpl;
use cpc_net::community_repo::CommunityRepo;
// use p2panda::prelude::NodeClient; // Package not available
use axum::middleware;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    // Set UI degradation threshold for impact reporting (0.15 = 15%)
    std::env::set_var("CPC_UI_DEGRADATION_THRESHOLD", "0.15");

    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == "--export-schema-json") {
        export_schema().await;
        return;
    }

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");

    // Initialize database
    let db = init_db()
        .await
        .expect("Failed to initialize database");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Failed to run database migrations");

    // Initialize auth state
    let auth_state = Arc::new(AuthState::new());

    // Initialize asset services
    let asset_storage = Arc::new(AssetStorageService::new(db.clone()).expect("Failed to initialize asset storage"));
    let asset_preview = Arc::new(AssetPreviewService::new(asset_storage.clone()));

    // See docs/architecture/impact-service.md for implementation details
    let impact_service = Arc::new(ImpactService::new(db.clone()));
    
    // Initialize impact calculator
    let impact_calculator = Arc::new(DefaultImpactCalculator::new(db.clone()));
    
    // Initialize feature flags with config value
    let feature_flags = Arc::new(FeatureFlags {
        impact_real_data_enabled: false, // Keep existing default
        ui_degradation_threshold: config.ui_degradation_threshold,
    });
    
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
    // let p2panda_client = Arc::new(NodeClient::new("http://localhost:2020", None).expect("Failed to create p2panda client"));
    // let community_repo = Arc::new(CommunityRepo::new(p2panda_client));
    // For now, create a mock community repo
    let community_repo = Arc::new(CommunityRepo::new_mock());

    // Initialize new repositories for android-rust-migration
    let user_repository = Arc::new(PgUserRepository::new(db.clone()));
    let social_repository = Arc::new(PgSocialRepository::new(db.clone()));
    let forum_repository = Arc::new(PgForumRepository::new(db.clone()));
    let governance_repository = Arc::new(PgGovernanceRepository::new(db.clone()));

    // Initialize new services for android-rust-migration
    let identity_service = Arc::new(IdentityService::new(user_repository.clone(), config.jwt_secret.clone())
        .expect("Failed to initialize identity service"));
    let social_service = Arc::new(SocialService::new(social_repository.clone(), user_repository.clone()));
    let forum_service = Arc::new(ForumService::new(forum_repository.clone(), user_repository.clone()));
    let governance_service = Arc::new(GovernanceService::new(governance_repository.clone(), user_repository.clone()));
    let project_repository = Arc::new(ProjectRepository::new(db.clone()));
    let project_service = Arc::new(project::service::ProjectService::new(project_repository.clone()));

    // Initialize finance services
    let ledger = cpc_core::finance::transactions::InMemoryLedger::new();
    let engine = Arc::new(cpc_core::finance::royalty_engine::RoyaltyEngine::new(ledger.clone()));
    let royalty_service = Arc::new(RoyaltyService::new(engine));

   // Define feature flags structure
   #[derive(Clone)]
   pub struct FeatureFlags {
       pub impact_real_data_enabled: bool,
       pub ui_degradation_threshold: f64,
   }

   impl FeatureFlags {
       fn new() -> Self {
           Self {
               impact_real_data_enabled: false, // Default to false for backward compatibility
               ui_degradation_threshold: 0.1, // Default to 10%
           }
       }
   }

   // Initialize security middleware
    let security_config = SecurityConfig::default();
    let security_middleware_service = Arc::new(SecurityMiddleware::new(security_config));

    // Initialize Dataloaders
    let customer_loader = DataLoader::new(CustomerLoader { pool: db.clone() }, tokio::spawn);
// Initialize website builder module
    let website_builder_module = website_builder::initialize(db.clone());

    // Create royalty subscription channel
    let (royalty_tx, royalty_rx) = tokio::sync::mpsc::unbounded_channel();
    
    // Build GraphQL schema
    let schema = Schema::build(RootQuery::default(), RootMutation::default(), RootSubscription::default())
        .data(db.clone())
        .data(customer_loader)
        .data(bi_service.clone())
        .data(asset_storage.clone())
        .data(asset_preview.clone())
        .data(impact_service.clone())
        .data(impact_calculator.clone())
        .data(feature_flags.clone())
        .data(supply_chain_service.clone())
        .data(financial_forecasting_service.clone())
        .data(expense_service.clone())
        .data(community_repo.clone())
        // Add new services for android-rust-migration
        .data(identity_service.clone())
        .data(social_service.clone())
        .data(forum_service.clone())
        .data(governance_service.clone())
       .data(project_service.clone())
// Add website builder components
        .data(website_builder_module.query)
        .data(website_builder_module.mutation)
        .data(website_builder_module.subscription)
        // Add finance services
        .data(royalty_service.clone())
        .data(royalty_rx)
        // Note: SimpleBroker is not a service, it does not need to be Arc-wrapped
        // and it is thread-safe. It's added directly to the schema data.
        .finish()
        .expect("Failed to build GraphQL schema");

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(routes::health_check))
        .route("/graphql", post(graphql_handler))
        .route("/graphql", get(graphql_playground))
.nest("/website-builder", website_builder_module.router)
        .nest("/api", create_api_router(
            social_service.clone(),
            forum_service.clone(),
            governance_service.clone(),
            identity_service.clone(),
            auth_state.clone(),
        ))
        .nest("/bi", BIService::router(bi_service.clone()))
        .with_state(schema)
        .layer(middleware::from_fn_with_state(
            security_middleware_service.clone(),
            security_middleware,
        ))
        .layer(middleware::from_fn(request_id_middleware))
        .layer(middleware::from_fn(logging_middleware))
        .layer(create_rate_limiter())
        .layer(create_security_middleware());

    // Create gRPC router
    let grpc_router = Server::builder()
        .add_service(BarcodeServiceServer::new(barcode_service))
        .add_service(ExpenseProcessingServer::new(ExpenseProcessingService::new(expense_service.clone())))
        .into_router();

    // Combine Axum and gRPC routers
    let app = app
        .merge(grpc_router);

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    println!("Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn create_api_router(
    social_service: Arc<SocialService>,
    forum_service: Arc<ForumService>,
    governance_service: Arc<GovernanceService>,
    identity_service: Arc<IdentityService>,
    auth_state: Arc<AuthState>,
) -> Router {
    Router::new()
        .route("/upload", post(routes::upload::upload_image))
        .merge(routes::publish::router())
        .merge(routes::update::router())
        .merge(routes::impact::router())
        // Add authentication routes (no auth middleware needed for login/register)
        .nest("/auth",
            routes::auth::router()
                .with_state((identity_service.clone(), auth_state.clone()))
        )
        // Add new routes for android-rust-migration with authentication
        .nest("/social",
            routes::social::router()
                .with_state(social_service)
                .layer(middleware::from_fn_with_state(
                    auth_state.clone(),
                    auth_middleware,
                ))
        )
        .nest("/forum",
            routes::forum::router()
                .with_state(forum_service)
                .layer(middleware::from_fn_with_state(
                    auth_state.clone(),
                    optional_auth_middleware,
                ))
        )
        .nest("/governance",
            routes::governance::router()
                .with_state(governance_service)
                .layer(middleware::from_fn_with_state(
                    auth_state.clone(),
                    auth_middleware,
                ))
        )
        // Add vendor routes with authentication
        .nest("/vendor",
            routes::vendor::router()
                .layer(middleware::from_fn_with_state(
                    auth_state.clone(),
                    auth_middleware,
                ))
        )
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

    // Initialize new repositories and services for android-rust-migration
    let user_repository = Arc::new(PgUserRepository::new(db.clone()));
    let social_repository = Arc::new(PgSocialRepository::new(db.clone()));
    let forum_repository = Arc::new(PgForumRepository::new(db.clone()));
    let governance_repository = Arc::new(PgGovernanceRepository::new(db.clone()));
    let identity_service = Arc::new(IdentityService::new(user_repository.clone(), config.jwt_secret.clone())
        .expect("Failed to initialize identity service"));
    let social_service = Arc::new(SocialService::new(social_repository.clone(), user_repository.clone()));
    let forum_service = Arc::new(ForumService::new(forum_repository.clone(), user_repository.clone()));
    let governance_service = Arc::new(GovernanceService::new(governance_repository.clone(), user_repository.clone()));
   let project_repository = Arc::new(ProjectRepository::new(db.clone()));
   let project_service = Arc::new(project::service::ProjectService::new(project_repository.clone()));

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
        .data(identity_service.clone())
        .data(social_service.clone())
        .data(forum_service.clone())
        .data(governance_service.clone())
       .data(project_service.clone())
        .finish()
        .expect("Failed to build GraphQL schema");

    let schema_json = serde_json::to_string_pretty(&schema.execute(async_graphql::IntrospectionQuery::new()).await).unwrap();
    fs::write("../../apps/cpc-platform/src/graphql/schema.json", schema_json).expect("Unable to write schema file");

    println!("GraphQL schema exported to ../../apps/cpc-platform/src/graphql/schema.json");
}