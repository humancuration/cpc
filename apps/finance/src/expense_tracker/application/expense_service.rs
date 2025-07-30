//! Expense service for managing expenses, receipts, and sharing preferences

use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::domain::{
    expense_tracker::{Expense, ExpenseCategory, Receipt, ReceiptImageData, ReceiptProcessingStatus, ExpenseSharingPreferences},
    primitives::Money,
    FinanceError,
};
use crate::application::{
    wallet_service::WalletService,
    budget_service::BudgetService,
};

/// Repository trait for expense persistence
#[async_trait]
pub trait ExpenseRepository {
    async fn save(&self, expense: &Expense) -> Result<(), FinanceError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Expense>, FinanceError>;
    async fn find_by_user_id(
        &self,
        user_id: Uuid,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
    ) -> Result<Vec<Expense>, FinanceError>;
    async fn delete(&self, id: Uuid) -> Result<(), FinanceError>;
    async fn update(&self, expense: &Expense) -> Result<(), FinanceError>;
}

/// Repository trait for receipt persistence
#[async_trait]
pub trait ReceiptRepository {
    async fn save(&self, receipt: &Receipt) -> Result<(), FinanceError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Receipt>, FinanceError>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Receipt>, FinanceError>;
    async fn delete(&self, id: Uuid) -> Result<(), FinanceError>;
    async fn update(&self, receipt: &Receipt) -> Result<(), FinanceError>;
}

/// Repository trait for expense sharing preferences
#[async_trait]
pub trait ExpenseSharingPreferenceRepository {
    async fn save(&self, preference: &ExpenseSharingPreferences) -> Result<(), FinanceError>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<ExpenseSharingPreferences>, FinanceError>;
    async fn create_default(&self, user_id: Uuid) -> Result<ExpenseSharingPreferences, FinanceError>;
}

/// Service trait for expense operations
#[async_trait]
pub trait ExpenseService {
    /// Create a new expense
    async fn create_expense(
        &self,
        user_id: Uuid,
        amount: Money,
        category: ExpenseCategory,
        date: DateTime<Utc>,
        description: String,
    ) -> Result<Expense, FinanceError>;

    /// Create a draft expense from a receipt (for processing flow)
    async fn create_draft_from_receipt(
        &self,
        user_id: Uuid,
        receipt_id: Uuid,
        description: String,
    ) -> Result<Expense, FinanceError>;

    /// Get all expenses for a user within a date range
    async fn get_user_expenses(
        &self,
        user_id: Uuid,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
    ) -> Result<Vec<Expense>, FinanceError>;

    /// Update an existing expense
    async fn update_expense(
        &self,
        expense_id: Uuid,
        amount: Option<Money>,
        category: Option<ExpenseCategory>,
        date: Option<DateTime<Utc>>,
        description: Option<String>,
        status: Option<ExpenseStatus>,
        linked_budget_id: Option<Uuid>,
    ) -> Result<Expense, FinanceError>;

    /// Delete an expense
    async fn delete_expense(&self, expense_id: Uuid) -> Result<(), FinanceError>;

    /// Link an expense to a budget category
    async fn link_to_budget(&self, expense_id: Uuid, budget_id: Uuid) -> Result<(), FinanceError>;

    /// Process a receipt to extract expense data
    async fn process_receipt(&self, receipt_id: Uuid) -> Result<Expense, FinanceError>;

    /// Get receipt details
    async fn get_receipt(&self, receipt_id: Uuid) -> Result<Option<Receipt>, FinanceError>;

    /// Save a new receipt
    async fn save_receipt(
        &self,
        user_id: Uuid,
        image_data: ReceiptImageData,
    ) -> Result<Receipt, FinanceError>;

    /// Update receipt processing status
    async fn update_receipt_status(&self, receipt_id: Uuid, status: ReceiptProcessingStatus) -> Result<(), FinanceError>;

    /// Get expense sharing preferences for a user
    async fn get_sharing_preferences(&self, user_id: Uuid) -> Result<ExpenseSharingPreferences, FinanceError>;

    /// Update expense sharing preferences for a user
    async fn update_sharing_preferences(
        &self,
        user_id: Uuid,
        enabled: bool,
        anonymized: bool,
        categories: Vec<ExpenseCategory>,
        time_limits: Option<crate::domain::expense_tracker::SharingTimeLimits>,
        recipient_specific_rules: Vec<crate::domain::expense_tracker::RecipientSharingRule>,
    ) -> Result<ExpenseSharingPreferences, FinanceError>;
}

/// Implementation of the ExpenseService
pub struct ExpenseServiceImpl {
    expense_repo: std::sync::Arc<dyn ExpenseRepository>,
    receipt_repo: std::sync::Arc<dyn ReceiptRepository>,
    sharing_preference_repo: std::sync::Arc<dyn ExpenseSharingPreferenceRepository>,
    wallet_service: std::sync::Arc<dyn WalletService>,
    budget_service: std::sync::Arc<dyn BudgetService>,
}

