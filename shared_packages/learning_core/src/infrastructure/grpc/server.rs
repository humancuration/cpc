use tonic::transport::Server;
use sqlx::PgPool;

use crate::application::LearningPlatformService;
use crate::infrastructure::repositories::{CourseRepositoryImpl, EnrollmentRepositoryImpl, CredentialRepositoryImpl, TipRepositoryImpl};
use crate::infrastructure::grpc::service::LearningPlatformServiceImpl;

tonic::include_proto!("learning_platform");

pub struct GrpcServer {
    pool: PgPool,
    address: String,
}

impl GrpcServer {
    pub fn new(pool: PgPool, address: String) -> Self {
        Self { pool, address }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let course_repo = Box::new(CourseRepositoryImpl::new(self.pool.clone()));
        let enrollment_repo = Box::new(EnrollmentRepositoryImpl::new(self.pool.clone()));
        let credential_repo = Box::new(CredentialRepositoryImpl::new(self.pool.clone()));
        let tip_repo = Box::new(TipRepositoryImpl::new(self.pool.clone()));

        let learning_service = LearningPlatformService::new(
            course_repo,
            enrollment_repo,
            credential_repo,
            tip_repo,
        );

        let service = LearningPlatformServiceImpl::new(learning_service);
        let service = learning_platform_server::LearningPlatformServer::new(service);

        println!("Starting gRPC server on {}", self.address);
        Server::builder()
            .add_service(service)
            .serve(self.address.parse()?)
            .await?;

        Ok(())
    }
}