//! Financial Impact Integration
//!
//! Integration points with other systems and modules in the CPC ecosystem.

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use rust_decimal::Decimal;
use cpay_core::{Transaction, WalletId, Currency, CPayCore};
use cpc_financial_core::{FinancialEvent, FinancialCategory, CPCFinancialCore};
use crate::{FinancialImpactTracker, FinancialAnalytics, FinancialReportGenerator, FinancialImpactError};

/// Integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialIntegrationConfig {
    pub enable_realtime_tracking: bool,
    pub enable_cause_linking: bool,
    pub enable_volunteer_linking: bool,
    pub enable_learning_linking: bool,
    pub auto_generate_reports: bool,
    pub report_frequency_days: i32,
}

/// Financial impact integration manager
pub struct FinancialIntegration {
    tracker: FinancialImpactTracker,
    analytics: FinancialAnalytics,
    report_generator: FinancialReportGenerator,
    cpay_core: CPayCore,
    financial_core: CPCFinancialCore,
    config: FinancialIntegrationConfig,
}

impl FinancialIntegration {
    /// Create a new financial integration manager
    pub fn new(
        tracker: FinancialImpactTracker,
        cpay_core: CPayCore,
        financial_core: CPCFinancialCore,
        config: FinancialIntegrationConfig,
    ) -> Self {
        let analytics = FinancialAnalytics::new(tracker.clone());
        let report_generator = FinancialReportGenerator::new(analytics.clone());
        
        Self {
            tracker,
            analytics,
            report_generator,
            cpay_core,
            financial_core,
            config,
        }
    }

    /// Process a financial transaction and track its impact
    pub async fn process_transaction(
        &self,
        transaction: &Transaction,
        category: FinancialCategory,
        impact_metadata: serde_json::Value,
    ) -> Result<(), FinancialImpactError> {
        // Calculate impact score based on various factors
        let impact_score = self.calculate_impact_score(transaction, &category, &impact_metadata).await?;
        
        // Record the financial impact
        self.tracker
            .record_transaction_impact(transaction, category, impact_score, impact_metadata)
            .await?;

        // If auto-reporting is enabled, check if we should generate a report
        if self.config.auto_generate_reports {
            self.check_and_generate_report().await?;
        }

        Ok(())
    }

    /// Calculate impact score for a transaction
    async fn calculate_impact_score(
        &self,
        transaction: &Transaction,
        category: &FinancialCategory,
        metadata: &serde_json::Value,
    ) -> Result<Decimal, FinancialImpactError> {
        // Base score based on transaction amount and category
        let mut score = match category {
            FinancialCategory::Donations => Decimal::from(100),
            FinancialCategory::Grants => Decimal::from(90),
            FinancialCategory::Revenue => Decimal::from(80),
            FinancialCategory::CommunityInvestment => Decimal::from(95),
            FinancialCategory::Education => Decimal::from(85),
            FinancialCategory::Infrastructure => Decimal::from(75),
            FinancialCategory::CommunityDevelopment => Decimal::from(90),
            FinancialCategory::Expenses => Decimal::from(20),
            _ => Decimal::from(50),
        };

        // Adjust score based on amount (larger positive impact, but diminishing returns)
        let amount_factor = (transaction.amount / Decimal::from(1000)).min(Decimal::from(2));
        score = score + (amount_factor * Decimal::from(10));

        // Adjust score based on community involvement metadata
        if let Some(contributor_type) = metadata.get("contributor_type") {
            match contributor_type.as_str() {
                Some("individual") => score = score + Decimal::from(5),
                Some("organization") => score = score + Decimal::from(10),
                Some("community_group") => score = score + Decimal::from(15),
                _ => {}
            }
        }

        // Adjust score based on cause alignment (if enabled)
        if self.config.enable_cause_linking {
            if let Some(cause_id) = metadata.get("cause_id").and_then(|v| v.as_str()) {
                let cause_alignment = self.calculate_cause_alignment(cause_id).await?;
                score = score + (cause_alignment * Decimal::from(20));
            }
        }

        // Adjust score based on volunteer involvement (if enabled)
        if self.config.enable_volunteer_linking {
            if let Some(volunteer_hours) = metadata.get("volunteer_hours").and_then(|v| v.as_f64()) {
                let volunteer_factor = Decimal::from_f64(volunteer_hours / 10.0).unwrap_or(Decimal::ZERO);
                score = score + (volunteer_factor * Decimal::from(5)).min(Decimal::from(25));
            }
        }

        // Normalize score to 0-100 range
        score = score.clamp(Decimal::ZERO, Decimal::from(100));
        
        // Convert to 0-1 scale
        Ok(score / Decimal::from(100))
    }