impl ExpenseServiceImpl {
    /// Create a new expense service
    pub fn new(
        expense_repo: std::sync::Arc<dyn ExpenseRepository>,
        receipt_repo: std::sync::Arc<dyn ReceiptRepository>,
        sharing_preference_repo: std::sync::Arc<dyn ExpenseSharingPreferenceRepository>,
        wallet_service: std::sync::Arc<dyn WalletService>,
        budget_service: std::sync::Arc<dyn BudgetService>,
    ) -> Self {
        Self {
            expense_repo,
            receipt_repo,
            sharing_preference_repo,
            wallet_service,
            budget_service,
        }
    }
}

#[async_trait]
impl ExpenseService for ExpenseServiceImpl {
    async fn create_expense(
        &self,
        user_id: Uuid,
        amount: Money,
        category: ExpenseCategory,
        date: DateTime<Utc>,
        description: String,
    ) -> Result<Expense, FinanceError> {
        // Create the expense
        let mut expense = Expense::new(user_id, amount.clone(), category, date, description);
        
        // Deduct from wallet using WalletService
        // This will handle both traditional currency and Dabloons
        match &amount.currency {
            crate::domain::primitives::Currency::Dabloons => {
                self.wallet_service
                    .subtract_dabloons(user_id, amount.clone(), Some(format!("Expense: {}", expense.description)))
                    .await?;
            }
            _ => {
                // For traditional currency, we use the subtract_traditional_currency method
                self.wallet_service
                    .subtract_traditional_currency(user_id, amount.clone(), Some(format!("Expense: {}", expense.description)))
                    .await?;
            }
        }
        
        // Save the expense
        self.expense_repo.save(&expense).await?;
        
        // If linked to a budget, update budget spent amount
        if let Some(budget_id) = expense.linked_budget_id {
            // Only update budget for Dabloons, as traditional currency isn't tracked in budgets
            if amount.currency == crate::domain::primitives::Currency::Dabloons {
                self.budget_service
                    .update_spent_with_dabloons(budget_id, amount)
                    .await?;
            }
        }
        
        Ok(expense)
    }

    async fn create_draft_from_receipt(
        &self,
        user_id: Uuid,
        receipt_id: Uuid,
        description: String,
    ) -> Result<Expense, FinanceError> {
        let expense = Expense::new_draft(user_id, receipt_id, description);
        self.expense_repo.save(&expense).await?;
        Ok(expense)
    }

    async fn get_user_expenses(
        &self,
        user_id: Uuid,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
    ) -> Result<Vec<Expense>, FinanceError> {
        self.expense_repo.find_by_user_id(user_id, start_date, end_date).await
    }

    async fn update_expense(
        &self,
        expense_id: Uuid,
        amount: Option<Money>,
        category: Option<ExpenseCategory>,
        date: Option<DateTime<Utc>>,
        description: Option<String>,
        status: Option<ExpenseStatus>,
        linked_budget_id: Option<Uuid>,
    ) -> Result<Expense, FinanceError> {
        let mut expense = self.expense_repo.find_by_id(expense_id).await?
            .ok_or_else(|| FinanceError::DatabaseError("Expense not found".to_string()))?;
        
        if let Some(new_amount) = amount {
            expense.update_amount(new_amount)?;
        }
        
        if let Some(new_category) = category {
            expense.category = new_category;
        }
        
        if let Some(new_date) = date {
            expense.date = new_date;
        }
        
        if let Some(new_description) = description {
            expense.description = new_description;
        }
        
        if let Some(new_status) = status {
            expense.status = new_status;
        }
        
        if let Some(new_budget_id) = linked_budget_id {
            expense.link_to_budget(new_budget_id);
        }
        
        expense.updated_at = Utc::now();
        self.expense_repo.update(&expense).await?;
        Ok(expense)
    }

    async fn delete_expense(&self, expense_id: Uuid) -> Result<(), FinanceError> {
        self.expense_repo.delete(expense_id).await
    }

    async fn link_to_budget(&self, expense_id: Uuid, budget_id: Uuid) -> Result<(), FinanceError> {
        let mut expense = self.expense_repo.find_by_id(expense_id).await?
            .ok_or_else(|| FinanceError::DatabaseError("Expense not found".to_string()))?;
        
        expense.link_to_budget(budget_id);
        self.expense_repo.update(&expense).await?;
        
        // Update budget spent amount
        self.budget_service
            .update_spent_with_dabloons(budget_id, expense.amount.clone())
            .await?;
        
        Ok(())
    }

    async fn process_receipt(&self, receipt_id: Uuid) -> Result<Expense, FinanceError> {
        let mut receipt = self.receipt_repo.find_by_id(receipt_id).await?
            .ok_or_else(|| FinanceError::DatabaseError("Receipt not found".to_string()))?;
        
        // Update receipt status to processing
        receipt.processing_status = ReceiptProcessingStatus::Processing;
        self.receipt_repo.update(&receipt).await?;
        
        // Extract items from receipt text (simplified implementation)
        let items = self.extract_items_from_receipt(&receipt.extracted_text);
        
        // Classify expense category
        let category = self.classify_expense_category(
            &receipt.merchant_name.clone().unwrap_or_else(|| "Unknown".to_string()),
            &items
        );
        
        // Create a draft expense with the classified category
        let expense = Expense::new_draft(
            receipt.user_id,
            receipt_id,
            receipt.merchant_name.clone().unwrap_or_else(|| "Scanned receipt".to_string())
        );
        
        // Update receipt with extracted data
        receipt.processing_status = ReceiptProcessingStatus::Processed;
        self.receipt_repo.update(&receipt).await?;
        
        Ok(expense)
    }

