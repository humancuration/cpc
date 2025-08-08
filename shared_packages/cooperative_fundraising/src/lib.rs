//! Cooperative Fundraising System
//!
//! This crate provides functionality for cooperative fundraising, including:
//! - Membership share management
//! - Campaign creation and management
//! - Contribution processing (both monetary and volunteer)
//! - Integration with cpay for monetary transactions
//! - Integration with skill_volunteering for volunteer tracking

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod optimization;

// Re-export generated protobuf types
#[allow(clippy::all)]
pub mod proto {
    tonic::include_proto!("cooperative_fundraising");
}

pub use proto::cooperative_fundraising_service_client::CooperativeFundraisingServiceClient;
pub use proto::cooperative_fundraising_service_server::CooperativeFundraisingServiceServer;

// Re-export key types
pub use domain::{Campaign, Contribution, Membership, CampaignType, CampaignStatus};
pub use application::{CampaignService, ContributionService, MembershipService};