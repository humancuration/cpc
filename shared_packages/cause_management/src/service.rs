//! Service implementation for Cause Management
//!
//! This module provides the gRPC service implementation for managing causes
//! within the CPC platform.

use crate::models::{
    Cause, CreateCauseRequest, UpdateCauseRequest, ListCausesRequest, 
    ListCausesResponse, CauseError
};
use crate::repository::CauseRepository;
use tonic::{Request, Response, Status};
use tracing::{info, error};
use uuid::Uuid;
use rust_decimal::Decimal;
use std::str::FromStr;

// Include gRPC generated code
#[allow(clippy::derive_partial_eq_without_eq)]
pub mod proto {
    tonic::include_proto!("cpay");
}

use proto::{
    cause_service_server::CauseService,
    CreateCauseRequest as ProtoCreateCauseRequest,
    CreateCauseResponse as ProtoCreateCauseResponse,
    GetCauseRequest as ProtoGetCauseRequest,
    GetCauseResponse as ProtoGetCauseResponse,
    UpdateCauseRequest as ProtoUpdateCauseRequest,
    UpdateCauseResponse as ProtoUpdateCauseResponse,
    DeleteCauseRequest as ProtoDeleteCauseRequest,
    DeleteCauseResponse as ProtoDeleteCauseResponse,
    ListCausesRequest as ProtoListCausesRequest,
    ListCausesResponse as ProtoListCausesResponse,
    Cause as ProtoCause,
};

/// Cause management service implementation
#[derive(Debug, Clone)]
pub struct CauseServiceImpl {
    cause_repository: std::sync::Arc<dyn CauseRepository>,
}

impl CauseServiceImpl {
    /// Create a new cause service
    pub fn new(cause_repository: std::sync::Arc<dyn CauseRepository>) -> Self {
        Self {
            cause_repository,
        }
    }
    
    /// Convert a proto cause to an internal cause model
    fn proto_cause_to_cause(cause: &ProtoCause) -> Result<Cause, Status> {
        let id = Uuid::parse_str(&cause.id)
            .map_err(|e| Status::invalid_argument(format!("Invalid cause ID: {}", e)))?;
        
        let total_donations = Decimal::from_str(&cause.total_donations)
            .map_err(|e| Status::invalid_argument(format!("Invalid total donations: {}", e)))?;
        
        Ok(Cause {
            id,
            name: cause.name.clone(),
            description: cause.description.clone(),
            image_url: if cause.image_url.is_empty() { None } else { Some(cause.image_url.clone()) },
            total_donations,
            created_at: chrono::Utc::now(), // This would be set from the database in real implementation
            updated_at: chrono::Utc::now(), // This would be set from the database in real implementation
        })
    }
    
    /// Convert an internal cause model to a proto cause
    fn cause_to_proto_cause(cause: &Cause) -> ProtoCause {
        ProtoCause {
            id: cause.id.to_string(),
            name: cause.name.clone(),
            description: cause.description.clone(),
            image_url: cause.image_url.clone().unwrap_or_default(),
            total_donations: cause.total_donations.to_string(),
        }
    }
}

#[async_trait::async_trait]
impl CauseService for CauseServiceImpl {
    /// Create a new cause
    async fn create_cause(
        &self,
        request: Request<ProtoCreateCauseRequest>,
    ) -> Result<Response<ProtoCreateCauseResponse>, Status> {
        let proto_request = request.into_inner();
        info!("Creating cause: {}", proto_request.name);
        
        let cause = Cause::new(
            proto_request.name,
            proto_request.description,
            if proto_request.image_url.is_some() && !proto_request.image_url.as_ref().unwrap().is_empty() {
                proto_request.image_url
            } else {
                None
            },
        );
        
        self.cause_repository
            .create_cause(&cause)
            .await
            .map_err(|e| {
                error!("Failed to create cause: {}", e);
                Status::internal("Failed to create cause")
            })?;
        
        let proto_cause = Self::cause_to_proto_cause(&cause);
        
        let response = ProtoCreateCauseResponse {
            cause: proto_cause,
        };
        
        Ok(Response::new(response))
    }
    
