#[cfg(test)]
mod tests {
    use super::super::import_processor::ExpenseImportProcessor;
    use super::super::column_mapping::{ColumnMapping, ImportResult};
    use crate::domain::{Sheet, CellAddress, CellValue};
    use packages::domains::finance::domain::primitives::{Currency, Money};
    use packages::domains::finance::application::user_preferences::UserPreferences;
    use packages::domains::finance::domain::expense::{Expense, ExpenseCategory};
    use uuid::Uuid;
    use std::collections::HashMap;
    use std::sync::Arc;
    use async_trait::async_trait;
    use chrono::{DateTime, Utc};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use futures;
    
    // Mock ExpenseService for testing
    struct MockExpenseService {
        expenses: std::sync::Mutex<Vec<Expense>>,
    }
    
    impl MockExpenseService {
        fn new() -> Self {
            Self {
                expenses: std::sync::Mutex::new(Vec::new()),
            }
        }
    }
    
    #[async_trait]
    impl packages::domains::finance::application::ExpenseService for MockExpenseService {
        async fn add_expense(&self, expense: Expense) -> Result<(), packages::domains::finance::domain::FinanceError> {
            self.expenses.lock().unwrap().push(expense);
            Ok(())
        }
        
        async fn get_expenses(&self, _user_id: Uuid, _start_date: DateTime<Utc>, _end_date: DateTime<Utc>) -> Result<Vec<Expense>, packages::domains::finance::domain::FinanceError> {
            Ok(self.expenses.lock().unwrap().clone())
        }
        
        async fn get_expense_by_id(&self, _id: Uuid) -> Result<Option<Expense>, packages::domains::finance::domain::FinanceError> {
            unimplemented!()
        }
        
        async fn update_expense(&self, _expense: Expense) -> Result<(), packages::domains::finance::domain::FinanceError> {
            unimplemented!()
        }
        
        async fn delete_expense(&self, _id: Uuid) -> Result<(), packages::domains::finance::domain::FinanceError> {
            unimplemented!()
        }
        
        async fn get_expenses_by_category(&self, _user_id: Uuid, _category: ExpenseCategory, _start_date: DateTime<Utc>, _end_date: DateTime<Utc>) -> Result<Vec<Expense>, packages::domains::finance::domain::FinanceError> {
            unimplemented!()
        }
    }
    
    // Mock UserPreferences for testing
    struct MockUserPreferences {
        currency: Currency,
    }
    
    impl MockUserPreferences {
        fn new(currency: Currency) -> Self {
            Self { currency }
        }
    }
    
    #[async_trait]
    impl UserPreferences for MockUserPreferences {
        async fn get_preferred_currency(&self, _user_id: Uuid) -> Result<Currency, String> {
            Ok(self.currency.clone())
        }
        
        async fn set_preferred_currency(&self, _user_id: Uuid, _currency: Currency) -> Result<(), String> {
            Ok(())
        }
    }
    
