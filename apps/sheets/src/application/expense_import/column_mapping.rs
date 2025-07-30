use serde::{Deserialize, Serialize};

/// Column mapping for expense import
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnMapping {
    pub date_column: String,    // e.g., "A"
    pub amount_column: String,
    pub category_column: String,
    pub description_column: Option<String>,
    pub vendor_column: Option<String>,
    pub account_column: Option<String>,
}

impl ColumnMapping {
    pub fn new(
        date_column: String,
        amount_column: String,
        category_column: String,
        description_column: Option<String>,
        vendor_column: Option<String>,
        account_column: Option<String>,
    ) -> Self {
        Self {
            date_column,
            amount_column,
            category_column,
            description_column,
            vendor_column,
            account_column,
        }
    }
}

/// Result of an import operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResult {
    pub total_rows: u32,
    pub successful_imports: u32,
    pub failed_rows: Vec<FailedRow>,
    pub errors: Vec<String>,
}

/// Information about a failed row import
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedRow {
    pub row_number: u32,
    pub error: String,
    pub data: serde_json::Value,
}