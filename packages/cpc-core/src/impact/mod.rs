//! Impact reporting module for business intelligence
//!
//! This module provides data structures and services for generating
//! comprehensive impact reports including environmental, social,
//! and economic metrics.

use std::collections::HashMap;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use async_graphql::SimpleObject;

pub mod service;
pub mod error;

pub use service::*;
pub use error::*;

/// Carbon footprint measured in metric tonnes of CO2 equivalent
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct MetricTonnes(pub f32);

impl MetricTonnes {
    /// Create a new MetricTonnes value
    pub fn new(value: f32) -> Self {
        Self(value)
    }
    
    /// Get the underlying float value
    pub fn value(&self) -> f32 {
        self.0
    }
}

/// Diversity statistics for impact reporting
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct DiversityStats {
    /// Gender balance ratio (0.5 = perfect balance)
    pub gender_balance: f32,
    /// Ethnicity breakdown by percentage
    pub ethnicity_breakdown: HashMap<String, f32>,
    /// Pay equity ratio (1.0 = perfect equity)
    pub pay_equity: f32,
}

/// Comprehensive impact report for an organization
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct ImpactReport {
    /// Organization UUID
    pub org_id: Uuid,
    /// Reporting year
    pub year: i32,
    /// Total carbon sequestered in metric tonnes
    pub total_carbon_sequestered: MetricTonnes,
    /// Total emissions offset in metric tonnes
    pub total_emissions_offset: MetricTonnes,
    /// Percentage of renewable energy usage
    pub renewable_energy_percentage: f64,
    /// Diversity and inclusion metrics
    pub diversity_breakdown: DiversityStats,
    /// Total number of suppliers
    pub supplier_count: u32,
    /// Total revenue for the year
    pub total_revenue: crate::accounting::money::Money,
    /// Value of carbon credits earned
    pub carbon_credit_value: crate::accounting::money::Money,
    /// Total community investment amount
    pub community_investment: crate::accounting::money::Money,
    /// Report creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl ImpactReport {
    /// Create a new impact report with all required fields
    pub fn new(
        org_id: Uuid,
        year: i32,
        total_carbon_sequestered: MetricTonnes,
        total_emissions_offset: MetricTonnes,
        renewable_energy_percentage: f64,
        diversity_breakdown: DiversityStats,
        supplier_count: u32,
        total_revenue: crate::accounting::money::Money,
        carbon_credit_value: crate::accounting::money::Money,
        community_investment: crate::accounting::money::Money,
    ) -> Result<Self, ImpactError> {
        let report = Self {
            org_id,
            year,
            total_carbon_sequestered,
            total_emissions_offset,
            renewable_energy_percentage,
            diversity_breakdown,
            supplier_count,
            total_revenue,
            carbon_credit_value,
            community_investment,
            created_at: chrono::Utc::now(),
        };
        
        report.validate()?;
        Ok(report)
    }

    /// Validate the impact report data
    pub fn validate(&self) -> Result<(), ImpactError> {
        // Validate year
        if self.year < 2000 || self.year > 2100 {
            return Err(ImpactError::Validation(format!(
                "Invalid year {}. Year must be between 2000 and 2100",
                self.year
            )));
        }

        // Validate percentages
        if self.renewable_energy_percentage < 0.0 || self.renewable_energy_percentage > 100.0 {
            return Err(ImpactError::Validation(
                "Renewable energy percentage must be between 0 and 100".to_string()
            ));
        }

        // Validate diversity breakdown
        self.diversity_breakdown.validate()?;

        // Validate positive values
        if self.supplier_count == 0 {
            return Err(ImpactError::Validation(
                "Supplier count must be greater than 0".to_string()
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests;

impl DiversityStats {
    /// Validate diversity statistics
    pub fn validate(&self) -> Result<(), ImpactError> {
        // Validate gender balance
        if self.gender_balance < 0.0 || self.gender_balance > 1.0 {
            return Err(ImpactError::Validation(
                "Gender balance must be between 0.0 and 1.0".to_string()
            ));
        }

        // Validate pay equity
        if self.pay_equity < 0.0 || self.pay_equity > 2.0 {
            return Err(ImpactError::Validation(
                "Pay equity must be between 0.0 and 2.0".to_string()
            ));
        }

        // Validate ethnicity breakdown percentages
        let total_percentage: f32 = self.ethnicity_breakdown.values().sum();
        let tolerance = 0.1; // Allow small rounding errors
        
        if (total_percentage - 100.0).abs() > tolerance {
            return Err(ImpactError::Validation(format!(
                "Ethnicity breakdown percentages must sum to 100%, got {:.1}%",
                total_percentage
            )));
        }

        Ok(())
    }
}