//! GraphQL resolvers for volunteer functionality

use async_graphql::{Context, Object, Result, SimpleObject, InputObject, ID};
use uuid::Uuid;
use rust_decimal::Decimal;
use volunteer_core::models::{VolunteerActivity, VolunteerVerification, DabloonConversion, VerificationStatus};
use volunteer_core::services::VolunteerService;
use common_utils::error::CommonError;

/// GraphQL input for logging volunteer hours
#[derive(InputObject)]
pub struct LogVolunteerHoursInput {
    /// Organization ID (optional)
    pub organization_id: Option<ID>,
    
    /// Description of the volunteer activity
    pub description: String,
    
    /// Number of hours volunteered
    pub hours: Decimal,
}

/// GraphQL input for verifying volunteer hours
#[derive(InputObject)]
pub struct VerifyVolunteerHoursInput {
    /// Whether to approve the hours
    pub approved: bool,
    
    /// Optional notes about the verification
    pub notes: Option<String>,
}

/// GraphQL object for volunteer activity
#[derive(SimpleObject)]
pub struct VolunteerActivityGQL {
    /// Unique identifier for the activity
    pub id: ID,
    
    /// User who performed the volunteer work
    pub user_id: ID,
    
    /// Organization associated with the activity (if any)
    pub organization_id: Option<ID>,
    
    /// Description of the volunteer activity
    pub description: String,
    
    /// Number of hours volunteered
    pub hours: Decimal,
    
    /// Whether the hours have been verified by an organization admin
    pub verified: bool,
    
    /// ID of the verifier (if verified)
    pub verified_by: Option<ID>,
    
    /// Timestamp when the activity was verified
    pub verified_at: Option<String>,
    
    /// Whether the volunteer hours have been converted to Dabloons
    pub converted_to_dabloons: bool,
    
    /// Timestamp when the activity was created
    pub created_at: String,
}

impl From<VolunteerActivity> for VolunteerActivityGQL {
    fn from(activity: VolunteerActivity) -> Self {
        Self {
            id: ID::from(activity.id.to_string()),
            user_id: ID::from(activity.user_id.to_string()),
            organization_id: activity.organization_id.map(|id| ID::from(id.to_string())),
            description: activity.description,
            hours: activity.hours,
            verified: activity.verified,
            verified_by: activity.verified_by.map(|id| ID::from(id.to_string())),
            verified_at: activity.verified_at.map(|dt| dt.to_rfc3339()),
            converted_to_dabloons: activity.converted_to_dabloons,
            created_at: activity.created_at.to_rfc3339(),
        }
    }
}

/// GraphQL object for Dabloon conversion
#[derive(SimpleObject)]
pub struct DabloonConversionGQL {
    /// Unique identifier for the conversion
    pub id: ID,
    
    /// ID of the volunteer activity that was converted
    pub activity_id: ID,
    
    /// User who performed the volunteer work
    pub user_id: ID,
    
    /// Number of hours that were converted
    pub hours_converted: Decimal,
    
    /// Amount of Dabloons credited
    pub dabloons_credited: f64,
    
    /// Timestamp when the conversion was created
    pub created_at: String,
}

impl From<DabloonConversion> for DabloonConversionGQL {
    fn from(conversion: DabloonConversion) -> Self {
        Self {
            id: ID::from(conversion.id.to_string()),
            activity_id: ID::from(conversion.activity_id.to_string()),
            user_id: ID::from(conversion.user_id.to_string()),
            hours_converted: conversion.hours_converted,
            dabloons_credited: conversion.dabloons_credited.amount.to_f64().unwrap_or(0.0),
            created_at: conversion.created_at.to_rfc3339(),
        }
    }
}

/// Root mutation object for volunteer functionality
pub struct VolunteerMutation;

#[Object]
impl VolunteerMutation {
    /// Log volunteer hours for the current user
    async fn log_volunteer_hours(
        &self,
        ctx: &Context<'_>,
        input: LogVolunteerHoursInput,
    ) -> Result<VolunteerActivityGQL> {
        // Get the volunteer service from context
        let volunteer_service = ctx.data::<Box<dyn VolunteerService>>()
            .map_err(|_| async_graphql::Error::new("VolunteerService not available in context"))?;
        
        // Get the current user ID from context
        let user_id = ctx.data::<Uuid>()
            .map_err(|_| async_graphql::Error::new("User ID not available in context"))?;
        
        // Parse organization ID if provided
        let organization_id = input.organization_id
            .map(|id| Uuid::parse_str(&id.to_string()))
            .transpose()
            .map_err(|_| async_graphql::Error::new("Invalid organization ID"))?;
        
        // Log the volunteer hours
        let activity = volunteer_service.log_volunteer_hours(
            *user_id,
            organization_id,
            input.description,
            input.hours,
        ).await
        .map_err(|e| match e {
            CommonError::ValidationError(msg) => async_graphql::Error::new(msg),
            CommonError::Unauthorized(msg) => async_graphql::Error::new(msg),
            _ => async_graphql::Error::new("Failed to log volunteer hours"),
        })?;
        
        Ok(activity.into())
    }
    
