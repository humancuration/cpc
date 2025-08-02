//! gRPC client for the cooperative fundraising service

use std::time::Duration;
use tonic::transport::{Channel, Endpoint};
use uuid::Uuid;
use tracing::instrument;

use crate::domain::models::{CampaignType, FundraisingCampaignData};
use crate::domain::errors::WebsiteBuilderError;

// Import the fundraising service proto
// This would be generated from the proto file
// For now, we'll define the necessary types

#[derive(Debug, Clone)]
pub struct CreateCampaignRequest {
    pub title: String,
    pub description: String,
    pub campaign_type: CampaignType,
    pub owner_user_id: Uuid,
    pub goal_amount: Option<u64>,
    pub currency: Option<String>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone)]
pub struct CreateCampaignResponse {
    pub campaign_id: Uuid,
    pub title: String,
    pub description: String,
    pub campaign_type: CampaignType,
    pub owner_user_id: Uuid,
    pub goal_amount: Option<u64>,
    pub current_amount: u64,
    pub currency: Option<String>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone)]
pub struct FundraisingClient {
    channel: Channel,
}

impl FundraisingClient {
    pub async fn new(endpoint: &str) -> Result<Self, WebsiteBuilderError> {
        let endpoint = Endpoint::from_shared(endpoint.to_string())
            .map_err(|e| WebsiteBuilderError::GrpcError(e.to_string()))?
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(10));

        let channel = endpoint
            .connect()
            .await
            .map_err(|e| WebsiteBuilderError::GrpcError(e.to_string()))?;

        Ok(Self { channel })
    }

    #[instrument(skip(self))]
    pub async fn create_campaign(
        &self,
        request: CreateCampaignRequest,
    ) -> Result<CreateCampaignResponse, WebsiteBuilderError> {
        // In a real implementation, this would call the actual gRPC service
        // For now, we'll simulate the call
        tracing::info!("Creating campaign: {}", request.title);
        
        // Simulate network delay
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        // Simulate successful response
        let response = CreateCampaignResponse {
            campaign_id: Uuid::new_v4(),
            title: request.title,
            description: request.description,
            campaign_type: request.campaign_type,
            owner_user_id: request.owner_user_id,
            goal_amount: request.goal_amount,
            current_amount: 0,
            currency: request.currency,
            start_date: request.start_date,
            end_date: request.end_date,
        };
        
        Ok(response)
    }
    
    #[instrument(skip(self))]
    pub async fn get_campaign(
        &self,
        campaign_id: Uuid,
    ) -> Result<CreateCampaignResponse, WebsiteBuilderError> {
        // In a real implementation, this would call the actual gRPC service
        // For now, we'll simulate the call
        tracing::info!("Getting campaign: {}", campaign_id);
        
        // Simulate network delay
        tokio::time::sleep(Duration::from_millis(50)).await;
        
        // Simulate successful response
        let response = CreateCampaignResponse {
            campaign_id,
            title: "Sample Campaign".to_string(),
            description: "Sample campaign description".to_string(),
            campaign_type: CampaignType::PureDonation,
            owner_user_id: Uuid::new_v4(),
            goal_amount: Some(100000),
            current_amount: 25000,
            currency: Some("USD".to_string()),
            start_date: chrono::Utc::now(),
            end_date: Some(chrono::Utc::now() + chrono::Duration::days(30)),
        };
        
        Ok(response)
    }
}

// Conversion functions
impl From<crate::domain::models::CampaignType> for CampaignType {
    fn from(campaign_type: crate::domain::models::CampaignType) -> Self {
        match campaign_type {
            crate::domain::models::CampaignType::CooperativeMembership => CampaignType::CooperativeMembership,
            crate::domain::models::CampaignType::PureDonation => CampaignType::PureDonation,
            crate::domain::models::CampaignType::RegCF => CampaignType::RegCF,
            crate::domain::models::CampaignType::RegA => CampaignType::RegA,
            crate::domain::models::CampaignType::RegD => CampaignType::RegD,
        }
    }
}

impl From<CampaignType> for crate::domain::models::CampaignType {
    fn from(campaign_type: CampaignType) -> Self {
        match campaign_type {
            CampaignType::CooperativeMembership => crate::domain::models::CampaignType::CooperativeMembership,
            CampaignType::PureDonation => crate::domain::models::CampaignType::PureDonation,
            CampaignType::RegCF => crate::domain::models::CampaignType::RegCF,
            CampaignType::RegA => crate::domain::models::CampaignType::RegA,
            CampaignType::RegD => crate::domain::models::CampaignType::RegD,
        }
    }
}

impl From<CreateCampaignResponse> for FundraisingCampaignData {
    fn from(response: CreateCampaignResponse) -> Self {
        Self {
            campaign_id: response.campaign_id,
            campaign_title: response.title,
            campaign_description: response.description,
            campaign_type: response.campaign_type.into(),
            goal_amount: response.goal_amount,
            current_amount: response.current_amount,
            start_date: response.start_date,
            end_date: response.end_date,
        }
    }
}