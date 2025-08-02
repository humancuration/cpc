//! Skill Volunteering Module
//!
//! This module provides functionality for skill-based volunteering opportunities,
//! including skill management, opportunity publishing, volunteer matching, and impact tracking.

pub mod skill_management;
pub mod opportunity_management;
pub mod user_skill_management;
pub mod service;
pub mod postgres;
pub mod proto {
    tonic::include_proto!("skill_volunteering");
}

// Re-export key types
pub use skill_management::models::Skill;
pub use opportunity_management::models::{VolunteerOpportunity, OpportunityApplication};
pub use user_skill_management::models::{UserSkill, UserSkillDetails};
pub use service::SkillVolunteeringServiceImpl;