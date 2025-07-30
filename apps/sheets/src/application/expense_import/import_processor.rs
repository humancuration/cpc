use crate::domain::{Sheet, CellAddress, CellValue};
use crate::application::expense_import::column_mapping::{ColumnMapping, ImportResult, FailedRow};
use packages::domains::finance::application::ExpenseService;
use packages::domains::finance::domain::expense::{Expense, ExpenseCategory};
use packages::domains::finance::domain::primitives::Money;
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};
use serde_json::Value;

/// Processor for importing expenses from sheets
pub struct ExpenseImportProcessor {
    expense_service: Box<dyn ExpenseService>,
}

impl ExpenseImportProcessor {
    pub fn new(expense_service: Box<dyn ExpenseService>) -> Self {
        Self { expense_service }
    }
    
    pub async fn process(&self, sheet: &Sheet, mapping: ColumnMapping, user_id: Uuid, user_preferences: &dyn packages::domains::finance::application::user_preferences::UserPreferences) -> Result<ImportResult, String> {
        let mut result = ImportResult {
            total_rows: 0,
            successful_imports: 0,
            failed_rows: Vec::new(),
            errors: Vec::new(),
        };
        
        let max_row = self.get_max_row(sheet);
        if max_row < 1 {
            return Ok(result);
        }
        
        // Skip header row (row 0)
        // Get user's preferred currency at the beginning of the process
        let currency = user_preferences.get_preferred_currency(user_id)
            .await
            .map_err(|e| format!("Failed to get user currency: {}", e))?;
        
        for row in 1..=max_row {
            result.total_rows += 1;
            
            match self.process_row(sheet, &mapping, row, user_id, &currency).await {
                Ok(_) => result.successful_imports += 1,
                Err(e) => {
                    // Collect row data for error reporting
                    let row_data = self.collect_row_data(sheet, row);
                    result.failed_rows.push(FailedRow {
                        row_number: row,
                        error: e,
                        data: row_data,
                    });
                }
            }
        }
        
        Ok(result)
    }
    
    async fn process_row(&self, sheet: &Sheet, mapping: &ColumnMapping, row: u32, user_id: Uuid, currency: &packages::domains::finance::domain::primitives::Currency) -> Result<(), String> {
        // Extract values from mapped columns
        let date = self.get_cell_as_date(sheet, &mapping.date_column, row)?;
        let amount = self.get_cell_as_number(sheet, &mapping.amount_column, row)?;
        let category_str = self.get_cell_as_string(sheet, &mapping.category_column, row)?;
        let description = mapping.description_column
            .as_ref()
            .map(|col| self.get_cell_as_string(sheet, col, row))
            .transpose()?
            .unwrap_or_default();
        
        // Convert category string to ExpenseCategory
        let category = self.parse_category(&category_str);
        
        // Create expense
        let expense = Expense::new(
            user_id,
            Money::new(amount, currency.clone()),
            category,
            description,
            date,
        );
        
        // Save expense
        self.expense_service.add_expense(expense)
            .await
            .map_err(|e| format!("Failed to save expense: {}", e))?;
        
        Ok(())
    }
    
    fn get_cell_as_date(&self, sheet: &Sheet, column: &str, row: u32) -> Result<DateTime<Utc>, String> {
        let address = self.parse_cell_reference(column, row)?;
        let cell = sheet.get_cell(&address)
            .ok_or_else(|| format!("Cell {}{} not found", column, row + 1))?;
        
        match &cell.value {
            CellValue::Text(s) => {
                // Try to parse as date
                // Try common date formats
                let formats = ["%Y-%m-%d", "%m/%d/%Y", "%d/%m/%Y", "%Y/%m/%d"];
                for format in &formats {
                    if let Ok(naive_date) = NaiveDate::parse_from_str(s, format) {
                        let datetime = naive_date.and_hms_opt(0, 0, 0)
                            .ok_or("Invalid time")?;
                        return Ok(DateTime::from_naive_utc_and_offset(datetime, Utc));
                    }
                }
                Err(format!("Cannot parse '{}' as date", s))
            }
            CellValue::DateTime(dt) => Ok(*dt),
            CellValue::Number(n) => {
                // Treat as Unix timestamp
                let timestamp = *n as i64;
                Ok(DateTime::from_timestamp(timestamp, 0)
                    .ok_or("Invalid timestamp")?
                    .with_timezone(&Utc))
            }
            _ => Err(format!("Cell {}{} does not contain a date", column, row + 1))
        }
    }
    
