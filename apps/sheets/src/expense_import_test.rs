//! Tests for the expense import functionality

#[cfg(test)]
mod tests {
    use crate::{
        domain::{Sheet, CellAddress, CellValue},
        application::expense_import::{ColumnMapping, ExpenseImportProcessor},
    };
    use uuid::Uuid;
    
    // Mock ExpenseService for testing
    struct MockExpenseService;
    
    #[async_trait::async_trait]
    impl crate::application::expense_import::ExpenseService for MockExpenseService {
        async fn add_expense(&self, _expense: crate::application::expense_import::Expense) -> Result<crate::application::expense_import::Expense, crate::application::expense_import::FinanceError> {
            // In a real implementation, this would add an expense
            Err(crate::application::expense_import::FinanceError::NotImplemented("Mock service".to_string()))
        }
        
        async fn get_expenses(&self, _user_id: Uuid, _start_date: chrono::DateTime<chrono::Utc>, _end_date: chrono::DateTime<chrono::Utc>) -> Result<Vec<crate::application::expense_import::Expense>, crate::application::expense_import::FinanceError> {
            Ok(Vec::new())
        }
        
        async fn get_expenses_by_category(&self, _user_id: Uuid, _category: crate::application::expense_import::ExpenseCategory) -> Result<Vec<crate::application::expense_import::Expense>, crate::application::expense_import::FinanceError> {
            Ok(Vec::new())
        }
        
        async fn update_expense(&self, _expense_id: Uuid, _amount: crate::application::expense_import::Money, _category: crate::application::expense_import::ExpenseCategory, _description: String, _date: chrono::DateTime<chrono::Utc>) -> Result<crate::application::expense_import::Expense, crate::application::expense_import::FinanceError> {
            Err(crate::application::expense_import::FinanceError::NotImplemented("Mock service".to_string()))
        }
        
        async fn delete_expense(&self, _expense_id: Uuid) -> Result<(), crate::application::expense_import::FinanceError> {
            Ok(())
        }
        
        async fn get_monthly_summary(&self, _user_id: Uuid, _year: i32, _month: u32) -> Result<std::collections::HashMap<crate::application::expense_import::ExpenseCategory, crate::application::expense_import::Money>, crate::application::expense_import::FinanceError> {
            Ok(std::collections::HashMap::new())
        }
    }
    
    #[test]
    fn test_column_mapping_creation() {
        let mapping = ColumnMapping::new(
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            Some("D".to_string()),
            Some("E".to_string()),
            Some("F".to_string()),
        );
        
        assert_eq!(mapping.date_column, "A");
        assert_eq!(mapping.amount_column, "B");
        assert_eq!(mapping.category_column, "C");
        assert_eq!(mapping.description_column, Some("D".to_string()));
        assert_eq!(mapping.vendor_column, Some("E".to_string()));
        assert_eq!(mapping.account_column, Some("F".to_string()));
    }
    
    #[test]
    fn test_import_processor_creation() {
        let expense_service = Box::new(MockExpenseService);
        let processor = ExpenseImportProcessor::new(expense_service);
        
        // Just test that it was created successfully
        assert!(true);
    }
    
    #[test]
    fn test_cell_reference_parsing() {
        let expense_service = Box::new(MockExpenseService);
        let processor = ExpenseImportProcessor::new(expense_service);
        
        // Test simple column parsing
        let result = processor.parse_cell_reference("A", 0);
        assert!(result.is_ok());
        let address = result.unwrap();
        assert_eq!(address.row, 0);
        assert_eq!(address.column, 0);
        
        // Test multi-letter column parsing
        let result = processor.parse_cell_reference("AA", 5);
        assert!(result.is_ok());
        let address = result.unwrap();
        assert_eq!(address.row, 5);
        assert_eq!(address.column, 26);
    }
    
    #[test]
    fn test_category_parsing() {
        let expense_service = Box::new(MockExpenseService);
        let processor = ExpenseImportProcessor::new(expense_service);
        
        // Test food categories
        assert_eq!(processor.parse_category("Food"), crate::application::expense_import::ExpenseCategory::Food);
        assert_eq!(processor.parse_category("Groceries"), crate::application::expense_import::ExpenseCategory::Food);
        assert_eq!(processor.parse_category("Restaurant"), crate::application::expense_import::ExpenseCategory::Food);
        
        // Test housing categories
        assert_eq!(processor.parse_category("Rent"), crate::application::expense_import::ExpenseCategory::Housing);
        assert_eq!(processor.parse_category("Mortgage"), crate::application::expense_import::ExpenseCategory::Housing);
        assert_eq!(processor.parse_category("Utilities"), crate::application::expense_import::ExpenseCategory::Housing);
        
        // Test default category
        assert_eq!(processor.parse_category("Unknown"), crate::application::expense_import::ExpenseCategory::Other);
    }
    
    #[test]
    fn test_column_number_to_letters() {
        let expense_service = Box::new(MockExpenseService);
        let processor = ExpenseImportProcessor::new(expense_service);
        
        // Test simple conversions
        assert_eq!(processor.column_number_to_letters(0), "A");
        assert_eq!(processor.column_number_to_letters(1), "B");
        assert_eq!(processor.column_number_to_letters(25), "Z");
        
        // Test multi-letter conversions
        assert_eq!(processor.column_number_to_letters(26), "AA");
        assert_eq!(processor.column_number_to_letters(27), "AB");
        assert_eq!(processor.column_number_to_letters(51), "AZ");
        assert_eq!(processor.column_number_to_letters(52), "BA");
    }
}