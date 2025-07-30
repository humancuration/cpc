use serde::{Deserialize, Serialize};
use std::fmt;

/// Financial categories for budgeting and expense tracking
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FinancialCategory {
    Housing,
    Transportation,
    Food,
    Utilities,
    Healthcare,
    Entertainment,
    Education,
    Personal,
    Savings,
    Investments,
    DebtPayments,
}

impl fmt::Display for FinancialCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FinancialCategory::Housing => write!(f, "Housing"),
            FinancialCategory::Transportation => write!(f, "Transportation"),
            FinancialCategory::Food => write!(f, "Food"),
            FinancialCategory::Utilities => write!(f, "Utilities"),
            FinancialCategory::Healthcare => write!(f, "Healthcare"),
            FinancialCategory::Entertainment => write!(f, "Entertainment"),
            FinancialCategory::Education => write!(f, "Education"),
            FinancialCategory::Personal => write!(f, "Personal"),
            FinancialCategory::Savings => write!(f, "Savings"),
            FinancialCategory::Investments => write!(f, "Investments"),
            FinancialCategory::DebtPayments => write!(f, "Debt Payments"),
        }
    }
}

/// Money representation with amount and currency
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Money {
    pub amount: f64,
    pub currency: String, // ISO 4217 code
}

impl Money {
    pub fn new(amount: f64, currency: &str) -> Self {
        Self {
            amount,
            currency: currency.to_string(),
        }
    }
}

/// Time period for budgeting and financial planning
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimePeriod {
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: chrono::DateTime<chrono::Utc>,
}

impl TimePeriod {
    pub fn new(start: chrono::DateTime<chrono::Utc>, end: chrono::DateTime<chrono::Utc>) -> Self {
        Self { start, end }
    }
    
    pub fn contains(&self, date: &chrono::DateTime<chrono::Utc>) -> bool {
        &self.start <= date && date <= &self.end
    }
}