//! Accessibility module for the Sheets application
//!
//! This module provides functionality for generating alt text for cell ranges,
//! implementing keyboard navigation, and supporting screen readers.

use crate::domain::{Sheet, CellAddress, CellRange, CellValue};
use std::collections::HashMap;
use visualization_context::AltTextPreferences;

/// Accessibility service for the Sheets application
pub struct AccessibilityService;

impl AccessibilityService {
    pub fn new() -> Self {
        Self
    }
    
    /// Generate alt text for a cell range
    pub fn generate_alt_text(&self, sheet: &Sheet, range: &CellRange) -> String {
        // For a single cell, describe the cell content
        if range.start == range.end {
            if let Some(cell) = sheet.get_cell(&range.start) {
                return self.describe_cell_content(&cell.value);
            } else {
                return format!("Empty cell at {}", self.format_cell_address(&range.start));
            }
        }
        
        // For a range, describe the range and its content
        let range_description = format!(
            "Cell range from {} to {}",
            self.format_cell_address(&range.start),
            self.format_cell_address(&range.end)
        );
        
        // Collect cell values in the range
        let mut values = Vec::new();
        for row in range.start.row..=range.end.row {
            for col in range.start.column..=range.end.column {
                let address = CellAddress::new(row, col);
                if let Some(cell) = sheet.get_cell(&address) {
                    if !matches!(cell.value, CellValue::Empty) {
                        values.push(self.describe_cell_content(&cell.value));
                    }
                }
            }
        }
        
        if values.is_empty() {
            format!("{} containing no data", range_description)
        } else {
            format!("{} with values: {}", range_description, values.join(", "))
        }
    }
    
    /// Describe cell content for accessibility
    fn describe_cell_content(&self, value: &CellValue) -> String {
        match value {
            CellValue::Empty => "empty".to_string(),
            CellValue::Text(s) => format!("text: {}", s),
            CellValue::Number(n) => format!("number: {}", n),
            CellValue::Boolean(b) => format!("boolean: {}", if *b { "true" } else { "false" }),
            CellValue::Error(e) => format!("error: {}", e),
            CellValue::DateTime(dt) => format!("date: {}", dt.format("%Y-%m-%d %H:%M:%S")),
        }
    }
    
    /// Format cell address in A1 notation for accessibility
    fn format_cell_address(&self, address: &CellAddress) -> String {
        // Convert column number to letter (A, B, ..., Z, AA, AB, ...)
        let column_letter = self.column_number_to_letter(address.column + 1);
        format!("{}{}", column_letter, address.row + 1)
    }
    
    /// Convert column number to letter notation (1 -> A, 2 -> B, ..., 27 -> AA)
    fn column_number_to_letter(&self, column: u32) -> String {
        let mut result = String::new();
        let mut col = column;
        
        while col > 0 {
            let remainder = (col - 1) % 26;
            result.insert(0, (b'A' + remainder as u8) as char);
            col = (col - 1) / 26;
        }
        
        result
    }
    
    /// Generate keyboard navigation hints for a sheet
    pub fn generate_navigation_hints(&self, sheet: &Sheet) -> HashMap<String, String> {
        let mut hints = HashMap::new();
        
        // Add basic navigation hints
        hints.insert("Arrow Keys".to_string(), "Move between cells".to_string());
        hints.insert("Enter".to_string(), "Move to next row".to_string());
        hints.insert("Tab".to_string(), "Move to next column".to_string());
        hints.insert("Shift+Tab".to_string(), "Move to previous column".to_string());
        hints.insert("F2".to_string(), "Edit cell content".to_string());
        hints.insert("Ctrl+C".to_string(), "Copy selected cells".to_string());
        hints.insert("Ctrl+V".to_string(), "Paste clipboard content".to_string());
        hints.insert("Ctrl+Z".to_string(), "Undo last action".to_string());
        hints.insert("Ctrl+Y".to_string(), "Redo last action".to_string());
        
        // Add hints for charts if any exist
        if !sheet.charts.is_empty() {
            hints.insert("Ctrl+Shift+C".to_string(), "Create new chart".to_string());
            hints.insert("Ctrl+Shift+E".to_string(), "Edit selected chart".to_string());
        }
        
        hints
    }
    
    /// Generate screen reader announcement for cell updates
    pub fn generate_cell_update_announcement(&self, address: &CellAddress, value: &CellValue) -> String {
        format!(
            "Cell {} updated with {}",
            self.format_cell_address(address),
            self.describe_cell_content(value)
        )
    }
    
