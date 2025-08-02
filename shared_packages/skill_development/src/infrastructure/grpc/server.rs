use tonic::transport::Server;
use sqlx::PgPool;

use crate::application::{SkillTrackingService, LearningPathService, CertificationService};
use crate::infrastructure::grpc::service::SkillDevelopmentServiceImpl;

tonic::include_proto!("skill_development");

pub struct GrpcServer {
    pool: PgPool,
    address: String,
}

impl GrpcServer {
    pub fn new(pool: PgPool, address: String) -> Self {
        Self { pool, address }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let skill_tracking_service = SkillTrackingService::new(self.pool.clone());
        let learning_path_service = LearningPathService::new(self.pool.clone());
        let certification_service = CertificationService::new(self.pool.clone());

        let service = SkillDevelopmentServiceImpl::new(
            skill_tracking_service,
            learning_path_service,
            certification_service,
        );

        let service = skill_development_server::SkillDevelopmentServer::new(service);

        println!("Starting gRPC server on {}", self.address);
        Server::builder()
            .add_service(service)
            .serve(self.address.parse()?)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;
    use std::env;

    #[tokio::test]
    async fn test_server_creation() {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://skill_dev_user:secure_password@localhost/skill_dev_test_db".to_string());
        
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to connect to test database");

        let server = GrpcServer::new(pool, "127.0.0.1:50051".to_string());
        // We won't actually start the server in tests, just verify it can be created
        assert_eq!(server.address, "127.0.0.1:50051");
    }
}