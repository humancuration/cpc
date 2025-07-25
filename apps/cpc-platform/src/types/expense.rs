use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Expense {
    pub id: Option<String>,
    pub project_id: Option<String>,
    pub category: String,
    pub description: String,
    pub amount: f64,
    pub currency: String,
    pub transaction_date: DateTime<Utc>,
}

impl Expense {
    pub fn is_valid(&self) -> bool {
        !self.category.is_empty() 
            && !self.description.is_empty() 
            && self.amount > 0.0 
            && !self.currency.is_empty()
    }
}