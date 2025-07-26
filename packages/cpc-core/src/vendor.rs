use crate::models::{Vendor, VerificationStatus};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VerificationError {
    #[error("Invalid state transition: {0} -> {1}")]
    InvalidTransition(VerificationStatus, VerificationStatus),
    
    #[error("Vendor not found")]
    VendorNotFound,
}

pub struct VerificationStateMachine<'a> {
    vendor: &'a mut Vendor,
}

impl<'a> VerificationStateMachine<'a> {
    pub fn new(vendor: &'a mut Vendor) -> Self {
        Self { vendor }
    }

    pub fn submit(&mut self) -> Result<(), VerificationError> {
        if self.vendor.verification_status != VerificationStatus::Pending {
            return Err(VerificationError::InvalidTransition(
                self.vendor.verification_status.clone(),
                VerificationStatus::UnderReview,
            ));
        }
        self.vendor.verification_status = VerificationStatus::UnderReview;
        Ok(())
    }

    pub fn approve(&mut self) -> Result<(), VerificationError> {
        if self.vendor.verification_status != VerificationStatus::UnderReview {
            return Err(VerificationError::InvalidTransition(
                self.vendor.verification_status.clone(),
                VerificationStatus::Verified,
            ));
        }
        self.vendor.verification_status = VerificationStatus::Verified;
        Ok(())
    }

    pub fn reject(&mut self) -> Result<(), VerificationError> {
        if self.vendor.verification_status != VerificationStatus::UnderReview {
            return Err(VerificationError::InvalidTransition(
                self.vendor.verification_status.clone(),
                VerificationStatus::Rejected,
            ));
        }
        self.vendor.verification_status = VerificationStatus::Rejected;
        Ok(())
    }
}