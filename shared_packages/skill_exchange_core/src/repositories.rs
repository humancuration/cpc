//! Repository traits for skill exchange data access

use async_trait::async_trait;
use uuid::Uuid;
use crate::models::{SkillListing, SkillClaim, SkillExchangeCompletion};
use common_utils::error::CommonError;

/// Repository trait for skill exchange persistence
#[async_trait]
pub trait SkillExchangeRepository {
    /// Save a skill listing
    async fn save_listing(&self, listing: &SkillListing) -> Result<(), CommonError>;
    
    /// Find a skill listing by ID
    async fn find_listing_by_id(&self, id: Uuid) -> Result<Option<SkillListing>, CommonError>;
    
    /// Find all active listings
    async fn find_active_listings(&self) -> Result<Vec<SkillListing>, CommonError>;
    
    /// Find listings by provider
    async fn find_listings_by_provider(&self, provider_id: Uuid) -> Result<Vec<SkillListing>, CommonError>;
    
    /// Find listings by category
    async fn find_listings_by_category(&self, category: &str) -> Result<Vec<SkillListing>, CommonError>;
    
    /// Find listings by search term
    async fn search_listings(&self, term: &str) -> Result<Vec<SkillListing>, CommonError>;
    
    /// Save a skill claim
    async fn save_claim(&self, claim: &SkillClaim) -> Result<(), CommonError>;
    
    /// Find a skill claim by ID
    async fn find_claim_by_id(&self, id: Uuid) -> Result<Option<SkillClaim>, CommonError>;
    
    /// Find claims for a listing
    async fn find_claims_by_listing_id(&self, listing_id: Uuid) -> Result<Vec<SkillClaim>, CommonError>;
    
    /// Find claims by claimant
    async fn find_claims_by_claimant_id(&self, claimant_id: Uuid) -> Result<Vec<SkillClaim>, CommonError>;
    
    /// Save a skill exchange completion
    async fn save_completion(&self, completion: &SkillExchangeCompletion) -> Result<(), CommonError>;
    
    /// Find a skill exchange completion by ID
    async fn find_completion_by_id(&self, id: Uuid) -> Result<Option<SkillExchangeCompletion>, CommonError>;
    
    /// Find completions for a provider
    async fn find_completions_by_provider(&self, provider_id: Uuid) -> Result<Vec<SkillExchangeCompletion>, CommonError>;
    
    /// Find completions for a claimant
    async fn find_completions_by_claimant(&self, claimant_id: Uuid) -> Result<Vec<SkillExchangeCompletion>, CommonError>;
}