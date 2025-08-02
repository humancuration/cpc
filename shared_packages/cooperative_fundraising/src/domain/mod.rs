//! Domain models for the cooperative fundraising system

pub mod campaign;
pub mod contribution;
pub mod membership;

pub use campaign::{Campaign, CampaignType, CampaignStatus, MembershipRequirements, DonationDetails};
pub use contribution::{Contribution, ContributionType, VerificationStatus};
pub use membership::Membership;