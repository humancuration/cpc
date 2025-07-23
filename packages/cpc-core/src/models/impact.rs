use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use async_graphql::SimpleObject;
use async_graphql::Enum;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Enum, Copy)]
pub enum ImpactCategory {
    Environmental,
    Social,
    Economic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactMetric {
    pub id: Uuid,
    pub name: String,
    pub category: ImpactCategory,
    pub unit: String,
    pub calculation_formula: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct ImpactReport {
    pub user_id: Uuid,
    pub generated_at: DateTime<Utc>,
    pub overall_score: f64,
    pub ethical_distribution: HashMap<ImpactCategory, f64>,
    pub timeline: Vec<ImpactTimelinePoint>,
    pub breakdown: Vec<ImpactBreakdownItem>,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct ImpactTimelinePoint {
    pub timestamp: DateTime<Utc>,
    pub value: f64,
    pub category: ImpactCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct ImpactBreakdownItem {
    pub item_id: Uuid,
    pub name: String,
    pub category: ImpactCategory,
    pub value: f64,
    pub ethical_score: f64,
}