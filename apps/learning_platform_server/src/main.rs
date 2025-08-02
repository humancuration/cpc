use std::net::SocketAddr;
use tonic::transport::Server;
use sqlx::PgPool;
use tracing::{info, error};

// Import our services
use crate::grpc::{
    course_service::CourseService,
    enrollment_service::EnrollmentService,
    credential_service::CredentialService,
    tip_service::TipService,
    auth_service::AuthService,
};
use crate::database::repository::DatabaseRepository;

// Include the generated protobuf code
tonic::include_proto!("cpc.learning_platform");

pub mod database;
pub mod middleware;
pub mod grpc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    // Load configuration
    let config = config::Config::builder()
        .add_source(config::Environment::with_prefix("APP"))
        .build()?;
    
    let database_url = config.get_string("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost/learning_platform".to_string());
    
    let server_addr: SocketAddr = config.get_string("SERVER_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:50051".to_string())
        .parse()?;
    
    // Create database connection pool
    info!("Connecting to database: {}", database_url);
    let pool = PgPool::connect(&database_url).await?;
    
    // Run migrations
    info!("Running database migrations");
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    // Create repository
    let repository = DatabaseRepository::new(pool);
    
    // Create services
    let course_service = CourseService::new(repository.clone());
    let enrollment_service = EnrollmentService::new(repository.clone());
    let credential_service = CredentialService::new(repository.clone());
    let tip_service = TipService::new(repository.clone());
    let auth_service = AuthService::new(repository.clone());
    
    // Create gRPC server
    info!("Starting gRPC server on {}", server_addr);
    
    Server::builder()
        .add_service(course_service_server::CourseServiceServer::new(course_service))
        .add_service(enrollment_service_server::EnrollmentServiceServer::new(enrollment_service))
        .add_service(credential_service_server::CredentialServiceServer::new(credential_service))
        .add_service(tip_service_server::TipServiceServer::new(tip_service))
        .add_service(auth_service_server::AuthServiceServer::new(auth_service))
        .serve(server_addr)
        .await?;
    
    Ok(())
}