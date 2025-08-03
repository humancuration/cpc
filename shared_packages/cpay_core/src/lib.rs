//! # CPay Core
//!
//! Core payment processing functionality for the CPC platform.
//!
//! This crate provides the core business logic for payment processing, including:
//! - Transaction processing engine
//! - Wallet management
//! - Compliance checks
//! - Integration with notifications and social features
//! - Integration with common_utils for standardized error handling, logging, crypto, and datetime
//!
//! ## Integration with common_utils
//! This module integrates with the common_utils package to provide:
//! - Standardized error handling through CommonError
//! - Unified logging through common_utils::logging
//! - Crypto functions through common_utils::crypto
//! - DateTime handling through common_utils::datetime
//!
//! This integration is now direct and no longer requires feature flags.

pub mod models;
pub mod transaction_engine;
pub mod repositories;

// Include gRPC generated code
#[allow(clippy::derive_partial_eq_without_eq)]
pub mod proto {
    tonic::include_proto!("cpay");
};

use notification_core::application::service::NotificationService;
use social_integration::application::social_integration_service::SocialIntegrationService;
use common_utils::logging::info;
use tonic::{transport::Server, Request, Response, Status};
use proto::{
    cpay_service_server::{CpayService, CpayServiceServer},
    payment_service_server::{PaymentService, PaymentServiceServer},
    PaymentRequest as ProtoPaymentRequest,
    PaymentResponse as ProtoPaymentResponse,
    TransactionHistoryRequest as ProtoTransactionHistoryRequest,
    TransactionHistoryResponse as ProtoTransactionHistoryResponse,
    FeaturedCausesRequest as ProtoFeaturedCausesRequest,
    FeaturedCausesResponse as ProtoFeaturedCausesResponse,
    SkillExchangeRatesRequest as ProtoSkillExchangeRatesRequest,
    SkillExchangeRatesResponse as ProtoSkillExchangeRatesResponse,
};
use std::str::FromStr;

/// Main service trait for CPay functionality
#[async_trait::async_trait]
pub trait CpayService: Clone + Send + Sync + 'static {
    /// Process a payment transaction
    async fn process_payment(&self, request: models::PaymentRequest) -> Result<models::PaymentResponse, models::PaymentError>;
    
    /// Get transaction history for a user
    async fn get_transaction_history(&self, user_id: uuid::Uuid) -> Result<Vec<models::Transaction>, models::PaymentError>;
    
    /// Get featured causes for donations
    async fn get_featured_causes(&self) -> Result<Vec<Cause>, models::PaymentError>;
    
    /// Get skill exchange rates for volunteer hour conversion
    async fn get_skill_exchange_rates(&self) -> Result<Vec<SkillRate>, models::PaymentError>;
}
}
#[async_trait::async_trait]
pub trait CpayService: Clone + Send + Sync + 'static {
    /// Process a payment transaction
    async fn process_payment(&self, request: models::PaymentRequest) -> Result<models::PaymentResponse, models::PaymentError>;
    
    /// Get transaction history for a user
    async fn get_transaction_history(&self, user_id: uuid::Uuid) -> Result<Vec<models::Transaction>, models::PaymentError>;
    
    /// Get featured causes for donations
    async fn get_featured_causes(&self) -> Result<Vec<Cause>, models::PaymentError>;
    
    /// Get skill exchange rates for volunteer hour conversion
    async fn get_skill_exchange_rates(&self) -> Result<Vec<SkillRate>, models::PaymentError>;
}
}

/// Implementation of the CPayService
#[derive(Clone)]
pub struct CpayServiceImpl {
    notification_service: std::sync::Arc<dyn NotificationService>,
    social_service: std::sync::Arc<dyn SocialIntegrationService>,
    transaction_engine: std::sync::Arc<transaction_engine::TransactionEngine>,
}

impl CpayServiceImpl {
    /// Create a new CPay service instance
    pub fn new(
        notification_service: std::sync::Arc<dyn NotificationService>,
        social_service: std::sync::Arc<dyn SocialIntegrationService>,
        transaction_engine: std::sync::Arc<transaction_engine::TransactionEngine>,
    ) -> Self {
        Self {
            notification_service,
            social_service,
            transaction_engine,
        }
    }
}


