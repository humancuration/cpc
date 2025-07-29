//! Tests for the formula evaluator financial functions

#[cfg(test)]
mod tests {
    use crate::{
        domain::{Sheet, CellAddress, CellValue, Formula},
        application::formula_evaluator::FormulaEvaluator,
    };
    use uuid::Uuid;
    use rust_decimal_macros::dec;
    
    #[test]
    fn test_pmt_function() {
        let evaluator = FormulaEvaluator::new();
        let sheet = Sheet::new("Test Sheet".to_string(), Uuid::new_v4());
        
        // Test basic PMT calculation
        // PMT(0.05/12, 60, 10000) should be approximately -188.71
        let result = evaluator.evaluate_expression("=PMT(0.05/12, 60, 10000)", &sheet);
        assert!(result.is_ok());
        
        let value = result.unwrap();
        if let CellValue::Number(num) = value {
            // Check that the result is approximately -188.71
            assert!(num > -189.0 && num < -188.5, "Expected approximately -188.71, got {}", num);
        } else {
            panic!("Expected a number, got {:?}", value);
        }
    }
    
    #[test]
    fn test_pmt_function_with_fv_and_type() {
        let evaluator = FormulaEvaluator::new();
        let sheet = Sheet::new("Test Sheet".to_string(), Uuid::new_v4());
        
        // Test PMT with future value and payment type
        // PMT(0.08, 10, 0, 10000, 1) should be approximately -653.06
        let result = evaluator.evaluate_expression("=PMT(0.08, 10, 0, 10000, 1)", &sheet);
        assert!(result.is_ok());
        
        let value = result.unwrap();
        if let CellValue::Number(num) = value {
            // Check that the result is approximately -653.06
            assert!(num > -654.0 && num < -652.0, "Expected approximately -653.06, got {}", num);
        } else {
            panic!("Expected a number, got {:?}", value);
        }
    }
    
    #[test]
    fn test_fv_function() {
        let evaluator = FormulaEvaluator::new();
        let sheet = Sheet::new("Test Sheet".to_string(), Uuid::new_v4());
        
        // Test basic FV calculation
        // FV(0.06/12, 120, -200) should be approximately 32772.68
        let result = evaluator.evaluate_expression("=FV(0.06/12, 120, -200)", &sheet);
        assert!(result.is_ok());
        
        let value = result.unwrap();
        if let CellValue::Number(num) = value {
            // Check that the result is approximately 32772.68
            assert!(num > 32000.0 && num < 33000.0, "Expected approximately 32772.68, got {}", num);
        } else {
            panic!("Expected a number, got {:?}", value);
        }
    }
    
    #[test]
    fn test_npv_function() {
        let evaluator = FormulaEvaluator::new();
        let sheet = Sheet::new("Test Sheet".to_string(), Uuid::new_v4());
        
        // Test basic NPV calculation
        // NPV(0.1, -10000, 3000, 4200, 6800) should be approximately 1428.25
        let result = evaluator.evaluate_expression("=NPV(0.1, -10000, 3000, 4200, 6800)", &sheet);
        assert!(result.is_ok());
        
        let value = result.unwrap();
        if let CellValue::Number(num) = value {
            // Check that the result is approximately 1428.25
            assert!(num > 1400.0 && num < 1500.0, "Expected approximately 1428.25, got {}", num);
        } else {
            panic!("Expected a number, got {:?}", value);
        }
    }
    
    #[test]
    fn test_irr_function() {
        let evaluator = FormulaEvaluator::new();
        let sheet = Sheet::new("Test Sheet".to_string(), Uuid::new_v4());
        
        // Test basic IRR calculation with a simple cash flow
        // IRR([-100, 20, 30, 40, 50]) should be approximately 0.1284 (12.84%)
        let result = evaluator.evaluate_expression("=IRR(-100, 20, 30, 40, 50)", &sheet);
        assert!(result.is_ok());
        
        let value = result.unwrap();
        if let CellValue::Number(num) = value {
            // Check that the result is approximately 0.1284
            assert!(num > 0.1 && num < 0.2, "Expected approximately 0.1284, got {}", num);
        } else {
            panic!("Expected a number, got {:?}", value);
        }
    }
    
    #[test]
    fn test_pmt_invalid_arguments() {
        let evaluator = FormulaEvaluator::new();
        let sheet = Sheet::new("Test Sheet".to_string(), Uuid::new_v4());
        
        // Test PMT with too few arguments
        let result = evaluator.evaluate_expression("=PMT(0.05/12, 60)", &sheet);
        assert!(result.is_err());
        
        // Test PMT with too many arguments
        let result = evaluator.evaluate_expression("=PMT(0.05/12, 60, 10000, 0, 0, 0)", &sheet);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_financial_functions_with_cell_references() {
        let evaluator = FormulaEvaluator::new();
        let mut sheet = Sheet::new("Test Sheet".to_string(), Uuid::new_v4());
        
        // Add some test data to the sheet
        let rate_cell = CellAddress::new(0, 0); // A1
        let nper_cell = CellAddress::new(0, 1); // B1
        let pv_cell = CellAddress::new(0, 2);   // C1
        
        sheet.update_cell(rate_cell, crate::domain::Cell::new(rate_cell, CellValue::Number(0.05/12.0)));
        sheet.update_cell(nper_cell, crate::domain::Cell::new(nper_cell, CellValue::Number(60.0)));
        sheet.update_cell(pv_cell, crate::domain::Cell::new(pv_cell, CellValue::Number(10000.0)));
        
        // Test PMT with cell references
        let result = evaluator.evaluate_expression("=PMT(A1, B1, C1)", &sheet);
        assert!(result.is_ok());
        
        let value = result.unwrap();
        if let CellValue::Number(num) = value {
            // Check that the result is approximately -188.71
            assert!(num > -189.0 && num < -188.5, "Expected approximately -188.71, got {}", num);
        } else {
            panic!("Expected a number, got {:?}", value);
        }
    }
}