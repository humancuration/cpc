//! Visualization integration with the bi_visualization package

use polars::prelude::*;
use crate::error::AnalyticsError;
use crate::engine::AnalyticsEngine;

/// Visualization integration service
pub struct VisualizationIntegration {
    engine: AnalyticsEngine,
}

impl VisualizationIntegration {
    /// Create a new visualization integration service
    pub fn new(engine: AnalyticsEngine) -> Self {
        Self { engine }
    }
    
    /// Convert DataFrame to visualization data format
    pub fn to_visualization_data(
        &self,
        df: &DataFrame,
        chart_type: VisualizationChartType,
    ) -> Result<VisualizationData, AnalyticsError> {
        // Convert DataFrame to the format expected by bi_visualization
        let json_data = self.engine.dataframe_to_json(df)?;
        
        Ok(VisualizationData {
            data_json: json_data,
            chart_type,
            metadata: serde_json::Map::new(),
        })
    }
    
    /// Create interactive filtering capabilities
    pub fn create_interactive_filter(
        &self,
        df: &DataFrame,
        filter_spec: &FilterSpecification,
    ) -> Result<InteractiveFilter, AnalyticsError> {
        // Create interactive filtering metadata for visualization
        Ok(InteractiveFilter {
            filter_spec: filter_spec.clone(),
            available_columns: df.get_column_names(),
        })
    }
    
    /// Enable drill-down capability for hierarchical data
    pub fn enable_drill_down(
        &self,
        df: &DataFrame,
        hierarchy_columns: &[String],
    ) -> Result<DrillDownCapability, AnalyticsError> {
        // Create drill-down metadata for visualization
        Ok(DrillDownCapability {
            hierarchy_columns: hierarchy_columns.to_vec(),
            max_depth: hierarchy_columns.len(),
        })
    }
    
    /// Stream data for large datasets in web environments
    pub fn stream_data_for_web(
        &self,
        df: &DataFrame,
        chunk_size: usize,
    ) -> Result<DataStream, AnalyticsError> {
        // For web/WASM environments, chunk large datasets for progressive loading
        let total_rows = df.height();
        let chunks = (total_rows + chunk_size - 1) / chunk_size; // Ceiling division
        
        Ok(DataStream {
            total_rows,
            chunk_size,
            total_chunks: chunks,
            current_chunk: 0,
        })
    }
}

/// Types of visualization charts supported
#[derive(Debug, Clone)]
pub enum VisualizationChartType {
    BarChart,
    LineChart,
    PieChart,
    ScatterPlot,
    Heatmap,
    AreaChart,
    Table,
}

/// Visualization data structure for integration with bi_visualization
pub struct VisualizationData {
    pub data_json: String,
    pub chart_type: VisualizationChartType,
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

/// Specification for interactive filtering
#[derive(Debug, Clone)]
pub struct FilterSpecification {
    pub column_filters: std::collections::HashMap<String, ColumnFilter>,
    pub global_search: Option<String>,
}

/// Column filter specification
#[derive(Debug, Clone)]
pub enum ColumnFilter {
    /// Numeric range filter
    NumericRange { min: Option<f64>, max: Option<f64> },
    
    /// Categorical value filter
    Categorical { values: Vec<String> },
    
    /// Text search filter
    TextSearch { pattern: String, case_sensitive: bool },
}

/// Interactive filter metadata
pub struct InteractiveFilter {
    pub filter_spec: FilterSpecification,
    pub available_columns: Vec<&'static str>,
}

/// Drill-down capability for hierarchical data
pub struct DrillDownCapability {
    pub hierarchy_columns: Vec<String>,
    pub max_depth: usize,
}

/// Data stream for progressive loading in web environments
pub struct DataStream {
    pub total_rows: usize,
    pub chunk_size: usize,
    pub total_chunks: usize,
    pub current_chunk: usize,
}

impl DataStream {
    /// Get the next chunk of data
    pub fn next_chunk(&mut self, df: &DataFrame) -> Result<Option<DataFrame>, AnalyticsError> {
        if self.current_chunk >= self.total_chunks {
            return Ok(None);
        }
        
        let start_row = self.current_chunk * self.chunk_size;
        let end_row = std::cmp::min(start_row + self.chunk_size, self.total_rows);
        
        let chunk = df.slice(start_row as i64, end_row - start_row);
        self.current_chunk += 1;
        
        Ok(Some(chunk))
    }
    
    /// Reset the stream to the beginning
    pub fn reset(&mut self) {
        self.current_chunk = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::df;
    
    #[test]
    fn test_visualization_integration_creation() {
        let engine = AnalyticsEngine::new();
        let integration = VisualizationIntegration::new(engine);
        // Integration should be created successfully
        assert!(true);
    }
    
    #[test]
    fn test_to_visualization_data() {
        let engine = AnalyticsEngine::new();
        let integration = VisualizationIntegration::new(engine);
        
        let df = df![
            "category" => ["A", "B", "C"],
            "value" => [10, 20, 30]
        ].unwrap();
        
        let viz_data = integration.to_visualization_data(&df, VisualizationChartType::BarChart).unwrap();
        assert!(!viz_data.data_json.is_empty());
        // Data should be valid JSON
        assert!(serde_json::from_str::<serde_json::Value>(&viz_data.data_json).is_ok());
    }
    
    #[test]
    fn test_data_stream() {
        let engine = AnalyticsEngine::new();
        let integration = VisualizationIntegration::new(engine);
        
        let df = df![
            "id" => [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            "value" => [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]
        ].unwrap();
        
        let mut stream = integration.stream_data_for_web(&df, 3).unwrap();
        assert_eq!(stream.total_rows, 10);
        assert_eq!(stream.chunk_size, 3);
        assert_eq!(stream.total_chunks, 4);
        
        // Test chunking
        let chunk1 = stream.next_chunk(&df).unwrap().unwrap();
        assert_eq!(chunk1.height(), 3);
        
        let chunk2 = stream.next_chunk(&df).unwrap().unwrap();
        assert_eq!(chunk2.height(), 3);
        
        let chunk3 = stream.next_chunk(&df).unwrap().unwrap();
        assert_eq!(chunk3.height(), 3);
        
        let chunk4 = stream.next_chunk(&df).unwrap().unwrap();
        assert_eq!(chunk4.height(), 1);
        
        // No more chunks
        let no_chunk = stream.next_chunk(&df).unwrap();
        assert!(no_chunk.is_none());
    }
}