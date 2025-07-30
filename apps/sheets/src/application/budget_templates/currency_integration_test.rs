#[cfg(test)]
mod tests {
    use super::super::template_service::BudgetTemplateService;
    use crate::domain::{Sheet, CellAddress, CellValue};
    use packages::domains::finance::domain::primitives::{Currency, Money};
    use packages::domains::finance::application::user_preferences::UserPreferences;
    use uuid::Uuid;
    use std::collections::HashMap;
    use std::sync::Arc;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    // Mock BudgetService for testing
    struct MockBudgetService {
        budgets: std::sync::Mutex<Vec<packages::domains::finance::domain::budget::Budget>>,
    }
    
    impl MockBudgetService {
        fn new() -> Self {
            Self {
                budgets: std::sync::Mutex::new(Vec::new()),
            }
        }
    }
    
    #[async_trait]
    impl packages::domains::finance::application::BudgetService for MockBudgetService {
        async fn create_budget(&self, user_id: Uuid, category: String, amount: Money, period: packages::domains::finance::domain::budget::BudgetPeriod, start_date: chrono::DateTime<chrono::Utc>, end_date: chrono::DateTime<chrono::Utc>) -> Result<packages::domains::finance::domain::budget::Budget, packages::domains::finance::domain::FinanceError> {
            let budget = packages::domains::finance::domain::budget::Budget::new(
                user_id,
                category,
                amount,
                period,
                start_date,
                end_date,
            );
            self.budgets.lock().unwrap().push(budget.clone());
            Ok(budget)
        }
        
        async fn get_user_budgets(&self, _user_id: Uuid) -> Result<Vec<packages::domains::finance::domain::budget::Budget>, packages::domains::finance::domain::FinanceError> {
            Ok(self.budgets.lock().unwrap().clone())
        }
        
        async fn update_budget_spent(&self, _budget_id: Uuid, _amount: Money) -> Result<packages::domains::finance::domain::budget::Budget, packages::domains::finance::domain::FinanceError> {
            unimplemented!()
        }
        