    async fn get_receipt(&self, receipt_id: Uuid) -> Result<Option<Receipt>, FinanceError> {
        self.receipt_repo.find_by_id(receipt_id).await
    }

    async fn save_receipt(
        &self,
        user_id: Uuid,
        image_data: ReceiptImageData,
    ) -> Result<Receipt, FinanceError> {
        let receipt = Receipt {
            id: Uuid::new_v4(),
            user_id,
            image_data,
            extracted_text: String::new(),
            merchant_name: None,
            transaction_date: None,
            total_amount: None,
            processing_status: ReceiptProcessingStatus::Uploaded,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        self.receipt_repo.save(&receipt).await?;
        Ok(receipt)
    }

    async fn update_receipt_status(&self, receipt_id: Uuid, status: ReceiptProcessingStatus) -> Result<(), FinanceError> {
        let mut receipt = self.receipt_repo.find_by_id(receipt_id).await?
            .ok_or_else(|| FinanceError::DatabaseError("Receipt not found".to_string()))?;
        
        receipt.processing_status = status;
        receipt.updated_at = Utc::now();
        self.receipt_repo.update(&receipt).await
    }

    async fn get_sharing_preferences(&self, user_id: Uuid) -> Result<ExpenseSharingPreferences, FinanceError> {
        match self.sharing_preference_repo.find_by_user_id(user_id).await? {
            Some(preference) => Ok(preference),
            None => self.sharing_preference_repo.create_default(user_id).await,
        }
    }

    async fn update_sharing_preferences(
        &self,
        user_id: Uuid,
        enabled: bool,
        anonymized: bool,
        categories: Vec<ExpenseCategory>,
        time_limits: Option<crate::domain::expense_tracker::SharingTimeLimits>,
        recipient_specific_rules: Vec<crate::domain::expense_tracker::RecipientSharingRule>,
    ) -> Result<ExpenseSharingPreferences, FinanceError> {
        let mut preference = self.get_sharing_preferences(user_id).await?;
        
        if enabled {
            preference.enable_sharing();
        } else {
            preference.disable_sharing();
        }
        
        if anonymized {
            preference.toggle_anonymization();
        }
        
        preference.set_shared_categories(categories);
        preference.time_limits = time_limits;
        preference.recipient_specific_rules = recipient_specific_rules;
        
        self.sharing_preference_repo.save(&preference).await?;
        Ok(preference)
    }
    
    /// Extract items from receipt text (simplified implementation)
    fn extract_items_from_receipt(&self, text: &str) -> Vec<String> {
        // This is a very simplified implementation
        // In a real application, this would be more sophisticated
        let mut items = Vec::new();
        
        // Split text into lines and look for item-like patterns
        for line in text.lines() {
            // Simple heuristic: lines with both text and numbers might be items
            if line.chars().any(|c| c.is_alphabetic()) && line.chars().any(|c| c.is_numeric()) {
                // Exclude lines that are clearly not items (like totals, taxes, etc.)
                let lower_line = line.to_lowercase();
                if !lower_line.contains("total") &&
                   !lower_line.contains("subtotal") &&
                   !lower_line.contains("tax") &&
                   !lower_line.contains("change") &&
                   !lower_line.contains("cash") &&
                   !lower_line.contains("card") {
                    items.push(line.trim().to_string());
                }
            }
        }
        
        items
    }
    
    /// Classify expense category based on merchant name and items
    fn classify_expense_category(&self, merchant_name: &str, items: &[String]) -> ExpenseCategory {
        // Simple classification based on keywords
        // In a real implementation, this would be more sophisticated
        let merchant_lower = merchant_name.to_lowercase();
        
        if merchant_lower.contains("starbucks") || merchant_lower.contains("coffee") {
            return ExpenseCategory::Food;
        }
        
        if merchant_lower.contains("shell") || merchant_lower.contains("gas") || merchant_lower.contains("exxon") {
            return ExpenseCategory::Transportation;
        }
        
        if merchant_lower.contains("walmart") || merchant_lower.contains("target") || merchant_lower.contains("grocery") {
            return ExpenseCategory::Shopping;
        }
        
        // Check items for keywords
        for item in items {
            let item_lower = item.to_lowercase();
            if item_lower.contains("coffee") || item_lower.contains("sandwich") || item_lower.contains("meal") {
                return ExpenseCategory::Food;
            }
            
            if item_lower.contains("gas") || item_lower.contains("fuel") {
                return ExpenseCategory::Transportation;
            }
        }
        
        // Default category
        ExpenseCategory::Other("Uncategorized".to_string())
    }
}