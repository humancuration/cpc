//! # Cause Management
//!
//! Cause management functionality for the CPC platform.
//!
//! This crate provides the core business logic for managing causes,
//! including creation, updating, deletion, and listing of causes
//! for donations within the CPC ecosystem.

pub mod models;
pub mod repository;
pub mod service;

// Include gRPC generated code
#[allow(clippy::derive_partial_eq_without_eq)]
pub mod proto {
    tonic::include_proto!("cpay");
}

use tracing::info;
use tonic::transport::Server;

/// Main service trait for Cause Management functionality
#[async_trait::async_trait]
pub trait CauseManagementService: Clone + Send + Sync + 'static {
    /// Start the gRPC server for internal service communication
    async fn start_grpc_server(&self, addr: std::net::SocketAddr) -> Result<(), Box<dyn std::error::Error>>;
}

/// Implementation of the CauseManagementService
#[derive(Clone)]
pub struct CauseManagementServiceImpl {
    cause_service: std::sync::Arc<service::CauseServiceImpl>,
}

impl CauseManagementServiceImpl {
    /// Create a new Cause Management service instance
    pub fn new(
        cause_service: std::sync::Arc<service::CauseServiceImpl>,
    ) -> Self {
        Self {
            cause_service,
        }
    }
}

#[async_trait::async_trait]
impl CauseManagementService for CauseManagementServiceImpl {
    async fn start_grpc_server(&self, addr: std::net::SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting Cause Management gRPC server on {}", addr);
        
        let svc = proto::cpay_service_server::CpayServiceServer::new(self.cause_service.clone());
        
        Server::builder()
            .add_service(svc)
            .serve(addr)
            .await?;
            
        Ok(())
    }
}