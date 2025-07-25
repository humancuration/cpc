use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use async_graphql::InputObject;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expense {
    pub id: Uuid,
    pub user_id: Uuid,
    pub project_id: Option<Uuid>,
    pub category: String,
    pub description: String,
    pub amount: f64,
    pub currency: String, // e.g., "USD", "EUR"
    pub transaction_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
    pub sync_version: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseFilter {
    pub user_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub category: Option<String>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub currency: Option<String>,
    pub search_term: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExpenseCategory {
    OfficeSupplies,
    Travel,
    Meals,
    Equipment,
    Software,
    Marketing,
    Utilities,
    Rent,
    ProfessionalServices,
    Training,
    Other,
}

#[derive(InputObject)]
pub struct ExpenseInput {
    pub project_id: Option<uuid::Uuid>,
    pub category: String,
    pub description: String,
    pub amount: f64,
    pub currency: String,
    pub transaction_date: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expense {
    pub id: Uuid,
    pub user_id: Uuid,
    pub project_id: Option<Uuid>,
    pub category: String,
    pub description: String,
    pub amount: f64,
    pub currency: String, // e.g., "USD", "EUR"
    pub transaction_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
    pub sync_version: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseFilter {
    pub user_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub category: Option<String>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub currency: Option<String>,
    pub search_term: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExpenseCategory {
    OfficeSupplies,
    Travel,
    Meals,
    Equipment,
    Software,
    Marketing,
    Utilities,
    Rent,
    ProfessionalServices,
    Training,
    Other,
}

impl Expense {
    pub fn new(user_id: Uuid, category: String, description: String, amount: f64, currency: String) -> Self {
        let now = Utc::now();
        
        Self {
            id: Uuid::new_v4(),
            user_id,
            project_id: None,
            category,
            description,
            amount,
            currency,
            transaction_date: now,
            created_at: now,
            updated_at: now,
            metadata: HashMap::new(),
            sync_version: 0,
        }
    }

    pub fn with_project_id(mut self, project_id: Uuid) -> Self {
        self.project_id = Some(project_id);
        self
    }

    pub fn with_transaction_date(mut self, date: DateTime<Utc>) -> Self {
        self.transaction_date = date;
        self
    }

    pub fn add_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    pub fn update_amount(&mut self, amount: f64) {
        self.amount = amount;
        self.updated_at = Utc::now();
        self.sync_version += 1;
    }

    pub fn update_description(&mut self, description: String) {
        self.description = description;
        self.updated_at = Utc::now();
        self.sync_version += 1;
    }

    pub fn update_category(&mut self, category: String) {
        self.category = category;
        self.updated_at = Utc::now();
        self.sync_version += 1;
    }

    pub fn update_transaction_date(&mut self, date: DateTime<Utc>) {
        self.transaction_date = date;
        self.updated_at = Utc::now();
        self.sync_version += 1;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseSummary {
    pub total_expenses: usize,
    pub total_amount: f64,
    pub currency: String,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub by_category: HashMap<String, f64>,
}