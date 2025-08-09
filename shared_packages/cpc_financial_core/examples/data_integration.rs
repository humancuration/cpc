//! Example demonstrating integration between CPC Financial Core and Data Lakehouse

use cpc_financial_core::{
    MonetaryAmount, CurrencyCode, RoundingStrategy
};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

// Note: In a real implementation, this would integrate with the actual data_lakehouse crate
// For this example, we'll simulate the integration concepts

/// Simulates a large financial dataset that needs memory management
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
            return Ok(MonetaryAmount::new(Decimal::ZERO, CurrencyCode::USD));
        }
        
        let mut total = self.transactions[0].clone();
        for transaction in &self.transactions[1..] {
            total = total.add(transaction)?;
        }
        Ok(total)
    }
    
    pub fn average_value(&self) -> Result<MonetaryAmount, String> {
        if self.transactions.is_empty() {
            return Ok(MonetaryAmount::new(Decimal::ZERO, CurrencyCode::USD));
        }
        
        let total = self.total_value()?;
        let count = Decimal::from(self.transactions.len() as u64);
        let average = total.divide(count)?;
        Ok(average)
    }
}

/// Simulates memory management strategies from data_lakehouse
pub enum ProcessingStrategy {
    LoadFull,      // Load entire dataset (requires sufficient memory)
    Downsample,    // Reduce dataset size for processing
    Stream,        // Process data in chunks
}

/// Simulates DataCapabilities from data_lakehouse
pub struct DataCapabilities {
    pub available_memory_mb: f64,
    pub recommended_strategy: ProcessingStrategy,
}

impl DataCapabilities {
    pub fn new(available_memory_mb: f64) -> Self {
        let strategy = if available_memory_mb < 5.0 {
            ProcessingStrategy::Stream
        } else if available_memory_mb < 50.0 {
            ProcessingStrategy::Downsample
        } else {
            ProcessingStrategy::LoadFull
        };
        
        Self {
            available_memory_mb,
            recommended_strategy: strategy,
        }
    }
    
    pub fn process_financial_data(
        &self,
        dataset: &FinancialDataset,
    ) -> Result<FinancialSummary, String> {
        println!("Processing financial data with {} MB available memory", 
                 self.available_memory_mb);
        println!("Recommended strategy: {:?}", self.recommended_strategy);
        
        match self.recommended_strategy {
            ProcessingStrategy::LoadFull => {
                println!("Loading full dataset for precise calculations");
                let total = dataset.total_value()?;
                let average = dataset.average_value()?;
                Ok(FinancialSummary::new(total, average, dataset.total_entries))
            },
            ProcessingStrategy::Downsample => {
                println!("Downsampling dataset for memory-efficient processing");
                // In a real implementation, we would downsample the data
                // For this example, we'll just process a subset
                let subset_size = (dataset.transactions.len() / 2).max(1);
                let subset: Vec<MonetaryAmount> = dataset.transactions
                    .iter()
                    .take(subset_size)
                    .cloned()
                    .collect();
                
                let subset_dataset = FinancialDataset {
                    transactions: subset,
                    total_entries: subset_size,
                };
                
                let total = subset_dataset.total_value()?;
                let average = subset_dataset.average_value()?;
                Ok(FinancialSummary::new(total, average, subset_dataset.total_entries))
            },
            ProcessingStrategy::Stream => {
                println!("Streaming data for minimal memory usage");
                // In a real implementation, we would process data in chunks
                // For this example, we'll just process in small batches
                let batch_size = 10;
                let mut total = MonetaryAmount::new(Decimal::ZERO, CurrencyCode::USD);
                let mut count = 0;
                
                for chunk in dataset.transactions.chunks(batch_size) {
                    for transaction in chunk {
                        total = total.add(transaction)?;
                        count += 1;
                    }
                }
                
                let average = if count > 0 {
                    total.divide(Decimal::from(count as u64))?
                } else {
                    MonetaryAmount::new(Decimal::ZERO, CurrencyCode::USD)
                };
                
                Ok(FinancialSummary::new(total, average, count))
            }
        }
    }
}

/// Summary of financial data processing
pub struct FinancialSummary {
    pub total_value: MonetaryAmount,
    pub average_value: MonetaryAmount,
    pub processed_entries: usize,
    pub precision_note: String,
}

impl FinancialSummary {
    pub fn new(total: MonetaryAmount, average: MonetaryAmount, entries: usize) -> Self {
        let precision_note = if entries > 100 {
            "Calculated with downsampled data for memory efficiency".to_string()
        } else {
            "Calculated with full precision".to_string()
        };
        
        Self {
            total_value: total,
            average_value: average,
            processed_entries: entries,
            precision_note,
        }
    }
}

fn main() {
    println!("=== CPC Financial Core + Data Lakehouse Integration ===\n");
    
    // Create a financial dataset
    let mut dataset = FinancialDataset::new();
    
    // Add sample transactions
    let transactions = vec![
        MonetaryAmount::new(dec!(125.50), CurrencyCode::USD),
        MonetaryAmount::new(dec!(89.99), CurrencyCode::USD),
        MonetaryAmount::new(dec!(250.00), CurrencyCode::USD),
        MonetaryAmount::new(dec!(15.75), CurrencyCode::USD),
        MonetaryAmount::new(dec!(320.25), CurrencyCode::USD),
        MonetaryAmount::new(dec!(75.00), CurrencyCode::USD),
        MonetaryAmount::new(dec!(180.30), CurrencyCode::USD),
        MonetaryAmount::new(dec!(95.25), CurrencyCode::USD),
    ];
    
    for transaction in transactions {
        dataset.add_transaction(transaction);
    }
    
    println!("Dataset created with {} transactions\n", dataset.total_entries);
    
    // Simulate different memory environments
    
    // High memory environment
    println!("1. High Memory Environment (64 MB available):");
    let high_memory = DataCapabilities::new(64.0);
    match high_memory.process_financial_data(&dataset) {
        Ok(summary) => {
            println!("   Total Value: {}", summary.total_value);
            println!("   Average Value: {}", summary.average_value);
            println!("   Processed Entries: {}", summary.processed_entries);
            println!("   Note: {}", summary.precision_note);
        },
        Err(e) => println!("   Error: {}", e),
    }
    
    println!();
    
    // Medium memory environment
    println!("2. Medium Memory Environment (25 MB available):");
    let medium_memory = DataCapabilities::new(25.0);
    match medium_memory.process_financial_data(&dataset) {
        Ok(summary) => {
            println!("   Total Value: {}", summary.total_value);
            println!("   Average Value: {}", summary.average_value);
            println!("   Processed Entries: {}", summary.processed_entries);
            println!("   Note: {}", summary.precision_note);
        },
        Err(e) => println!("   Error: {}", e),
    }
    
    println!();
    
    // Low memory environment
    println!("3. Low Memory Environment (2 MB available):");
    let low_memory = DataCapabilities::new(2.0);
    match low_memory.process_financial_data(&dataset) {
        Ok(summary) => {
            println!("   Total Value: {}", summary.total_value);
            println!("   Average Value: {}", summary.average_value);
            println!("   Processed Entries: {}", summary.processed_entries);
            println!("   Note: {}", summary.precision_note);
        },
        Err(e) => println!("   Error: {}", e),
    }
    
    println!("\n=== Integration Example Completed ===");
    println!("This example demonstrates:");
    println!("  - How financial data can be processed in different memory environments");
    println!("  - Adaptive processing strategies based on available resources");
    println!("  - Maintaining financial precision while managing system resources");
    println!("  - Integration between financial core and data management systems");
}