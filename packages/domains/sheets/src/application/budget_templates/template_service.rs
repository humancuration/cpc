use crate::domain::Sheet;
use packages::domains::finance::application::BudgetService;
use packages::domains::finance::domain::{
    budget::{Budget, BudgetPeriod},
    primitives::Money,
};
use uuid::Uuid;
use chrono::Utc;
use crate::application::budget_templates::template_models::{TemplateType, TemplateIdentification};

/// Service for applying budget templates to sheets
pub struct BudgetTemplateService {
    budget_service: Box<dyn BudgetService>,
}

impl BudgetTemplateService {
    pub fn new(budget_service: Box<dyn BudgetService>) -> Self {
        Self { budget_service }
    }
    
    pub fn apply_template(&self, sheet: &Sheet, user_id: Uuid) -> Result<(), String> {
        // Identify template type from metadata
        let template_type = self.identify_template(sheet)?;
        
        match template_type.template_type {
            TemplateType::MonthlyBudget => self.process_monthly_budget_template(sheet, user_id),
            TemplateType::WeeklyBudget => self.process_weekly_budget_template(sheet, user_id),
            TemplateType::ProjectBudget => self.process_project_budget_template(sheet, user_id),
            TemplateType::Custom => self.process_custom_template(sheet, user_id),
        }
    }
    
    fn identify_template(&self, sheet: &Sheet) -> Result<TemplateIdentification, String> {
        // Check for template metadata in sheet properties
        // For now, we'll use a simple heuristic based on sheet structure
        let max_row = self.get_max_row(sheet);
        let max_col = self.get_max_column(sheet);
        
        // Check if it looks like a monthly budget template
        // Heuristic: At least 3 rows, at least 2 columns, with typical budget headers
        if max_row >= 3 && max_col >= 2 {
            // Check for common budget headers in first row
            let headers = self.get_row_values(sheet, 0, max_col);
            let budget_indicators = ["category", "budget", "allocated", "amount", "expense"];
            
            let matches = headers.iter().filter(|header| {
                let lower = header.to_lowercase();
                budget_indicators.iter().any(|indicator| lower.contains(indicator))
            }).count();
            
            if matches >= 2 {
                return Ok(TemplateIdentification {
                    template_type: TemplateType::MonthlyBudget,
                    confidence: matches as f64 / budget_indicators.len() as f64,
                });
            }
        }
        
        // Default to custom template
        Ok(TemplateIdentification {
            template_type: TemplateType::Custom,
            confidence: 0.5,
        })
    }
    
    fn process_monthly_budget_template(&self, sheet: &Sheet, user_id: Uuid) -> Result<(), String> {
        // Standard template structure:
        // Row 0: Headers (Category, Allocated Amount, Period)
        // Row 1+: Budget entries
        
        let max_row = self.get_max_row(sheet);
        if max_row < 1 {
            return Err("Sheet is empty".to_string());
        }
        
        // Find column indices for category and amount
        let headers = self.get_row_values(sheet, 0, self.get_max_column(sheet));
        let category_col = self.find_column_index(&headers, &["category", "item", "description"]);
        let amount_col = self.find_column_index(&headers, &["amount", "allocated", "budget"]);
        
        if category_col.is_none() || amount_col.is_none() {
            return Err("Could not find required columns (category, amount)".to_string());
        }
        
        let category_col = category_col.unwrap();
        let amount_col = amount_col.unwrap();
        
        // Process each row
        for row in 1..=max_row {
            let category_cell = sheet.get_cell(&crate::domain::CellAddress::new(row, category_col));
            let amount_cell = sheet.get_cell(&crate::domain::CellAddress::new(row, amount_col));
            
            if let (Some(category_cell), Some(amount_cell)) = (category_cell, amount_cell) {
                let category = match &category_cell.value {
                    crate::domain::CellValue::Text(s) => s.clone(),
                    crate::domain::CellValue::Number(n) => n.to_string(),
                    _ => continue, // Skip rows with invalid category
                };
                
                let amount = match &amount_cell.value {
                    crate::domain::CellValue::Number(n) => *n,
                    crate::domain::CellValue::Text(s) => {
                        s.parse::<f64>().map_err(|_| format!("Invalid amount in row {}: {}", row + 1, s))?
                    },
                    _ => continue, // Skip rows with invalid amount
                };
                
                // Skip empty categories
                if category.trim().is_empty() {
                    continue;
                }
                
                // Create budget entry
                let _budget = self.budget_service.create_budget(
                    user_id,
                    category,
                    Money::new(amount, "USD"),
                    BudgetPeriod::Monthly,
                    Utc::now(),
                    Utc::now() + chrono::Duration::days(30),
                ).map_err(|e| format!("Failed to create budget: {}", e))?;
            }
        }
        
        Ok(())
    }
    
