//! Finance module integration example for the BI visualization toolkit

use bi_visualization::{
    VisualizationService,
    ChartConfig,
    ChartType,
    VisualizationTheme,
    SeriesConfig,
    DataSeries,
    TimeSeriesPoint,
    infrastructure::export::{VisualizationExporter, ExportFormat},
};
use image::ImageBuffer;
use chrono::{Utc, Duration};

/// Finance service that uses the BI visualization toolkit
struct FinanceService;

impl FinanceService {
    /// Generate monthly spending chart
    fn generate_monthly_spending_chart(&self) -> Result<ImageBuffer<image::Rgba<u8>, Vec<u8>>, Box<dyn std::error::Error>> {
        // Generate monthly spending chart
        let chart_config = ChartConfig {
            chart_type: ChartType::Line,
            title: "Monthly Spending".to_string(),
            dimensions: (800, 600),
            theme: VisualizationTheme::Dark,
            series: vec![
                SeriesConfig::new("Groceries", "#FF0000"),
                SeriesConfig::new("Transport", "#0000FF"),
                SeriesConfig::new("Entertainment", "#00FF00"),
            ],
        };
        
        // Create sample spending data
        let mut groceries_points = Vec::new();
        let mut transport_points = Vec::new();
        let mut entertainment_points = Vec::new();
        
        for i in 0..30 {
            let timestamp = Utc::now() - Duration::days(30 - i);
            groceries_points.push(TimeSeriesPoint::new(timestamp, 50.0 + (i as f64 * 2.0) % 100.0));
            transport_points.push(TimeSeriesPoint::new(timestamp, 30.0 + (i as f64 * 1.5) % 80.0));
            entertainment_points.push(TimeSeriesPoint::new(timestamp, 20.0 + (i as f64 * 1.0) % 60.0));
        }
        
        let groceries_series = DataSeries::from_time_series("Groceries".to_string(), groceries_points);
        let transport_series = DataSeries::from_time_series("Transport".to_string(), transport_points);
        let entertainment_series = DataSeries::from_time_series("Entertainment".to_string(), entertainment_points);
        
        // For this example, we'll just use the first series
        // In a real implementation, we would combine multiple series
        let chart_image = VisualizationService::generate_chart(chart_config, groceries_series)?;
        
        Ok(chart_image)
    }
    
    /// Save chart as image
    fn save_spending_chart(&self) -> Result<(), Box<dyn std::error::Error>> {
        let chart_image = self.generate_monthly_spending_chart()?;
        
        // Export to PNG
        VisualizationExporter::export_image(
            &chart_image,
            ExportFormat::Png,
            "spending_trend.png",
        )?;
        
        println!("Spending trend chart saved as spending_trend.png");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create finance service
    let finance_service = FinanceService;
    
    // Generate and save spending chart
    finance_service.save_spending_chart()?;
    
    println!("Finance integration example completed successfully!");
    Ok(())
}