//! XLSX parser for importing spreadsheet data

use calamine::{open_workbook_auto, DataType, Range};
use crate::domain::{Sheet, Cell, CellAddress, CellValue};
use crate::infrastructure::compliance_handler::ComplianceHandler;
use uuid::Uuid;

/// XLSX parser for importing spreadsheet files
pub struct XlsxParser;

impl XlsxParser {
    pub fn new() -> Self {
        Self
    }
    /// Import an XLSX file
    pub fn import(file_path: &str, owner: Uuid, source_region: Option<String>) -> Result<Sheet, Box<dyn std::error::Error>> {
        let mut workbook = open_workbook_auto(file_path)?;
        let sheet_names = workbook.sheet_names();
        
        if sheet_names.is_empty() {
            return Err("No sheets found in workbook".into());
        }
        
        let range = workbook.worksheet_range(sheet_names[0].as_str())?;
        let mut sheet = self.convert_range_to_sheet(range, owner);
        
        // Apply compliance rules
        let compliance_handler = ComplianceHandler::new();
        if let Some(region) = source_region {
            compliance_handler.apply_data_sovereignty_rules(&mut sheet, region);
        }
        compliance_handler.scan_for_pii(&mut sheet);
        
        Ok(sheet)
    }
    }
    
    /// Convert a calamine Range to a Sheet
    fn convert_range_to_sheet(&self, range: Range<DataType>, owner: Uuid) -> Sheet {
        let mut cells = std::collections::HashMap::new();
        
        for (row, cells_in_row) in range.rows().enumerate() {
            for (col, cell) in cells_in_row.iter().enumerate() {
                let address = CellAddress::new(row as u32, col as u32);
                
                let cell_value = match cell {
                    DataType::Empty => CellValue::Empty,
                    DataType::String(s) => CellValue::Text(s.clone()),
                    DataType::Float(f) => CellValue::Number(*f),
                    DataType::Bool(b) => CellValue::Boolean(*b),
                    DataType::DateTime(dt) => {
                        // Convert Excel datetime to chrono DateTime
                        let timestamp = (*dt * 86400.0) as i64;
                        let naive = chrono::NaiveDateTime::from_timestamp_opt(timestamp, 0)
                            .unwrap_or_else(|| chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap());
                        CellValue::DateTime(chrono::DateTime::<chrono::Utc>::from_utc(naive, chrono::Utc))
                    },
                    DataType::Error(e) => CellValue::Error(format!("{:?}", e)),
                    DataType::Formula(s, _) => CellValue::Text(s.clone()), // For now, treat formulas as text
                };
                
                let sheet_cell = Cell::new(address, cell_value);
                cells.insert(address, sheet_cell);
            }
        }
        
        let mut sheet = Sheet::new("Imported Sheet".to_string(), owner);
        sheet.cells = cells;
        
        sheet
    }
    
    /// Export a Sheet to XLSX format
    pub fn export(sheet: &Sheet, file_path: &str, target_region: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        // Check data sovereignty
        let compliance_handler = ComplianceHandler::new();
        if let Some(region) = &target_region {
            if !compliance_handler.check_data_sovereignty(sheet, region)? {
                return Err("Data sovereignty violation: Cannot export to this region".into());
            }
        }
        
        // Redact PII if required
        let export_sheet = compliance_handler.redact_pii_for_export(sheet);
        
        // In a real implementation, this would export to XLSX format
        // For now, we'll just create a placeholder
        println!("Exporting sheet {} to {}", export_sheet.name, file_path);
        Ok(())
    }
}