    fn process_weekly_budget_template(&self, sheet: &Sheet, user_id: Uuid) -> Result<(), String> {
        // Similar to monthly but with weekly period
        let max_row = self.get_max_row(sheet);
        if max_row < 1 {
            return Err("Sheet is empty".to_string());
        }
        
        // Find column indices for category and amount
        let headers = self.get_row_values(sheet, 0, self.get_max_column(sheet));
        let category_col = self.find_column_index(&headers, &["category", "item", "description"]);
        let amount_col = self.find_column_index(&headers, &["amount", "allocated", "budget"]);
        
        if category_col.is_none() || amount_col.is_none() {
            return Err("Could not find required columns (category, amount)".to_string());
        }
        
        let category_col = category_col.unwrap();
        let amount_col = amount_col.unwrap();
        
        // Process each row
        for row in 1..=max_row {
            let category_cell = sheet.get_cell(&crate::domain::CellAddress::new(row, category_col));
            let amount_cell = sheet.get_cell(&crate::domain::CellAddress::new(row, amount_col));
            
            if let (Some(category_cell), Some(amount_cell)) = (category_cell, amount_cell) {
                let category = match &category_cell.value {
                    crate::domain::CellValue::Text(s) => s.clone(),
                    crate::domain::CellValue::Number(n) => n.to_string(),
                    _ => continue, // Skip rows with invalid category
                };
                
                let amount = match &amount_cell.value {
                    crate::domain::CellValue::Number(n) => *n,
                    crate::domain::CellValue::Text(s) => {
                        s.parse::<f64>().map_err(|_| format!("Invalid amount in row {}: {}", row + 1, s))?
                    },
                    _ => continue, // Skip rows with invalid amount
                };
                
                // Skip empty categories
                if category.trim().is_empty() {
                    continue;
                }
                
                // Create budget entry
                let _budget = self.budget_service.create_budget(
                    user_id,
                    category,
                    Money::new(amount, "USD"),
                    BudgetPeriod::Weekly,
                    Utc::now(),
                    Utc::now() + chrono::Duration::days(7),
                ).map_err(|e| format!("Failed to create budget: {}", e))?;
            }
        }
        
        Ok(())
    }
    
    fn process_project_budget_template(&self, sheet: &Sheet, user_id: Uuid) -> Result<(), String> {
        // Project budget template processing
        // This would be similar to monthly but with project-specific logic
        self.process_custom_template(sheet, user_id)
    }
    
    fn process_custom_template(&self, sheet: &Sheet, user_id: Uuid) -> Result<(), String> {
        // Custom template processing
        // For now, we'll treat it as a monthly budget
        self.process_monthly_budget_template(sheet, user_id)
    }
    
    /// Get the maximum row index with data
    fn get_max_row(&self, sheet: &Sheet) -> u32 {
        sheet.cells.keys().map(|addr| addr.row).max().unwrap_or(0)
    }
    
    /// Get the maximum column index with data
    fn get_max_column(&self, sheet: &Sheet) -> u32 {
        sheet.cells.keys().map(|addr| addr.column).max().unwrap_or(0)
    }
    
    /// Get values from a specific row
    fn get_row_values(&self, sheet: &Sheet, row: u32, max_col: u32) -> Vec<String> {
        let mut values = Vec::new();
        for col in 0..=max_col {
            let address = crate::domain::CellAddress::new(row, col);
            if let Some(cell) = sheet.get_cell(&address) {
                match &cell.value {
                    crate::domain::CellValue::Text(s) => values.push(s.clone()),
                    crate::domain::CellValue::Number(n) => values.push(n.to_string()),
                    crate::domain::CellValue::Boolean(b) => values.push(b.to_string()),
                    _ => values.push("".to_string()),
                }
            } else {
                values.push("".to_string());
            }
        }
        values
    }
    
    /// Find column index by matching header names
    fn find_column_index(&self, headers: &[String], keywords: &[&str]) -> Option<u32> {
        headers.iter().position(|header| {
            let lower = header.to_lowercase();
            keywords.iter().any(|keyword| lower.contains(keyword))
        }).map(|i| i as u32)
    }
}