    /// Get a specific cause
    async fn get_cause(
        &self,
        request: Request<ProtoGetCauseRequest>,
    ) -> Result<Response<ProtoGetCauseResponse>, Status> {
        let proto_request = request.into_inner();
        info!("Getting cause: {}", proto_request.cause_id);
        
        let cause_id = Uuid::parse_str(&proto_request.cause_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid cause ID: {}", e)))?;
        
        let cause = self.cause_repository
            .find_cause_by_id(cause_id)
            .await
            .map_err(|e| {
                error!("Failed to get cause: {}", e);
                Status::internal("Failed to get cause")
            })?;
        
        match cause {
            Some(cause) => {
                let proto_cause = Self::cause_to_proto_cause(&cause);
                let response = ProtoGetCauseResponse {
                    cause: proto_cause,
                };
                Ok(Response::new(response))
            }
            None => Err(Status::not_found("Cause not found")),
        }
    }
    
    /// Update a cause
    async fn update_cause(
        &self,
        request: Request<ProtoUpdateCauseRequest>,
    ) -> Result<Response<ProtoUpdateCauseResponse>, Status> {
        let proto_request = request.into_inner();
        info!("Updating cause: {}", proto_request.cause_id);
        
        let cause_id = Uuid::parse_str(&proto_request.cause_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid cause ID: {}", e)))?;
        
        let mut cause = self.cause_repository
            .find_cause_by_id(cause_id)
            .await
            .map_err(|e| {
                error!("Failed to find cause: {}", e);
                Status::internal("Failed to find cause")
            })?
            .ok_or_else(|| Status::not_found("Cause not found"))?;
        
        cause.update(
            proto_request.name,
            proto_request.description,
            if proto_request.image_url.is_some() && !proto_request.image_url.as_ref().unwrap().is_empty() {
                proto_request.image_url
            } else {
                None
            },
        );
        
        self.cause_repository
            .update_cause(&cause)
            .await
            .map_err(|e| {
                error!("Failed to update cause: {}", e);
                Status::internal("Failed to update cause")
            })?;
        
        let proto_cause = Self::cause_to_proto_cause(&cause);
        
        let response = ProtoUpdateCauseResponse {
            cause: proto_cause,
        };
        
        Ok(Response::new(response))
    }
    
    /// Delete a cause
    async fn delete_cause(
        &self,
        request: Request<ProtoDeleteCauseRequest>,
    ) -> Result<Response<ProtoDeleteCauseResponse>, Status> {
        let proto_request = request.into_inner();
        info!("Deleting cause: {}", proto_request.cause_id);
        
        let cause_id = Uuid::parse_str(&proto_request.cause_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid cause ID: {}", e)))?;
        
        self.cause_repository
            .delete_cause(cause_id)
            .await
            .map_err(|e| {
                error!("Failed to delete cause: {}", e);
                Status::internal("Failed to delete cause")
            })?;
        
        let response = ProtoDeleteCauseResponse {
            success: true,
        };
        
        Ok(Response::new(response))
    }
    
    /// List causes
    async fn list_causes(
        &self,
        request: Request<ProtoListCausesRequest>,
    ) -> Result<Response<ProtoListCausesResponse>, Status> {
        let proto_request = request.into_inner();
        info!("Listing causes");
        
        let internal_request = ListCausesRequest {
            limit: proto_request.limit,
            offset: proto_request.offset,
        };
        
        let result = self.cause_repository
            .list_causes(internal_request)
            .await
            .map_err(|e| {
                error!("Failed to list causes: {}", e);
                Status::internal("Failed to list causes")
            })?;
        
        let proto_causes: Vec<ProtoCause> = result.causes
            .into_iter()
            .map(|c| Self::cause_to_proto_cause(&c))
            .collect();
        
        let response = ProtoListCausesResponse {
            causes: proto_causes,
            total_count: result.total_count,
        };
        
        Ok(Response::new(response))
    }
    
    /// Get featured causes
    async fn get_featured_causes(
        &self,
        _request: Request<proto::FeaturedCausesRequest>,
    ) -> Result<Response<proto::FeaturedCausesResponse>, Status> {
        // For now, return all causes as featured
        // In a real implementation, this would have specific logic for featured causes
        let result = self.cause_repository
            .list_causes(ListCausesRequest {
                limit: Some(10),
                offset: Some(0),
            })
            .await
            .map_err(|e| {
                error!("Failed to get featured causes: {}", e);
                Status::internal("Failed to get featured causes")
            })?;
        
        let proto_causes: Vec<ProtoCause> = result.causes
            .into_iter()
            .map(|c| Self::cause_to_proto_cause(&c))
            .collect();
        
        let response = proto::FeaturedCausesResponse {
            causes: proto_causes,
        };
        
        Ok(Response::new(response))
    }
}