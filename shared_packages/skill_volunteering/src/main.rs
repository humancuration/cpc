//! Server for the Skill Volunteering service.

use skill_volunteering::{
    postgres::{
        opportunity_repository::PostgresOpportunityRepository,
        skill_repository::PostgresSkillRepository,
        user_skill_repository::PostgresUserSkillRepository,
    },
    proto::skill_volunteering_service_server::SkillVolunteeringServiceServer,
    service::SkillVolunteeringServiceImpl,
    opportunity_management::service::OpportunityService,
    skill_management::service::SkillService,
    user_skill_management::service::UserSkillService,
};
use std::sync::Arc;
use tonic::transport::Server;
use sqlx::PgPool;
use cause_management::proto::cause_service_client::CauseServiceClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Initialize logging
    println!("Starting Skill Volunteering service...");

    // TODO: Get database URL from config/env
    let database_url = "postgres://user:password@localhost/cpc";
    let pool = PgPool::connect(database_url).await?;

    // TODO: Get CauseService URL from config/env
    let cause_service_url = "http://[::1]:50051";
    let cause_service_client = CauseServiceClient::connect(cause_service_url).await?;

    // Initialize repositories
    let skill_repo = Arc::new(PostgresSkillRepository::new(pool.clone()));
    let opp_repo = Arc::new(PostgresOpportunityRepository::new(pool.clone()));
    let user_skill_repo = Arc::new(PostgresUserSkillRepository::new(pool.clone()));

    // Initialize services
    let skill_service = Arc::new(SkillService::new(skill_repo));
    let user_skill_service = Arc::new(UserSkillService::new(user_skill_repo));
    let opp_service = OpportunityService::new(opp_repo, skill_service.clone(), cause_service_client);

    // Initialize the gRPC service
    let service = SkillVolunteeringServiceImpl::new(opp_service, skill_service, user_skill_service);

    // TODO: Get server address from config/env
    let addr = "[::1]:50052".parse()?;
    
    println!("Skill Volunteering service listening on {}", addr);

    Server::builder()
        .add_service(SkillVolunteeringServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}