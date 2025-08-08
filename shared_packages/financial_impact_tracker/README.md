# Financial Impact Tracker

A comprehensive Rust library for tracking, measuring, and reporting on the financial impact of community activities and initiatives within the CPC ecosystem.

## Overview

The Financial Impact Tracker provides tools to:

- Track financial transactions and their community impact
- Analyze financial data to identify trends and insights
- Generate detailed reports with visualizations
- Integrate with other CPC systems (cpay_core, cpc-financial-core)
- Link financial activities to causes, volunteer work, and learning outcomes

## Features

### Financial Impact Tracking
- Record financial transactions with impact scoring
- Categorize financial activities (Donations, Grants, Investments, etc.)
- Track community trade and service payments
- Privacy-preserving data collection

### Advanced Analytics
- Category-based impact breakdown
- Time series analysis of financial trends
- Top contributor identification
- Return on Investment (ROI) metrics
- Sustainability metrics calculation

### Reporting & Visualization
- Comprehensive financial impact reports
- Customizable visualizations (charts, graphs, heatmaps)
- Export reports in multiple formats (JSON, CSV)
- Automated report generation

### Integration Capabilities
- Seamless integration with cpay_core for transaction tracking
- Linking financial activities to causes, volunteer work, and learning
- Real-time impact tracking
- Configurable integration settings

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
financial_impact_tracker = { path = "../shared_packages/financial_impact_tracker" }
```

## Usage

### Basic Setup

```rust
use financial_impact_tracker::{FinancialImpactTracker, FinancialAnalytics, FinancialReportGenerator};
use sqlx::PgPool;

// Initialize the tracker
let db_pool = PgPool::connect("postgresql://user:pass@localhost/db").await?;
let tracker = FinancialImpactTracker::new(db_pool);

// Initialize analytics engine
let analytics = FinancialAnalytics::new(tracker.clone());

// Initialize report generator
let report_generator = FinancialReportGenerator::new(analytics);
```

### Tracking Financial Impact

```rust
use financial_impact_tracker::{FinancialEventType, FinancialCategory};
use rust_decimal::Decimal;
use chrono::Utc;

// Record a financial event impact
let impact_record = tracker.record_event_impact(
    FinancialEventType::Donation,
    Decimal::from(1000),
    "USD".to_string(),
    FinancialCategory::Donations,
    "Community donation".to_string(),
    Decimal::from(95) / Decimal::from(100), // 95% impact score
    serde_json::json!({"donor": "community_member_123"})
).await?;
```

### Generating Analytics

```rust
use chrono::{Utc, Duration};

let start_time = Utc::now() - Duration::days(30);
let end_time = Utc::now();

let analytics = analytics.generate_impact_analytics(start_time, end_time).await?;
println!("Total financial impact: {}", analytics.total_impact);
```

### Creating Reports

```rust
let report = report_generator.generate_report(start_time, end_time).await?;
println!("Report generated at: {}", report.generated_at);

// Export as JSON
let json_report = report_generator.export_report(&report, ExportFormat::Json).await?;
```

## Integration with Other Systems

### CPay Core Integration

```rust
use financial_impact_tracker::FinancialIntegration;
use cpay_core::CPayCore;
use cpc_financial_core::CPCFinancialCore;

let cpay_core = CPayCore::new(/* config */);
let financial_core = CPCFinancialCore::new(/* config */);

let integration = FinancialIntegration::new(
    tracker,
    cpay_core,
    financial_core,
    FinancialIntegrationConfig {
        enable_realtime_tracking: true,
        enable_cause_linking: true,
        enable_volunteer_linking: true,
        enable_learning_linking: true,
        auto_generate_reports: true,
        report_frequency_days: 7,
    }
);

// Process transactions and track their impact
integration.process_transaction(&transaction, category, metadata).await?;
```

## Modules

- `tracker` - Core financial impact tracking functionality
- `analytics` - Advanced financial analytics and metrics
- `reporting` - Report generation and visualization
- `integration` - Integration with other CPC systems

## License

This project is licensed under the CPC License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please read our contributing guidelines before submitting pull requests.