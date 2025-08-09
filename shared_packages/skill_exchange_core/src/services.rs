//! Service layer for skill exchange functionality

use async_trait::async_trait;
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::models::{SkillListing, SkillClaim, SkillExchangeCompletion, SkillRating, ClaimStatus};
use crate::repositories::SkillExchangeRepository;
use cpc_wallet::application::WalletService;
use cpc_wallet::domain::primitives::{Money, Currency};
use notification_core::application::service::NotificationService;
use notification_core::domain::{types::{Notification, NotificationCategory, NotificationPriority, ChannelType}, preferences::UserPreferences};
use social_integration::domain::social_event::SocialEvent;
use social_integration::application::social_integration_service::SocialIntegrationService;
use common_utils::error::CommonError;
use std::sync::Arc;
use tokio::sync::broadcast;

// Conditionally import common_utils logging or fallback to tracing
#[cfg(feature = "common-utils-integration")]
use common_utils::logging::{info, warn, error, debug};
#[cfg(not(feature = "common-utils-integration"))]
use tracing::{info, warn, error, debug};

/// Service trait for skill exchange operations
#[async_trait]
pub trait SkillExchangeService {
    /// Create a new skill listing
    async fn create_listing(
        &self,
        provider_id: Uuid,
        title: String,
        description: String,
        category: String,
        estimated_time: Option<Decimal>,
    ) -> Result<SkillListing, CommonError>;
    
    /// Update an existing skill listing
    async fn update_listing(
        &self,
        listing_id: Uuid,
        provider_id: Uuid,
        title: Option<String>,
        description: Option<String>,
        category: Option<String>,
        estimated_time: Option<Decimal>,
    ) -> Result<SkillListing, CommonError>;
    
    /// Deactivate a skill listing
    async fn deactivate_listing(
        &self,
        listing_id: Uuid,
        provider_id: Uuid,
    ) -> Result<SkillListing, CommonError>;
    
    /// Search for active skill listings
    async fn search_listings(
        &self,
        term: Option<String>,
        category: Option<String>,
    ) -> Result<Vec<SkillListing>, CommonError>;
    
    /// Claim a skill listing
    async fn claim_listing(
        &self,
        listing_id: Uuid,
        claimant_id: Uuid,
        message: Option<String>,
    ) -> Result<SkillClaim, CommonError>;
    
    /// Accept a claim on a skill listing
    async fn accept_claim(
        &self,
        claim_id: Uuid,
        provider_id: Uuid,
    ) -> Result<SkillClaim, CommonError>;
    
    /// Reject a claim on a skill listing
    async fn reject_claim(
        &self,
        claim_id: Uuid,
        provider_id: Uuid,
    ) -> Result<SkillClaim, CommonError>;
    
    /// Complete a skill exchange
    async fn complete_exchange(
        &self,
        claim_id: Uuid,
        claimant_id: Uuid,
        rating: Option<u32>,
        review: Option<String>,
        payment_amount: Option<Money>,
    ) -> Result<SkillExchangeCompletion, CommonError>;
    
    /// Get listings by provider
    async fn get_listings_by_provider(&self, provider_id: Uuid) -> Result<Vec<SkillListing>, CommonError>;
    
    /// Get claims by claimant
    async fn get_claims_by_claimant(&self, claimant_id: Uuid) -> Result<Vec<SkillClaim>, CommonError>;
}

/// Implementation of the SkillExchangeService
pub struct SkillExchangeServiceImpl {
    skill_repo: Arc<dyn SkillExchangeRepository>,
    wallet_service: Arc<dyn WalletService>,
    notification_service: Arc<dyn NotificationService>,
    social_service: Arc<dyn SocialIntegrationService>,
    // For broadcasting events
    skill_events: broadcast::Sender<SkillExchangeEvent>,
}

/// Events that can be broadcast by the skill exchange service
#[derive(Debug, Clone)]
pub enum SkillExchangeEvent {
    /// A new skill listing was created
    ListingCreated { listing_id: Uuid, provider_id: Uuid },
    
    /// A skill listing was claimed
    ListingClaimed { listing_id: Uuid, claim_id: Uuid, claimant_id: Uuid },
    
