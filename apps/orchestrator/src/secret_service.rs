use tonic::{Request, Response, Status};
use crate::cpc_orchestrator::secret_service_server::SecretService;
use crate::cpc_orchestrator::{RotateSecretRequest, RotateSecretResponse};
use crate::secret_manager::SecretManager;
use std::sync::Arc;
use jsonwebtoken::DecodingKey;
use tracing::info;

pub struct SecretServiceImpl {
    secret_manager: Arc<SecretManager>,
}

impl SecretServiceImpl {
    pub fn new(secret_manager: Arc<SecretManager>) -> Self {
        Self { secret_manager }
    }
}

#[tonic::async_trait]
impl SecretService for SecretServiceImpl {
    async fn rotate_secret(
        &self,
        request: Request<RotateSecretRequest>,
    ) -> Result<Response<RotateSecretResponse>, Status> {
        let req = request.into_inner();
        
        // Validate admin credentials
        if req.admin_token != std::env::var("ADMIN_TOKEN").unwrap_or_default() {
            return Err(Status::permission_denied("Invalid admin token"));
        }
        
        // Validate new secret strength
        if req.new_secret.len() < 32 {
            return Err(Status::invalid_argument("Secret must be at least 32 characters"));
        }
        
        // Perform rotation
        self.secret_manager.rotate_secret(req.new_secret.clone())
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        
        info!("JWT secret rotated successfully");
        
        Ok(Response::new(RotateSecretResponse {
            success: true,
            message: "Secret rotated successfully".to_string(),
        }))
    }
}