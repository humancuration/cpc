//! Expense service for managing and categorizing expenses

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use regex::Regex;
use uuid::Uuid;
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use cpc_core::finance::{Money, Currency};

use crate::domain::models::{Expense, ExpenseCategory, FinanceError};

/// Trait for receipt scanning using OCR
#[async_trait::async_trait]
pub trait ReceiptScanner: Send + Sync {
    async fn scan_receipt(&self, image_data: Vec<u8>) -> Result<String, FinanceError>;
}

/// Mock OCR service for development
pub struct MockReceiptScanner;

#[async_trait::async_trait]
impl ReceiptScanner for MockReceiptScanner {
    async fn scan_receipt(&self, _image_data: Vec<u8>) -> Result<String, FinanceError> {
        // Mock OCR text extraction
        Ok("GROCERY STORE - Walmart\nTotal: $45.67\nItems: Milk, Bread, Eggs, Cheese".to_string())
    }
}

/// Rule-based expense categorization following royalty service pattern
#[derive(Debug, Clone)]
pub struct CategorizationRule {
    pub keywords: Vec<String>,
    pub category: String,
    pub priority: u32,
}

impl CategorizationRule {
    pub fn new(keywords: Vec<String>, category: String, priority: u32) -> Self {
        Self {
            keywords,
            category,
            priority,
        }
    }

    pub fn matches(&self, text: &str) -> bool {
        let text_lower = text.to_lowercase();
        self.keywords.iter().any(|keyword| {
            text_lower.contains(&keyword.to_lowercase())
        })
    }
}

/// Expense service for managing expenses and categorization
pub struct ExpenseService {
    categorization_rules: Arc<RwLock<HashMap<String, Vec<CategorizationRule>>>>,
    receipt_scanner: Arc<dyn ReceiptScanner>,
}

impl ExpenseService {
    pub fn new(receipt_scanner: Arc<dyn ReceiptScanner>) -> Self {
        let rules = Self::default_categorization_rules();
        Self {
            categorization_rules: Arc::new(RwLock::new(rules)),
            receipt_scanner,
        }
    }

