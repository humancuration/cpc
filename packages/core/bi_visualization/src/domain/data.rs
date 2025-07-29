//! Data models
//! 
//! This module defines the core data structures for visualization.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Data point for time series
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesPoint {
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Value
    pub value: f64,
}

impl TimeSeriesPoint {
    /// Create a new time series point
    pub fn new(timestamp: DateTime<Utc>, value: f64) -> Self {
        Self { timestamp, value }
    }
}

/// Data point for scatter plot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScatterPoint {
    /// X coordinate
    pub x: f64,
    
    /// Y coordinate
    pub y: f64,
    
    /// Optional label
    pub label: Option<String>,
}

impl ScatterPoint {
    /// Create a new scatter point
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y, label: None }
    }
    
    /// Create a new scatter point with label
    pub fn new_with_label(x: f64, y: f64, label: String) -> Self {
        Self { x, y, label: Some(label) }
    }
}

/// Data point for heatmap
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeatmapPoint {
    /// X coordinate
    pub x: i32,
    
    /// Y coordinate
    pub y: i32,
    
    /// Value/intensity
    pub value: f64,
}

impl HeatmapPoint {
    /// Create a new heatmap point
    pub fn new(x: i32, y: i32, value: f64) -> Self {
        Self { x, y, value }
    }
}

/// Data series for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSeries {
    /// Name of the series
    pub name: String,
    
    /// Data points
    pub points: Vec<DataPoint>,
}

impl DataSeries {
    /// Create a new data series
    pub fn new(name: String, points: Vec<DataPoint>) -> Self {
        Self { name, points }
    }
    
    /// Create a data series from time series data
    pub fn from_time_series(name: String, points: Vec<TimeSeriesPoint>) -> Self {
        let data_points = points
            .into_iter()
            .map(|p| DataPoint::TimeSeries(p))
            .collect();
        Self::new(name, data_points)
    }
    
    /// Create a data series from scatter data
    pub fn from_scatter(name: String, points: Vec<ScatterPoint>) -> Self {
        let data_points = points
            .into_iter()
            .map(|p| DataPoint::Scatter(p))
            .collect();
        Self::new(name, data_points)
    }
    
    /// Create a data series from heatmap data
    pub fn from_heatmap(name: String, points: Vec<HeatmapPoint>) -> Self {
        let data_points = points
            .into_iter()
            .map(|p| DataPoint::Heatmap(p))
            .collect();
        Self::new(name, data_points)
    }
}

/// Data point enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataPoint {
    /// Time series point
    TimeSeries(TimeSeriesPoint),
    
    /// Scatter point
    Scatter(ScatterPoint),
    
    /// Heatmap point
    Heatmap(HeatmapPoint),
}

/// Processed data for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedData {
    /// Series of data
    pub series: Vec<DataSeries>,
    
    /// Metadata
    pub metadata: serde_json::Value,
}

impl ProcessedData {
    /// Create new processed data
    pub fn new(series: Vec<DataSeries>, metadata: serde_json::Value) -> Self {
        Self { series, metadata }
    }
}

/// Raw data for transformation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawData {
    /// Raw data values
    pub values: Vec<serde_json::Value>,
    
    /// Data source
    pub source: String,
}

impl RawData {
    /// Create new raw data
    pub fn new(values: Vec<serde_json::Value>, source: String) -> Self {
        Self { values, source }
    }
}