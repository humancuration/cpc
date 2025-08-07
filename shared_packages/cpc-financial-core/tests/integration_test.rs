//! Integration test demonstrating how cpc-financial-core works with other CPC components

use cpc_financial_core::{
    MonetaryAmount, CurrencyCode, RoundingStrategy,
    audit::FinancialAuditable
};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use uuid::Uuid;
use chrono::Utc;

// Note: In a real implementation, these would be the actual imports
// For this test, we'll simulate the integration with simplified versions

/// Simulates data capabilities from data_lakehouse
#[derive(Debug, Clone)]
pub struct DataCapabilities {
    pub available_memory_mb: f64,
    pub max_rows: Option<usize>,
    pub auto_downsample: bool,
}

impl DataCapabilities {
    pub fn new(available_memory_mb: f64) -> Self {
        Self {
            available_memory_mb,
            max_rows: Some(5000),
            auto_downsample: true,
        }
    }
    
    pub fn new_web_default() -> Self {
        Self::new(5.0) // 5MB limit for web environments
    }
    
    pub fn new_desktop_default() -> Self {
        Self::new(1024.0) // 1GB limit for desktop environments
    }
    
    pub fn exceeds_memory_limit(&self, estimated_size_bytes: usize) -> bool {
        let memory_limit_bytes = (self.available_memory_mb * 1024.0 * 1024.0) as usize;
        estimated_size_bytes > memory_limit_bytes
    }
}

/// Simulates processing strategy from data_lakehouse
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessingStrategy {
    LoadFull,
    Downsample(usize),
    Stream,
}

/// Simulates memory manager from data_lakehouse
pub struct MemoryManager {
    capabilities: DataCapabilities,
}

impl MemoryManager {
    pub fn new(capabilities: DataCapabilities) -> Self {
        Self { capabilities }
    }
    
    pub fn get_processing_strategy(&self, row_count: usize, estimated_size_bytes: usize) -> ProcessingStrategy {
        // Check memory constraints first
        if self.capabilities.exceeds_memory_limit(estimated_size_bytes) {
            return ProcessingStrategy::Stream;
        }
        
        // Check row count constraints
        if let Some(max_rows) = self.capabilities.max_rows {
            if row_count > max_rows {
                if self.capabilities.auto_downsample {
                    let sample_size = max_rows.min(row_count / 2);
                    return ProcessingStrategy::Downsample(sample_size);
                }
                return ProcessingStrategy::Stream;
            }
        }
        
        ProcessingStrategy::LoadFull
    }
}

/// Simulates a financial dataset that needs memory management
#[derive(Debug, Clone)]
pub struct FinancialDataset {
    pub transactions: Vec<MonetaryAmount>,
    pub total_entries: usize,
}

impl FinancialDataset {
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
            total_entries: 0,
        }
    }
    
    pub fn add_transaction(&mut self, amount: MonetaryAmount) {
        self.transactions.push(amount);
        self.total_entries += 1;
    }
    
    pub fn total_value(&self) -> Result<MonetaryAmount, String> {
        if self.transactions.is_empty() {
            return Ok(MonetaryAmount::new(Decimal::ZERO, CurrencyCode::DBL));
        }
        
        let mut total = self.transactions[0].clone();
        for transaction in &self.transactions[1..] {
            total = total.add(transaction)
                .map_err(|e| format!("Addition error: {}", e))?;
        }
        Ok(total)
    }
}

/// Simulates a payment processor that integrates financial core with data management
pub struct PaymentProcessor {
    memory_manager: MemoryManager,
}

impl PaymentProcessor {
    pub fn new(memory_manager: MemoryManager) -> Self {
        Self { memory_manager }
    }
    