    fn get_cell_as_number(&self, sheet: &Sheet, column: &str, row: u32) -> Result<f64, String> {
        let address = self.parse_cell_reference(column, row)?;
        let cell = sheet.get_cell(&address)
            .ok_or_else(|| format!("Cell {}{} not found", column, row + 1))?;
        
        match &cell.value {
            CellValue::Number(n) => Ok(*n),
            CellValue::Text(s) => {
                // Try to parse as number
                s.parse::<f64>().map_err(|_| format!("Cannot parse '{}' as number", s))
            }
            _ => Err(format!("Cell {}{} does not contain a number", column, row + 1))
        }
    }
    
    fn get_cell_as_string(&self, sheet: &Sheet, column: &str, row: u32) -> Result<String, String> {
        let address = self.parse_cell_reference(column, row)?;
        let cell = sheet.get_cell(&address)
            .ok_or_else(|| format!("Cell {}{} not found", column, row + 1))?;
        
        match &cell.value {
            CellValue::Text(s) => Ok(s.clone()),
            CellValue::Number(n) => Ok(n.to_string()),
            CellValue::Boolean(b) => Ok(b.to_string()),
            _ => Ok("".to_string())
        }
    }
    
    fn parse_cell_reference(&self, column: &str, row: u32) -> Result<CellAddress, String> {
        // Convert column letters to number (A=0, B=1, ..., Z=25, AA=26, etc.)
        let mut column_num = 0u32;
        for &letter in column.chars().collect::<Vec<_>>().iter() {
            if letter.is_ascii_alphabetic() {
                column_num = column_num * 26 + (letter.to_ascii_uppercase() as u32 - 'A' as u32);
            } else {
                return Err(format!("Invalid column letter: {}", letter));
            }
        }
        
        Ok(CellAddress::new(row, column_num))
    }
    
    fn parse_category(&self, category_str: &str) -> ExpenseCategory {
        let lower = category_str.to_lowercase();
        match lower.as_str() {
            "food" | "dining" | "restaurant" | "groceries" => ExpenseCategory::Food,
            "housing" | "rent" | "mortgage" | "utilities" => ExpenseCategory::Housing,
            "transportation" | "gas" | "car" | "public transit" => ExpenseCategory::Transportation,
            "entertainment" | "movies" | "games" | "hobbies" => ExpenseCategory::Entertainment,
            "healthcare" | "medical" | "doctor" | "pharmacy" => ExpenseCategory::Healthcare,
            "shopping" | "clothing" | "electronics" => ExpenseCategory::Shopping,
            "travel" | "vacation" | "flight" | "hotel" => ExpenseCategory::Travel,
            "education" | "school" | "books" | "tuition" => ExpenseCategory::Education,
            "personal care" | "beauty" | "gym" => ExpenseCategory::PersonalCare,
            "gifts" | "donations" => ExpenseCategory::Gifts,
            "business" | "office" | "supplies" => ExpenseCategory::Business,
            "other" | _ => ExpenseCategory::Other,
        }
    }
    
    /// Get the maximum row index with data
    fn get_max_row(&self, sheet: &Sheet) -> u32 {
        sheet.cells.keys().map(|addr| addr.row).max().unwrap_or(0)
    }
    
    /// Collect row data for error reporting
    fn collect_row_data(&self, sheet: &Sheet, row: u32) -> Value {
        let mut data = serde_json::Map::new();
        
        // Get all columns with data in this row
        for (address, cell) in &sheet.cells {
            if address.row == row {
                let column = self.column_number_to_letters(address.column);
                let value = match &cell.value {
                    CellValue::Text(s) => Value::String(s.clone()),
                    CellValue::Number(n) => Value::Number(serde_json::Number::from_f64(*n).unwrap_or(serde_json::Number::from(0))),
                    CellValue::Boolean(b) => Value::Bool(*b),
                    CellValue::Empty => Value::Null,
                    CellValue::Error(e) => Value::String(format!("Error: {}", e)),
                    CellValue::DateTime(dt) => Value::String(dt.to_rfc3339()),
                };
                data.insert(column, value);
            }
        }
        
        Value::Object(data)
    }
    
    /// Convert column number to letters (0 -> A, 1 -> B, ..., 25 -> Z, 26 -> AA, etc.)
    fn column_number_to_letters(&self, column: u32) -> String {
        let mut result = String::new();
        let mut col = column;
        
        loop {
            let remainder = col % 26;
            result.insert(0, (b'A' + remainder as u8) as char);
            col = col / 26;
            if col == 0 {
                break;
            }
            col -= 1;
        }
        
        result
    }
}