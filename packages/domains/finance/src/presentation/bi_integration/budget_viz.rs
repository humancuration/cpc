//! Budget visualization using the BI Visualization Toolkit

use cpc_core::bi_visualization::{
    VisualizationService,
    ChartConfig,
    ChartType,
    VisualizationTheme,
    SeriesConfig,
    DataSeries,
    TimeSeriesPoint,
};
use crate::domain::{Budget, Expense};
use image::DynamicImage;

/// Budget visualization service
pub struct BudgetVisualization;

impl BudgetVisualization {
    /// Generate a budget vs actual chart
    pub fn generate_budget_vs_actual(
        budget: &Budget,
        expenses: &[Expense],
    ) -> Result<DynamicImage, Box<dyn std::error::Error>> {
        // Configure chart
        let config = ChartConfig::new(
            ChartType::Line,
            format!("Budget vs Actual: {}", budget.name),
            (800, 600),
            VisualizationTheme::Finance, // Custom finance theme
            vec![
                SeriesConfig::new("Budget", "#4CAF50"),
                SeriesConfig::new("Actual", "#FF5252"),
            ],
        );

        // Transform domain data to visualization format
        let budget_points = vec![
            TimeSeriesPoint::new(budget.period.start, budget.amount.amount),
            TimeSeriesPoint::new(budget.period.end, budget.amount.amount),
        ];

        let actual_points: Vec<TimeSeriesPoint> = expenses.iter()
            .map(|e| TimeSeriesPoint::new(e.date, e.amount.amount))
            .collect();

        let budget_series = DataSeries::from_time_series("Budget".to_string(), budget_points);
        let actual_series = DataSeries::from_time_series("Actual".to_string(), actual_points);

        // Generate both series in a single chart
        VisualizationService::generate_multi_series_chart(
            config,
            vec![budget_series, actual_series]
        )
    }
    
    /// Generate a category budget distribution chart
    pub fn generate_category_distribution(
        budgets: &[Budget],
    ) -> Result<DynamicImage, Box<dyn std::error::Error>> {
        // Configure chart
        let config = ChartConfig::new(
            ChartType::Pie,
            "Budget Distribution by Category".to_string(),
            (800, 600),
            VisualizationTheme::Finance,
            vec![], // No series config needed for pie charts
        );

        // Transform domain data to visualization format
        let pie_points: Vec<(String, f64)> = budgets.iter()
            .map(|b| (b.category.to_string(), b.amount.amount))
            .collect();

        let data_series = DataSeries::from_pie("Budget Distribution".to_string(), pie_points);

        // Generate chart
        VisualizationService::generate_chart(config, data_series)
    }
}