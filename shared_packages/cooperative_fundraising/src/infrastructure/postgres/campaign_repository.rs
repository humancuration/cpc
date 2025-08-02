//! PostgreSQL implementation of CampaignRepository

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::{Campaign, CampaignType, CampaignStatus, MembershipRequirements, DonationDetails};
use crate::application::repository::{CampaignRepository, PaginatedCampaigns, RepositoryError};
use crate::application::ApplicationError;
use serde_json::Value;
use rust_decimal::Decimal;

pub struct PostgresCampaignRepository {
    pool: PgPool,
}

impl PostgresCampaignRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CampaignRepository for PostgresCampaignRepository {
    async fn save(&self, campaign: &Campaign) -> Result<(), ApplicationError> {
        // Begin transaction
        let mut tx = self.pool.begin().await.map_err(RepositoryError::DatabaseError)?;
        
        // Save the main campaign
        sqlx::query!(
            r#"
            INSERT INTO campaigns (id, type, title, description, created_at, owner_user_id, status)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                type = $2,
                title = $3,
                description = $4,
                owner_user_id = $6,
                status = $7
            "#,
            campaign.id,
            campaign.campaign_type.to_string(),
            campaign.title,
            campaign.description,
            campaign.created_at,
            campaign.owner_user_id,
            campaign.status.to_string(),
        )
        .execute(&mut *tx)
        .await
        .map_err(RepositoryError::DatabaseError)?;
        
        // Save membership requirements if this is a membership campaign
        if let Some(requirements) = &campaign.membership_requirements {
            sqlx::query!(
                r#"
                INSERT INTO membership_requirements (campaign_id, max_participants, required_actions)
                VALUES ($1, $2, $3)
                ON CONFLICT (campaign_id) DO UPDATE SET
                    max_participants = $2,
                    required_actions = $3
                "#,
                campaign.id,
                requirements.max_participants as Option<i32>,
                &serde_json::to_value(&requirements.required_actions).map_err(|_| RepositoryError::Unexpected)?,
            )
            .execute(&mut *tx)
            .await
            .map_err(RepositoryError::DatabaseError)?;
        }
        
        // Save donation details if this is a donation campaign
        if let Some(details) = &campaign.donation_details {
            sqlx::query!(
                r#"
                INSERT INTO donation_campaigns (campaign_id, funding_goal, external_use_case, currency)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (campaign_id) DO UPDATE SET
                    funding_goal = $2,
                    external_use_case = $3,
                    currency = $4
                "#,
                campaign.id,
                details.funding_goal,
                details.external_use_case,
                details.currency,
            )
            .execute(&mut *tx)
            .await
            .map_err(RepositoryError::DatabaseError)?;
        }
        
        // Commit transaction
        tx.commit().await.map_err(RepositoryError::DatabaseError)?;
        
        Ok(())
    }
    
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Campaign>, ApplicationError> {
        // Get the main campaign
        let row = sqlx::query!(
            r#"
            SELECT id, type, title, description, created_at, owner_user_id, status
            FROM campaigns
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;
        
        let row = match row {
            Some(row) => row,
            None => return Ok(None),
        };
        
        // Parse campaign type
        let campaign_type = match row.r#type.as_str() {
            "cooperative_membership" => CampaignType::CooperativeMembership,
            "pure_donation" => CampaignType::PureDonation,
            "reg_cf" => CampaignType::RegCF,
            "reg_a" => CampaignType::RegA,
            "reg_d" => CampaignType::RegD,
            _ => return Err(RepositoryError::Unexpected.into()),
        };
        
        // Parse campaign status
        let status = match row.status.as_str() {
            "draft" => CampaignStatus::Draft,
            "active" => CampaignStatus::Active,
            "completed" => CampaignStatus::Completed,
            "failed" => CampaignStatus::Failed,
            "cancelled" => CampaignStatus::Cancelled,
            _ => return Err(RepositoryError::Unexpected.into()),
        };
        
        // Get membership requirements if this is a membership campaign
        let membership_requirements = if matches!(campaign_type, CampaignType::CooperativeMembership) {
            let req_row = sqlx::query!(
                r#"
                SELECT max_participants, required_actions
                FROM membership_requirements
                WHERE campaign_id = $1
                "#,
                id
            )
            .fetch_optional(&self.pool)
            .await
            .map_err(RepositoryError::DatabaseError)?;
            
            req_row.map(|row| MembershipRequirements {
                max_participants: row.max_participants.map(|i| i as u32),
                required_actions: serde_json::from_value(row.required_actions)
                    .unwrap_or_else(|_| vec![]),
            })
        } else {
            None
        };
        
        // Get donation details if this is a donation campaign
        let donation_details = if matches!(campaign_type, CampaignType::PureDonation | CampaignType::RegCF | CampaignType::RegA | CampaignType::RegD) {
            let details_row = sqlx::query!(
                r#"
                SELECT funding_goal, external_use_case, currency
                FROM donation_campaigns
                WHERE campaign_id = $1
                "#,
                id
            )
            .fetch_optional(&self.pool)
            .await
            .map_err(RepositoryError::DatabaseError)?;
            
            details_row.map(|row| DonationDetails {
                funding_goal: row.funding_goal,
                external_use_case: row.external_use_case,
                currency: row.currency,
            })
        } else {
            None
        };
        
        Ok(Some(Campaign {
            id: row.id,
            campaign_type,
            title: row.title,
            description: row.description,
            created_at: row.created_at,
            owner_user_id: row.owner_user_id,
            status,
            membership_requirements,
            donation_details,
        }))
    }
    
