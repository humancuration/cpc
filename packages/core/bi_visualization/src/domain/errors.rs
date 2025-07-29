//! Error types for the BI visualization toolkit

/// Error type for visualization operations
#[derive(thiserror::Error, Debug)]
pub enum VisualizationError {
    #[error("Chart generation failed: {0}")]
    ChartGenerationError(String),
    
    #[error("Data transformation failed: {0}")]
    DataTransformationError(String),
    
    #[error("Invalid chart configuration: {0}")]
    InvalidConfiguration(String),
    
    #[error("Unsupported chart type: {0}")]
    UnsupportedChartType(String),
    
    #[error("Data series not found")]
    DataSeriesNotFound,
    
    #[error("Export failed: {0}")]
    ExportError(String),
    
    #[error("Rendering failed: {0}")]
    RenderingError(String),
}