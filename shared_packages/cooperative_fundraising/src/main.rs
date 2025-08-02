use std::net::SocketAddr;
use std::env;
use std::time::Duration;

use cooperative_fundraising::application::CampaignService;
use cooperative_fundraising::infrastructure::postgres::campaign_repository::PostgresCampaignRepository;
use cooperative_fundraising::infrastructure::postgres::contribution_repository::PostgresContributionRepository;
use cooperative_fundraising::infrastructure::postgres::membership_repository::PostgresMembershipRepository;
use cooperative_fundraising::CooperativeFundraisingServiceServer;

use tonic::transport::Server;
use sqlx::postgres::PgPoolOptions;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   tracing_subscriber::fmt::init();

   // Read database URL from environment (fallback for local dev)
   let database_url = env::var("DATABASE_URL")
       .unwrap_or_else(|_| "postgres://postgres:postgres@127.0.0.1:5432/cpc".to_string());

   // Create Postgres connection pool
   let pool = PgPoolOptions::new()
       .max_connections(10)
       .acquire_timeout(Duration::from_secs(10))
       .connect(&database_url)
       .await?;

   // Initialize repositories
   let campaign_repo = PostgresCampaignRepository::new(pool.clone());
   let contribution_repo = PostgresContributionRepository::new(pool.clone());
   let membership_repo = PostgresMembershipRepository::new(pool.clone());

   // Wire repositories into CampaignService
   let _campaign_service = CampaignService::new(
       Box::new(campaign_repo),
       Box::new(contribution_repo),
       Box::new(membership_repo),
   );

   // gRPC bind address
   let addr: SocketAddr = env::var("COOP_FUNDRAISING_GRPC_ADDR")
       .unwrap_or_else(|_| "127.0.0.1:50055".to_string())
       .parse()?;

   info!("Starting Cooperative Fundraising gRPC server on {}", addr);

   // Minimal placeholder service implementation until full methods are wired.
   // This compiles and allows the server to start; methods should be implemented
   // in the proper infrastructure layer later.
   struct GrpcServiceImpl;

   #[tonic::async_trait]
   impl cooperative_fundraising::proto::cooperative_fundraising_service_server::CooperativeFundraisingService for GrpcServiceImpl {}

   Server::builder()
       .add_service(CooperativeFundraisingServiceServer::new(GrpcServiceImpl))
       .serve(addr)
       .await?;

   Ok(())
}