    /// Verify volunteer hours (organization admin only)
    async fn verify_volunteer_hours(
        &self,
        ctx: &Context<'_>,
        activity_id: ID,
        input: VerifyVolunteerHoursInput,
    ) -> Result<VolunteerActivityGQL> {
        // Get the volunteer service from context
        let volunteer_service = ctx.data::<Box<dyn VolunteerService>>()
            .map_err(|_| async_graphql::Error::new("VolunteerService not available in context"))?;
        
        // Get the current user ID from context (verifier)
        let verifier_id = ctx.data::<Uuid>()
            .map_err(|_| async_graphql::Error::new("User ID not available in context"))?;
        
        // Parse activity ID
        let activity_uuid = Uuid::parse_str(&activity_id.to_string())
            .map_err(|_| async_graphql::Error::new("Invalid activity ID"))?;
        
        // Verify the volunteer hours
        let activity = volunteer_service.verify_volunteer_hours(
            activity_uuid,
            *verifier_id,
            input.approved,
            input.notes,
        ).await
        .map_err(|e| match e {
            CommonError::ValidationError(msg) => async_graphql::Error::new(msg),
            CommonError::Unauthorized(msg) => async_graphql::Error::new(msg),
            CommonError::NotFound(msg) => async_graphql::Error::new(msg),
            _ => async_graphql::Error::new("Failed to verify volunteer hours"),
        })?;
        
        Ok(activity.into())
    }
    
    /// Convert verified volunteer hours to Dabloons
    async fn convert_to_dabloons(
        &self,
        ctx: &Context<'_>,
        activity_id: ID,
    ) -> Result<DabloonConversionGQL> {
        // Get the volunteer service from context
        let volunteer_service = ctx.data::<Box<dyn VolunteerService>>()
            .map_err(|_| async_graphql::Error::new("VolunteerService not available in context"))?;
        
        // Get the current user ID from context
        let user_id = ctx.data::<Uuid>()
            .map_err(|_| async_graphql::Error::new("User ID not available in context"))?;
        
        // Parse activity ID
        let activity_uuid = Uuid::parse_str(&activity_id.to_string())
            .map_err(|_| async_graphql::Error::new("Invalid activity ID"))?;
        
        // Convert to Dabloons
        let conversion = volunteer_service.convert_to_dabloons(
            activity_uuid,
            *user_id,
        ).await
        .map_err(|e| match e {
            CommonError::ValidationError(msg) => async_graphql::Error::new(msg),
            CommonError::Unauthorized(msg) => async_graphql::Error::new(msg),
            CommonError::NotFound(msg) => async_graphql::Error::new(msg),
            _ => async_graphql::Error::new("Failed to convert volunteer hours to Dabloons"),
        })?;
        
        Ok(conversion.into())
    }
}

/// Root query object for volunteer functionality
pub struct VolunteerQuery;

#[Object]
impl VolunteerQuery {
    /// Get all volunteer activities for the current user
    async fn my_volunteer_activities(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<VolunteerActivityGQL>> {
        // Get the volunteer service from context
        let volunteer_service = ctx.data::<Box<dyn VolunteerService>>()
            .map_err(|_| async_graphql::Error::new("VolunteerService not available in context"))?;
        
        // Get the current user ID from context
        let user_id = ctx.data::<Uuid>()
            .map_err(|_| async_graphql::Error::new("User ID not available in context"))?;
        
        // Get user activities
        let activities = volunteer_service.get_user_activities(*user_id).await
            .map_err(|_| async_graphql::Error::new("Failed to fetch volunteer activities"))?;
        
        Ok(activities.into_iter().map(|a| a.into()).collect())
    }
    
    /// Get verified volunteer activities for the current user
    async fn my_verified_volunteer_activities(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<VolunteerActivityGQL>> {
        // Get the volunteer service from context
        let volunteer_service = ctx.data::<Box<dyn VolunteerService>>()
            .map_err(|_| async_graphql::Error::new("VolunteerService not available in context"))?;
        
        // Get the current user ID from context
        let user_id = ctx.data::<Uuid>()
            .map_err(|_| async_graphql::Error::new("User ID not available in context"))?;
        
        // Get verified user activities
        let activities = volunteer_service.get_verified_user_activities(*user_id).await
            .map_err(|_| async_graphql::Error::new("Failed to fetch verified volunteer activities"))?;
        
        Ok(activities.into_iter().map(|a| a.into()).collect())
    }
}