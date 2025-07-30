//! Example demonstrating the Bevy visualization system for BI analytics

use cpc_bi_analytics::presentation::bevy_visualization::BiVisualizationApp;
use cpc_bi_analytics::domain::report::{Report, VisualizationType};
use chrono::Utc;

fn main() {
    println!("🚀 Starting BI Analytics Bevy Visualization Demo");
    
    // Create sample reports for demonstration
    let reports = vec![
        create_sample_bar_chart(),
        create_sample_line_chart(),
        create_sample_pie_chart(),
    ];
    
    // Create visualization app
    let mut app = BiVisualizationApp::new();
    
    // Add all reports
    for report in reports {
        println!("📊 Adding visualization: {}", report.title);
        app.add_report_visualization(&report);
    }
    
    // Run the app
    app.run();
}

fn create_sample_bar_chart() -> Report {
    Report {
        id: "bar-chart-1".to_string(),
        title: "Monthly Sales Revenue".to_string(),
        description: "Revenue comparison across months".to_string(),
        query: "SELECT month, revenue FROM sales_data".to_string(),
        data_json: r#"[
            {"x": "Jan", "y": 45000},
            {"x": "Feb", "y": 52000},
            {"x": "Mar", "y": 48000},
            {"x": "Apr", "y": 61000},
            {"x": "May", "y": 55000},
            {"x": "Jun", "y": 67000}
        ]"#.to_string(),
        visualization_type: VisualizationType::BarChart,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn create_sample_line_chart() -> Report {
    Report {
        id: "line-chart-1".to_string(),
        title: "User Growth Over Time".to_string(),
        description: "Daily active users trend".to_string(),
        query: "SELECT day, active_users FROM user_metrics".to_string(),
        data_json: r#"[
            {"x": 1, "y": 100},
            {"x": 2, "y": 150},
            {"x": 3, "y": 200},
            {"x": 4, "y": 180},
            {"x": 5, "y": 220},
            {"x": 6, "y": 280},
            {"x": 7, "y": 350}
        ]"#.to_string(),
        visualization_type: VisualizationType::LineChart,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn create_sample_pie_chart() -> Report {
    Report {
        id: "pie-chart-1".to_string(),
        title: "Market Share Distribution".to_string(),
        description: "Market share by product category".to_string(),
        query: "SELECT category, share FROM market_data".to_string(),
        data_json: r#"[
            {"label": "Electronics", "value": 35},
            {"label": "Clothing", "value": 25},
            {"label": "Home & Garden", "value": 20},
            {"label": "Sports", "value": 12},
            {"label": "Other", "value": 8}
        ]"#.to_string(),
        visualization_type: VisualizationType::PieChart,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}