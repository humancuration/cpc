//! Domain models for the expense tracker module
//! 
//! This module contains the core business logic and models for expense tracking.

use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::domain::primitives::{Money, Currency};

/// Primary expense categories
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum ExpenseCategory {
    Food,
    Transportation,
    Housing,
    Utilities,
    Entertainment,
    Healthcare,
    Education,
    PersonalCare,
    Shopping,
    Travel,
    Business,
    Other(String), // Custom category with user-provided name
}

/// Expense status (for tracking processing state)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum ExpenseStatus {
    Draft,
    Processed,
    Verified,
    Rejected,
    Archived,
}

/// Represents a single expense transaction
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Expense {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: Money, // Supports dual-currency
    pub category: ExpenseCategory,
    pub date: DateTime<Utc>,
    pub description: String,
    pub status: ExpenseStatus,
    pub receipt_id: Option<Uuid>,
    pub is_recurring: bool,
    pub recurrence_pattern: Option<String>, // CRON-like pattern for recurring expenses
    pub linked_budget_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Expense {
    /// Create a new expense with minimum required fields
    pub fn new(
        user_id: Uuid,
        amount: Money,
        category: ExpenseCategory,
        date: DateTime<Utc>,
        description: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            amount,
            category,
            date,
            description,
            status: ExpenseStatus::Processed,
            receipt_id: None,
            is_recurring: false,
            recurrence_pattern: None,
            linked_budget_id: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Create a new draft expense (for receipt scanning flow)
    pub fn new_draft(
        user_id: Uuid,
        receipt_id: Uuid,
        description: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            amount: Money::zero(Currency::USD), // Will be populated from receipt
            category: ExpenseCategory::Other("Unprocessed".to_string()),
            date: now,
            description,
            status: ExpenseStatus::Draft,
            receipt_id: Some(receipt_id),
            is_recurring: false,
            recurrence_pattern: None,
            linked_budget_id: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Update expense amount (handles validation)
    pub fn update_amount(&mut self, amount: Money) -> Result<(), crate::domain::FinanceError> {
        self.amount = amount;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Link to a budget category
    pub fn link_to_budget(&mut self, budget_id: Uuid) {
        self.linked_budget_id = Some(budget_id);
        self.updated_at = Utc::now();
    }

    /// Mark as recurring with specified pattern
    pub fn mark_as_recurring(&mut self, pattern: String) {
        self.is_recurring = true;
        self.recurrence_pattern = Some(pattern);
        self.updated_at = Utc::now();
    }
}

/// Receipt data model for scanned receipts
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Receipt {
    pub id: Uuid,
    pub user_id: Uuid,
    pub image_data: ReceiptImageData,
    pub extracted_text: String,
    pub merchant_name: Option<String>,
    pub transaction_date: Option<DateTime<Utc>>,
    pub total_amount: Option<Money>,
    pub processing_status: ReceiptProcessingStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ReceiptProcessingStatus {
    Uploaded,
    Processing,
    Processed,
    Failed(String), // Error message
    Verified,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ReceiptImageData {
    LocalPath(String),  // For mobile/desktop apps with access to local storage
    Base64Data(String), // For web applications
    ReferenceId(Uuid),  // For cloud storage references
}

/// Time limits for sharing preferences
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct SharingTimeLimits {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

/// Recipient-specific sharing rules
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct RecipientSharingRule {
    pub recipient_id: Uuid,
    pub allowed_categories: Vec<ExpenseCategory>,
    pub time_limits: Option<SharingTimeLimits>,
}

/// Domain model for expense sharing preferences
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct ExpenseSharingPreferences {
    pub id: Uuid,
    pub user_id: Uuid,
    pub sharing_enabled: bool,
    pub anonymized: bool,
    pub shared_categories: Vec<ExpenseCategory>,
    pub time_limits: Option<SharingTimeLimits>,
    pub recipient_specific_rules: Vec<RecipientSharingRule>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ExpenseSharingPreferences {
    pub fn new(user_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            sharing_enabled: false,
            anonymized: false,
            shared_categories: vec![],
            time_limits: None,
            recipient_specific_rules: vec![],
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn enable_sharing(&mut self) {
        self.sharing_enabled = true;
        self.updated_at = Utc::now();
    }
    
    pub fn disable_sharing(&mut self) {
        self.sharing_enabled = false;
        self.updated_at = Utc::now();
    }
    
    pub fn toggle_anonymization(&mut self) {
        self.anonymized = !self.anonymized;
        self.updated_at = Utc::now();
    }
    
    pub fn set_shared_categories(&mut self, categories: Vec<ExpenseCategory>) {
        self.shared_categories = categories;
        self.updated_at = Utc::now();
    }
}