use tonic::{Request, Response, Status};
use crate::error::AppError;

// Import the generated protobuf types
tonic::include_proto!("grpc.health.v1");

pub struct HealthService;

impl HealthService {
    pub fn new() -> Self {
        Self
    }
}

#[tonic::async_trait]
impl health_server::Health for HealthService {
    async fn check(
        &self,
        _request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        let response = HealthCheckResponse {
            status: health_check_response::ServingStatus::Serving as i32,
        };
        
        Ok(Response::new(response))
    }

    async fn watch(
        &self,
        _request: Request<HealthCheckRequest>,
    ) -> Result<Response<tonic::Streaming<HealthCheckResponse>>, Status> {
        // For simplicity, we're not implementing the watch functionality
        Err(Status::unimplemented("Watch not implemented"))
    }
}