    pub fn process_large_financial_dataset(
        &self,
        dataset: &FinancialDataset,
    ) -> Result<FinancialSummary, String> {
        let row_count = dataset.total_entries;
        let estimated_size_bytes = row_count * 1024; // Rough estimate
        
        let strategy = self.memory_manager.get_processing_strategy(row_count, estimated_size_bytes);
        
        match strategy {
            ProcessingStrategy::LoadFull => {
                // Process full dataset
                let total = dataset.total_value()?;
                let average = if row_count > 0 {
                    total.divide(Decimal::from(row_count as u64))
                        .map_err(|e| format!("Division error: {}", e))?
                } else {
                    MonetaryAmount::new(Decimal::ZERO, CurrencyCode::DBL)
                };
                
                Ok(FinancialSummary::new(total, average, row_count, "Full precision calculation"))
            },
            ProcessingStrategy::Downsample(size) => {
                // Process subset of data
                let subset: Vec<MonetaryAmount> = dataset.transactions
                    .iter()
                    .take(size)
                    .cloned()
                    .collect();
                
                let subset_dataset = FinancialDataset {
                    transactions: subset,
                    total_entries: size,
                };
                
                let total = subset_dataset.total_value()?;
                let average = if size > 0 {
                    total.divide(Decimal::from(size as u64))
                        .map_err(|e| format!("Division error: {}", e))?
                } else {
                    MonetaryAmount::new(Decimal::ZERO, CurrencyCode::DBL)
                };
                
                Ok(FinancialSummary::new(total, average, size, "Downsampled calculation"))
            },
            ProcessingStrategy::Stream => {
                // Process in chunks
                let chunk_size = 100;
                let mut total = MonetaryAmount::new(Decimal::ZERO, CurrencyCode::DBL);
                let mut processed_count = 0;
                
                for chunk in dataset.transactions.chunks(chunk_size) {
                    for transaction in chunk {
                        total = total.add(transaction)
                            .map_err(|e| format!("Addition error: {}", e))?;
                        processed_count += 1;
                    }
                }
                
                let average = if processed_count > 0 {
                    total.divide(Decimal::from(processed_count as u64))
                        .map_err(|e| format!("Division error: {}", e))?
                } else {
                    MonetaryAmount::new(Decimal::ZERO, CurrencyCode::DBL)
                };
                
                Ok(FinancialSummary::new(total, average, processed_count, "Streamed calculation"))
            },
        }
    }
}

/// Summary of financial data processing
pub struct FinancialSummary {
    pub total_value: MonetaryAmount,
    pub average_value: MonetaryAmount,
    pub processed_entries: usize,
    pub calculation_note: String,
}

