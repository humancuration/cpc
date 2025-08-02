//! gRPC service implementation for Cooperative Fundraising

use tonic::{Request, Response, Status};
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::proto::*;
use crate::application::{CampaignService, ContributionService, MembershipService};
use crate::domain::{CampaignType, CampaignStatus, VerificationStatus};

pub struct CooperativeFundraisingServiceImpl {
    campaign_service: Box<dyn CampaignService>,
    contribution_service: Box<dyn ContributionService>,
    membership_service: Box<dyn MembershipService>,
}

impl CooperativeFundraisingServiceImpl {
    pub fn new(
        campaign_service: Box<dyn CampaignService>,
        contribution_service: Box<dyn ContributionService>,
        membership_service: Box<dyn MembershipService>,
    ) -> Self {
        Self {
            campaign_service,
            contribution_service,
            membership_service,
        }
    }
}

#[tonic::async_trait]
impl cooperative_fundraising_service_server::CooperativeFundraisingService for CooperativeFundraisingServiceImpl {
    /// Create a new campaign
    async fn create_campaign(
        &self,
        request: Request<CreateCampaignRequest>,
    ) -> Result<Response<CreateCampaignResponse>, Status> {
        let req = request.into_inner();
        
        // Convert campaign type
        let campaign_type = match req.r#type() {
            CampaignType::CooperativeMembership => crate::domain::CampaignType::CooperativeMembership,
            CampaignType::PureDonation => crate::domain::CampaignType::PureDonation,
            CampaignType::RegCf => crate::domain::CampaignType::RegCF,
            CampaignType::RegA => crate::domain::CampaignType::RegA,
            CampaignType::RegD => crate::domain::CampaignType::RegD,
        };
        
        // Parse owner user ID
        let owner_user_id = Uuid::parse_str(&req.owner_user_id)
            .map_err(|_| Status::invalid_argument("Invalid owner user ID"))?;
        
        // Create campaign based on type
        let campaign = if matches!(campaign_type, crate::domain::CampaignType::CooperativeMembership) {
            // Membership campaign
            let requirements = req.membership_requirements
                .ok_or_else(|| Status::invalid_argument("Membership requirements required for membership campaign"))?;
            
            let membership_requirements = crate::domain::MembershipRequirements {
                max_participants: requirements.max_participants.map(|i| i as u32),
                required_actions: requirements.required_actions.clone(),
            };
            
            crate::domain::Campaign::new_membership_campaign(
                req.title.clone(),
                req.description.clone(),
                owner_user_id,
                membership_requirements,
            )
        } else {
            // Donation campaign
            let details = req.donation_details
                .ok_or_else(|| Status::invalid_argument("Donation details required for donation campaign"))?;
            
            let donation_details = crate::domain::DonationDetails {
                funding_goal: details.funding_goal.as_ref().and_then(|s| s.parse().ok()),
                external_use_case: details.external_use_case.clone(),
                currency: details.currency.clone(),
            };
            
            crate::domain::Campaign::new_donation_campaign(
                campaign_type,
                req.title.clone(),
                req.description.clone(),
                owner_user_id,
                donation_details,
            )
        };
        
        // Create the campaign
        let campaign = self.campaign_service
            .create_campaign(campaign)
            .await
            .map_err(|e| Status::internal(format!("Failed to create campaign: {:?}", e)))?;
        
        // Convert back to proto
        let proto_campaign = self.convert_campaign_to_proto(campaign);
        
        let response = CreateCampaignResponse {
            campaign: Some(proto_campaign),
        };
        
