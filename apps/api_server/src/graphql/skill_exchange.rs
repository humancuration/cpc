//! GraphQL resolvers for skill exchange functionality

use async_graphql::{Context, Object, Result, SimpleObject, InputObject, ID};
use uuid::Uuid;
use rust_decimal::Decimal;
use skill_exchange_core::models::{SkillListing, SkillClaim, SkillExchangeCompletion, ClaimStatus};
use skill_exchange_core::services::SkillExchangeService;
use wallet::domain::primitives::{Money, Currency};
use common_utils::error::CommonError;

/// GraphQL input for creating a skill listing
#[derive(InputObject)]
pub struct CreateSkillListingInput {
    /// Title of the skill listing
    pub title: String,
    
    /// Description of what's being offered
    pub description: String,
    
    /// Category of the skill
    pub category: String,
    
    /// Estimated time required for the exchange
    pub estimated_time: Option<Decimal>,
}

/// GraphQL input for updating a skill listing
#[derive(InputObject)]
pub struct UpdateSkillListingInput {
    /// Title of the skill listing
    pub title: Option<String>,
    
    /// Description of what's being offered
    pub description: Option<String>,
    
    /// Category of the skill
    pub category: Option<String>,
    
    /// Estimated time required for the exchange
    pub estimated_time: Option<Decimal>,
}

/// GraphQL input for claiming a skill listing
#[derive(InputObject)]
pub struct ClaimSkillListingInput {
    /// Optional message from the claimant
    pub message: Option<String>,
}

/// GraphQL input for completing a skill exchange
#[derive(InputObject)]
pub struct CompleteSkillExchangeInput {
    /// Optional rating provided by the claimant (1-5)
    pub rating: Option<i32>,
    
    /// Optional review comment
    pub review: Option<String>,
    
    /// Optional payment amount if applicable
    pub payment_amount: Option<f64>,
}

/// GraphQL object for skill listing
#[derive(SimpleObject)]
pub struct SkillListingGQL {
    /// Unique identifier for the listing
    pub id: ID,
    
    /// User offering the skill
    pub provider_id: ID,
    
    /// Title of the skill listing
    pub title: String,
    
    /// Description of what's being offered
    pub description: String,
    
    /// Category of the skill
    pub category: String,
    
    /// Estimated time required for the exchange
    pub estimated_time: Option<Decimal>,
    
    /// Whether the listing is active
    pub is_active: bool,
    
    /// Timestamp when the listing was created
    pub created_at: String,
}

impl From<SkillListing> for SkillListingGQL {
    fn from(listing: SkillListing) -> Self {
        Self {
            id: ID::from(listing.id.to_string()),
            provider_id: ID::from(listing.provider_id.to_string()),
            title: listing.title,
            description: listing.description,
            category: listing.category,
            estimated_time: listing.estimated_time,
            is_active: listing.is_active,
            created_at: listing.created_at.to_rfc3339(),
        }
    }
}

/// GraphQL object for skill claim
#[derive(SimpleObject)]
pub struct SkillClaimGQL {
    /// Unique identifier for the claim
    pub id: ID,
    
    /// ID of the skill listing being claimed
    pub listing_id: ID,
    
    /// User claiming the skill
    pub claimant_id: ID,
    
    /// Status of the claim
    pub status: String,
    
    /// Optional message from the claimant
    pub message: Option<String>,
    
    /// Timestamp when the claim was created
    pub created_at: String,
}

impl From<SkillClaim> for SkillClaimGQL {
    fn from(claim: SkillClaim) -> Self {
        Self {
            id: ID::from(claim.id.to_string()),
            listing_id: ID::from(claim.listing_id.to_string()),
            claimant_id: ID::from(claim.claimant_id.to_string()),
            status: match claim.status {
                ClaimStatus::Pending => "pending".to_string(),
                ClaimStatus::Accepted => "accepted".to_string(),
                ClaimStatus::Rejected => "rejected".to_string(),
                ClaimStatus::Completed => "completed".to_string(),
            },
            message: claim.message,
            created_at: claim.created_at.to_rfc3339(),
        }
    }
}

/// GraphQL object for skill exchange completion
#[derive(SimpleObject)]
pub struct SkillExchangeCompletionGQL {
    /// Unique identifier for the completion record
    pub id: ID,
    
    /// ID of the skill listing that was exchanged
    pub listing_id: ID,
    
    /// ID of the claim that was completed
    pub claim_id: ID,
    
