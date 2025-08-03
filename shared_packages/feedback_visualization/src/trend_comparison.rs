//! Trend comparison visualization component

use feedback_analysis::TrendResult;
use feedback_core::FeedbackError;
use plotters::prelude::*;
use chrono::NaiveDate;

/// Trend comparison visualization component
pub struct TrendComparison {
    title: String,
}

impl TrendComparison {
    /// Create a new trend comparison with the specified title
    pub fn new(title: String) -> Self {
        Self { title }
    }
    
    /// Generate an SVG vector image of the trend comparison
    pub fn render_svg(&self, trends: &[TrendResult]) -> Result<String, FeedbackError> {
        // Create an SVG buffer
        let mut buffer = String::new();
        
        {
            let root = SVGBackend::new(&mut buffer, (800, 600)).into_drawing_area();
            root.fill(&WHITE).map_err(|e| FeedbackError::Visualization(e.to_string()))?;
            
            if trends.is_empty() || trends.iter().all(|t| t.is_empty()) {
                return Ok(buffer);
            }
            
            // Find the overall min and max values for scaling
            let mut all_averages = Vec::new();
            for trend in trends {
                all_averages.extend(&trend.averages);
            }
            
            if all_averages.is_empty() {
                return Ok(buffer);
            }
            
            let min_value = *all_averages.iter().fold(f32::INFINITY, |a, &b| a.min(b));
            let max_value = *all_averages.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
            
            // For simplicity, we'll just use indices as x-axis values
            // In a real implementation, we'd parse the period strings as dates
            let max_periods = trends.iter().map(|t| t.periods.len()).max().unwrap_or(0);
            
            let mut chart = ChartBuilder::on(&root)
                .caption(&self.title, ("sans-serif", 30))
                .margin(10)
                .x_label_area_size(40)
                .y_label_area_size(40)
                .build_cartesian_2d(0..max_periods, min_value..max_value)
                .map_err(|e| FeedbackError::Visualization(e.to_string()))?;
            
            chart.configure_mesh().draw().map_err(|e| FeedbackError::Visualization(e.to_string()))?;
            
            // Draw trend lines for each trend result
            let colors = [RED, BLUE, GREEN, ORANGE, PURPLE];
            for (i, trend) in trends.iter().enumerate() {
                if trend.is_empty() {
                    continue;
                }
                
                let color = colors[i % colors.len()];
                let data: Vec<(usize, f32)> = trend.averages.iter().enumerate().map(|(i, &v)| (i, v)).collect();
                
                chart.draw_series(LineSeries::new(data.iter().map(|&(i, v)| (i, v)), &color))
                    .map_err(|e| FeedbackError::Visualization(e.to_string()))?
                    .label(format!("Trend {}", i + 1))
                    .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &color));
            }
            
            chart.configure_series_labels()
                .background_style(WHITE.mix(0.8))
                .border_style(BLACK)
                .draw()
                .map_err(|e| FeedbackError::Visualization(e.to_string()))?;
            
            root.present().map_err(|e| FeedbackError::Visualization(e.to_string()))?;
        }
        
        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use feedback_analysis::TrendResult;

    #[test]
    fn test_trend_comparison_creation() {
        let comparison = TrendComparison::new("Test Trend Comparison".to_string());
        assert_eq!(comparison.title, "Test Trend Comparison");
    }

    #[test]
    fn test_trend_comparison_empty() {
        let comparison = TrendComparison::new("Test Trend Comparison".to_string());
        let trends = vec![];
        let result = comparison.render_svg(&trends).unwrap();
        assert!(!result.is_empty()); // Should still produce SVG structure
    }
}