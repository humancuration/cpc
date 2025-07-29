//! Data transformation
//! 
//! This module handles the transformation of raw data into visualization-ready formats.

use crate::domain::{
    data::{RawData, ProcessedData, DataSeries, TimeSeriesPoint, ScatterPoint, HeatmapPoint},
    VisualizationError,
};
use chrono::{DateTime, Utc};

/// Data transformer for converting raw data to visualization formats
pub struct DataTransformer;

impl DataTransformer {
    /// Transform raw financial data to time series
    pub fn transform_financial_data(data: RawData) -> Result<ProcessedData, VisualizationError> {
        let mut series = Vec::new();
        
        // Extract time series data from financial records
        let mut points = Vec::new();
        
        for value in &data.values {
            if let Some(timestamp) = value.get("timestamp").and_then(|t| t.as_str()) {
                if let Some(amount) = value.get("amount").and_then(|a| a.as_f64()) {
                    if let Ok(ts) = timestamp.parse::<DateTime<Utc>>() {
                        points.push(TimeSeriesPoint::new(ts, amount));
                    }
                }
            }
        }
        
        series.push(DataSeries::from_time_series("Spending".to_string(), points));
        
        let metadata = serde_json::json!({
            "source": data.source,
            "data_type": "financial",
            "series_count": series.len(),
        });
        
        Ok(ProcessedData::new(series, metadata))
    }
    
    /// Transform raw health data to time series
    pub fn transform_health_data(data: RawData) -> Result<ProcessedData, VisualizationError> {
        let mut series = Vec::new();
        
        // Extract sleep quality data
        let mut sleep_points = Vec::new();
        // Extract heart rate data
        let mut heart_rate_points = Vec::new();
        
        for value in &data.values {
            if let Some(timestamp) = value.get("timestamp").and_then(|t| t.as_str()) {
                if let Ok(ts) = timestamp.parse::<DateTime<Utc>>() {
                    if let Some(sleep_quality) = value.get("sleep_quality").and_then(|s| s.as_f64()) {
                        sleep_points.push(TimeSeriesPoint::new(ts, sleep_quality));
                    }
                    
                    if let Some(heart_rate) = value.get("heart_rate").and_then(|h| h.as_f64()) {
                        heart_rate_points.push(TimeSeriesPoint::new(ts, heart_rate));
                    }
                }
            }
        }
        
        if !sleep_points.is_empty() {
            series.push(DataSeries::from_time_series("Sleep Quality".to_string(), sleep_points));
        }
        
        if !heart_rate_points.is_empty() {
            series.push(DataSeries::from_time_series("Heart Rate".to_string(), heart_rate_points));
        }
        
        let metadata = serde_json::json!({
            "source": data.source,
            "data_type": "health",
            "series_count": series.len(),
        });
        
        Ok(ProcessedData::new(series, metadata))
    }
    
    /// Transform raw data to scatter plot
    pub fn transform_to_scatter(data: RawData) -> Result<ProcessedData, VisualizationError> {
        let mut points = Vec::new();
        
        for value in &data.values {
            if let Some(x) = value.get("x").and_then(|x| x.as_f64()) {
                if let Some(y) = value.get("y").and_then(|y| y.as_f64()) {
                    let label = value.get("label").and_then(|l| l.as_str()).map(|s| s.to_string());
                    points.push(ScatterPoint { x, y, label });
                }
            }
        }
        
        let series = vec![DataSeries::from_scatter("Scatter Data".to_string(), points)];
        
        let metadata = serde_json::json!({
            "source": data.source,
            "data_type": "scatter",
            "point_count": points.len(),
        });
        
        Ok(ProcessedData::new(series, metadata))
    }
    
    /// Transform raw data to heatmap
    pub fn transform_to_heatmap(data: RawData) -> Result<ProcessedData, VisualizationError> {
        let mut points = Vec::new();
        
        for value in &data.values {
            if let Some(x) = value.get("x").and_then(|x| x.as_i64()) {
                if let Some(y) = value.get("y").and_then(|y| y.as_i64()) {
                    if let Some(value) = value.get("value").and_then(|v| v.as_f64()) {
                        points.push(HeatmapPoint::new(x as i32, y as i32, value));
                    }
                }
            }
        }
        
        let series = vec![DataSeries::from_heatmap("Heatmap Data".to_string(), points)];
        
        let metadata = serde_json::json!({
            "source": data.source,
            "data_type": "heatmap",
            "point_count": points.len(),
        });
        
        Ok(ProcessedData::new(series, metadata))
    }
    
    /// Aggregate time series data
    pub fn aggregate_time_series(
        data: DataSeries,
        aggregation: TimeAggregation,
    ) -> Result<DataSeries, VisualizationError> {
        // In a real implementation, this would aggregate time series data
        // For now, we'll just return the original data
        Ok(data)
    }
}

/// Time aggregation types
#[derive(Debug, Clone)]
pub enum TimeAggregation {
    /// Hourly aggregation
    Hourly,
    
    /// Daily aggregation
    Daily,
    
    /// Weekly aggregation
    Weekly,
    
    /// Monthly aggregation
    Monthly,
    
    /// Yearly aggregation
    Yearly,
}