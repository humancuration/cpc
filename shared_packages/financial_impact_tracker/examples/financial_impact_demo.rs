//! Financial Impact Tracker Demo
//!
//! This example demonstrates how to use the financial impact tracker to monitor
//! and analyze the financial impact of community activities.

use financial_impact_tracker::{
    FinancialImpactTracker, 
    FinancialAnalytics, 
    FinancialReportGenerator,
    FinancialIntegration,
    FinancialIntegrationConfig,
    FinancialEventType,
    FinancialCategory
};
use cpay_core::{CPayCore, Transaction, TransactionType, Currency};
use cpc_financial_core::CPCFinancialCore;
use sqlx::PgPool;
use rust_decimal::Decimal;
use chrono::{Utc, Duration};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database connection (in a real app, you'd use proper connection settings)
    // For this demo, we'll mock the database functionality
    
    println!("Financial Impact Tracker Demo");
    println!("============================");
    
    // Create sample transactions
    let donation_transaction = Transaction {
        id: Uuid::new_v4(),
        wallet_id: Uuid::new_v4(),
        transaction_type: TransactionType::Donation,
        amount: Decimal::from(500),
        currency: Currency::USD,
        description: "Community garden donation".to_string(),
        timestamp: Utc::now(),
        metadata: serde_json::json!({
            "donor": "local_business",
            "cause": "community_garden"
        }),
    };
    
    let grant_transaction = Transaction {
        id: Uuid::new_v4(),
        wallet_id: Uuid::new_v4(),
        transaction_type: TransactionType::Grant,
        amount: Decimal::from(2000),
        currency: Currency::USD,
        description: "Education grant".to_string(),
        timestamp: Utc::now(),
        metadata: serde_json::json!({
            "grantor": "education_foundation",
            "program": "literacy_initiative"
        }),
    };
    
    // Demonstrate impact tracking
    println!("\n1. Tracking Financial Impact");
    
    // In a real implementation, we would connect to a database
    // For this demo, we'll show what the tracking would look like
    
    println!("   Tracking donation: {} USD for community garden", donation_transaction.amount);
    println!("   Tracking grant: {} USD for literacy initiative", grant_transaction.amount);
    
    // Demonstrate impact scoring
    println!("\n2. Financial Impact Scoring");
    
    let donation_impact_score = calculate_impact_score(
        &donation_transaction, 
        &FinancialCategory::Donations,
        &donation_transaction.metadata
    );
    
    let grant_impact_score = calculate_impact_score(
        &grant_transaction, 
        &FinancialCategory::Grants,
        &grant_transaction.metadata
    );
    
    println!("   Donation impact score: {:.2}%", donation_impact_score * Decimal::from(100));
    println!("   Grant impact score: {:.2}%", grant_impact_score * Decimal::from(100));
    
    // Demonstrate analytics
    println!("\n3. Financial Analytics");
    
    // Sample analytics data (in a real implementation, this would come from the database)
    let sample_analytics = create_sample_analytics();
    
    println!("   Total financial impact: {} USD", sample_analytics.total_impact);
    println!("   Top category: CommunityInvestment");
    println!("   Top contributor: Local Education Foundation");
    println!("   Financial health score: {:.1}%", sample_analytics.sustainability_metrics.financial_health_score * Decimal::from(100));
    
    // Demonstrate reporting
    println!("\n4. Financial Impact Reporting");
    
    let start_time = Utc::now() - Duration::days(30);
    let end_time = Utc::now();
    
    println!("   Generating report for period: {} to {}", 
             start_time.format("%Y-%m-%d"), 
             end_time.format("%Y-%m-%d"));
    
    // Sample report data
    let sample_report = create_sample_report(start_time, end_time, sample_analytics);
    
    println!("   Report generated successfully");
    println!("   Key insights:");
    for insight in &sample_report.summary.key_insights {
        println!("     - {}", insight);
    }
    
    // Demonstrate integration features
    println!("\n5. System Integration");
    
    println!("   Linked to 3 community causes");
    println!("   Connected to 12 volunteer activities");
    println!("   Associated with 5 learning programs");
    println!("   Synchronized with cpay_core transactions");
    
    println!("\nDemo completed successfully!");
    
    Ok(())
}

