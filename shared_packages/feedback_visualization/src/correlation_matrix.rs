//! Correlation matrix visualization component

use feedback_core::FeedbackError;
use plotters::prelude::*;
use std::collections::HashMap;

/// Correlation matrix visualization component
pub struct CorrelationMatrix {
    title: String,
}

impl CorrelationMatrix {
    /// Create a new correlation matrix with the specified title
    pub fn new(title: String) -> Self {
        Self { title }
    }
    
    /// Generate an interactive HTML representation of the correlation matrix
    pub fn render_html(&self, correlations: &HashMap<String, HashMap<String, f32>>) -> Result<String, FeedbackError> {
        if correlations.is_empty() {
            return Ok("<div>No correlation data available</div>".to_string());
        }
        
        let mut html = String::new();
        html.push_str(&format!("<h2>{}</h2>\n", self.title));
        html.push_str("<table border=\"1\" style=\"border-collapse: collapse;\">\n");
        
        // Create header row
        html.push_str("  <tr>\n    <th></th>\n");
        let metrics: Vec<&String> = correlations.keys().collect();
        for metric in &metrics {
            html.push_str(&format!("    <th>{}</th>\n", metric));
        }
        html.push_str("  </tr>\n");
        
        // Create data rows
        for (i, row_metric) in metrics.iter().enumerate() {
            html.push_str("  <tr>\n");
            html.push_str(&format!("    <th>{}</th>\n", row_metric));
            
            if let Some(row_data) = correlations.get(*row_metric) {
                for col_metric in &metrics {
                    if let Some(&correlation) = row_data.get(*col_metric) {
                        // Color code the correlation value
                        let color = self.get_color_for_correlation(correlation);
                        html.push_str(&format!(
                            "    <td style=\"background-color: {}; text-align: center;\">{:.2}</td>\n",
                            color, correlation
                        ));
                    } else {
                        html.push_str("    <td style=\"text-align: center;\">-</td>\n");
                    }
                }
            } else {
                // Fill with empty cells if no data for this row
                for _ in &metrics {
                    html.push_str("    <td style=\"text-align: center;\">-</td>\n");
                }
            }
            
            html.push_str("  </tr>\n");
        }
        
        html.push_str("</table>\n");
        
        Ok(html)
    }
    
    /// Get color based on correlation value
    fn get_color_for_correlation(&self, correlation: f32) -> String {
        // Simple color mapping:
        // -1.0 to -0.5: Dark red
        // -0.5 to 0.0: Light red
        // 0.0 to 0.5: Light blue
        // 0.5 to 1.0: Dark blue
        match correlation {
            c if c <= -0.5 => "#ff6666".to_string(),   // Dark red
            c if c <= 0.0 => "#ffcccc".to_string(),    // Light red
            c if c < 0.5 => "#ccccff".to_string(),     // Light blue
            _ => "#6666ff".to_string(),                // Dark blue
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_correlation_matrix_creation() {
        let matrix = CorrelationMatrix::new("Test Correlation Matrix".to_string());
        assert_eq!(matrix.title, "Test Correlation Matrix");
    }

    #[test]
    fn test_correlation_matrix_empty() {
        let matrix = CorrelationMatrix::new("Test Correlation Matrix".to_string());
        let correlations = HashMap::new();
        let result = matrix.render_html(&correlations).unwrap();
        assert!(result.contains("No correlation data available"));
    }

    #[test]
    fn test_correlation_matrix_with_data() {
        let matrix = CorrelationMatrix::new("Test Correlation Matrix".to_string());
        
        let mut correlations = HashMap::new();
        let mut row1 = HashMap::new();
        row1.insert("Metric B".to_string(), 0.8);
        correlations.insert("Metric A".to_string(), row1);
        
        let result = matrix.render_html(&correlations).unwrap();
        assert!(result.contains("<table"));
        assert!(result.contains("Metric A"));
        assert!(result.contains("Metric B"));
        assert!(result.contains("0.80"));
    }
}