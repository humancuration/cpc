//! Example: Financial trend analysis and fraud detection

use ml_core::MLEngine;
use ml_core::models::FinancialTrendModel;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Financial Trend Analysis Example");
    println!("===============================");
    
    // Create ML engine
    let engine = MLEngine::new();
    
    // Create financial trend model
    let model = engine.create_financial_trend_model();
    
    // Example: Predict financial sustainability
    println!("\n1. Predicting financial sustainability...");
    let financial_data = vec![0.85, 0.9, 0.75, 0.88, 0.82]; // Placeholder data
    let sustainability_score = model.predict_sustainability(&financial_data)?;
    println!("   Predicted sustainability score: {:.2}", sustainability_score);
    
    // Example: Detect anomalous transactions
    println!("\n2. Detecting anomalous transactions...");
    let transaction_data = vec![0.1, 0.2, 0.15, 5.0, 0.3, 0.25]; // Placeholder data with anomaly
    let anomalies = model.detect_anomalous_transactions(&transaction_data)?;
    println!("   Detected {} anomalous transactions:", anomalies.len());
    for (i, anomaly_index) in anomalies.iter().enumerate() {
        println!("     {}. Transaction #{}", i + 1, anomaly_index);
    }
    
    // Example: Forecast economic trends
    println!("\n3. Forecasting economic trends...");
    let economic_data = vec![0.75, 0.8, 0.7, 0.85, 0.78]; // Placeholder data
    let trends = model.forecast_economic_trends(&economic_data)?;
    println!("   Generated {} trend forecasts:", trends.len());
    for (trend_name, forecast) in &trends {
        println!("     {}: {:.2}", trend_name, forecast);
    }
    
    // Example: Recommend resource allocation
    println!("\n4. Recommending resource allocation...");
    let resource_data = vec![0.8, 0.7, 0.9, 0.6, 0.85]; // Placeholder data
    let allocations = model.recommend_resource_allocation(&resource_data)?;
    println!("   Generated {} allocation recommendations:", allocations.len());
    for (i, allocation) in allocations.iter().enumerate() {
        println!("     {}. {}: {:.2} units", i + 1, allocation.category, allocation.amount);
    }
    
    println!("\nExample completed successfully!");
    Ok(())
}