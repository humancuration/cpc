//! Integration points for the advanced CRM module
//!
//! This module contains shared data structures for integration with other modules.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDate};
use uuid::Uuid;

/// Sales performance data shared with HR module
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SalesPerformanceData {
    pub user_id: Uuid,
    pub period: Period,
    pub deals_closed: u32,
    pub revenue_generated: i64, // in cents
    pub average_deal_size: i64,
    pub conversion_rate: f32,
    pub sales_velocity: f32,
    pub pipeline_health: PipelineHealth,
}

/// Wellness data from Health module for lead scoring
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WellnessData {
    pub user_id: Uuid,
    pub date: NaiveDate,
    pub wellness_score: u8,     // 0-100
    pub stress_level: u8,      // 0-100
    pub activity_level: u8,   // 0-100
}

/// Time period for performance data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Period {
    Daily(NaiveDate),
    Weekly(NaiveDate),  // Start of week
    Monthly(NaiveDate), // Start of month
    Quarterly(NaiveDate), // Start of quarter
}

/// Health of a sales pipeline
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PipelineHealth {
    Strong,
    Moderate,
    Weak,
}