#[async_trait::async_trait]
impl CpayService for CpayServiceImpl {
    async fn process_payment(&self, request: models::PaymentRequest) -> Result<models::PaymentResponse, models::PaymentError> {
        info!("Processing payment for user: {}", request.user_id);
        self.transaction_engine.process_payment(request).await
    }
    
    async fn get_transaction_history(&self, user_id: uuid::Uuid) -> Result<Vec<models::Transaction>, models::PaymentError> {
        info!("Fetching transaction history for user: {}", user_id);
        self.transaction_engine.get_transaction_history(user_id).await
    }
    
    async fn get_featured_causes(&self) -> Result<Vec<Cause>, models::PaymentError> {
        info!("Fetching featured causes");
        // In a real implementation, this would fetch from a causes repository
        Ok(vec![])
    }
    
    async fn get_skill_exchange_rates(&self) -> Result<Vec<SkillRate>, models::PaymentError> {
        info!("Fetching skill exchange rates");
        // In a real implementation, this would fetch from a skills repository
        Ok(vec![])
    }
}

// Implementation of the gRPC service
#[async_trait::async_trait]
impl cpay_service_server::CpayService for CpayServiceImpl {
    async fn process_payment(
        &self,
        request: Request<ProtoPaymentRequest>,
    ) -> Result<Response<ProtoPaymentResponse>, Status> {
        let proto_request = request.into_inner();
        
        // Convert proto request to internal model
        let user_id = uuid::Uuid::parse_str(&proto_request.user_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid user ID: {}", e)))?;
            
        let recipient_id = uuid::Uuid::parse_str(&proto_request.recipient_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid recipient ID: {}", e)))?;
            
        let amount = rust_decimal::Decimal::from_str(&proto_request.amount)
            .map_err(|e| Status::invalid_argument(format!("Invalid amount: {}", e)))?;
            
        let currency = match proto_request.currency {
            0 => models::Currency::Dabloons,
            1 => models::Currency::USD,
            2 => models::Currency::EUR,
            3 => models::Currency::GBP,
            4 => models::Currency::JPY,
            _ => return Err(Status::invalid_argument("Invalid currency")),
        };
        
        let internal_request = models::PaymentRequest::new(
            user_id,
            recipient_id,
            amount,
            currency,
            proto_request.description,
            proto_request.is_public,
            proto_request.share_to_social,
            proto_request.cause_id.as_ref().and_then(|id| uuid::Uuid::parse_str(id).ok()),
            proto_request.volunteer_hours.as_ref().and_then(|vh| rust_decimal::Decimal::from_str(vh).ok()),
        );
        
        // Process the payment
        match self.process_payment(internal_request).await {
            Ok(response) => {
                let proto_response = ProtoPaymentResponse {
                    transaction_id: response.transaction_id.to_string(),
                    status: match response.status {
                        models::TransactionStatus::Pending => 0,
                        models::TransactionStatus::Completed => 1,
                        models::TransactionStatus::Failed => 2,
                        models::TransactionStatus::Cancelled => 3,
                    },
                    timestamp: response.timestamp.timestamp(),
                };
                Ok(Response::new(proto_response))
            }
            Err(e) => Err(Status::internal(format!("Payment processing failed: {}", e))),
        }
    }
    
    async fn get_transaction_history(
        &self,
        request: Request<ProtoTransactionHistoryRequest>,
    ) -> Result<Response<ProtoTransactionHistoryResponse>, Status> {
        let proto_request = request.into_inner();
        
        let user_id = uuid::Uuid::parse_str(&proto_request.user_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid user ID: {}", e)))?;
        
        match self.get_transaction_history(user_id).await {
            Ok(transactions) => {
                let proto_transactions: Vec<proto::Transaction> = transactions
                    .into_iter()
                    .map(|t| proto::Transaction {
                        id: t.id.to_string(),
                        sender_id: t.sender_id.to_string(),
                        recipient_id: t.recipient_id.to_string(),
                        amount: t.amount.to_string(),
                        currency: match t.currency {
                            models::Currency::Dabloons => 0,
                            models::Currency::USD => 1,
                            models::Currency::EUR => 2,
                            models::Currency::GBP => 3,
                            models::Currency::JPY => 4,
                        },
                        status: match t.status {
                            models::TransactionStatus::Pending => 0,
                            models::TransactionStatus::Completed => 1,
                            models::TransactionStatus::Failed => 2,
                            models::TransactionStatus::Cancelled => 3,
                        },
                        description: t.description,
                        social_post_id: t.social_post_id.map(|id| id.to_string()).unwrap_or_default(),
                        volunteer_hours: t.volunteer_hours.map(|vh| vh.to_string()).unwrap_or_default(),
                        created_at: t.created_at.timestamp(),
                        completed_at: t.completed_at.map(|t| t.timestamp()),
                    })
                    .collect();
                
                let response = ProtoTransactionHistoryResponse {
                    transactions: proto_transactions,
                };
                
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::internal(format!("Failed to fetch transaction history: {}", e))),
        }
    }
    
    async fn get_featured_causes(
        &self,
        _request: Request<ProtoFeaturedCausesRequest>,
    ) -> Result<Response<ProtoFeaturedCausesResponse>, Status> {
        match self.get_featured_causes().await {
            Ok(causes) => {
                let proto_causes: Vec<proto::Cause> = causes
                    .into_iter()
                    .map(|c| proto::Cause {
                        id: c.id.to_string(),
                        name: c.name,
                        description: c.description,
                        image_url: c.image_url.unwrap_or_default(),
                        total_donations: c.total_donations.to_string(),
                    })
                    .collect();
                
                let response = ProtoFeaturedCausesResponse {
                    causes: proto_causes,
                };
                
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::internal(format!("Failed to fetch featured causes: {}", e))),
        }
    }
    
    async fn get_skill_exchange_rates(
        &self,
        _request: Request<ProtoSkillExchangeRatesRequest>,
    ) -> Result<Response<ProtoSkillExchangeRatesResponse>, Status> {
        match self.get_skill_exchange_rates().await {
            Ok(rates) => {
                let proto_rates: Vec<proto::SkillRate> = rates
                    .into_iter()
                    .map(|r| proto::SkillRate {
                        skill_name: r.skill_name,
                        rate_per_hour: r.rate_per_hour.to_string(),
                        currency: match r.currency {
                            models::Currency::Dabloons => 0,
                            models::Currency::USD => 1,
                            models::Currency::EUR => 2,
                            models::Currency::GBP => 3,
                            models::Currency::JPY => 4,
                        },
                    })
                    .collect();
                
                let response = ProtoSkillExchangeRatesResponse {
                    rates: proto_rates,
                };
                
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::internal(format!("Failed to fetch skill exchange rates: {}", e))),
        }
    }
}

#[async_trait::async_trait]
impl payment_service_server::PaymentService for CpayServiceImpl {
    async fn process_payment(
        &self,
        request: Request<ProtoPaymentRequest>,
    ) -> Result<Response<ProtoPaymentResponse>, Status> {
        let proto_request = request.into_inner();
        
        // Convert proto request to internal model
        let user_id = uuid::Uuid::parse_str(&proto_request.user_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid user ID: {}", e)))?;
            
        let recipient_id = uuid::Uuid::parse_str(&proto_request.recipient_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid recipient ID: {}", e)))?;
            
        let amount = rust_decimal::Decimal::from_str(&proto_request.amount)
            .map_err(|e| Status::invalid_argument(format!("Invalid amount: {}", e)))?;
            
        let currency = match proto_request.currency {
            0 => models::Currency::Dabloons,
            1 => models::Currency::USD,
            2 => models::Currency::EUR,
            3 => models::Currency::GBP,
            4 => models::Currency::JPY,
            _ => return Err(Status::invalid_argument("Invalid currency")),
        };
        
        let internal_request = models::PaymentRequest::new(
            user_id,
            recipient_id,
            amount,
            currency,
            proto_request.description,
            proto_request.is_public,
            proto_request.share_to_social,
            proto_request.cause_id.as_ref().and_then(|id| uuid::Uuid::parse_str(id).ok()),
            proto_request.volunteer_hours.as_ref().and_then(|vh| rust_decimal::Decimal::from_str(vh).ok()),
        );
        
        // Process the payment
        match self.process_payment(internal_request).await {
            Ok(response) => {
                let proto_response = ProtoPaymentResponse {
                    transaction_id: response.transaction_id.to_string(),
                    status: match response.status {
                        models::TransactionStatus::Pending => 0,
                        models::TransactionStatus::Completed => 1,
                        models::TransactionStatus::Failed => 2,
                        models::TransactionStatus::Cancelled => 3,
                    },
                    timestamp: response.timestamp.timestamp(),
                };
                Ok(Response::new(proto_response))
            }
            Err(e) => Err(Status::internal(format!("Payment processing failed: {}", e))),
        }
    }
    
    async fn get_transaction_history(
        &self,
        request: Request<ProtoTransactionHistoryRequest>,
    ) -> Result<Response<ProtoTransactionHistoryResponse>, Status> {
        let proto_request = request.into_inner();
        
        let user_id = uuid::Uuid::parse_str(&proto_request.user_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid user ID: {}", e)))?;
        
        match self.get_transaction_history(user_id).await {
            Ok(transactions) => {
                let proto_transactions: Vec<proto::Transaction> = transactions
                    .into_iter()
                    .map(|t| proto::Transaction {
                        id: t.id.to_string(),
                        sender_id: t.sender_id.to_string(),
                        recipient_id: t.recipient_id.to_string(),
                        amount: t.amount.to_string(),
                        currency: match t.currency {
                            models::Currency::Dabloons => 0,
                            models::Currency::USD => 1,
                            models::Currency::EUR => 2,
                            models::Currency::GBP => 3,
                            models::Currency::JPY => 4,
                        },
                        status: match t.status {
                            models::TransactionStatus::Pending => 0,
                            models::TransactionStatus::Completed => 1,
                            models::TransactionStatus::Failed => 2,
                            models::TransactionStatus::Cancelled => 3,
                        },
                        description: t.description,
                        social_post_id: t.social_post_id.map(|id| id.to_string()).unwrap_or_default(),
                        volunteer_hours: t.volunteer_hours.map(|vh| vh.to_string()).unwrap_or_default(),
                        created_at: t.created_at.timestamp(),
                        completed_at: t.completed_at.map(|t| t.timestamp()),
                    })
                    .collect();
                
                let response = ProtoTransactionHistoryResponse {
                    transactions: proto_transactions,
                };
                
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::internal(format!("Failed to fetch transaction history: {}", e))),
        }
    }
    
    async fn get_skill_exchange_rates(
        &self,
        _request: Request<ProtoSkillExchangeRatesRequest>,
    ) -> Result<Response<ProtoSkillExchangeRatesResponse>, Status> {
        match self.get_skill_exchange_rates().await {
            Ok(rates) => {
                let proto_rates: Vec<proto::SkillRate> = rates
                    .into_iter()
                    .map(|r| proto::SkillRate {
                        skill_name: r.skill_name,
                        rate_per_hour: r.rate_per_hour.to_string(),
                        currency: match r.currency {
                            models::Currency::Dabloons => 0,
                            models::Currency::USD => 1,
                            models::Currency::EUR => 2,
                            models::Currency::GBP => 3,
                            models::Currency::JPY => 4,
                        },
                    })
                    .collect();
                
                let response = ProtoSkillExchangeRatesResponse {
                    rates: proto_rates,
                };
                
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::internal(format!("Failed to fetch skill exchange rates: {}", e))),
        }
    }
}
    async fn process_payment(
        &self,
        request: Request<ProtoPaymentRequest>,
    ) -> Result<Response<ProtoPaymentResponse>, Status> {
        let proto_request = request.into_inner();
        
        // Convert proto request to internal model
        let user_id = uuid::Uuid::parse_str(&proto_request.user_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid user ID: {}", e)))?;
            
        let recipient_id = uuid::Uuid::parse_str(&proto_request.recipient_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid recipient ID: {}", e)))?;
            
        let amount = rust_decimal::Decimal::from_str(&proto_request.amount)
            .map_err(|e| Status::invalid_argument(format!("Invalid amount: {}", e)))?;
            
        let currency = match proto_request.currency {
            0 => models::Currency::Dabloons,
            1 => models::Currency::USD,
            2 => models::Currency::EUR,
            3 => models::Currency::GBP,
            4 => models::Currency::JPY,
            _ => return Err(Status::invalid_argument("Invalid currency")),
        };
        
        let internal_request = models::PaymentRequest::new(
            user_id,
            recipient_id,
            amount,
            currency,
            proto_request.description,
            proto_request.is_public,
            proto_request.share_to_social,
            proto_request.cause_id.as_ref().and_then(|id| uuid::Uuid::parse_str(id).ok()),
            proto_request.volunteer_hours.as_ref().and_then(|vh| rust_decimal::Decimal::from_str(vh).ok()),
        );
        
        // Process the payment
        match self.process_payment(internal_request).await {
            Ok(response) => {
                let proto_response = ProtoPaymentResponse {
                    transaction_id: response.transaction_id.to_string(),
                    status: match response.status {
                        models::TransactionStatus::Pending => 0,
                        models::TransactionStatus::Completed => 1,
                        models::TransactionStatus::Failed => 2,
                        models::TransactionStatus::Cancelled => 3,
                    },
                    timestamp: response.timestamp.timestamp(),
                };
                Ok(Response::new(proto_response))
            }
            Err(e) => Err(Status::internal(format!("Payment processing failed: {}", e))),
        }
    }
    
    async fn get_transaction_history(
        &self,
        request: Request<ProtoTransactionHistoryRequest>,
    ) -> Result<Response<ProtoTransactionHistoryResponse>, Status> {
        let proto_request = request.into_inner();
        
        let user_id = uuid::Uuid::parse_str(&proto_request.user_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid user ID: {}", e)))?;
        
        match self.get_transaction_history(user_id).await {
            Ok(transactions) => {
                let proto_transactions: Vec<proto::Transaction> = transactions
                    .into_iter()
                    .map(|t| proto::Transaction {
                        id: t.id.to_string(),
                        sender_id: t.sender_id.to_string(),
                        recipient_id: t.recipient_id.to_string(),
                        amount: t.amount.to_string(),
                        currency: match t.currency {
                            models::Currency::Dabloons => 0,
                            models::Currency::USD => 1,
                            models::Currency::EUR => 2,
                            models::Currency::GBP => 3,
                            models::Currency::JPY => 4,
                        },
                        status: match t.status {
                            models::TransactionStatus::Pending => 0,
                            models::TransactionStatus::Completed => 1,
                            models::TransactionStatus::Failed => 2,
                            models::TransactionStatus::Cancelled => 3,
                        },
                        description: t.description,
                        social_post_id: t.social_post_id.map(|id| id.to_string()).unwrap_or_default(),
                        volunteer_hours: t.volunteer_hours.map(|vh| vh.to_string()).unwrap_or_default(),
                        created_at: t.created_at.timestamp(),
                        completed_at: t.completed_at.map(|t| t.timestamp()),
                    })
                    .collect();
                
                let response = ProtoTransactionHistoryResponse {
                    transactions: proto_transactions,
                };
                
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::internal(format!("Failed to fetch transaction history: {}", e))),
        }
    }
    
    async fn get_featured_causes(
        &self,
        _request: Request<ProtoFeaturedCausesRequest>,
    ) -> Result<Response<ProtoFeaturedCausesResponse>, Status> {
        match self.get_featured_causes().await {
            Ok(causes) => {
                let proto_causes: Vec<proto::Cause> = causes
                    .into_iter()
                    .map(|c| proto::Cause {
                        id: c.id.to_string(),
                        name: c.name,
                        description: c.description,
                        image_url: c.image_url.unwrap_or_default(),
                        total_donations: c.total_donations.to_string(),
                    })
                    .collect();
                
                let response = ProtoFeaturedCausesResponse {
                    causes: proto_causes,
                };
                
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::internal(format!("Failed to fetch featured causes: {}", e))),
        }
    }
    
    async fn get_skill_exchange_rates(
        &self,
        _request: Request<ProtoSkillExchangeRatesRequest>,
    ) -> Result<Response<ProtoSkillExchangeRatesResponse>, Status> {
        match self.get_skill_exchange_rates().await {
            Ok(rates) => {
                let proto_rates: Vec<proto::SkillRate> = rates
                    .into_iter()
                    .map(|r| proto::SkillRate {
                        skill_name: r.skill_name,
                        rate_per_hour: r.rate_per_hour.to_string(),
                        currency: match r.currency {
                            models::Currency::Dabloons => 0,
                            models::Currency::USD => 1,
                            models::Currency::EUR => 2,
                            models::Currency::GBP => 3,
                            models::Currency::JPY => 4,
                        },
                    })
                    .collect();
                
                let response = ProtoSkillExchangeRatesResponse {
                    rates: proto_rates,
                };
                
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::internal(format!("Failed to fetch skill exchange rates: {}", e))),
        }
    }
}