    /// Create default categorization rules
    fn default_categorization_rules() -> HashMap<String, Vec<CategorizationRule>> {
        let mut rules = HashMap::new();

        // Food & Dining
        rules.insert("Food & Dining".to_string(), vec![
            CategorizationRule::new(
                vec!["restaurant".to_string(), "food".to_string(), "grocery".to_string(), 
                     "walmart".to_string(), "kroger".to_string(), "starbucks".to_string()],
                "Food & Dining".to_string(),
                1
            ),
        ]);

        // Transportation
        rules.insert("Transportation".to_string(), vec![
            CategorizationRule::new(
                vec!["gas".to_string(), "fuel".to_string(), "uber".to_string(), 
                     "taxi".to_string(), "parking".to_string(), "transit".to_string()],
                "Transportation".to_string(),
                1
            ),
        ]);

        // Entertainment
        rules.insert("Entertainment".to_string(), vec![
            CategorizationRule::new(
                vec!["movie".to_string(), "cinema".to_string(), "netflix".to_string(),
                     "spotify".to_string(), "game".to_string(), "concert".to_string()],
                "Entertainment".to_string(),
                1
            ),
        ]);

        // Shopping
        rules.insert("Shopping".to_string(), vec![
            CategorizationRule::new(
                vec!["amazon".to_string(), "store".to_string(), "shopping".to_string(),
                     "retail".to_string(), "clothing".to_string()],
                "Shopping".to_string(),
                1
            ),
        ]);

        // Utilities
        rules.insert("Utilities".to_string(), vec![
            CategorizationRule::new(
                vec!["electric".to_string(), "water".to_string(), "gas".to_string(),
                     "internet".to_string(), "phone".to_string(), "utility".to_string()],
                "Utilities".to_string(),
                1
            ),
        ]);

        // Healthcare
        rules.insert("Healthcare".to_string(), vec![
            CategorizationRule::new(
                vec!["pharmacy".to_string(), "doctor".to_string(), "medical".to_string(),
                     "health".to_string(), "prescription".to_string()],
                "Healthcare".to_string(),
                1
            ),
        ]);

        rules
    }
/// Record a new expense with optional receipt scanning
pub async fn record_expense(
    &self,
    user_id: Uuid,
    amount: Money,
    description: String,
    payment_method: String,
    receipt_image: Option<Vec<u8>>,
) -> Result<Expense, FinanceError> {
    let mut expense = Expense::new(
        user_id,
        amount,
        description.clone(),
        payment_method,
    );

    // If receipt image provided, scan and categorize
    if let Some(image_data) = receipt_image {
        self.categorize_from_receipt(&mut expense, image_data).await?;
    } else {
        // Categorize based on description
        self.categorize_from_text(&mut expense, &description).await?;
    }

    Ok(expense)
}
    }

    /// Categorize expense from receipt image
    pub async fn categorize_from_receipt(
        &self,
        expense: &mut Expense,
        receipt_image: Vec<u8>,
    ) -> Result<(), FinanceError> {
        let ocr_text = self.receipt_scanner.scan_receipt(receipt_image).await?;
        
        // Extract merchant from receipt
        if let Some(merchant) = self.extract_merchant(&ocr_text) {
            expense.merchant = Some(merchant);
        }

        // Categorize based on receipt text
        self.categorize_from_text(expense, &ocr_text).await?;
        
        Ok(())
    }

    /// Categorize expense from text
    pub async fn categorize_from_text(
        &self,
        expense: &mut Expense,
        text: &str,
    ) -> Result<(), FinanceError> {
        let rules = self.categorization_rules.read()
            .map_err(|_| FinanceError::CategorizationError("Failed to read rules".to_string()))?;

        let mut best_category = "Uncategorized".to_string();
        let mut best_priority = 0;

        // Apply rules to find best category
        for (category, category_rules) in rules.iter() {
            for rule in category_rules {
                if rule.matches(text) && rule.priority >= best_priority {
                    best_category = category.clone();
                    best_priority = rule.priority;
                }
            }
        }

        expense.category = best_category;
        Ok(())
    }

    /// Extract merchant name from receipt text
    fn extract_merchant(&self, text: &str) -> Option<String> {
        // Simple regex to extract merchant name (first line or specific patterns)
        let lines: Vec<&str> = text.lines().collect();
        if !lines.is_empty() {
            let first_line = lines[0].trim();
            if !first_line.is_empty() {
                return Some(first_line.to_string());
            }
        }
        None
    }

    /// Add new categorization rule
    pub async fn add_categorization_rule(
        &self,
        category: String,
        keywords: Vec<String>,
        priority: u32,
    ) -> Result<(), FinanceError> {
        let rule = CategorizationRule::new(keywords, category.clone(), priority);
        
        let mut rules = self.categorization_rules.write()
            .map_err(|_| FinanceError::CategorizationError("Failed to write rules".to_string()))?;

        rules.entry(category).or_insert_with(Vec::new).push(rule);
        Ok(())
    }

    /// Update expense category manually
    pub async fn update_category(
        &self,
        expense: &mut Expense,
        new_category: String,
    ) -> Result<(), FinanceError> {
        expense.category = new_category;
        Ok(())
    }

    /// Analyze expenses by category
    /// Analyze expenses by category
    pub async fn analyze_expenses_by_category(
        &self,
        expenses: Vec<Expense>,
    ) -> Result<ExpenseAnalysis, FinanceError> {
        // For analysis, we'll convert to a common currency (USD) if needed
        // In a real implementation, we'd need proper currency conversion
        let mut category_totals = HashMap::new();
        let mut monthly_totals = HashMap::new();
        
        for expense in &expenses {
            // Add amounts to category totals (assuming same currency for simplicity)
            let current_total = category_totals.entry(expense.category.clone()).or_insert(Money::zero(Currency::USD));
            *current_total = current_total.add(&expense.amount).unwrap_or_else(|_| current_total.clone());
            
            let month_key = expense.date.format("%Y-%m").to_string();
            let current_monthly_total = monthly_totals.entry(month_key).or_insert(Money::zero(Currency::USD));
            *current_monthly_total = current_monthly_total.add(&expense.amount).unwrap_or_else(|_| current_monthly_total.clone());
        }

        // Calculate total spent (assuming same currency for simplicity)
        let mut total_spent = Money::zero(Currency::USD);
        for expense in &expenses {
            total_spent = total_spent.add(&expense.amount).unwrap_or_else(|_| total_spent.clone());
        }
        
        // Calculate average expense
        let average_expense = if expenses.is_empty() {
            Money::zero(Currency::USD)
        } else {
            let total_amount = total_spent.amount;
            let average_amount = total_amount / Decimal::from(expenses.len() as u64);
            Money::new(average_amount, Currency::USD)
        };

        Ok(ExpenseAnalysis {
            total_spent,
            expense_count: expenses.len(),
            average_expense,
            category_breakdown: category_totals,
            monthly_breakdown: monthly_totals,
            top_merchants: self.get_top_merchants(&expenses),
        })
    }
    /// Get top merchants by spending
    fn get_top_merchants(&self, expenses: &[Expense]) -> Vec<(String, Money)> {
        let mut merchant_totals = HashMap::new();
        
        for expense in expenses {
            if let Some(ref merchant) = expense.merchant {
                let current_total = merchant_totals.entry(merchant.clone()).or_insert(Money::zero(Currency::USD));
                *current_total = current_total.add(&expense.amount).unwrap_or_else(|_| current_total.clone());
            }
        }

        let mut merchants: Vec<(String, Money)> = merchant_totals.into_iter().collect();
        merchants.sort_by(|a, b| b.1.amount.cmp(&a.1.amount));
        merchants.truncate(10); // Top 10 merchants
        merchants
    }

    /// Find similar expenses
    pub async fn find_similar_expenses(
        &self,
        expenses: &[Expense],
        target_expense: &Expense,
    ) -> Vec<Expense> {
        expenses.iter()
            .filter(|e| {
                e.id != target_expense.id &&
                e.category == target_expense.category &&
                (e.description.to_lowercase().contains(&target_expense.description.to_lowercase()) ||
                 target_expense.description.to_lowercase().contains(&e.description.to_lowercase()))
            })
            .cloned()
            .collect()
    }
}