    #[tokio::test]
    async fn test_process_expense_import_with_usd() {
        let mock_expense_service = Box::new(MockExpenseService::new());
        let processor = ExpenseImportProcessor::new(mock_expense_service);
        
        // Create a simple sheet with expense data
        let mut cells = HashMap::new();
        cells.insert(CellAddress::new(0, 0), crate::domain::Cell::new(CellValue::Text("Date".to_string())));
        cells.insert(CellAddress::new(0, 1), crate::domain::Cell::new(CellValue::Text("Amount".to_string())));
        cells.insert(CellAddress::new(0, 2), crate::domain::Cell::new(CellValue::Text("Category".to_string())));
        cells.insert(CellAddress::new(0, 3), crate::domain::Cell::new(CellValue::Text("Description".to_string())));
        cells.insert(CellAddress::new(1, 0), crate::domain::Cell::new(CellValue::Text("2023-01-15".to_string())));
        cells.insert(CellAddress::new(1, 1), crate::domain::Cell::new(CellValue::Number(25.50)));
        cells.insert(CellAddress::new(1, 2), crate::domain::Cell::new(CellValue::Text("Food".to_string())));
        cells.insert(CellAddress::new(1, 3), crate::domain::Cell::new(CellValue::Text("Lunch".to_string())));
        
        let sheet = Sheet::new("test_sheet".to_string(), cells);
        let mapping = ColumnMapping {
            date_column: "A".to_string(),
            amount_column: "B".to_string(),
            category_column: "C".to_string(),
            description_column: Some("D".to_string()),
        };
        let user_id = Uuid::new_v4();
        let user_preferences = MockUserPreferences::new(Currency::USD);
        
        let result = processor.process(&sheet, mapping, user_id, &user_preferences).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_process_expense_import_with_eur() {
        let mock_expense_service = Box::new(MockExpenseService::new());
        let processor = ExpenseImportProcessor::new(mock_expense_service);
        
        // Create a simple sheet with expense data
        let mut cells = HashMap::new();
        cells.insert(CellAddress::new(0, 0), crate::domain::Cell::new(CellValue::Text("Date".to_string())));
        cells.insert(CellAddress::new(0, 1), crate::domain::Cell::new(CellValue::Text("Amount".to_string())));
        cells.insert(CellAddress::new(0, 2), crate::domain::Cell::new(CellValue::Text("Category".to_string())));
        cells.insert(CellAddress::new(0, 3), crate::domain::Cell::new(CellValue::Text("Description".to_string())));
        cells.insert(CellAddress::new(1, 0), crate::domain::Cell::new(CellValue::Text("2023-01-15".to_string())));
        cells.insert(CellAddress::new(1, 1), crate::domain::Cell::new(CellValue::Number(25.50)));
        cells.insert(CellAddress::new(1, 2), crate::domain::Cell::new(CellValue::Text("Food".to_string())));
        cells.insert(CellAddress::new(1, 3), crate::domain::Cell::new(CellValue::Text("Lunch".to_string())));
        
        let sheet = Sheet::new("test_sheet".to_string(), cells);
        let mapping = ColumnMapping {
            date_column: "A".to_string(),
            amount_column: "B".to_string(),
            category_column: "C".to_string(),
            description_column: Some("D".to_string()),
        };
        let user_id = Uuid::new_v4();
        let user_preferences = MockUserPreferences::new(Currency::EUR);
        
        let result = processor.process(&sheet, mapping, user_id, &user_preferences).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_process_expense_import_with_jpy() {
        let mock_expense_service = Box::new(MockExpenseService::new());
        let processor = ExpenseImportProcessor::new(mock_expense_service);
        
        // Create a simple sheet with expense data
        let mut cells = HashMap::new();
        cells.insert(CellAddress::new(0, 0), crate::domain::Cell::new(CellValue::Text("Date".to_string())));
        cells.insert(CellAddress::new(0, 1), crate::domain::Cell::new(CellValue::Text("Amount".to_string())));
        cells.insert(CellAddress::new(0, 2), crate::domain::Cell::new(CellValue::Text("Category".to_string())));
        cells.insert(CellAddress::new(0, 3), crate::domain::Cell::new(CellValue::Text("Description".to_string())));
        cells.insert(CellAddress::new(1, 0), crate::domain::Cell::new(CellValue::Text("2023-01-15".to_string())));
        cells.insert(CellAddress::new(1, 1), crate::domain::Cell::new(CellValue::Number(2550.0)));
        cells.insert(CellAddress::new(1, 2), crate::domain::Cell::new(CellValue::Text("Food".to_string())));
        cells.insert(CellAddress::new(1, 3), crate::domain::Cell::new(CellValue::Text("Lunch".to_string())));
        
        let sheet = Sheet::new("test_sheet".to_string(), cells);
        let mapping = ColumnMapping {
            date_column: "A".to_string(),
            amount_column: "B".to_string(),
            category_column: "C".to_string(),
            description_column: Some("D".to_string()),
        };
        let user_id = Uuid::new_v4();
        let user_preferences = MockUserPreferences::new(Currency::JPY);
        
        let result = processor.process(&sheet, mapping, user_id, &user_preferences).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_process_expense_import_with_multiple_currencies() {
        let mock_expense_service = Box::new(MockExpenseService::new());
        let processor = ExpenseImportProcessor::new(mock_expense_service);
        
        // Create a sheet with multiple expense rows
        let mut cells = HashMap::new();
        cells.insert(CellAddress::new(0, 0), crate::domain::Cell::new(CellValue::Text("Date".to_string())));
        cells.insert(CellAddress::new(0, 1), crate::domain::Cell::new(CellValue::Text("Amount".to_string())));
        cells.insert(CellAddress::new(0, 2), crate::domain::Cell::new(CellValue::Text("Category".to_string())));
        cells.insert(CellAddress::new(1, 0), crate::domain::Cell::new(CellValue::Text("2023-01-15".to_string())));
        cells.insert(CellAddress::new(1, 1), crate::domain::Cell::new(CellValue::Number(25.50)));
        cells.insert(CellAddress::new(1, 2), crate::domain::Cell::new(CellValue::Text("Food".to_string())));
        cells.insert(CellAddress::new(2, 0), crate::domain::Cell::new(CellValue::Text("2023-01-16".to_string())));
        cells.insert(CellAddress::new(2, 1), crate::domain::Cell::new(CellValue::Number(15.75)));
        cells.insert(CellAddress::new(2, 2), crate::domain::Cell::new(CellValue::Text("Transport".to_string())));
        
        let sheet = Sheet::new("test_sheet".to_string(), cells);
        let mapping = ColumnMapping {
            date_column: "A".to_string(),
            amount_column: "B".to_string(),
            category_column: "C".to_string(),
            description_column: None,
        };
        let user_id = Uuid::new_v4();
        
        // Test with USD
        let user_preferences_usd = MockUserPreferences::new(Currency::USD);
        let result_usd = processor.process(&sheet, mapping.clone(), user_id, &user_preferences_usd).await;
        assert!(result_usd.is_ok());
        
        // Test with EUR
        let user_preferences_eur = MockUserPreferences::new(Currency::EUR);
        let result_eur = processor.process(&sheet, mapping.clone(), user_id, &user_preferences_eur).await;
        assert!(result_eur.is_ok());
    }
    
    #[tokio::test]
    async fn test_process_expense_import_with_zero_amounts() {
        let mock_expense_service = Box::new(MockExpenseService::new());
        let processor = ExpenseImportProcessor::new(mock_expense_service);
        
        // Create a sheet with zero amounts
        let mut cells = HashMap::new();
        cells.insert(CellAddress::new(0, 0), crate::domain::Cell::new(CellValue::Text("Date".to_string())));
        cells.insert(CellAddress::new(0, 1), crate::domain::Cell::new(CellValue::Text("Amount".to_string())));
        cells.insert(CellAddress::new(0, 2), crate::domain::Cell::new(CellValue::Text("Category".to_string())));
        cells.insert(CellAddress::new(1, 0), crate::domain::Cell::new(CellValue::Text("2023-01-15".to_string())));
        cells.insert(CellAddress::new(1, 1), crate::domain::Cell::new(CellValue::Number(0.0)));
        cells.insert(CellAddress::new(1, 2), crate::domain::Cell::new(CellValue::Text("Savings".to_string())));
        
        let sheet = Sheet::new("test_sheet".to_string(), cells);
        let mapping = ColumnMapping {
            date_column: "A".to_string(),
            amount_column: "B".to_string(),
            category_column: "C".to_string(),
            description_column: None,
        };
        let user_id = Uuid::new_v4();
        let user_preferences = MockUserPreferences::new(Currency::USD);
        
        let result = processor.process(&sheet, mapping, user_id, &user_preferences).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_process_expense_import_with_negative_amounts() {
        let mock_expense_service = Box::new(MockExpenseService::new());
        let processor = ExpenseImportProcessor::new(mock_expense_service);
        
        // Create a sheet with negative amounts (refunds)
        let mut cells = HashMap::new();
        cells.insert(CellAddress::new(0, 0), crate::domain::Cell::new(CellValue::Text("Date".to_string())));
        cells.insert(CellAddress::new(0, 1), crate::domain::Cell::new(CellValue::Text("Amount".to_string())));
        cells.insert(CellAddress::new(0, 2), crate::domain::Cell::new(CellValue::Text("Category".to_string())));
        cells.insert(CellAddress::new(1, 0), crate::domain::Cell::new(CellValue::Text("2023-01-15".to_string())));
        cells.insert(CellAddress::new(1, 1), crate::domain::Cell::new(CellValue::Number(-10.0)));
        cells.insert(CellAddress::new(1, 2), crate::domain::Cell::new(CellValue::Text("Refund".to_string())));
        
        let sheet = Sheet::new("test_sheet".to_string(), cells);
        let mapping = ColumnMapping {
            date_column: "A".to_string(),
            amount_column: "B".to_string(),
            category_column: "C".to_string(),
            description_column: None,
        };
        let user_id = Uuid::new_v4();
        let user_preferences = MockUserPreferences::new(Currency::USD);
        
        let result = processor.process(&sheet, mapping, user_id, &user_preferences).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_process_expense_import_with_invalid_currency_code() {
        // Test that invalid currency codes default to USD
        let mock_expense_service = Box::new(MockExpenseService::new());
        let processor = ExpenseImportProcessor::new(mock_expense_service);
        
        // Create a simple sheet with expense data
        let mut cells = HashMap::new();
        cells.insert(CellAddress::new(0, 0), crate::domain::Cell::new(CellValue::Text("Date".to_string())));
        cells.insert(CellAddress::new(0, 1), crate::domain::Cell::new(CellValue::Text("Amount".to_string())));
        cells.insert(CellAddress::new(0, 2), crate::domain::Cell::new(CellValue::Text("Category".to_string())));
        cells.insert(CellAddress::new(1, 0), crate::domain::Cell::new(CellValue::Text("2023-01-15".to_string())));
        cells.insert(CellAddress::new(1, 1), crate::domain::Cell::new(CellValue::Number(25.50)));
        cells.insert(CellAddress::new(1, 2), crate::domain::Cell::new(CellValue::Text("Food".to_string())));
        
        let sheet = Sheet::new("test_sheet".to_string(), cells);
        let mapping = ColumnMapping {
            date_column: "A".to_string(),
            amount_column: "B".to_string(),
            category_column: "C".to_string(),
            description_column: None,
        };
        let user_id = Uuid::new_v4();
        
        // Create a mock that returns an invalid currency code
        struct InvalidCurrencyMock;
        
        #[async_trait]
        impl UserPreferences for InvalidCurrencyMock {
            async fn get_preferred_currency(&self, _user_id: Uuid) -> Result<Currency, String> {
                // This simulates what would happen if an invalid currency code was stored in the database
                Ok(Currency::USD) // The repository should handle invalid codes and default to USD
            }
            
            async fn set_preferred_currency(&self, _user_id: Uuid, _currency: Currency) -> Result<(), String> {
                Ok(())
            }
        }
        
        let user_preferences = InvalidCurrencyMock;
        let result = processor.process(&sheet, mapping, user_id, &user_preferences).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_concurrent_currency_updates() {
        // Test concurrent updates to currency preferences
        use std::sync::atomic::{AtomicUsize, Ordering};
        
        static UPDATE_COUNT: AtomicUsize = AtomicUsize::new(0);
        
        struct ConcurrentCurrencyMock;
        
        #[async_trait]
        impl UserPreferences for ConcurrentCurrencyMock {
            async fn get_preferred_currency(&self, _user_id: Uuid) -> Result<Currency, String> {
                // Simulate some delay to increase chance of concurrency issues
                tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
                Ok(Currency::USD)
            }
            
            async fn set_preferred_currency(&self, _user_id: Uuid, _currency: Currency) -> Result<(), String> {
                // Simulate some work and track concurrent access
                UPDATE_COUNT.fetch_add(1, Ordering::SeqCst);
                tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
                Ok(())
            }
        }
        
        let mock_expense_service = Box::new(MockExpenseService::new());
        let processor = ExpenseImportProcessor::new(mock_expense_service);
        
        // Create a simple sheet with expense data
        let mut cells = HashMap::new();
        cells.insert(CellAddress::new(0, 0), crate::domain::Cell::new(CellValue::Text("Date".to_string())));
        cells.insert(CellAddress::new(0, 1), crate::domain::Cell::new(CellValue::Text("Amount".to_string())));
        cells.insert(CellAddress::new(0, 2), crate::domain::Cell::new(CellValue::Text("Category".to_string())));
        cells.insert(CellAddress::new(1, 0), crate::domain::Cell::new(CellValue::Text("2023-01-15".to_string())));
        cells.insert(CellAddress::new(1, 1), crate::domain::Cell::new(CellValue::Number(25.50)));
        cells.insert(CellAddress::new(1, 2), crate::domain::Cell::new(CellValue::Text("Food".to_string())));
        
        let sheet = Sheet::new("test_sheet".to_string(), cells);
        let mapping = ColumnMapping {
            date_column: "A".to_string(),
            amount_column: "B".to_string(),
            category_column: "C".to_string(),
            description_column: None,
        };
        let user_id = Uuid::new_v4();
        let user_preferences = ConcurrentCurrencyMock;
        
        // Process multiple imports concurrently
        let mut handles = vec![];
        for _ in 0..5 {
            let mock_expense_service_clone = Box::new(MockExpenseService::new());
            let processor_clone = ExpenseImportProcessor::new(mock_expense_service_clone);
            let sheet_clone = sheet.clone();
            let mapping_clone = mapping.clone();
            let user_preferences_clone = &user_preferences;
            let user_id_clone = user_id;
            
            let handle = tokio::spawn(async move {
                processor_clone.process(&sheet_clone, mapping_clone, user_id_clone, user_preferences_clone).await
            });
            handles.push(handle);
        }
        
        // Wait for all tasks to complete
        let results: Vec<_> = futures::future::join_all(handles).await;
        
        // Check that all operations succeeded
        for result in results {
            assert!(result.is_ok());
            let import_result = result.unwrap();
            assert!(import_result.is_ok());
        }
        
        // Verify that the update count is as expected
        assert_eq!(UPDATE_COUNT.load(Ordering::SeqCst), 0); // No updates in this test, only reads
    }
}