        Ok(Response::new(response))
    }
    
    /// Get a campaign by ID
    async fn get_campaign(
        &self,
        request: Request<GetCampaignRequest>,
    ) -> Result<Response<GetCampaignResponse>, Status> {
        let req = request.into_inner();
        
        // Parse campaign ID
        let campaign_id = Uuid::parse_str(&req.campaign_id)
            .map_err(|_| Status::invalid_argument("Invalid campaign ID"))?;
        
        // Get the campaign
        let campaign = self.campaign_service
            .get_campaign(campaign_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to get campaign: {:?}", e)))?
            .ok_or_else(|| Status::not_found("Campaign not found"))?;
        
        // Convert to proto
        let proto_campaign = self.convert_campaign_to_proto(campaign);
        
        let response = GetCampaignResponse {
            campaign: Some(proto_campaign),
        };
        
        Ok(Response::new(response))
    }
    
    /// List campaigns
    async fn list_campaigns(
        &self,
        request: Request<ListCampaignsRequest>,
    ) -> Result<Response<ListCampaignsResponse>, Status> {
        let req = request.into_inner();
        
        // Convert optional parameters
        let campaign_type = req.r#type.map(|t| match t() {
            CampaignType::CooperativeMembership => crate::domain::CampaignType::CooperativeMembership,
            CampaignType::PureDonation => crate::domain::CampaignType::PureDonation,
            CampaignType::RegCf => crate::domain::CampaignType::RegCF,
            CampaignType::RegA => crate::domain::CampaignType::RegA,
            CampaignType::RegD => crate::domain::CampaignType::RegD,
        });
        
        let status = req.status.map(|s| match s() {
            CampaignStatus::Draft => crate::domain::CampaignStatus::Draft,
            CampaignStatus::Active => crate::domain::CampaignStatus::Active,
            CampaignStatus::Completed => crate::domain::CampaignStatus::Completed,
            CampaignStatus::Failed => crate::domain::CampaignStatus::Failed,
            CampaignStatus::Cancelled => crate::domain::CampaignStatus::Cancelled,
        });
        
        // List campaigns
        let campaigns = self.campaign_service
            .list_campaigns(campaign_type, status, req.limit, req.offset)
            .await
            .map_err(|e| Status::internal(format!("Failed to list campaigns: {:?}", e)))?;
        
        // Convert to proto
        let proto_campaigns: Vec<Campaign> = campaigns
            .into_iter()
            .map(|c| self.convert_campaign_to_proto(c))
            .collect();
        
        // In a real implementation, we would get the total count from the service
        let total_count = proto_campaigns.len() as i32;
        
        let response = ListCampaignsResponse {
            campaigns: proto_campaigns,
            total_count,
        };
        
        Ok(Response::new(response))
    }
    
    /// Update a campaign
    async fn update_campaign(
        &self,
        request: Request<UpdateCampaignRequest>,
    ) -> Result<Response<UpdateCampaignResponse>, Status> {
        let req = request.into_inner();
        
        // Parse campaign ID
        let campaign_id = Uuid::parse_str(&req.campaign_id)
            .map_err(|_| Status::invalid_argument("Invalid campaign ID"))?;
        
        // Get existing campaign
        let mut campaign = self.campaign_service
            .get_campaign(campaign_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to get campaign: {:?}", e)))?
            .ok_or_else(|| Status::not_found("Campaign not found"))?;
        
        // Update fields if provided
        if !req.title.is_empty() {
            campaign.title = req.title.clone();
        }
        
        if !req.description.is_empty() {
            campaign.description = req.description.clone();
        }
        
        // Update the campaign
        let campaign = self.campaign_service
            .update_campaign(campaign)
            .await
            .map_err(|e| Status::internal(format!("Failed to update campaign: {:?}", e)))?;
        
        // Convert back to proto
        let proto_campaign = self.convert_campaign_to_proto(campaign);
        
        let response = UpdateCampaignResponse {
            campaign: Some(proto_campaign),
        };
        
        Ok(Response::new(response))
    }
    
    /// Delete a campaign
    async fn delete_campaign(
        &self,
        request: Request<DeleteCampaignRequest>,
    ) -> Result<Response<DeleteCampaignResponse>, Status> {
        let req = request.into_inner();
        
        // Parse campaign ID
        let campaign_id = Uuid::parse_str(&req.campaign_id)
            .map_err(|_| Status::invalid_argument("Invalid campaign ID"))?;
        
        // Delete the campaign
        self.campaign_service
            .delete_campaign(campaign_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to delete campaign: {:?}", e)))?;
        
        let response = DeleteCampaignResponse {
            success: true,
        };
        
        Ok(Response::new(response))
    }
    
    /// Join the cooperative
    async fn join_cooperative(
        &self,
        request: Request<JoinCooperativeRequest>,
    ) -> Result<Response<JoinCooperativeResponse>, Status> {
        let req = request.into_inner();
        
        // Parse user ID and campaign ID
        let user_id = Uuid::parse_str(&req.user_id)
            .map_err(|_| Status::invalid_argument("Invalid user ID"))?;
        
        let campaign_id = Uuid::parse_str(&req.campaign_id)
            .map_err(|_| Status::invalid_argument("Invalid campaign ID"))?;
        
        // Join the cooperative
        let result = self.membership_service
            .join_cooperative(user_id, campaign_id)
            .await;
        
        match result {
            Ok(_) => {
                let response = JoinCooperativeResponse {
                    success: true,
                    message: "Successfully joined the cooperative".to_string(),
                };
                Ok(Response::new(response))
            }
            Err(e) => {
                let response = JoinCooperativeResponse {
                    success: false,
                    message: format!("Failed to join cooperative: {:?}", e),
                };
                Ok(Response::new(response))
            }
        }
    }
    
    /// Get user membership
    async fn get_user_membership(
        &self,
        request: Request<GetUserMembershipRequest>,
    ) -> Result<Response<GetUserMembershipResponse>, Status> {
        let req = request.into_inner();
        
        // Parse user ID
        let user_id = Uuid::parse_str(&req.user_id)
            .map_err(|_| Status::invalid_argument("Invalid user ID"))?;
        
        // Check if user has membership
        let has_membership = self.membership_service
            .user_has_membership(user_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to check membership: {:?}", e)))?;
        
        // Get membership details if they have one
        let campaign_id = if has_membership {
            self.membership_service
                .get_user_membership(user_id)
                .await
                .map_err(|e| Status::internal(format!("Failed to get membership: {:?}", e)))?
                .map(|m| m.campaign_id.to_string())
        } else {
            None
        };
        
        let response = GetUserMembershipResponse {
            has_membership,
            campaign_id,
        };
        
        Ok(Response::new(response))
    }
    
    /// Make a monetary contribution
    async fn make_monetary_contribution(
        &self,
        request: Request<MakeMonetaryContributionRequest>,
    ) -> Result<Response<MakeMonetaryContributionResponse>, Status> {
        let req = request.into_inner();
        
        // Parse IDs
        let user_id = Uuid::parse_str(&req.user_id)
            .map_err(|_| Status::invalid_argument("Invalid user ID"))?;
        
        let campaign_id = Uuid::parse_str(&req.campaign_id)
            .map_err(|_| Status::invalid_argument("Invalid campaign ID"))?;
        
        let cpay_transaction_id = Uuid::parse_str(&req.cpay_transaction_id)
            .map_err(|_| Status::invalid_argument("Invalid cpay transaction ID"))?;
        
        // Parse amount
        let amount = req.amount.parse::<Decimal>()
            .map_err(|_| Status::invalid_argument("Invalid amount"))?;
        
        // Make the contribution
        let contribution = self.contribution_service
            .make_monetary_contribution(
                campaign_id,
                user_id,
                amount,
                req.currency.clone(),
                cpay_transaction_id,
            )
            .await
            .map_err(|e| Status::internal(format!("Failed to make contribution: {:?}", e)))?;
        
        // Convert to proto
        let proto_contribution = self.convert_contribution_to_proto(contribution);
        
        let response = MakeMonetaryContributionResponse {
            contribution: Some(proto_contribution),
        };
        
        Ok(Response::new(response))
    }
    
    /// Record a volunteer contribution
    async fn record_volunteer_contribution(
        &self,
        request: Request<RecordVolunteerContributionRequest>,
    ) -> Result<Response<RecordVolunteerContributionResponse>, Status> {
        let req = request.into_inner();
        
        // Parse IDs
        let user_id = Uuid::parse_str(&req.user_id)
            .map_err(|_| Status::invalid_argument("Invalid user ID"))?;
        
        let campaign_id = Uuid::parse_str(&req.campaign_id)
            .map_err(|_| Status::invalid_argument("Invalid campaign ID"))?;
        
        let opportunity_id = Uuid::parse_str(&req.opportunity_id)
            .map_err(|_| Status::invalid_argument("Invalid opportunity ID"))?;
        
        // Record the contribution
        let contribution = self.contribution_service
            .record_volunteer_contribution(
                campaign_id,
                user_id,
                opportunity_id,
                req.hours,
            )
            .await
            .map_err(|e| Status::internal(format!("Failed to record contribution: {:?}", e)))?;
        
        // Convert to proto
        let proto_contribution = self.convert_contribution_to_proto(contribution);
        
        let response = RecordVolunteerContributionResponse {
            contribution: Some(proto_contribution),
        };
        
        Ok(Response::new(response))
    }
    
    /// List contributions
    async fn list_contributions(
        &self,
        request: Request<ListContributionsRequest>,
    ) -> Result<Response<ListContributionsResponse>, Status> {
        let req = request.into_inner();
        
        // Parse campaign ID
        let campaign_id = Uuid::parse_str(&req.campaign_id)
            .map_err(|_| Status::invalid_argument("Invalid campaign ID"))?;
        
        // Parse optional user ID
        let user_id = if !req.user_id.is_empty() {
            Some(Uuid::parse_str(&req.user_id)
                .map_err(|_| Status::invalid_argument("Invalid user ID"))?)
        } else {
            None
        };
        
        // List contributions
        let contributions = self.contribution_service
            .list_contributions(campaign_id, user_id, req.limit, req.offset)
            .await
            .map_err(|e| Status::internal(format!("Failed to list contributions: {:?}", e)))?;
        
        // Convert to proto
        let proto_contributions: Vec<Contribution> = contributions
            .into_iter()
            .map(|c| self.convert_contribution_to_proto(c))
            .collect();
        
        // In a real implementation, we would get the total count from the service
        let total_count = proto_contributions.len() as i32;
        
        let response = ListContributionsResponse {
            contributions: proto_contributions,
            total_count,
        };
        
        Ok(Response::new(response))
    }
    
    /// Activate a campaign
    async fn activate_campaign(
        &self,
        request: Request<ActivateCampaignRequest>,
    ) -> Result<Response<ActivateCampaignResponse>, Status> {
        let req = request.into_inner();
        
        // Parse campaign ID
        let campaign_id = Uuid::parse_str(&req.campaign_id)
            .map_err(|_| Status::invalid_argument("Invalid campaign ID"))?;
        
        // Activate the campaign
        let campaign = self.campaign_service
            .activate_campaign(campaign_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to activate campaign: {:?}", e)))?;
        
        // Convert to proto
        let proto_campaign = self.convert_campaign_to_proto(campaign);
        
        let response = ActivateCampaignResponse {
            campaign: Some(proto_campaign),
        };
        
        Ok(Response::new(response))
    }
    
    /// Complete a campaign
    async fn complete_campaign(
        &self,
        request: Request<CompleteCampaignRequest>,
    ) -> Result<Response<CompleteCampaignResponse>, Status> {
        let req = request.into_inner();
        
        // Parse campaign ID
        let campaign_id = Uuid::parse_str(&req.campaign_id)
            .map_err(|_| Status::invalid_argument("Invalid campaign ID"))?;
        
        // Complete the campaign
        let campaign = self.campaign_service
            .complete_campaign(campaign_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to complete campaign: {:?}", e)))?;
        
        // Convert to proto
        let proto_campaign = self.convert_campaign_to_proto(campaign);
        
        let response = CompleteCampaignResponse {
            campaign: Some(proto_campaign),
        };
        
        Ok(Response::new(response))
    }
}

impl CooperativeFundraisingServiceImpl {
    /// Convert a domain Campaign to a proto Campaign
    fn convert_campaign_to_proto(&self, campaign: crate::domain::Campaign) -> Campaign {
        let campaign_type = match campaign.campaign_type {
            crate::domain::CampaignType::CooperativeMembership => CampaignType::CooperativeMembership,
            crate::domain::CampaignType::PureDonation => CampaignType::PureDonation,
            crate::domain::CampaignType::RegCF => CampaignType::RegCf,
            crate::domain::CampaignType::RegA => CampaignType::RegA,
            crate::domain::CampaignType::RegD => CampaignType::RegD,
        };
        
        let status = match campaign.status {
            crate::domain::CampaignStatus::Draft => CampaignStatus::Draft,
            crate::domain::CampaignStatus::Active => CampaignStatus::Active,
            crate::domain::CampaignStatus::Completed => CampaignStatus::Completed,
            crate::domain::CampaignStatus::Failed => CampaignStatus::Failed,
            crate::domain::CampaignStatus::Cancelled => CampaignStatus::Cancelled,
        };
        
        let membership_requirements = campaign.membership_requirements.map(|req| MembershipRequirements {
            max_participants: req.max_participants.map(|i| i as i32),
            required_actions: req.required_actions,
        });
        
        let donation_details = campaign.donation_details.map(|details| DonationDetails {
            funding_goal: details.funding_goal.map(|d| d.to_string()),
            external_use_case: details.external_use_case,
            currency: details.currency,
        });
        
        Campaign {
            id: campaign.id.to_string(),
            r#type: campaign_type as i32,
            title: campaign.title,
            description: campaign.description,
            created_at: Some(prost_types::Timestamp {
                seconds: campaign.created_at.timestamp(),
                nanos: campaign.created_at.timestamp_subsec_nanos() as i32,
            }),
            owner_user_id: campaign.owner_user_id.to_string(),
            status: status as i32,
            membership_requirements,
            donation_details,
        }
    }
    
    /// Convert a domain Contribution to a proto Contribution
    fn convert_contribution_to_proto(&self, contribution: crate::domain::Contribution) -> Contribution {
        let contribution_type = if contribution.is_monetary() {
            let monetary = MonetaryContribution {
                amount: contribution.amount.map(|d| d.to_string()).unwrap_or_default(),
                currency: contribution.currency.unwrap_or_default(),
                cpay_transaction_id: contribution.cpay_transaction_id.map(|id| id.to_string()),
            };
            Some(contribution::ContributionType::Monetary(monetary))
        } else {
            let verification_status = contribution.verification_status.as_ref().map(|status| {
                match status {
                    VerificationStatus::Pending => VerificationStatus::Pending,
                    VerificationStatus::Verified => VerificationStatus::Verified,
                    VerificationStatus::Disputed => VerificationStatus::Disputed,
                    VerificationStatus::Rejected => VerificationStatus::Rejected,
                } as i32
            });
            
            let volunteer = VolunteerContribution {
                opportunity_id: contribution.opportunity_id.map(|id| id.to_string()),
                hours: contribution.hours.unwrap_or(0),
                verification_status,
            };
            Some(contribution::ContributionType::Volunteer(volunteer))
        };
        
        Contribution {
            id: contribution.id.to_string(),
            campaign_id: contribution.campaign_id.to_string(),
            user_id: contribution.user_id.to_string(),
            created_at: Some(prost_types::Timestamp {
                seconds: contribution.created_at.timestamp(),
                nanos: contribution.created_at.timestamp_subsec_nanos() as i32,
            }),
            contribution_type,
        }
    }
}