    /// Provider of the skill
    pub provider_id: ID,
    
    /// User who claimed the skill
    pub claimant_id: ID,
    
    /// Optional rating provided by the claimant
    pub rating_value: Option<i32>,
    
    /// Optional review comment
    pub rating_comment: Option<String>,
    
    /// Optional payment amount if applicable
    pub payment_amount: Option<f64>,
    
    /// Timestamp when the exchange was completed
    pub completed_at: String,
}

impl From<SkillExchangeCompletion> for SkillExchangeCompletionGQL {
    fn from(completion: SkillExchangeCompletion) -> Self {
        Self {
            id: ID::from(completion.id.to_string()),
            listing_id: ID::from(completion.listing_id.to_string()),
            claim_id: ID::from(completion.claim_id.to_string()),
            provider_id: ID::from(completion.provider_id.to_string()),
            claimant_id: ID::from(completion.claimant_id.to_string()),
            rating_value: completion.rating.as_ref().map(|r| r.value as i32),
            rating_comment: completion.rating.as_ref().map(|r| r.comment.clone()).flatten(),
            payment_amount: completion.payment.as_ref().map(|m| m.amount.to_f64().unwrap_or(0.0)),
            completed_at: completion.completed_at.to_rfc3339(),
        }
    }
}

/// Root mutation object for skill exchange functionality
pub struct SkillExchangeMutation;

#[Object]
impl SkillExchangeMutation {
    /// Create a new skill listing
    async fn create_skill_listing(
        &self,
        ctx: &Context<'_>,
        input: CreateSkillListingInput,
    ) -> Result<SkillListingGQL> {
        // Get the skill exchange service from context
        let skill_service = ctx.data::<Box<dyn SkillExchangeService>>()
            .map_err(|_| async_graphql::Error::new("SkillExchangeService not available in context"))?;
        
        // Get the current user ID from context
        let provider_id = ctx.data::<Uuid>()
            .map_err(|_| async_graphql::Error::new("User ID not available in context"))?;
        
        // Create the skill listing
        let listing = skill_service.create_listing(
            *provider_id,
            input.title,
            input.description,
            input.category,
            input.estimated_time,
        ).await
        .map_err(|e| match e {
            CommonError::ValidationError(msg) => async_graphql::Error::new(msg),
            CommonError::Unauthorized(msg) => async_graphql::Error::new(msg),
            _ => async_graphql::Error::new("Failed to create skill listing"),
        })?;
        
        Ok(listing.into())
    }
    
    /// Update an existing skill listing
    async fn update_skill_listing(
        &self,
        ctx: &Context<'_>,
        listing_id: ID,
        input: UpdateSkillListingInput,
    ) -> Result<SkillListingGQL> {
        // Get the skill exchange service from context
        let skill_service = ctx.data::<Box<dyn SkillExchangeService>>()
            .map_err(|_| async_graphql::Error::new("SkillExchangeService not available in context"))?;
        
        // Get the current user ID from context
        let provider_id = ctx.data::<Uuid>()
            .map_err(|_| async_graphql::Error::new("User ID not available in context"))?;
        
        // Parse listing ID
        let listing_uuid = Uuid::parse_str(&listing_id.to_string())
            .map_err(|_| async_graphql::Error::new("Invalid listing ID"))?;
        
        // Update the skill listing
        let listing = skill_service.update_listing(
            listing_uuid,
            *provider_id,
            input.title,
            input.description,
            input.category,
            input.estimated_time,
        ).await
        .map_err(|e| match e {
            CommonError::ValidationError(msg) => async_graphql::Error::new(msg),
            CommonError::Unauthorized(msg) => async_graphql::Error::new(msg),
            CommonError::NotFound(msg) => async_graphql::Error::new(msg),
            _ => async_graphql::Error::new("Failed to update skill listing"),
        })?;
        
        Ok(listing.into())
    }
    
