use std::net::SocketAddr;
use tonic::transport::Server;
use sqlx::PgPool;

// Import our services
use crate::grpc::{
    course_service::CourseService,
    enrollment_service::EnrollmentService,
    credential_service::CredentialService,
    tip_service::TipService,
    auth_service::AuthService,
    user_service::UserService,
    health_service::HealthService,
};
use crate::database::repository::DatabaseRepository;
use crate::config::AppConfig;
use crate::logging;

// Include the generated protobuf code
tonic::include_proto!("cpc.learning_platform");
tonic::include_proto!("cpc.learning_platform_server");
tonic::include_proto!("grpc.health.v1");

pub mod database;
pub mod middleware;
pub mod grpc;
pub mod config;
pub mod logging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    logging::init_logging();
    
    // Load configuration
    let config = AppConfig::from_env()?;
    
    // Create database connection pool
    log_info!("Connecting to database: {}", config.database_url);
    let pool = PgPool::connect(&config.database_url).await?;
    
    // Run migrations
    log_info!("Running database migrations");
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    // Create repository
    let repository = DatabaseRepository::new(pool);
    
    // Create services
    let course_service = CourseService::new(repository.clone());
    let enrollment_service = EnrollmentService::new(repository.clone());
    let credential_service = CredentialService::new(repository.clone());
    let tip_service = TipService::new(repository.clone());
    let auth_service = AuthService::new(repository.clone());
    let user_service = UserService::new(repository.clone());
    let health_service = HealthService::new();
    
    // Create gRPC server
    log_info!("Starting gRPC server on {}", config.server_addr);
    
    Server::builder()
        .add_service(health_server::HealthServer::new(health_service))
        .add_service(course_service_server::CourseServiceServer::new(course_service))
        .add_service(enrollment_service_server::EnrollmentServiceServer::new(enrollment_service))
        .add_service(credential_service_server::CredentialServiceServer::new(credential_service))
        .add_service(tip_service_server::TipServiceServer::new(tip_service))
        .add_service(auth_service_server::AuthServiceServer::new(auth_service))
        .add_service(user_service_server::UserServiceServer::new(user_service))
        .serve(config.server_addr)
        .await?;
    
    Ok(())
}