    /// A claim was accepted
    ClaimAccepted { claim_id: Uuid, listing_id: Uuid },
    
    /// A claim was rejected
    ClaimRejected { claim_id: Uuid, listing_id: Uuid },
    
    /// A skill exchange was completed
    ExchangeCompleted { completion_id: Uuid, claim_id: Uuid, provider_id: Uuid, claimant_id: Uuid },
}

impl SkillExchangeServiceImpl {
    /// Create a new skill exchange service
    pub fn new(
        skill_repo: Arc<dyn SkillExchangeRepository>,
        wallet_service: Arc<dyn WalletService>,
        notification_service: Arc<dyn NotificationService>,
        social_service: Arc<dyn SocialIntegrationService>,
    ) -> Self {
        let (skill_events, _) = broadcast::channel(100);
        Self {
            skill_repo,
            wallet_service,
            notification_service,
            social_service,
            skill_events,
        }
    }
    
    /// Subscribe to skill exchange events
    pub fn subscribe_skill_events(&self) -> broadcast::Receiver<SkillExchangeEvent> {
        self.skill_events.subscribe()
    }
    
    /// Send a notification to a user
    async fn send_notification(
        &self,
        user_id: Uuid,
        title: String,
        body: String,
        category: NotificationCategory,
    ) -> Result<(), CommonError> {
        let notification = Notification::new_immediate(
            user_id.to_string(),
            category,
            NotificationPriority::Normal,
            title,
            body,
            serde_json::Value::Null,
            vec![ChannelType::InApp, ChannelType::Email],
        );
        
        self.notification_service.send_notification(notification).await
            .map_err(|e| CommonError::ServiceError(format!("Failed to send notification: {}", e)))
    }
    
    /// Create a social event for a skill exchange
    async fn create_social_event(&self, event: SocialEvent) -> Result<(), CommonError> {
        self.social_service.handle_social_event(event).await
            .map_err(|e| CommonError::ServiceError(format!("Failed to create social event: {}", e)))
    }
}

#[async_trait]
impl SkillExchangeService for SkillExchangeServiceImpl {
    async fn create_listing(
        &self,
        provider_id: Uuid,
        title: String,
        description: String,
        category: String,
        estimated_time: Option<Decimal>,
    ) -> Result<SkillListing, CommonError> {
        // Validate inputs
        if title.trim().is_empty() {
            return Err(CommonError::ValidationError("Title cannot be empty".to_string()));
        }
        
        if description.trim().is_empty() {
            return Err(CommonError::ValidationError("Description cannot be empty".to_string()));
        }
        
        // Create the listing
        let listing = SkillListing::new(provider_id, title, description, category, estimated_time);
        
        // Save the listing
        self.skill_repo.save_listing(&listing).await?;
        
        // Broadcast the event
        let _ = self.skill_events.send(SkillExchangeEvent::ListingCreated {
            listing_id: listing.id,
            provider_id: listing.provider_id,
        });
        
        Ok(listing)
    }
    
    async fn update_listing(
        &self,
        listing_id: Uuid,
        provider_id: Uuid,
        title: Option<String>,
        description: Option<String>,
        category: Option<String>,
        estimated_time: Option<Decimal>,
    ) -> Result<SkillListing, CommonError> {
        // Find the listing
        let mut listing = self.skill_repo.find_listing_by_id(listing_id).await?
            .ok_or_else(|| CommonError::NotFound("Skill listing not found".to_string()))?;
        
        // Check if the provider owns the listing
        if listing.provider_id != provider_id {
            return Err(CommonError::Unauthorized("You can only update your own listings".to_string()));
        }
        
        // Update the listing
        listing.update(title, description, category, estimated_time);
        
        // Save the updated listing
        self.skill_repo.save_listing(&listing).await?;
        
        Ok(listing)
    }
    
    async fn deactivate_listing(
        &self,
        listing_id: Uuid,
        provider_id: Uuid,
    ) -> Result<SkillListing, CommonError> {
        // Find the listing
        let mut listing = self.skill_repo.find_listing_by_id(listing_id).await?
            .ok_or_else(|| CommonError::NotFound("Skill listing not found".to_string()))?;
        
        // Check if the provider owns the listing
        if listing.provider_id != provider_id {
            return Err(CommonError::Unauthorized("You can only deactivate your own listings".to_string()));
        }
        
        // Deactivate the listing
        listing.deactivate();
        
        // Save the updated listing
        self.skill_repo.save_listing(&listing).await?;
        
        Ok(listing)
    }
    
