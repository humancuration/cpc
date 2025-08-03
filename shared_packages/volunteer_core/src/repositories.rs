//! Repository traits for volunteer data access

use async_trait::async_trait;
use uuid::Uuid;
use crate::models::{VolunteerActivity, VolunteerVerification, DabloonConversion};
use common_utils::error::CommonError;

/// Repository trait for volunteer activity persistence
#[async_trait]
pub trait VolunteerRepository {
    /// Save a volunteer activity
    async fn save_activity(&self, activity: &VolunteerActivity) -> Result<(), CommonError>;
    
    /// Find a volunteer activity by ID
    async fn find_activity_by_id(&self, id: Uuid) -> Result<Option<VolunteerActivity>, CommonError>;
    
    /// Find all volunteer activities for a user
    async fn find_activities_by_user_id(&self, user_id: Uuid) -> Result<Vec<VolunteerActivity>, CommonError>;
    
    /// Find unverified activities for an organization
    async fn find_unverified_activities_by_organization(&self, organization_id: Uuid) -> Result<Vec<VolunteerActivity>, CommonError>;
    
    /// Save a volunteer verification
    async fn save_verification(&self, verification: &VolunteerVerification) -> Result<(), CommonError>;
    
    /// Find a volunteer verification by ID
    async fn find_verification_by_id(&self, id: Uuid) -> Result<Option<VolunteerVerification>, CommonError>;
    
    /// Find verifications for an activity
    async fn find_verifications_by_activity_id(&self, activity_id: Uuid) -> Result<Vec<VolunteerVerification>, CommonError>;
    
    /// Save a Dabloon conversion
    async fn save_conversion(&self, conversion: &DabloonConversion) -> Result<(), CommonError>;
    
    /// Find a Dabloon conversion by ID
    async fn find_conversion_by_id(&self, id: Uuid) -> Result<Option<DabloonConversion>, CommonError>;
    
    /// Find conversions for a user
    async fn find_conversions_by_user_id(&self, user_id: Uuid) -> Result<Vec<DabloonConversion>, CommonError>;
}