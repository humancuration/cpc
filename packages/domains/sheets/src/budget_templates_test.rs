//! Tests for the budget template service

#[cfg(test)]
mod tests {
    use crate::{
        domain::{Sheet, CellAddress, CellValue},
        application::budget_templates::{BudgetTemplateService, TemplateType},
    };
    use uuid::Uuid;
    
    // Mock BudgetService for testing
    struct MockBudgetService;
    
    #[async_trait::async_trait]
    impl crate::application::budget_templates::BudgetService for MockBudgetService {
        async fn create_budget(&self, _user_id: Uuid, _category: String, _allocated_amount: crate::application::budget_templates::Money, _period: crate::application::budget_templates::BudgetPeriod, _start_date: chrono::DateTime<chrono::Utc>, _end_date: chrono::DateTime<chrono::Utc>) -> Result<crate::application::budget_templates::Budget, crate::application::budget_templates::FinanceError> {
            // In a real implementation, this would create a budget
            Err(crate::application::budget_templates::FinanceError::NotImplemented("Mock service".to_string()))
        }
        
        async fn create_mixed_budget(&self, _user_id: Uuid, _category: String, _primary_amount: crate::application::budget_templates::Money, _dabloons_amount: crate::application::budget_templates::Money, _period: crate::application::budget_templates::BudgetPeriod, _start_date: chrono::DateTime<chrono::Utc>, _end_date: chrono::DateTime<chrono::Utc>) -> Result<crate::application::budget_templates::Budget, crate::application::budget_templates::FinanceError> {
            // In a real implementation, this would create a mixed budget
            Err(crate::application::budget_templates::FinanceError::NotImplemented("Mock service".to_string()))
        }
        
        async fn get_user_budgets(&self, _user_id: Uuid) -> Result<Vec<crate::application::budget_templates::Budget>, crate::application::budget_templates::FinanceError> {
            Ok(Vec::new())
        }
        
        async fn get_budget_by_category(&self, _user_id: Uuid, _category: &str) -> Result<Option<crate::application::budget_templates::Budget>, crate::application::budget_templates::FinanceError> {
            Ok(None)
        }
        
        async fn update_spent_amount(&self, _user_id: Uuid, _category: &str, _amount: crate::application::budget_templates::Money) -> Result<crate::application::budget_templates::Budget, crate::application::budget_templates::FinanceError> {
            Err(crate::application::budget_templates::FinanceError::NotImplemented("Mock service".to_string()))
        }
        
        async fn update_spent_with_dabloons(&self, _user_id: Uuid, _category: &str, _amount: crate::application::budget_templates::Money) -> Result<crate::application::budget_templates::Budget, crate::application::budget_templates::FinanceError> {
            Err(crate::application::budget_templates::FinanceError::NotImplemented("Mock service".to_string()))
        }
        
        async fn reset_monthly_budgets(&self, _user_id: Uuid) -> Result<(), crate::application::budget_templates::FinanceError> {
            Ok(())
        }
        
        async fn get_monthly_ubi_income(&self, _user_id: Uuid) -> Result<crate::application::budget_templates::Money, crate::application::budget_templates::FinanceError> {
            Ok(crate::application::budget_templates::Money::zero(crate::application::budget_templates::Currency::USD))
        }
    }
    
    #[test]
    fn test_template_identification() {
        let budget_service = Box::new(MockBudgetService);
        let template_service = BudgetTemplateService::new(budget_service);
        
        // Create a sheet that looks like a budget template
        let mut sheet = Sheet::new("Budget Template".to_string(), Uuid::new_v4());
        
        // Add header row
        let category_header = CellAddress::new(0, 0); // A1
        let amount_header = CellAddress::new(0, 1);   // B1
        
        sheet.update_cell(category_header, crate::domain::Cell::new(category_header, CellValue::Text("Category".to_string())));
        sheet.update_cell(amount_header, crate::domain::Cell::new(amount_header, CellValue::Text("Amount".to_string())));
        
        // Add some budget data
        let rent_category = CellAddress::new(1, 0); // A2
        let rent_amount = CellAddress::new(1, 1);   // B2
        
        sheet.update_cell(rent_category, crate::domain::Cell::new(rent_category, CellValue::Text("Rent".to_string())));
        sheet.update_cell(rent_amount, crate::domain::Cell::new(rent_amount, CellValue::Number(1000.0)));
        
        let food_category = CellAddress::new(2, 0); // A3
        let food_amount = CellAddress::new(2, 1);   // B3
        
        sheet.update_cell(food_category, crate::domain::Cell::new(food_category, CellValue::Text("Food".to_string())));
        sheet.update_cell(food_amount, crate::domain::Cell::new(food_amount, CellValue::Number(300.0)));
        
        // Test template identification
        let result = template_service.identify_template(&sheet);
        assert!(result.is_ok());
        
        let identification = result.unwrap();
        assert_eq!(identification.template_type, TemplateType::MonthlyBudget);
        assert!(identification.confidence > 0.5);
    }
    
    #[test]
    fn test_template_identification_empty_sheet() {
        let budget_service = Box::new(MockBudgetService);
        let template_service = BudgetTemplateService::new(budget_service);
        
        // Create an empty sheet
        let sheet = Sheet::new("Empty Sheet".to_string(), Uuid::new_v4());
        
        // Test template identification
        let result = template_service.identify_template(&sheet);
        assert!(result.is_ok());
        
        let identification = result.unwrap();
        assert_eq!(identification.template_type, TemplateType::Custom);
    }
}