    /// Calculate cause alignment score
    async fn calculate_cause_alignment(&self, cause_id: &str) -> Result<Decimal, FinancialImpactError> {
        // In a real implementation, this would check how well the financial activity
        // aligns with community causes and priorities
        // For now, we'll return a moderate alignment score
        Ok(Decimal::from(75) / Decimal::from(100))
    }

    /// Link financial activity to a cause
    pub async fn link_to_cause(
        &self,
        transaction_id: Uuid,
        cause_id: Uuid,
        alignment_score: Decimal,
    ) -> Result<(), FinancialImpactError> {
        sqlx::query(
            r#"
            INSERT INTO financial_cause_links (transaction_id, cause_id, alignment_score)
            VALUES ($1, $2, $3)
            ON CONFLICT (transaction_id, cause_id) 
            DO UPDATE SET alignment_score = $3
            "#
        )
        .bind(transaction_id)
        .bind(cause_id)
        .bind(alignment_score)
        .execute(&self.tracker.db_pool)
        .await?;

        Ok(())
    }

    /// Link financial activity to volunteer work
    pub async fn link_to_volunteer_work(
        &self,
        transaction_id: Uuid,
        volunteer_id: Uuid,
        hours_contributed: Decimal,
    ) -> Result<(), FinancialImpactError> {
        sqlx::query(
            r#"
            INSERT INTO financial_volunteer_links (transaction_id, volunteer_id, hours_contributed)
            VALUES ($1, $2, $3)
            ON CONFLICT (transaction_id, volunteer_id) 
            DO UPDATE SET hours_contributed = $3
            "#
        )
        .bind(transaction_id)
        .bind(volunteer_id)
        .bind(hours_contributed)
        .execute(&self.tracker.db_pool)
        .await?;

        Ok(())
    }

    /// Link financial activity to learning outcomes
    pub async fn link_to_learning_outcomes(
        &self,
        transaction_id: Uuid,
        learning_program_id: Uuid,
        impact_score: Decimal,
    ) -> Result<(), FinancialImpactError> {
        sqlx::query(
            r#"
            INSERT INTO financial_learning_links (transaction_id, learning_program_id, impact_score)
            VALUES ($1, $2, $3)
            ON CONFLICT (transaction_id, learning_program_id) 
            DO UPDATE SET impact_score = $3
            "#
        )
        .bind(transaction_id)
        .bind(learning_program_id)
        .bind(impact_score)
        .execute(&self.tracker.db_pool)
        .await?;

        Ok(())
    }

    /// Check if it's time to generate a report and do so if needed
    async fn check_and_generate_report(&self) -> Result<(), FinancialImpactError> {
        // In a real implementation, this would check the last report generation time
        // and generate a new report if the configured frequency has passed
        // For now, we'll just return Ok without generating a report
        Ok(())
    }