/// Calculate impact score for a transaction (simplified for demo)
fn calculate_impact_score(
    transaction: &Transaction,
    category: &FinancialCategory,
    metadata: &serde_json::Value,
) -> Decimal {
    // Base score based on category
    let mut score = match category {
        FinancialCategory::Donations => Decimal::from(95),
        FinancialCategory::Grants => Decimal::from(90),
        FinancialCategory::CommunityInvestment => Decimal::from(100),
        FinancialCategory::Education => Decimal::from(85),
        _ => Decimal::from(70),
    };

    // Adjust based on amount
    if transaction.amount > Decimal::from(1000) {
        score = score + Decimal::from(5);
    } else if transaction.amount > Decimal::from(500) {
        score = score + Decimal::from(3);
    }

    // Adjust based on contributor type
    if let Some(contributor_type) = metadata.get("donor").or_else(|| metadata.get("grantor")) {
        match contributor_type.as_str() {
            Some("local_business") => score = score + Decimal::from(5),
            Some("education_foundation") => score = score + Decimal::from(10),
            _ => {}
        }
    }

    // Normalize to 0-1 scale
    score / Decimal::from(100)
}

/// Create sample analytics data for demo
fn create_sample_analytics() -> financial_impact_tracker::analytics::FinancialImpactAnalytics {
    use financial_impact_tracker::analytics::*;
    
    FinancialImpactAnalytics {
        total_impact: Decimal::from(50000),
        category_breakdown: vec![
            CategoryImpact {
                category: FinancialCategory::CommunityInvestment,
                total_amount: Decimal::from(20000),
                impact_score: Decimal::from(95) / Decimal::from(100),
                weighted_impact: Decimal::from(19000),
                transaction_count: 15,
            },
            CategoryImpact {
                category: FinancialCategory::Education,
                total_amount: Decimal::from(15000),
                impact_score: Decimal::from(90) / Decimal::from(100),
                weighted_impact: Decimal::from(13500),
                transaction_count: 12,
            },
        ],
        time_series: vec![
            TimeSeriesPoint {
                timestamp: Utc::now() - Duration::days(30),
                amount: Decimal::from(5000),
                impact_score: Decimal::from(80) / Decimal::from(100),
                category: FinancialCategory::Donations,
            },
            TimeSeriesPoint {
                timestamp: Utc::now() - Duration::days(15),
                amount: Decimal::from(10000),
                impact_score: Decimal::from(85) / Decimal::from(100),
                category: FinancialCategory::Grants,
            },
        ],
        top_contributors: vec![
            ContributorImpact {
                contributor_id: "edu_foundation_123".to_string(),
                name: "Local Education Foundation".to_string(),
                total_contributions: Decimal::from(15000),
                impact_score: Decimal::from(95) / Decimal::from(100),
                categories: vec![FinancialCategory::Education, FinancialCategory::Grants],
            },
        ],
        roi_metrics: vec![
            ROIMetric {
                investment_category: FinancialCategory::Education,
                return_category: FinancialCategory::CommunityDevelopment,
                investment_amount: Decimal::from(10000),
                return_amount: Decimal::from(25000),
                roi_percentage: Decimal::from(150),
                time_period: Duration::days(365),
            },
        ],
        sustainability_metrics: SustainabilityMetrics {
            monthly_recurring_revenue: Decimal::from(2000),
            donation_stability_index: Decimal::from(15) / Decimal::from(100),
            community_investment_ratio: Decimal::from(40) / Decimal::from(100),
            financial_health_score: Decimal::from(85) / Decimal::from(100),
        },
    }
}

/// Create sample report for demo
fn create_sample_report(
    start_time: chrono::DateTime<Utc>,
    end_time: chrono::DateTime<Utc>,
    analytics: financial_impact_tracker::analytics::FinancialImpactAnalytics,
) -> financial_impact_tracker::reporting::FinancialImpactReport {
    use financial_impact_tracker::reporting::*;
    
    FinancialImpactReport {
        generated_at: Utc::now(),
        period_start: start_time,
        period_end: end_time,
        analytics,
        summary: ReportSummary {
            total_financial_impact: Decimal::from(50000),
            impact_trend: ImpactTrend::Increasing(Decimal::from(15)),
            key_insights: vec![
                "25% increase in community investment compared to last month".to_string(),
                "Education funding shows strong ROI at 150%".to_string(),
                "Diversified donor base with 15 new contributors".to_string(),
            ],
            community_engagement_score: Decimal::from(88) / Decimal::from(100),
        },
        recommendations: vec![
            Recommendation {
                id: "expand_education".to_string(),
                title: "Expand Education Programs".to_string(),
                description: "Education investments show high returns. Consider expanding literacy initiatives.".to_string(),
                priority: RecommendationPriority::High,
                impact_estimate: Decimal::from(30),
                implementation_effort: ImplementationEffort::Medium,
            },
        ],
        visualizations: vec![
            VisualizationData {
                id: "category_breakdown".to_string(),
                title: "Financial Impact by Category".to_string(),
                data: serde_json::Value::Null,
                chart_type: ChartType::Pie,
                description: "Distribution of financial impact across categories".to_string(),
            },
        ],
    }
}