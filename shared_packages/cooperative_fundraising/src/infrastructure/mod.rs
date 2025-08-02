//! Infrastructure implementations for the cooperative fundraising system

pub mod postgres;
pub mod grpc;

pub use postgres::{PostgresCampaignRepository, PostgresContributionRepository, PostgresMembershipRepository};
pub use grpc::CooperativeFundraisingServiceImpl;