    /// Deactivate a skill listing
    async fn deactivate_skill_listing(
        &self,
        ctx: &Context<'_>,
        listing_id: ID,
    ) -> Result<SkillListingGQL> {
        // Get the skill exchange service from context
        let skill_service = ctx.data::<Box<dyn SkillExchangeService>>()
            .map_err(|_| async_graphql::Error::new("SkillExchangeService not available in context"))?;
        
        // Get the current user ID from context
        let provider_id = ctx.data::<Uuid>()
            .map_err(|_| async_graphql::Error::new("User ID not available in context"))?;
        
        // Parse listing ID
        let listing_uuid = Uuid::parse_str(&listing_id.to_string())
            .map_err(|_| async_graphql::Error::new("Invalid listing ID"))?;
        
        // Deactivate the skill listing
        let listing = skill_service.deactivate_listing(
            listing_uuid,
            *provider_id,
        ).await
        .map_err(|e| match e {
            CommonError::ValidationError(msg) => async_graphql::Error::new(msg),
            CommonError::Unauthorized(msg) => async_graphql::Error::new(msg),
            CommonError::NotFound(msg) => async_graphql::Error::new(msg),
            _ => async_graphql::Error::new("Failed to deactivate skill listing"),
        })?;
        
        Ok(listing.into())
    }
    
    /// Claim a skill listing
    async fn claim_skill_listing(
        &self,
        ctx: &Context<'_>,
        listing_id: ID,
        input: ClaimSkillListingInput,
    ) -> Result<SkillClaimGQL> {
        // Get the skill exchange service from context
        let skill_service = ctx.data::<Box<dyn SkillExchangeService>>()
            .map_err(|_| async_graphql::Error::new("SkillExchangeService not available in context"))?;
        
        // Get the current user ID from context
        let claimant_id = ctx.data::<Uuid>()
            .map_err(|_| async_graphql::Error::new("User ID not available in context"))?;
        
        // Parse listing ID
        let listing_uuid = Uuid::parse_str(&listing_id.to_string())
            .map_err(|_| async_graphql::Error::new("Invalid listing ID"))?;
        
        // Claim the skill listing
        let claim = skill_service.claim_listing(
            listing_uuid,
            *claimant_id,
            input.message,
        ).await
        .map_err(|e| match e {
            CommonError::ValidationError(msg) => async_graphql::Error::new(msg),
            CommonError::Unauthorized(msg) => async_graphql::Error::new(msg),
            CommonError::NotFound(msg) => async_graphql::Error::new(msg),
            _ => async_graphql::Error::new("Failed to claim skill listing"),
        })?;
        
        Ok(claim.into())
    }
    
    /// Accept a claim on a skill listing
    async fn accept_claim(
        &self,
        ctx: &Context<'_>,
        claim_id: ID,
    ) -> Result<SkillClaimGQL> {
        // Get the skill exchange service from context
        let skill_service = ctx.data::<Box<dyn SkillExchangeService>>()
            .map_err(|_| async_graphql::Error::new("SkillExchangeService not available in context"))?;
        
        // Get the current user ID from context
        let provider_id = ctx.data::<Uuid>()
            .map_err(|_| async_graphql::Error::new("User ID not available in context"))?;
        
        // Parse claim ID
        let claim_uuid = Uuid::parse_str(&claim_id.to_string())
            .map_err(|_| async_graphql::Error::new("Invalid claim ID"))?;
        
        // Accept the claim
        let claim = skill_service.accept_claim(
            claim_uuid,
            *provider_id,
        ).await
        .map_err(|e| match e {
            CommonError::ValidationError(msg) => async_graphql::Error::new(msg),
            CommonError::Unauthorized(msg) => async_graphql::Error::new(msg),
            CommonError::NotFound(msg) => async_graphql::Error::new(msg),
            _ => async_graphql::Error::new("Failed to accept claim"),
        })?;
        
        Ok(claim.into())
    }
    
    /// Reject a claim on a skill listing
    async fn reject_claim(
        &self,
        ctx: &Context<'_>,
        claim_id: ID,
    ) -> Result<SkillClaimGQL> {
        // Get the skill exchange service from context
        let skill_service = ctx.data::<Box<dyn SkillExchangeService>>()
            .map_err(|_| async_graphql::Error::new("SkillExchangeService not available in context"))?;
        
        // Get the current user ID from context
        let provider_id = ctx.data::<Uuid>()
            .map_err(|_| async_graphql::Error::new("User ID not available in context"))?;
        
        // Parse claim ID
        let claim_uuid = Uuid::parse_str(&claim_id.to_string())
            .map_err(|_| async_graphql::Error::new("Invalid claim ID"))?;
        
        // Reject the claim
        let claim = skill_service.reject_claim(
            claim_uuid,
            *provider_id,
        ).await
        .map_err(|e| match e {
            CommonError::ValidationError(msg) => async_graphql::Error::new(msg),
            CommonError::Unauthorized(msg) => async_graphql::Error::new(msg),
            CommonError::NotFound(msg) => async_graphql::Error::new(msg),
            _ => async_graphql::Error::new("Failed to reject claim"),
        })?;
        
        Ok(claim.into())
    }
    