    async fn list(
        &self,
        campaign_type: Option<CampaignType>,
        status: Option<CampaignStatus>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<PaginatedCampaigns, ApplicationError> {
        // Build the query dynamically
        let mut query = "SELECT id, type, title, description, created_at, owner_user_id, status FROM campaigns".to_string();
        let mut conditions = vec![];
        let mut params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Postgres> + Send>> = vec![];
        
        if let Some(campaign_type) = &campaign_type {
            conditions.push(format!("type = ${}", params.len() + 1));
            params.push(Box::new(campaign_type.to_string()));
        }
        
        if let Some(status) = &status {
            conditions.push(format!("status = ${}", params.len() + 1));
            params.push(Box::new(status.to_string()));
        }
        
        if !conditions.is_empty() {
            query.push_str(&format!(" WHERE {}", conditions.join(" AND ")));
        }
        
        query.push_str(" ORDER BY created_at DESC");
        
        // Add limit and offset if provided
        if let Some(limit) = limit {
            query.push_str(&format!(" LIMIT ${}", params.len() + 1));
            params.push(Box::new(limit));
        }
        
        if let Some(offset) = offset {
            query.push_str(&format!(" OFFSET ${}", params.len() + 1));
            params.push(Box::new(offset));
        }
        
        // Execute the query
        let mut query_builder = sqlx::query(&query);
        for param in params {
            query_builder = query_builder.bind(param);
        }
        
        let rows = query_builder
            .fetch_all(&self.pool)
            .await
            .map_err(RepositoryError::DatabaseError)?;
        
        let campaigns: Vec<Campaign> = rows
            .into_iter()
            .map(|row| {
                let id: Uuid = row.get("id");
                let campaign_type_str: &str = row.get("type");
                let status_str: &str = row.get("status");
                
                // Parse campaign type
                let campaign_type = match campaign_type_str {
                    "cooperative_membership" => CampaignType::CooperativeMembership,
                    "pure_donation" => CampaignType::PureDonation,
                    "reg_cf" => CampaignType::RegCF,
                    "reg_a" => CampaignType::RegA,
                    "reg_d" => CampaignType::RegD,
                    _ => return Err(RepositoryError::Unexpected),
                };
                
                // Parse campaign status
                let status = match status_str {
                    "draft" => CampaignStatus::Draft,
                    "active" => CampaignStatus::Active,
                    "completed" => CampaignStatus::Completed,
                    "failed" => CampaignStatus::Failed,
                    "cancelled" => CampaignStatus::Cancelled,
                    _ => return Err(RepositoryError::Unexpected),
                };
                
                // Get membership requirements if this is a membership campaign
                let membership_requirements = if matches!(campaign_type, CampaignType::CooperativeMembership) {
                    // In a real implementation, we would fetch these from the database
                    // For now, we'll create a placeholder
                    Some(MembershipRequirements {
                        max_participants: None,
                        required_actions: vec![],
                    })
                } else {
                    None
                };
                
                // Get donation details if this is a donation campaign
                let donation_details = if matches!(campaign_type, CampaignType::PureDonation | CampaignType::RegCF | CampaignType::RegA | CampaignType::RegD) {
                    // In a real implementation, we would fetch these from the database
                    // For now, we'll create a placeholder
                    Some(DonationDetails {
                        funding_goal: None,
                        external_use_case: "Placeholder".to_string(),
                        currency: "USD".to_string(),
                    })
                } else {
                    None
                };
                
                Ok(Campaign {
                    id,
                    campaign_type,
                    title: row.get("title"),
                    description: row.get("description"),
                    created_at: row.get("created_at"),
                    owner_user_id: row.get("owner_user_id"),
                    status,
                    membership_requirements,
                    donation_details,
                })
            })
            .collect::<Result<Vec<Campaign>, RepositoryError>>()?;
        
        // Get total count
        let count_query = "SELECT COUNT(*) as count FROM campaigns".to_string();
        let count: i64 = sqlx::query_scalar(&count_query)
            .fetch_one(&self.pool)
            .await
            .map_err(RepositoryError::DatabaseError)?;
        
        Ok(PaginatedCampaigns {
            campaigns,
            total_count: count,
        })
    }
    
    async fn delete(&self, id: Uuid) -> Result<(), ApplicationError> {
        // In a real implementation, we would need to handle cascading deletes
        // and ensure we don't delete campaigns with active contributions
        sqlx::query!("DELETE FROM campaigns WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(RepositoryError::DatabaseError)?;
        
        Ok(())
    }
    
    async fn exists_for_campaign(&self, campaign_id: Uuid) -> Result<bool, ApplicationError> {
        let exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM campaigns
                WHERE id = $1
            )
            "#,
            campaign_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;
        
        Ok(exists.unwrap_or(false))
    }
    
    async fn soft_delete(&self, id: Uuid) -> Result<(), ApplicationError> {
        sqlx::query!(
            r#"
            UPDATE campaigns
            SET status = 'cancelled'
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;
        
        Ok(())
    }
}

// Helper implementations for converting enums to strings
impl CampaignType {
    pub fn to_string(&self) -> &'static str {
        match self {
            CampaignType::CooperativeMembership => "cooperative_membership",
            CampaignType::PureDonation => "pure_donation",
            CampaignType::RegCF => "reg_cf",
            CampaignType::RegA => "reg_a",
            CampaignType::RegD => "reg_d",
        }
    }
}

impl CampaignStatus {
    pub fn to_string(&self) -> &'static str {
        match self {
            CampaignStatus::Draft => "draft",
            CampaignStatus::Active => "active",
            CampaignStatus::Completed => "completed",
            CampaignStatus::Failed => "failed",
            CampaignStatus::Cancelled => "cancelled",
        }
    }
}