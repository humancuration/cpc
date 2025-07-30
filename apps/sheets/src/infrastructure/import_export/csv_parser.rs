//! CSV parser for importing spreadsheet data

use crate::domain::{Sheet, Cell, CellAddress, CellValue};
use uuid::Uuid;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// CSV parser for importing spreadsheet files
pub struct CsvParser;

impl CsvParser {
    pub fn new() -> Self {
        Self
    }
    
    /// Import a CSV file
    pub fn import(file_path: &str, owner: Uuid) -> Result<Sheet, Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut cells = std::collections::HashMap::new();
        
        for (row_idx, line) in reader.lines().enumerate() {
            let line = line?;
            let fields: Vec<&str> = line.split(',').collect();
            
            for (col_idx, field) in fields.iter().enumerate() {
                let address = CellAddress::new(row_idx as u32, col_idx as u32);
                
                // Try to parse as number first, then as boolean, otherwise as text
                let cell_value = if let Ok(number) = field.parse::<f64>() {
                    CellValue::Number(number)
                } else if field.to_lowercase() == "true" {
                    CellValue::Boolean(true)
                } else if field.to_lowercase() == "false" {
                    CellValue::Boolean(false)
                } else {
                    CellValue::Text(field.to_string())
                };
                
                let sheet_cell = Cell::new(address, cell_value);
                cells.insert(address, sheet_cell);
            }
        }
        
        let mut sheet = Sheet::new("Imported CSV".to_string(), owner);
        sheet.cells = cells;
        
        Ok(sheet)
    }
    
    /// Export a Sheet to CSV format
    pub fn export(sheet: &Sheet, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // In a real implementation, this would export to CSV format
        // For now, we'll just create a placeholder
        println!("Exporting sheet {} to {}", sheet.name, file_path);
        Ok(())
    }
}