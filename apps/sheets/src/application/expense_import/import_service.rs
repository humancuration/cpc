use crate::domain::Sheet;
use crate::application::expense_import::{ColumnMapping, ImportResult, ExpenseImportProcessor};
use packages::domains::finance::application::ExpenseService;
use uuid::Uuid;

/// Service for importing expenses from sheets
pub struct ExpenseImportService {
    processor: ExpenseImportProcessor,
}

impl ExpenseImportService {
    pub fn new(expense_service: Box<dyn ExpenseService>) -> Self {
        Self {
            processor: ExpenseImportProcessor::new(expense_service),
        }
    }
    
    pub async fn process(&self, sheet: Sheet, mapping: ColumnMapping, user_id: Uuid, user_preferences: &dyn packages::domains::finance::application::user_preferences::UserPreferences) -> Result<ImportResult, String> {
        self.processor.process(&sheet, mapping, user_id, user_preferences).await
    }
}