//! Example usage of statistical analysis features in cause management
//!
//! This example demonstrates how to use the statistical analysis capabilities
//! for forecasting donations and measuring cause impact.

#[cfg(feature = "statistics")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use cause_management::domain::{
        statistical_models::{DonationForecast, DonationTrend, TrendType},
        impact_models::{ImpactAnalysis, ImpactMetric},
    };
    use cause_management::application::{
        statistical_analysis::StatisticalAnalysisService,
        impact_measurement::{ImpactMeasurementService, ImpactOutcome},
    };
    use cause_management::domain::models::Cause;
    use rust_decimal_macros::dec;
    use chrono::Utc;
    
    println!("Cause Management Statistical Analysis Example");
    println!("==========================================");
    
    // Create a sample cause
    let cause = Cause::new(
        "Community Garden Project".to_string(),
        "Building sustainable community gardens to provide fresh produce".to_string(),
        Some("https://example.com/garden.jpg".to_string()),
    );
    
    // Create sample donation data
    let donations = vec![
        dec!(100.0),
        dec!(120.0),
        dec!(90.0),
        dec!(110.0),
        dec!(130.0),
        dec!(125.0),
        dec!(140.0),
    ];
    
    // Forecast future donations
    println!("\n1. Donation Forecasting:");
    match StatisticalAnalysisService::forecast_donations(&donations, 5, 0.95) {
        Ok(forecast) => {
            println!("   Forecast generated successfully!");
            println!("   {}", forecast.explanation());
            println!("   Cooperative explanation: {}", forecast.cooperative_explanation());
        },
        Err(e) => {
            println!("   Error generating forecast: {}", e);
        }
    }
    
    // Analyze donation trends
    println!("\n2. Donation Trend Analysis:");
    match StatisticalAnalysisService::analyze_donation_trends(&donations) {
        Ok(trends) => {
            for trend in trends {
                println!("   Trend identified: {:?}", trend.trend_type);
                println!("   Strength: {:.2}", trend.strength);
                println!("   Significance: {:.4}", trend.p_value);
                println!("   {}", trend.explanation());
            }
        },
        Err(e) => {
            println!("   Error analyzing trends: {}", e);
        }
    }
    
    // Measure cause impact
    println!("\n3. Impact Measurement:");
    let outcomes = vec![
        ImpactOutcome::new(5.0, Utc::now(), "Community members engaged".to_string()),
        ImpactOutcome::new(7.0, Utc::now(), "Garden plots established".to_string()),
        ImpactOutcome::new(9.0, Utc::now(), "Produce distributed".to_string()),
    ];
    
    match ImpactMeasurementService::measure_impact(
        &cause,
        &donations,
        &outcomes,
        ImpactMetric::CommunityEngagement,
    ) {
        Ok(analysis) => {
            println!("   Impact analysis completed!");
            println!("   Impact score: {:.1}/10", analysis.impact_score);
            println!("   {}", analysis.explanation());
            println!("   Cooperative explanation: {}", analysis.cooperative_explanation());
        },
        Err(e) => {
            println!("   Error measuring impact: {}", e);
        }
    }
    
    println!("\nExample completed successfully!");
    Ok(())
}

#[cfg(not(feature = "statistics"))]
fn main() {
    println!("This example requires the 'statistics' feature to be enabled.");
    println!("Run with: cargo run --example statistical_analysis --features statistics");
}