impl FinancialSummary {
    pub fn new(total: MonetaryAmount, average: MonetaryAmount, entries: usize, note: &str) -> Self {
        Self {
            total_value: total,
            average_value: average,
            processed_entries: entries,
            calculation_note: note.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_financial_core_with_data_management() {
        // Create a memory manager with web defaults (5MB limit)
        let memory_manager = MemoryManager::new(DataCapabilities::new_web_default());
        let processor = PaymentProcessor::new(memory_manager);
        
        // Create a financial dataset
        let mut dataset = FinancialDataset::new();
        
        // Add sample transactions
        let transactions = vec![
            MonetaryAmount::new(dec!(125.50), CurrencyCode::DBL),
            MonetaryAmount::new(dec!(89.99), CurrencyCode::DBL),
            MonetaryAmount::new(dec!(250.00), CurrencyCode::DBL),
            MonetaryAmount::new(dec!(15.75), CurrencyCode::DBL),
            MonetaryAmount::new(dec!(320.25), CurrencyCode::DBL),
        ];
        
        for transaction in transactions {
            dataset.add_transaction(transaction);
        }
        
        // Process the dataset
        let result = processor.process_large_financial_dataset(&dataset);
        assert!(result.is_ok());
        
        let summary = result.unwrap();
        assert_eq!(summary.processed_entries, 5);
        assert_eq!(summary.calculation_note, "Full precision calculation");
        
        // Verify calculations are correct
        let expected_total = MonetaryAmount::new(dec!(801.49), CurrencyCode::DBL);
        let expected_average = MonetaryAmount::new(dec!(160.298), CurrencyCode::DBL);
        
        // Round for comparison (since we're using Banker's rounding)
        let rounded_total = summary.total_value.round(RoundingStrategy::Bankers);
        let rounded_average = summary.average_value.round(RoundingStrategy::Bankers);
        let rounded_expected_total = expected_total.round(RoundingStrategy::Bankers);
        let rounded_expected_average = expected_average.round(RoundingStrategy::Bankers);
        
        assert_eq!(rounded_total, rounded_expected_total);
        assert_eq!(rounded_average, rounded_expected_average);
    }
    
    #[test]
    fn test_memory_management_strategies() {
        // Test with small dataset (should load full)
        let memory_manager = MemoryManager::new(DataCapabilities::new_web_default());
        let strategy = memory_manager.get_processing_strategy(100, 1024); // 100 rows, 1KB
        assert_eq!(strategy, ProcessingStrategy::LoadFull);
        
        // Test with large dataset that exceeds row limit (should downsample)
        let strategy = memory_manager.get_processing_strategy(10000, 1024); // 10000 rows, 1KB
        assert!(matches!(strategy, ProcessingStrategy::Downsample(5000)));
        
        // Test with dataset that exceeds memory limit (should stream)
        let strategy = memory_manager.get_processing_strategy(100, 10 * 1024 * 1024); // 100 rows, 10MB
        assert_eq!(strategy, ProcessingStrategy::Stream);
    }
    
    #[test]
    fn test_currency_operations() {
        // Test different currencies
        let usd_amount = MonetaryAmount::new(dec!(100.50), CurrencyCode::USD);
        let eur_amount = MonetaryAmount::new(dec!(85.25), CurrencyCode::EUR);
        let gbp_amount = MonetaryAmount::new(dec!(75.00), CurrencyCode::GBP);
        let jpy_amount = MonetaryAmount::new(dec!(11000), CurrencyCode::JPY); // JPY has no decimal places
        let dbl_amount = MonetaryAmount::new(dec!(500.75), CurrencyCode::DBL);
        
        // Verify decimal places are correct
        assert_eq!(usd_amount.precision(), 2);
        assert_eq!(eur_amount.precision(), 2);
        assert_eq!(gbp_amount.precision(), 2);
        assert_eq!(jpy_amount.precision(), 0);
        assert_eq!(dbl_amount.precision(), 2);
        
        // Test addition within same currency
        let result = usd_amount.add(&MonetaryAmount::new(dec!(25.25), CurrencyCode::USD));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value(), dec!(125.75));
        
        // Test addition with different currencies (should fail)
        let result = usd_amount.add(&eur_amount);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_rounding_strategies() {
        let amount = MonetaryAmount::new(dec!(100.565), CurrencyCode::USD);
        
        // Test Banker's rounding (round half to even)
        let bankers_rounded = amount.round(RoundingStrategy::Bankers);
        assert_eq!(bankers_rounded.value(), dec!(100.56)); // Rounds to even
        
        // Test ceiling rounding
        let ceiling_rounded = amount.round(RoundingStrategy::Ceiling);
        assert_eq!(ceiling_rounded.value(), dec!(100.57));
        
        // Test floor rounding
        let floor_rounded = amount.round(RoundingStrategy::Floor);
        assert_eq!(floor_rounded.value(), dec!(100.56));
    }
    
    #[test]
    fn test_audit_trail_integration() {
        let amount = MonetaryAmount::new(dec!(100.50), CurrencyCode::USD);
        let metadata = serde_json::json!({"test": "value"});
        
        // Create audit event
        let audit_event = amount.create_audit_event(
            Some("user123".to_string()),
            audit_framework::domain::event::AuditAction::Read,
            audit_framework::domain::event::PurposeCode::UserView,
            metadata,
        );
        
        // Verify audit event properties
        assert_eq!(audit_event.domain, "finance");
        assert_eq!(audit_event.target, "monetary_amount_USD");
        assert_eq!(audit_event.user_id, Some("user123".to_string()));
    }
}