    async fn search_listings(
        &self,
        term: Option<String>,
        category: Option<String>,
    ) -> Result<Vec<SkillListing>, CommonError> {
        if let Some(term) = term {
            // Search by term
            self.skill_repo.search_listings(&term).await
        } else if let Some(category) = category {
            // Search by category
            self.skill_repo.find_listings_by_category(&category).await
        } else {
            // Get all active listings
            self.skill_repo.find_active_listings().await
        }
    }
    
    async fn claim_listing(
        &self,
        listing_id: Uuid,
        claimant_id: Uuid,
        message: Option<String>,
    ) -> Result<SkillClaim, CommonError> {
        // Find the listing
        let listing = self.skill_repo.find_listing_by_id(listing_id).await?
            .ok_or_else(|| CommonError::NotFound("Skill listing not found".to_string()))?;
        
        // Check if the listing is active
        if !listing.is_active {
            return Err(CommonError::ValidationError("This listing is no longer active".to_string()));
        }
        
        // Check if the claimant is the provider (can't claim your own listing)
        if listing.provider_id == claimant_id {
            return Err(CommonError::ValidationError("You cannot claim your own listing".to_string()));
        }
        
        // Create the claim
        let claim = SkillClaim::new(listing_id, claimant_id, message);
        
        // Save the claim
        self.skill_repo.save_claim(&claim).await?;
        
        // Send notification to provider
        self.send_notification(
            listing.provider_id,
            "Skill Listing Claimed".to_string(),
            format!("Your skill listing '{}' has been claimed.", listing.title),
            NotificationCategory::Social,
        ).await?;
        
        // Broadcast the event
        let _ = self.skill_events.send(SkillExchangeEvent::ListingClaimed {
            listing_id: listing.id,
            claim_id: claim.id,
            claimant_id: claim.claimant_id,
        });
        
        Ok(claim)
    }
    
    async fn accept_claim(
        &self,
        claim_id: Uuid,
        provider_id: Uuid,
    ) -> Result<SkillClaim, CommonError> {
        // Find the claim
        let mut claim = self.skill_repo.find_claim_by_id(claim_id).await?
            .ok_or_else(|| CommonError::NotFound("Claim not found".to_string()))?;
        
        // Find the listing
        let listing = self.skill_repo.find_listing_by_id(claim.listing_id).await?
            .ok_or_else(|| CommonError::NotFound("Skill listing not found".to_string()))?;
        
        // Check if the provider owns the listing
        if listing.provider_id != provider_id {
            return Err(CommonError::Unauthorized("You can only accept claims on your own listings".to_string()));
        }
        
        // Accept the claim
        claim.accept();
        
        // Save the updated claim
        self.skill_repo.save_claim(&claim).await?;
        
        // Send notification to claimant
        self.send_notification(
            claim.claimant_id,
            "Claim Accepted".to_string(),
            format!("Your claim on '{}' has been accepted.", listing.title),
            NotificationCategory::Social,
        ).await?;
        
        // Broadcast the event
        let _ = self.skill_events.send(SkillExchangeEvent::ClaimAccepted {
            claim_id: claim.id,
            listing_id: listing.id,
        });
        
        Ok(claim)
    }
    
    async fn reject_claim(
        &self,
        claim_id: Uuid,
        provider_id: Uuid,
    ) -> Result<SkillClaim, CommonError> {
        // Find the claim
        let mut claim = self.skill_repo.find_claim_by_id(claim_id).await?
            .ok_or_else(|| CommonError::NotFound("Claim not found".to_string()))?;
        
        // Find the listing
        let listing = self.skill_repo.find_listing_by_id(claim.listing_id).await?
            .ok_or_else(|| CommonError::NotFound("Skill listing not found".to_string()))?;
        
        // Check if the provider owns the listing
        if listing.provider_id != provider_id {
            return Err(CommonError::Unauthorized("You can only reject claims on your own listings".to_string()));
        }
        
        // Reject the claim
        claim.reject();
        
        // Save the updated claim
        self.skill_repo.save_claim(&claim).await?;
        
        // Send notification to claimant
        self.send_notification(
            claim.claimant_id,
            "Claim Rejected".to_string(),
            format!("Your claim on '{}' has been rejected.", listing.title),
            NotificationCategory::Social,
        ).await?;
        
        // Broadcast the event
        let _ = self.skill_events.send(SkillExchangeEvent::ClaimRejected {
            claim_id: claim.id,
            listing_id: listing.id,
        });
        
        Ok(claim)
    }
    
