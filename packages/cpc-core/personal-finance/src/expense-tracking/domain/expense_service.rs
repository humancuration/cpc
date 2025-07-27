//! Expense service for managing expenses and tracking

use async_trait::async_trait;
use uuid::Uuid;
use crate::expense_tracking::domain::models::{Expense, Receipt, ExpenseCategory};
use crate::domain::models::FinanceError;
use cpc_core::finance::Money;

#[async_trait]
pub trait ExpenseRepository {
    async fn save(&self, expense: &Expense) -> Result<(), FinanceError>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Expense>, FinanceError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Expense>, FinanceError>;
    async fn delete(&self, id: Uuid) -> Result<(), FinanceError>;
}

#[async_trait]
pub trait TreasuryService {
    async fn categorize_expense(&self, description: &str, amount: Money) -> Result<ExpenseCategory, FinanceError>;
    async fn get_spending_insights(&self, user_id: Uuid) -> Result<Vec<ExpenseCategory>, FinanceError>;
}

#[async_trait]
pub trait OcrService {
    async fn process_receipt(&self, image_data: &[u8]) -> Result<Receipt, FinanceError>;
}

#[async_trait]
pub trait ExpenseService {
    async fn create_expense(&self, user_id: Uuid, amount: Money, description: String, payment_method: String) -> Result<Expense, FinanceError>;
    async fn get_user_expenses(&self, user_id: Uuid) -> Result<Vec<Expense>, FinanceError>;
    async fn get_expense_by_id(&self, id: Uuid) -> Result<Option<Expense>, FinanceError>;
    async fn update_expense_category(&self, expense_id: Uuid, category: String) -> Result<Expense, FinanceError>;
    async fn delete_expense(&self, expense_id: Uuid) -> Result<(), FinanceError>;
    async fn process_receipt_image(&self, user_id: Uuid, image_data: &[u8]) -> Result<Expense, FinanceError>;
    async fn get_spending_insights(&self, user_id: Uuid) -> Result<Vec<ExpenseCategory>, FinanceError>;
}

pub struct ExpenseServiceImpl {
    expense_repo: std::sync::Arc<dyn ExpenseRepository>,
    treasury_service: std::sync::Arc<dyn TreasuryService>,
    ocr_service: std::sync::Arc<dyn OcrService>,
}

impl ExpenseServiceImpl {
    pub fn new(
        expense_repo: std::sync::Arc<dyn ExpenseRepository>,
        treasury_service: std::sync::Arc<dyn TreasuryService>,
        ocr_service: std::sync::Arc<dyn OcrService>,
    ) -> Self {
        Self {
            expense_repo,
            treasury_service,
            ocr_service,
        }
    }
}

#[async_trait]
impl ExpenseService for ExpenseServiceImpl {
    async fn create_expense(&self, user_id: Uuid, amount: Money, description: String, payment_method: String) -> Result<Expense, FinanceError> {
        // Auto-categorize the expense using the treasury service
        let category = self.treasury_service.categorize_expense(&description, amount.clone()).await
            .unwrap_or_else(|_| ExpenseCategory::new("Uncategorized".to_string()));
        
        let mut expense = Expense::new(user_id, amount, description, payment_method);
        expense.category = category.name;
        expense.tags = category.keywords;
        
        self.expense_repo.save(&expense).await?;
        Ok(expense)
    }

    async fn get_user_expenses(&self, user_id: Uuid) -> Result<Vec<Expense>, FinanceError> {
        self.expense_repo.find_by_user_id(user_id).await
    }

    async fn get_expense_by_id(&self, id: Uuid) -> Result<Option<Expense>, FinanceError> {
        self.expense_repo.find_by_id(id).await
    }

    async fn update_expense_category(&self, expense_id: Uuid, category: String) -> Result<Expense, FinanceError> {
        let mut expense = self.expense_repo.find_by_id(expense_id).await?
            .ok_or(FinanceError::ExpenseNotFound(expense_id))?;
        expense.category = category;
        self.expense_repo.save(&expense).await?;
        Ok(expense)
    }

    async fn delete_expense(&self, expense_id: Uuid) -> Result<(), FinanceError> {
        self.expense_repo.delete(expense_id).await
    }

    async fn process_receipt_image(&self, user_id: Uuid, image_data: &[u8]) -> Result<Expense, FinanceError> {
        // Process the receipt image using OCR
        let receipt = self.ocr_service.process_receipt(image_data).await?;
        
        // Create an expense from the receipt data
        let expense = Expense::new(
            user_id,
            receipt.total_amount.clone(),
            receipt.merchant.clone().unwrap_or_else(|| "Unknown Merchant".to_string()),
            "Unknown".to_string(), // Payment method not available from receipt
        );
        
        self.expense_repo.save(&expense).await?;
        Ok(expense)
    }

    async fn get_spending_insights(&self, user_id: Uuid) -> Result<Vec<ExpenseCategory>, FinanceError> {
        self.treasury_service.get_spending_insights(user_id).await
    }
}