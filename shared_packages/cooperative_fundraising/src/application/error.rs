//! Error types for the cooperative fundraising application

use thiserror::Error;
use crate::domain::{CampaignError, ContributionError};
use crate::application::repository::RepositoryError;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Campaign error: {0}")]
    CampaignError(#[from] CampaignError),
    
    #[error("Contribution error: {0}")]
    ContributionError(#[from] ContributionError),
    
    #[error("Repository error: {0}")]
    RepositoryError(#[from] RepositoryError),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("User already has membership")]
    UserAlreadyHasMembership,
    
    #[error("Monetary contributions not allowed for this campaign type")]
    MonetaryNotAllowed,
    
    #[error("Volunteer contributions not allowed for this campaign type")]
    VolunteerNotAllowed,
    
    #[error("Campaign requirements not met")]
    RequirementsNotMet,
    
    #[error("Insufficient funds")]
    InsufficientFunds,
    
    #[error("Payment processing error: {0}")]
    PaymentError(String),
    
    #[error("Integration error: {0}")]
    IntegrationError(String),
    
    #[error("Unexpected error")]
    Unexpected,
}