    async fn complete_exchange(
        &self,
        claim_id: Uuid,
        claimant_id: Uuid,
        rating_value: Option<u32>,
        review: Option<String>,
        payment_amount: Option<Money>,
    ) -> Result<SkillExchangeCompletion, CommonError> {
        // Find the claim
        let mut claim = self.skill_repo.find_claim_by_id(claim_id).await?
            .ok_or_else(|| CommonError::NotFound("Claim not found".to_string()))?;
        
        // Check if the claim belongs to the claimant
        if claim.claimant_id != claimant_id {
            return Err(CommonError::Unauthorized("You can only complete your own claims".to_string()));
        }
        
        // Check if the claim is accepted
        if claim.status != ClaimStatus::Accepted {
            return Err(CommonError::ValidationError("Claim must be accepted before completion".to_string()));
        }
        
        // Find the listing
        let listing = self.skill_repo.find_listing_by_id(claim.listing_id).await?
            .ok_or_else(|| CommonError::NotFound("Skill listing not found".to_string()))?;
        
        // Create rating if provided
        let rating = if let Some(value) = rating_value {
            Some(SkillRating::new(value, review)?)
        } else {
            None
        };
        
        // Handle payment if provided
        let (payment, transaction_id) = if let Some(amount) = payment_amount {
            // Validate currency
            if amount.currency != Currency::Dabloons {
                return Err(CommonError::ValidationError("Only Dabloons are supported for payments".to_string()));
            }
            
            // Transfer Dabloons from claimant to provider
            let (_, _) = self.wallet_service.transfer_dabloons(
                claimant_id,
                listing.provider_id,
                amount.clone(),
                Some(format!("Payment for skill exchange: {}", listing.title)),
            ).await?;
            
            // Use claim ID as transaction ID proxy
            (Some(amount), Some(claim.id))
        } else {
            (None, None)
        };
        
        // Create the completion record
        let completion = SkillExchangeCompletion::new(
            listing.id,
            claim.id,
            listing.provider_id,
            claim.claimant_id,
            rating,
            payment,
            transaction_id,
        );
        
        // Save the completion
        self.skill_repo.save_completion(&completion).await?;
        
        // Mark claim as completed
        claim.complete();
        self.skill_repo.save_claim(&claim).await?;
        
        // Send notification to provider
        self.send_notification(
            listing.provider_id,
            "Skill Exchange Completed".to_string(),
            format!("Your skill exchange '{}' has been completed.", listing.title),
            NotificationCategory::Social,
        ).await?;
        
        // Create social event
        let event = SocialEvent::Volunteered {
            user_id: claim.claimant_id,
            opportunity_id: listing.id, // Using listing ID as proxy
            hours_contributed: 1.0, // Default value
            timestamp: chrono::Utc::now(),
        };
        self.create_social_event(event).await?;
        
        // Broadcast the event
        let _ = self.skill_events.send(SkillExchangeEvent::ExchangeCompleted {
            completion_id: completion.id,
            claim_id: claim.id,
            provider_id: listing.provider_id,
            claimant_id: claim.claimant_id,
        });
        
        Ok(completion)
    }
    
    async fn get_listings_by_provider(&self, provider_id: Uuid) -> Result<Vec<SkillListing>, CommonError> {
        self.skill_repo.find_listings_by_provider(provider_id).await
    }
    
    async fn get_claims_by_claimant(&self, claimant_id: Uuid) -> Result<Vec<SkillClaim>, CommonError> {
        self.skill_repo.find_claims_by_claimant_id(claimant_id).await
    }
}