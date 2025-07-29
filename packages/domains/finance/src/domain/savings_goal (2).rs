use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::domain::primitives::Money;

/// Visual style for savings goal visualization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GoalVisualStyle {
    pub color: String,
    pub icon: String,
}

/// Savings goal entity representing a financial target
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SavingsGoal {
    pub id: Uuid,
    pub name: String,
    pub target_amount: Money,
    pub current_amount: Money,
    pub target_date: DateTime<Utc>,
    pub progress: f64, // 0.0 to 1.0
    pub visual_style: GoalVisualStyle, // For BI visualization
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SavingsGoal {
    pub fn new(
        name: String,
        target_amount: Money,
        target_date: DateTime<Utc>,
        visual_style: GoalVisualStyle,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            target_amount: target_amount.clone(),
            current_amount: Money::new(0.0, &target_amount.currency),
            target_date,
            progress: 0.0,
            visual_style,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn update_current_amount(&mut self, amount: Money) {
        self.current_amount = amount;
        self.progress = (self.current_amount.amount / self.target_amount.amount).min(1.0);
        self.updated_at = Utc::now();
    }
    
    pub fn add_to_current_amount(&mut self, amount: Money) {
        if self.current_amount.currency == amount.currency {
            self.current_amount.amount += amount.amount;
            self.progress = (self.current_amount.amount / self.target_amount.amount).min(1.0);
            self.updated_at = Utc::now();
        }
    }
    
    pub fn update_target_amount(&mut self, new_target: Money) {
        self.target_amount = new_target;
        self.progress = (self.current_amount.amount / self.target_amount.amount).min(1.0);
        self.updated_at = Utc::now();
    }
    
    pub fn update_target_date(&mut self, new_date: DateTime<Utc>) {
        self.target_date = new_date;
        self.updated_at = Utc::now();
    }
    
    pub fn update_visual_style(&mut self, style: GoalVisualStyle) {
        self.visual_style = style;
        self.updated_at = Utc::now();
    }
    
    /// Check if the goal is completed
    pub fn is_completed(&self) -> bool {
        self.progress >= 1.0
    }
    
    /// Check if the goal is overdue
    pub fn is_overdue(&self) -> bool {
        let now = Utc::now();
        self.target_date < now && self.progress < 1.0
    }
}