    /// Generate alt text for a chart based on the data range and preferences
    pub fn generate_chart_alt_text(sheet: &Sheet, range: &CellRange, preferences: &AltTextPreferences) -> String {
        // For charts, we want to provide a more structured description
        let range_description = format!(
            "Chart data from {} to {}",
            Self.format_cell_address(&range.start),
            Self.format_cell_address(&range.end)
        );
        
        // Extract headers for chart series names
        let headers = Self.extract_headers(sheet, range);
        
        // Count data points
        let data_point_count = if range.end.row > range.start.row {
            range.end.row - range.start.row
        } else {
            0
        };
        
        // Create description based on detail level
        match preferences.detail_level {
            0 => {
                // Brief description
                format!("Chart with {} data series and {} data points", headers.len().saturating_sub(1), data_point_count)
            },
            1 => {
                // Detailed description
                if headers.len() > 1 {
                    format!("{} with {} series: {}", range_description, headers.len() - 1, headers[1..].join(", "))
                } else {
                    range_description
                }
            },
            _ => {
                // Verbose description
                let mut description = format!("{} with {} data series", range_description, headers.len().saturating_sub(1));
                
                // Add series information
                for (i, header) in headers.iter().enumerate().skip(1) {
                    description.push_str(&format!(". Series {}: {}", i, header));
                }
                
                // Add data point count
                description.push_str(&format!(". Total data points: {}", data_point_count));
                
                description
            }
        }
    }
    
    /// Extract headers from the first row of a range
    fn extract_headers(sheet: &Sheet, range: &CellRange) -> Vec<String> {
        let mut headers = Vec::new();
        
        for col in range.start.column..=range.end.column {
            let address = CellAddress::new(range.start.row, col);
            let header = if let Some(cell) = sheet.get_cell(&address) {
                match &cell.value {
                    CellValue::Text(s) => s.clone(),
                    CellValue::Number(n) => n.to_string(),
                    _ => format!("Column {}", col),
                }
            } else {
                format!("Column {}", col)
            };
            headers.push(header);
        }
        
        headers
    }
    
    /// Announce a message to screen readers
    pub fn announce_screen_reader(message: &str) {
        // In a real implementation, this would interface with screen reader APIs
        // For now, we'll just log the message
        println!("Screen reader announcement: {}", message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Cell, CellStyle};
    use uuid::Uuid;
    use chrono::Utc;
    
    #[test]
    fn test_column_number_to_letter() {
        let service = AccessibilityService::new();
        assert_eq!(service.column_number_to_letter(1), "A");
        assert_eq!(service.column_number_to_letter(26), "Z");
        assert_eq!(service.column_number_to_letter(27), "AA");
        assert_eq!(service.column_number_to_letter(52), "AZ");
        assert_eq!(service.column_number_to_letter(53), "BA");
    }
    
    #[test]
    fn test_format_cell_address() {
        let service = AccessibilityService::new();
        let address = CellAddress::new(0, 0); // A1
        assert_eq!(service.format_cell_address(&address), "A1");
        
        let address = CellAddress::new(1, 26); // AA2
        assert_eq!(service.format_cell_address(&address), "AA2");
    }
    
    #[test]
    fn test_generate_alt_text_single_cell() {
        let service = AccessibilityService::new();
        let mut sheet = Sheet::new("Test Sheet".to_string(), Uuid::new_v4());
        let address = CellAddress::new(0, 0);
        let cell = Cell::new(address, CellValue::Text("Hello".to_string()));
        sheet.update_cell(address, cell);
        
        let range = CellRange {
            start: address,
            end: address,
        };
        
        let alt_text = service.generate_alt_text(&sheet, &range);
        assert_eq!(alt_text, "text: Hello");
    }
    
    #[test]
    fn test_generate_alt_text_range() {
        let service = AccessibilityService::new();
        let mut sheet = Sheet::new("Test Sheet".to_string(), Uuid::new_v4());
        
        // Add some cells
        let cell1 = Cell::new(CellAddress::new(0, 0), CellValue::Text("Name".to_string()));
        let cell2 = Cell::new(CellAddress::new(0, 1), CellValue::Text("Age".to_string()));
        let cell3 = Cell::new(CellAddress::new(1, 0), CellValue::Text("Alice".to_string()));
        let cell4 = Cell::new(CellAddress::new(1, 1), CellValue::Number(30.0));
        
        sheet.update_cell(CellAddress::new(0, 0), cell1);
        sheet.update_cell(CellAddress::new(0, 1), cell2);
        sheet.update_cell(CellAddress::new(1, 0), cell3);
        sheet.update_cell(CellAddress::new(1, 1), cell4);
        
        let range = CellRange {
            start: CellAddress::new(0, 0),
            end: CellAddress::new(1, 1),
        };
        
        let alt_text = service.generate_alt_text(&sheet, &range);
        assert!(alt_text.contains("Cell range from A1 to B2"));
        assert!(alt_text.contains("text: Name"));
        assert!(alt_text.contains("text: Age"));
        assert!(alt_text.contains("text: Alice"));
        assert!(alt_text.contains("number: 30"));
    }
}