    /// Complete a skill exchange
    async fn complete_skill_exchange(
        &self,
        ctx: &Context<'_>,
        claim_id: ID,
        input: CompleteSkillExchangeInput,
    ) -> Result<SkillExchangeCompletionGQL> {
        // Get the skill exchange service from context
        let skill_service = ctx.data::<Box<dyn SkillExchangeService>>()
            .map_err(|_| async_graphql::Error::new("SkillExchangeService not available in context"))?;
        
        // Get the current user ID from context
        let claimant_id = ctx.data::<Uuid>()
            .map_err(|_| async_graphql::Error::new("User ID not available in context"))?;
        
        // Parse claim ID
        let claim_uuid = Uuid::parse_str(&claim_id.to_string())
            .map_err(|_| async_graphql::Error::new("Invalid claim ID"))?;
        
        // Validate rating if provided
        let rating_value = if let Some(rating) = input.rating {
            if rating < 1 || rating > 5 {
                return Err(async_graphql::Error::new("Rating must be between 1 and 5"));
            }
            Some(rating as u32)
        } else {
            None
        };
        
        // Create payment amount if provided
        let payment_amount = if let Some(amount) = input.payment_amount {
            if amount <= 0.0 {
                return Err(async_graphql::Error::new("Payment amount must be positive"));
            }
            Some(Money::new(
                Decimal::from_f64(amount).unwrap_or_default(),
                Currency::Dabloons,
            ))
        } else {
            None
        };
        
        // Complete the skill exchange
        let completion = skill_service.complete_exchange(
            claim_uuid,
            *claimant_id,
            rating_value,
            input.review,
            payment_amount,
        ).await
        .map_err(|e| match e {
            CommonError::ValidationError(msg) => async_graphql::Error::new(msg),
            CommonError::Unauthorized(msg) => async_graphql::Error::new(msg),
            CommonError::NotFound(msg) => async_graphql::Error::new(msg),
            _ => async_graphql::Error::new("Failed to complete skill exchange"),
        })?;
        
        Ok(completion.into())
    }
}

/// Root query object for skill exchange functionality
pub struct SkillExchangeQuery;

#[Object]
impl SkillExchangeQuery {
    /// Search for active skill listings
    async fn search_skill_listings(
        &self,
        ctx: &Context<'_>,
        term: Option<String>,
        category: Option<String>,
    ) -> Result<Vec<SkillListingGQL>> {
        // Get the skill exchange service from context
        let skill_service = ctx.data::<Box<dyn SkillExchangeService>>()
            .map_err(|_| async_graphql::Error::new("SkillExchangeService not available in context"))?;
        
        // Search for listings
        let listings = skill_service.search_listings(term, category).await
            .map_err(|_| async_graphql::Error::new("Failed to search skill listings"))?;
        
        Ok(listings.into_iter().map(|l| l.into()).collect())
    }
    
    /// Get skill listings by provider
    async fn my_skill_listings(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<SkillListingGQL>> {
        // Get the skill exchange service from context
        let skill_service = ctx.data::<Box<dyn SkillExchangeService>>()
            .map_err(|_| async_graphql::Error::new("SkillExchangeService not available in context"))?;
        
        // Get the current user ID from context
        let provider_id = ctx.data::<Uuid>()
            .map_err(|_| async_graphql::Error::new("User ID not available in context"))?;
        
        // Get listings by provider
        let listings = skill_service.get_listings_by_provider(*provider_id).await
            .map_err(|_| async_graphql::Error::new("Failed to fetch skill listings"))?;
        
        Ok(listings.into_iter().map(|l| l.into()).collect())
    }
    
    /// Get claims by claimant
    async fn my_skill_claims(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<SkillClaimGQL>> {
        // Get the skill exchange service from context
        let skill_service = ctx.data::<Box<dyn SkillExchangeService>>()
            .map_err(|_| async_graphql::Error::new("SkillExchangeService not available in context"))?;
        
        // Get the current user ID from context
        let claimant_id = ctx.data::<Uuid>()
            .map_err(|_| async_graphql::Error::new("User ID not available in context"))?;
        
        // Get claims by claimant
        let claims = skill_service.get_claims_by_claimant(*claimant_id).await
            .map_err(|_| async_graphql::Error::new("Failed to fetch skill claims"))?;
        
        Ok(claims.into_iter().map(|c| c.into()).collect())
    }
}