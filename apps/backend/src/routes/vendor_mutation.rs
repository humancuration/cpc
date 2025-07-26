use async_graphql::{Context, Error, Object, Result, Uuid};
use crate::models::vendor::{Vendor, VerificationStatus};
use crate::services::database::Database;
use validator::Validate;

#[derive(Default)]
pub struct VendorMutation;

#[Object]
impl VendorMutation {
    async fn create_vendor(&self, ctx: &Context<'_>, input: VendorInput) -> Result<Vendor> {
        let db = ctx.data::<Database>()?;
        let mut vendor = Vendor {
            id: Uuid::new_v4(),
            name: input.name,
            description: input.description,
            verification_status: VerificationStatus::Pending,
            user_id: Uuid::parse_str(&input.user_id).map_err(|e| Error::new(e.to_string()))?,
        };

        vendor.validate().map_err(|e| Error::new(e.to_string()))?;

        let created = db.create_vendor(&vendor).await?;
        Ok(created)
    }

    async fn update_vendor_verification(
        &self,
        ctx: &Context<'_>,
        input: UpdateVerificationInput,
    ) -> Result<Vendor> {
        let db = ctx.data::<Database>()?;
        let mut vendor = db.get_vendor(&input.vendor_id).await?;
        
        vendor.verification_status = match input.status.as_str() {
            "PENDING" => VerificationStatus::Pending,
            "UNDER_REVIEW" => VerificationStatus::UnderReview,
            "VERIFIED" => VerificationStatus::Verified,
            "REJECTED" => VerificationStatus::Rejected,
            _ => return Err(Error::new("Invalid verification status")),
        };

        let updated = db.update_vendor(&vendor).await?;
        Ok(updated)
    }
}

#[derive(async_graphql::InputObject, Validate)]
pub struct VendorInput {
    #[validate(length(min = 1, message = "Name must be 1-100 characters"))]
    pub name: String,
    
    #[validate(length(max = 500, message = "Description must be less than 500 characters"))]
    pub description: Option<String>,
    
    #[validate(length(min = 1, message = "User ID is required"))]
    pub user_id: String,
}

#[derive(async_graphql::InputObject)]
pub struct UpdateVerificationInput {
    pub vendor_id: String,
    pub status: String,
}