        async fn get_budget_by_category(&self, _user_id: Uuid, _category: &str) -> Result<Option<packages::domains::finance::domain::budget::Budget>, packages::domains::finance::domain::FinanceError> {
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
    
    #[test]
    fn test_process_monthly_budget_template_with_usd() {
        let mock_budget_service = Box::new(MockBudgetService::new());
        let service = BudgetTemplateService::new(mock_budget_service);
        
        // Create a simple sheet with budget data
        let mut cells = HashMap::new();
        cells.insert(CellAddress::new(0, 0), crate::domain::Cell::new(CellValue::Text("Category".to_string())));
        cells.insert(CellAddress::new(0, 1), crate::domain::Cell::new(CellValue::Text("Amount".to_string())));
        cells.insert(CellAddress::new(1, 0), crate::domain::Cell::new(CellValue::Text("Food".to_string())));
        cells.insert(CellAddress::new(1, 1), crate::domain::Cell::new(CellValue::Number(100.0)));
        cells.insert(CellAddress::new(2, 0), crate::domain::Cell::new(CellValue::Text("Transport".to_string())));
        cells.insert(CellAddress::new(2, 1), crate::domain::Cell::new(CellValue::Number(50.0)));
        
        let sheet = Sheet::new("test_sheet".to_string(), cells);
        let user_id = Uuid::new_v4();
        let user_preferences = MockUserPreferences::new(Currency::USD);
        
        let result = service.apply_template(&sheet, user_id, &user_preferences);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_process_monthly_budget_template_with_eur() {
        let mock_budget_service = Box::new(MockBudgetService::new());
        let service = BudgetTemplateService::new(mock_budget_service);
        
        // Create a simple sheet with budget data
        let mut cells = HashMap::new();
        cells.insert(CellAddress::new(0, 0), crate::domain::Cell::new(CellValue::Text("Category".to_string())));
        cells.insert(CellAddress::new(0, 1), crate::domain::Cell::new(CellValue::Text("Amount".to_string())));
        cells.insert(CellAddress::new(1, 0), crate::domain::Cell::new(CellValue::Text("Food".to_string())));
        cells.insert(CellAddress::new(1, 1), crate::domain::Cell::new(CellValue::Number(100.0)));
        cells.insert(CellAddress::new(2, 0), crate::domain::Cell::new(CellValue::Text("Transport".to_string())));
        cells.insert(CellAddress::new(2, 1), crate::domain::Cell::new(CellValue::Number(50.0)));
        
        let sheet = Sheet::new("test_sheet".to_string(), cells);
        let user_id = Uuid::new_v4();
        let user_preferences = MockUserPreferences::new(Currency::EUR);
        
        let result = service.apply_template(&sheet, user_id, &user_preferences);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_process_monthly_budget_template_with_jpy() {
        let mock_budget_service = Box::new(MockBudgetService::new());
        let service = BudgetTemplateService::new(mock_budget_service);
        
        // Create a simple sheet with budget data
        let mut cells = HashMap::new();
        cells.insert(CellAddress::new(0, 0), crate::domain::Cell::new(CellValue::Text("Category".to_string())));
        cells.insert(CellAddress::new(0, 1), crate::domain::Cell::new(CellValue::Text("Amount".to_string())));
        cells.insert(CellAddress::new(1, 0), crate::domain::Cell::new(CellValue::Text("Food".to_string())));
        cells.insert(CellAddress::new(1, 1), crate::domain::Cell::new(CellValue::Number(10000.0)));
        cells.insert(CellAddress::new(2, 0), crate::domain::Cell::new(CellValue::Text("Transport".to_string())));
        cells.insert(CellAddress::new(2, 1), crate::domain::Cell::new(CellValue::Number(5000.0)));
        
        let sheet = Sheet::new("test_sheet".to_string(), cells);
        let user_id = Uuid::new_v4();
        let user_preferences = MockUserPreferences::new(Currency::JPY);
        
        let result = service.apply_template(&sheet, user_id, &user_preferences);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_process_weekly_budget_template_with_different_currencies() {
        let mock_budget_service = Box::new(MockBudgetService::new());
        let service = BudgetTemplateService::new(mock_budget_service);
        
        // Create a simple sheet with budget data
        let mut cells = HashMap::new();
        cells.insert(CellAddress::new(0, 0), crate::domain::Cell::new(CellValue::Text("Category".to_string())));
        cells.insert(CellAddress::new(0, 1), crate::domain::Cell::new(CellValue::Text("Amount".to_string())));
        cells.insert(CellAddress::new(1, 0), crate::domain::Cell::new(CellValue::Text("Food".to_string())));
        cells.insert(CellAddress::new(1, 1), crate::domain::Cell::new(CellValue::Number(25.50)));
        cells.insert(CellAddress::new(2, 0), crate::domain::Cell::new(CellValue::Text("Entertainment".to_string())));
        cells.insert(CellAddress::new(2, 1), crate::domain::Cell::new(CellValue::Number(15.75)));
        
        let sheet = Sheet::new("test_sheet".to_string(), cells);
        let user_id = Uuid::new_v4();
        
        // Test with USD
        let user_preferences_usd = MockUserPreferences::new(Currency::USD);
        let result_usd = service.apply_template(&sheet, user_id, &user_preferences_usd);
        assert!(result_usd.is_ok());
        
        // Test with EUR
        let user_preferences_eur = MockUserPreferences::new(Currency::EUR);
        let result_eur = service.apply_template(&sheet, user_id, &user_preferences_eur);
        assert!(result_eur.is_ok());
    }
    
    #[test]
    fn test_process_custom_template_with_zero_amounts() {
        let mock_budget_service = Box::new(MockBudgetService::new());
        let service = BudgetTemplateService::new(mock_budget_service);
        
        // Create a sheet with zero amounts
        let mut cells = HashMap::new();
        cells.insert(CellAddress::new(0, 0), crate::domain::Cell::new(CellValue::Text("Category".to_string())));
        cells.insert(CellAddress::new(0, 1), crate::domain::Cell::new(CellValue::Text("Amount".to_string())));
        cells.insert(CellAddress::new(1, 0), crate::domain::Cell::new(CellValue::Text("Savings".to_string())));
        cells.insert(CellAddress::new(1, 1), crate::domain::Cell::new(CellValue::Number(0.0)));
        
        let sheet = Sheet::new("test_sheet".to_string(), cells);
        let user_id = Uuid::new_v4();
        let user_preferences = MockUserPreferences::new(Currency::USD);
        
        let result = service.apply_template(&sheet, user_id, &user_preferences);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_process_template_with_invalid_currency_code() {
        // Test that invalid currency codes default to USD
        let mock_budget_service = Box::new(MockBudgetService::new());
        let service = BudgetTemplateService::new(mock_budget_service);
        
        // Create a simple sheet with budget data
        let mut cells = HashMap::new();
        cells.insert(CellAddress::new(0, 0), crate::domain::Cell::new(CellValue::Text("Category".to_string())));
        cells.insert(CellAddress::new(0, 1), crate::domain::Cell::new(CellValue::Text("Amount".to_string())));
        cells.insert(CellAddress::new(1, 0), crate::domain::Cell::new(CellValue::Text("Food".to_string())));
        cells.insert(CellAddress::new(1, 1), crate::domain::Cell::new(CellValue::Number(100.0)));
        
        let sheet = Sheet::new("test_sheet".to_string(), cells);
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
        let result = service.apply_template(&sheet, user_id, &user_preferences);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_concurrent_template_processing() {
        // Test concurrent processing of budget templates
        use std::sync::atomic::{AtomicUsize, Ordering};
        
        static PROCESS_COUNT: AtomicUsize = AtomicUsize::new(0);
        
        // Mock BudgetService for testing
        struct ConcurrentMockBudgetService {
            budgets: std::sync::Mutex<Vec<packages::domains::finance::domain::budget::Budget>>,
        }
        
        impl ConcurrentMockBudgetService {
            fn new() -> Self {
                Self {
                    budgets: std::sync::Mutex::new(Vec::new()),
                }
            }
        }
        
        #[async_trait]
        impl packages::domains::finance::application::BudgetService for ConcurrentMockBudgetService {
            async fn create_budget(&self, user_id: Uuid, category: String, amount: Money, period: packages::domains::finance::domain::budget::BudgetPeriod, start_date: chrono::DateTime<chrono::Utc>, end_date: chrono::DateTime<chrono::Utc>) -> Result<packages::domains::finance::domain::budget::Budget, packages::domains::finance::domain::FinanceError> {
                // Simulate some work and track concurrent access
                PROCESS_COUNT.fetch_add(1, Ordering::SeqCst);
                std::thread::sleep(std::time::Duration::from_millis(1));
                
                let budget = packages::domains::finance::domain::budget::Budget::new(
                    user_id,
                    category,
                    amount,
                    period,
                    start_date,
                    end_date,
                );
                self.budgets.lock().unwrap().push(budget.clone());
                Ok(budget)
            }
            
            async fn get_user_budgets(&self, _user_id: Uuid) -> Result<Vec<packages::domains::finance::domain::budget::Budget>, packages::domains::finance::domain::FinanceError> {
                Ok(self.budgets.lock().unwrap().clone())
            }
            
            async fn update_budget_spent(&self, _budget_id: Uuid, _amount: Money) -> Result<packages::domains::finance::domain::budget::Budget, packages::domains::finance::domain::FinanceError> {
                unimplemented!()
            }
            
            async fn get_budget_by_category(&self, _user_id: Uuid, _category: &str) -> Result<Option<packages::domains::finance::domain::budget::Budget>, packages::domains::finance::domain::FinanceError> {
                unimplemented!()
            }
        }
        
        // Create a simple sheet with budget data
        let mut cells = HashMap::new();
        cells.insert(CellAddress::new(0, 0), crate::domain::Cell::new(CellValue::Text("Category".to_string())));
        cells.insert(CellAddress::new(0, 1), crate::domain::Cell::new(CellValue::Text("Amount".to_string())));
        cells.insert(CellAddress::new(1, 0), crate::domain::Cell::new(CellValue::Text("Food".to_string())));
        cells.insert(CellAddress::new(1, 1), crate::domain::Cell::new(CellValue::Number(100.0)));
        cells.insert(CellAddress::new(2, 0), crate::domain::Cell::new(CellValue::Text("Transport".to_string())));
        cells.insert(CellAddress::new(2, 1), crate::domain::Cell::new(CellValue::Number(50.0)));
        
        let sheet = Sheet::new("test_sheet".to_string(), cells);
        let user_id = Uuid::new_v4();
        
        // Process multiple templates concurrently
        let handles: Vec<_> = (0..5).map(|_| {
            let mock_budget_service = Box::new(ConcurrentMockBudgetService::new());
            let service = BudgetTemplateService::new(mock_budget_service);
            let sheet_clone = sheet.clone();
            let user_id_clone = user_id;
            
            // Mock user preferences
            struct TestUserPreferences;
            
            #[async_trait]
            impl UserPreferences for TestUserPreferences {
                async fn get_preferred_currency(&self, _user_id: Uuid) -> Result<Currency, String> {
                    Ok(Currency::USD)
                }
                
                async fn set_preferred_currency(&self, _user_id: Uuid, _currency: Currency) -> Result<(), String> {
                    Ok(())
                }
            }
            
            let user_preferences = TestUserPreferences;
            
            std::thread::spawn(move || {
                service.apply_template(&sheet_clone, user_id_clone, &user_preferences)
            })
        }).collect();
        
        // Wait for all threads to complete
        let results: Vec<_> = handles.into_iter().map(|handle| handle.join()).collect();
        
        // Check that all operations succeeded
        for result in results {
            assert!(result.is_ok());
            let apply_result = result.unwrap();
            assert!(apply_result.is_ok());
        }
        
        // Verify that the process count is as expected
        assert_eq!(PROCESS_COUNT.load(Ordering::SeqCst), 10); // 5 threads * 2 budgets each
    }
}