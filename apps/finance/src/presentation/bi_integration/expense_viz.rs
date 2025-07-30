//! Expense visualization using the BI Visualization Toolkit

use cpc_core::bi_visualization::{
    VisualizationService,
    ChartConfig,
    ChartType,
    VisualizationTheme,
    SeriesConfig,
    DataSeries,
    TimeSeriesPoint,
};
use crate::domain::{Expense, FinancialCategory};
use image::DynamicImage;

/// Expense visualization service
pub struct ExpenseVisualization;

impl ExpenseVisualization {
    /// Generate an expense trend chart over time
    pub fn generate_expense_trend(
        expenses: &[Expense],
        category: Option<FinancialCategory>,
    ) -> Result<DynamicImage, Box<dyn std::error::Error>> {
        // Filter expenses by category if specified
        let filtered_expenses: Vec<&Expense> = if let Some(cat) = category {
            expenses.iter().filter(|e| e.category == cat).collect()
        } else {
            expenses.iter().collect()
        };

        // Configure chart
        let config = ChartConfig::new(
            ChartType::Line,
            if category.is_some() {
                format!("Expense Trend: {}", category.unwrap())
            } else {
                "Overall Expense Trend".to_string()
            },
            (800, 600),
            VisualizationTheme::Finance,
            vec![
                SeriesConfig::new("Expenses", "#FF5252"),
            ],
        );

        // Transform domain data to visualization format
        let mut points: Vec<TimeSeriesPoint> = filtered_expenses.iter()
            .map(|e| TimeSeriesPoint::new(e.date, e.amount.amount))
            .collect();
        
        // Sort by date
        points.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        let data_series = DataSeries::from_time_series("Expenses".to_string(), points);

        // Generate chart
        VisualizationService::generate_chart(config, data_series)
    }
    
    /// Generate a category expense distribution chart
    pub fn generate_category_distribution(
        expenses: &[Expense],
    ) -> Result<DynamicImage, Box<dyn std::error::Error>> {
        // Group expenses by category
        let mut category_totals: std::collections::HashMap<FinancialCategory, f64> = std::collections::HashMap::new();
        
        for expense in expenses {
            *category_totals.entry(expense.category.clone()).or_insert(0.0) += expense.amount.amount;
        }

        // Configure chart
        let config = ChartConfig::new(
            ChartType::Pie,
            "Expense Distribution by Category".to_string(),
            (800, 600),
            VisualizationTheme::Finance,
            vec![], // No series config needed for pie charts
        );

        // Transform domain data to visualization format
        let pie_points: Vec<(String, f64)> = category_totals
            .into_iter()
            .map(|(category, total)| (category.to_string(), total))
            .collect();

        let data_series = DataSeries::from_pie("Expense Distribution".to_string(), pie_points);

        // Generate chart
        VisualizationService::generate_chart(config, data_series)
    }
    
    /// Generate a monthly expense comparison chart
    pub fn generate_monthly_comparison(
        expenses: &[Expense],
    ) -> Result<DynamicImage, Box<dyn std::error::Error>> {
        // Group expenses by month
        let mut monthly_totals: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
        
        for expense in expenses {
            let month_key = expense.date.format("%Y-%m").to_string();
            *monthly_totals.entry(month_key).or_insert(0.0) += expense.amount.amount;
        }

        // Configure chart
        let config = ChartConfig::new(
            ChartType::Bar,
            "Monthly Expense Comparison".to_string(),
            (800, 600),
            VisualizationTheme::Finance,
            vec![
                SeriesConfig::new("Monthly Expenses", "#FF5252"),
            ],
        );

        // Transform domain data to visualization format
        let mut bar_points: Vec<(String, f64)> = monthly_totals
            .into_iter()
            .collect();
        
        // Sort by month
        bar_points.sort_by(|a, b| a.0.cmp(&b.0));

        let data_series = DataSeries::from_bar("Monthly Expenses".to_string(), bar_points);

        // Generate chart
        VisualizationService::generate_chart(config, data_series)
    }
}