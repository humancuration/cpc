//! Impact service for managing organization and user impact data
//!
//! Provides business logic for calculating and managing impact reports
//! across different levels (user, organization, community).

use anyhow::{Result, Context};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use sqlx::{query, query_as, postgres::PgRow, Row};
use tracing::info;

use crate::db::DbPool;
use cpc_core::impact::{ImpactCalculator, OrganizationImpactReport as CoreOrgReport, DiversityMetrics as CoreDiversityMetrics};

/// Service for managing impact-related operations
#[derive(Debug)]
pub struct ImpactService {
    db: DbPool,
    calculator: Arc<ImpactCalculator>,
}

impl ImpactService {
    /// Create a new ImpactService instance
    pub fn new(db: DbPool) -> Self {
        let calculator = Arc::new(ImpactCalculator::new());
        Self { db, calculator }
    }

    /// Get impact report for a specific organization
    pub async fn get_organization_impact_report(
        &self,
        org_id: Uuid,
        year: i32
    ) -> Result<Option<OrganizationImpactReport>> {
        // Try to fetch existing report from database
        if let Some(report) = self.fetch_report_from_db(org_id, year).await? {
            return Ok(Some(report));
        }

        // If not found, generate new report
        self.generate_organization_impact_report(org_id, year).await?;
        self.fetch_report_from_db(org_id, year)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Failed to generate impact report"))
            .map(Some)
    }

    /// Generate impact report for an organization
    pub async fn generate_organization_impact_report(&self, org_id: Uuid, year: i32) -> Result<Uuid> {
        let report_id = Uuid::new_v4();
        info!("Generating impact report for organization {} (year {})", org_id, year);

        // Calculate metrics using core calculator
        let carbon_footprint = self.calculator.calculate_carbon_footprint(org_id, year).await?;
        let community_investment = self.calculator.calculate_community_investment(org_id, year).await?;
        let diversity_metrics = self.calculator.calculate_diversity_metrics(org_id).await?;
        let supply_chain_score = self.calculator.calculate_supply_chain_score(org_id, year).await?;

        // Store report in database
        self.store_report_in_db(
            report_id,
            org_id,
            year,
            carbon_footprint,
            community_investment,
            diversity_metrics.gender_diversity,
            diversity_metrics.ethnic_diversity,
            supply_chain_score
        ).await?;

        Ok(report_id)
    }

    async fn fetch_report_from_db(&self, org_id: Uuid, year: i32) -> Result<Option<OrganizationImpactReport>> {
        let report = query_as!(
            OrganizationImpactReport,
            r#"
            SELECT
                id,
                organization_id,
                year,
                created_at as generated_at,
                carbon_footprint,
                community_investment,
                gender_diversity,
                ethnic_diversity,
                supply_chain_score
            FROM organization_impact_reports
            WHERE organization_id = $1 AND year = $2
            "#,
            org_id,
            year
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(report)
    }

    async fn store_report_in_db(
        &self,
        report_id: Uuid,
        org_id: Uuid,
        year: i32,
        carbon_footprint: f64,
        community_investment: f64,
        gender_diversity: f64,
        ethnic_diversity: f64,
        supply_chain_score: f64
    ) -> Result<()> {
        query!(
            r#"
            INSERT INTO organization_impact_reports (
                id,
                organization_id,
                year,
                carbon_footprint,
                community_investment,
                gender_diversity,
                ethnic_diversity,
                supply_chain_score
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            report_id,
            org_id,
            year,
            carbon_footprint,
            community_investment,
            gender_diversity,
            ethnic_diversity,
            supply_chain_score
        )
        .execute(&self.db)
        .await
        .context("Failed to store impact report in database")?;

        Ok(())
    }
}

/// Organization-level impact report (DB representation)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OrganizationImpactReport {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub year: i32,
    pub generated_at: DateTime<Utc>,
    pub carbon_footprint: f64,
    pub community_investment: f64,
    pub gender_diversity: f64,
    pub ethnic_diversity: f64,
    pub supply_chain_score: f64,
}