    /// Generate a comprehensive financial impact report
    pub async fn generate_financial_impact_report(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Vec<u8>, FinancialImpactError> {
        let report = self.report_generator.generate_report(start_time, end_time).await?;
        self.report_generator.export_report(&report, crate::reporting::ExportFormat::Json).await
    }

    /// Get financial analytics for a specific cause
    pub async fn get_cause_financial_analytics(
        &self,
        cause_id: Uuid,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<crate::analytics::FinancialImpactAnalytics, FinancialImpactError> {
        // Get transactions linked to this cause
        let transaction_ids: Vec<Uuid> = sqlx::query_scalar(
            r#"
            SELECT transaction_id 
            FROM financial_cause_links 
            WHERE cause_id = $1
            "#
        )
        .bind(cause_id)
        .fetch_all(&self.tracker.db_pool)
        .await?;

        if transaction_ids.is_empty() {
            // Return empty analytics if no transactions are linked to this cause
            return Ok(crate::analytics::FinancialImpactAnalytics {
                total_impact: Decimal::ZERO,
                category_breakdown: vec![],
                time_series: vec![],
                top_contributors: vec![],
                roi_metrics: vec![],
                sustainability_metrics: crate::analytics::SustainabilityMetrics {
                    monthly_recurring_revenue: Decimal::ZERO,
                    donation_stability_index: Decimal::ZERO,
                    community_investment_ratio: Decimal::ZERO,
                    financial_health_score: Decimal::ZERO,
                },
            });
        }

        // For now, we'll generate analytics for the specified time period
        // In a real implementation, we would filter by the transaction IDs as well
        self.analytics.generate_impact_analytics(start_time, end_time).await
    }

    /// Get financial impact for volunteer activities
    pub async fn get_volunteer_financial_impact(
        &self,
        volunteer_id: Uuid,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Decimal, FinancialImpactError> {
        let impact: Option<Decimal> = sqlx::query_scalar(
            r#"
            SELECT SUM(fir.amount * fir.impact_score)
            FROM financial_impact_records fir
            JOIN financial_volunteer_links fvl ON fir.transaction_id = fvl.transaction_id
            WHERE fvl.volunteer_id = $1 
                AND fir.timestamp >= $2 
                AND fir.timestamp <= $3
            "#
        )
        .bind(volunteer_id)
        .bind(start_time)
        .bind(end_time)
        .fetch_one(&self.tracker.db_pool)
        .await?;

        Ok(impact.unwrap_or(Decimal::ZERO))
    }

    /// Get financial impact for learning programs
    pub async fn get_learning_financial_impact(
        &self,
        learning_program_id: Uuid,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Decimal, FinancialImpactError> {
        let impact: Option<Decimal> = sqlx::query_scalar(
            r#"
            SELECT SUM(fir.amount * fir.impact_score)
            FROM financial_impact_records fir
            JOIN financial_learning_links fll ON fir.transaction_id = fll.transaction_id
            WHERE fll.learning_program_id = $1 
                AND fir.timestamp >= $2 
                AND fir.timestamp <= $3
            "#
        )
        .bind(learning_program_id)
        .bind(start_time)
        .bind(end_time)
        .fetch_one(&self.tracker.db_pool)
        .await?;

        Ok(impact.unwrap_or(Decimal::ZERO))
    }

    /// Synchronize with cpay_core to track all financial transactions
    pub async fn synchronize_with_cpay(&self) -> Result<u64, FinancialImpactError> {
        // Get recent transactions from cpay_core that haven't been tracked yet
        let recent_transactions = self.cpay_core.get_recent_transactions(Utc::now() - chrono::Duration::days(1)).await?;
        
        let mut tracked_count = 0;
        
        for transaction in recent_transactions {
            // Check if this transaction has already been tracked
            let already_tracked: Option<i64> = sqlx::query_scalar(
                "SELECT COUNT(*) FROM financial_impact_records WHERE transaction_id = $1"
            )
            .bind(transaction.id)
            .fetch_one(&self.tracker.db_pool)
            .await?;
            
            if already_tracked.unwrap_or(0) == 0 {
                // Track this transaction with a default category
                // In a real implementation, we would determine the appropriate category
                let category = FinancialCategory::Revenue;
                let metadata = serde_json::json!({
                    "source": "cpay_sync",
                    "sync_timestamp": Utc::now().to_rfc3339()
                });
                
                self.tracker
                    .record_transaction_impact(&transaction, category, Decimal::from(50) / Decimal::from(100), metadata)
                    .await?;
                
                tracked_count += 1;
            }
        }
        
        Ok(tracked_count)
    }
}