/// Expense analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseAnalysis {
    pub total_spent: Money,
    pub expense_count: usize,
    pub average_expense: Money,
    pub category_breakdown: HashMap<String, Money>,
    pub monthly_breakdown: HashMap<String, Money>,
    pub top_merchants: Vec<(String, Money)>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use chrono::Utc;

    #[tokio::test]
    async fn test_record_expense() {
        let scanner = Arc::new(MockReceiptScanner);
        let service = ExpenseService::new(scanner);
        
        let expense = service
            .record_expense(
                Uuid::new_v4(),
                dec!(45.67),
                "USD".to_string(),
                "Grocery shopping".to_string(),
                "Credit Card".to_string(),
                None,
            )
            .await
            .unwrap();

        assert_eq!(expense.amount, dec!(45.67));
        assert_eq!(expense.category, "Food & Dining");
    }

    #[tokio::test]
    async fn test_categorize_from_text() {
        let scanner = Arc::new(MockReceiptScanner);
        let service = ExpenseService::new(scanner);
        
        let mut expense = Expense::new(
            Uuid::new_v4(),
            dec!(20.00),
            "USD".to_string(),
            "Netflix subscription".to_string(),
            "Credit Card".to_string(),
        );

        service
            .categorize_from_text(&mut expense, "Netflix monthly subscription")
            .await
            .unwrap();

        assert_eq!(expense.category, "Entertainment");
    }

    #[tokio::test]
    async fn test_expense_analysis() {
        let scanner = Arc::new(MockReceiptScanner);
        let service = ExpenseService::new(scanner);
        
        let expenses = vec![
            Expense::new(
                Uuid::new_v4(),
                dec!(45.67),
                "USD".to_string(),
                "Walmart groceries".to_string(),
                "Credit Card".to_string(),
            ),
            Expense::new(
                Uuid::new_v4(),
                dec!(15.00),
                "USD".to_string(),
                "Netflix".to_string(),
                "Credit Card".to_string(),
            ),
            Expense::new(
                Uuid::new_v4(),
                dec!(30.00),
                "USD".to_string(),
                "Starbucks coffee".to_string(),
                "Credit Card".to_string(),
            ),
        ];

        let analysis = service
            .analyze_expenses_by_category(expenses)
            .await
            .unwrap();

        assert_eq!(analysis.total_spent, dec!(90.67));
        assert_eq!(analysis.expense_count, 3);
        assert!(analysis.category_breakdown.contains_key("Food & Dining"));
        assert!(analysis.category_breakdown.contains_key("Entertainment"));
    }

    #[tokio::test]
    async fn test_add_categorization_rule() {
        let scanner = Arc::new(MockReceiptScanner);
        let service = ExpenseService::new(scanner);
        
        service
            .add_categorization_rule(
                "Technology".to_string(),
                vec!["laptop".to_string(), "computer".to_string(), "software".to_string()],
                1,
            )
            .await
            .unwrap();

        let mut expense = Expense::new(
            Uuid::new_v4(),
            dec!(999.00),
            "USD".to_string(),
            "New laptop".to_string(),
            "Credit Card".to_string(),
        );

        service
            .categorize_from_text(&mut expense, "Dell laptop purchase")
            .await
            .unwrap();

        assert_eq!(expense.category, "Technology");
    }

    #[tokio::test]
    async fn test_find_similar_expenses() {
        let scanner = Arc::new(MockReceiptScanner);
        let service = ExpenseService::new(scanner);
        
        let expenses = vec![
            Expense::new(
                Uuid::new_v4(),
                dec!(45.67),
                "USD".to_string(),
                "Walmart groceries".to_string(),
                "Credit Card".to_string(),
            ),
            Expense::new(
                Uuid::new_v4(),
                dec!(35.00),
                "USD".to_string(),
                "Target groceries".to_string(),
                "Credit Card".to_string(),
            ),
            Expense::new(
                Uuid::new_v4(),
                dec!(15.00),
                "USD".to_string(),
                "Netflix".to_string(),
                "Credit Card".to_string(),
            ),
        ];

        let target = expenses[0].clone();
        let similar = service
            .find_similar_expenses(&expenses, &target)
            .await;

        assert_eq!(similar.len(), 1);
        assert_eq!(similar[0].description, "Target groceries");
    }
}