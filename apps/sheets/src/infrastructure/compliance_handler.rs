//! Compliance handler for import/export operations
//!
//! This module implements data sovereignty checks and PII redaction for
//! import/export operations in the Sheets application.

use crate::domain::{Sheet, Cell, CellValue, ComplianceMetadata};
use std::collections::HashMap;
use uuid::Uuid;

/// Compliance handler for import/export operations
pub struct ComplianceHandler;

impl ComplianceHandler {
    pub fn new() -> Self {
        Self
    }
    
    /// Check data sovereignty for export operations
    pub fn check_data_sovereignty(&self, sheet: &Sheet, target_region: &str) -> Result<bool, Box<dyn std::error::Error>> {
        // In a real implementation, this would check if the data can be exported to the target region
        // based on data sovereignty rules
        Ok(sheet.compliance_metadata.data_sovereignty == target_region || target_region == "US")
    }
    
    /// Redact PII for export operations
    pub fn redact_pii_for_export(&self, sheet: &Sheet) -> Sheet {
        let mut redacted_sheet = sheet.clone();
        
        // Redact PII in cells if required
        if sheet.compliance_metadata.pii_redacted {
            for (_, cell) in redacted_sheet.cells.iter_mut() {
                if self.contains_pii(&cell.value) && !cell.pii_redacted {
                    cell.redact_pii();
                    // In a real implementation, this would actually redact the PII content
                    // For now, we'll just mark the cell as redacted
                }
            }
        }
        
        redacted_sheet
    }
    
    /// Check if a cell value contains PII
    fn contains_pii(&self, value: &CellValue) -> bool {
        match value {
            CellValue::Text(s) => {
                // Simple PII detection - in a real implementation, this would be more sophisticated
                s.contains("@") ||  // Email
                s.contains("+") ||  // Phone number
                (s.len() == 11 && s.chars().all(|c| c.is_ascii_digit())) || // SSN-like
                (s.len() == 9 && s.chars().all(|c| c.is_ascii_digit())) // Another SSN format
            },
            _ => false,
        }
    }
    
    /// Apply data sovereignty rules to imported data
    pub fn apply_data_sovereignty_rules(&self, sheet: &mut Sheet, source_region: String) {
        sheet.compliance_metadata.data_sovereignty = source_region;
    }
    
    /// Scan for PII in imported data
    pub fn scan_for_pii(&self, sheet: &mut Sheet) {
        let mut pii_found = false;
        
        for (_, cell) in sheet.cells.iter_mut() {
            if self.contains_pii(&cell.value) {
                pii_found = true;
                // Mark cell as containing PII
                // In a real implementation, this might trigger additional compliance actions
            }
        }
        
        // Update compliance metadata
        sheet.compliance_metadata.pii_redacted = pii_found;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    
    #[test]
    fn test_data_sovereignty_check() {
        let handler = ComplianceHandler::new();
        let mut sheet = Sheet::new("Test Sheet".to_string(), Uuid::new_v4());
        sheet.compliance_metadata.data_sovereignty = "US".to_string();
        
        assert!(handler.check_data_sovereignty(&sheet, "US").unwrap());
        assert!(handler.check_data_sovereignty(&sheet, "CA").unwrap()); // Assuming US data can go to CA
        
        sheet.compliance_metadata.data_sovereignty = "EU".to_string();
        assert!(!handler.check_data_sovereignty(&sheet, "US").unwrap());
    }
    
    #[test]
    fn test_pii_detection() {
        let handler = ComplianceHandler::new();
        let email_cell = Cell::new(CellAddress::new(0, 0), CellValue::Text("user@example.com".to_string()));
        let phone_cell = Cell::new(CellAddress::new(0, 1), CellValue::Text("123-456-7890".to_string()));
        let ssn_cell = Cell::new(CellAddress::new(0, 2), CellValue::Text("123456789".to_string()));
        let normal_cell = Cell::new(CellAddress::new(0, 3), CellValue::Text("Normal text".to_string()));
        
        assert!(handler.contains_pii(&email_cell.value));
        assert!(handler.contains_pii(&phone_cell.value));
        assert!(handler.contains_pii(&ssn_cell.value));
        assert!(!handler.contains_pii(&normal_cell.value));
    }
    
    #[test]
    fn test_pii_redaction() {
        let handler = ComplianceHandler::new();
        let mut sheet = Sheet::new("Test Sheet".to_string(), Uuid::new_v4());
        sheet.compliance_metadata.pii_redacted = true;
        
        let mut pii_cell = Cell::new(CellAddress::new(0, 0), CellValue::Text("user@example.com".to_string()));
        pii_cell.pii_redacted = false;
        sheet.update_cell(CellAddress::new(0, 0), pii_cell);
        
        let redacted_sheet = handler.redact_pii_for_export(&sheet);
        let cell = redacted_sheet.get_cell(&CellAddress::new(0, 0)).unwrap();
        assert!(cell.pii_redacted);
    }
}