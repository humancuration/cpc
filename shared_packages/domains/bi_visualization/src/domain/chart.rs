//! Chart types & configurations
//! 
//! This module defines the core chart structures and configurations.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Chart type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChartType {
    /// Line chart
    Line,
    /// Bar chart
    Bar,
    /// Pie chart
    Pie,
    /// Scatter plot
    Scatter,
    /// Histogram
    Histogram,
    /// Heatmap
    Heatmap,
    /// Area chart
    Area,
}

impl fmt::Display for ChartType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChartType::Line => write!(f, "Line"),
            ChartType::Bar => write!(f, "Bar"),
            ChartType::Pie => write!(f, "Pie"),
            ChartType::Scatter => write!(f, "Scatter"),
            ChartType::Histogram => write!(f, "Histogram"),
            ChartType::Heatmap => write!(f, "Heatmap"),
            ChartType::Area => write!(f, "Area"),
        }
    }
}

/// Visualization theme
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VisualizationTheme {
    /// Light theme
    Light,
    /// Dark theme
    Dark,
    /// High contrast theme
    HighContrast,
}

impl fmt::Display for VisualizationTheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VisualizationTheme::Light => write!(f, "Light"),
            VisualizationTheme::Dark => write!(f, "Dark"),
            VisualizationTheme::HighContrast => write!(f, "HighContrast"),
        }
    }
}

/// Series configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesConfig {
    /// Name of the series
    pub name: String,
    
    /// Color of the series
    pub color: String,
    
    /// Whether to show in legend
    pub show_in_legend: bool,
}

impl SeriesConfig {
    /// Create a new series configuration
    pub fn new(name: &str, color: &str) -> Self {
        Self {
            name: name.to_string(),
            color: color.to_string(),
            show_in_legend: true,
        }
    }
}

/// Standardized chart configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartConfig {
    /// Type of chart
    pub chart_type: ChartType,
    
    /// Title of the chart
    pub title: String,
    
    /// Dimensions of the chart (width, height)
    pub dimensions: (u32, u32),
    
    /// Theme for the visualization
    pub theme: VisualizationTheme,
    
    /// Series configurations
    pub series: Vec<SeriesConfig>,
}

impl ChartConfig {
    /// Create a new chart configuration
    pub fn new(
        chart_type: ChartType,
        title: String,
        dimensions: (u32, u32),
        theme: VisualizationTheme,
        series: Vec<SeriesConfig>,
    ) -> Self {
        Self {
            chart_type,
            title,
            dimensions,
            theme,
            series,
        }
    }
}

/// Interactive chart configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveConfig {
    /// Type of chart
    pub chart_type: ChartType,
    
    /// Title of the chart
    pub title: String,
    
    /// Dimensions of the chart (width, height)
    pub dimensions: (u32, u32),
    
    /// Interactive elements to include
    pub interactive_elements: Vec<InteractiveElement>,
}

impl InteractiveConfig {
    /// Create a new interactive chart configuration
    pub fn new(
        chart_type: ChartType,
        title: String,
        dimensions: (u32, u32),
        interactive_elements: Vec<InteractiveElement>,
    ) -> Self {
        Self {
            chart_type,
            title,
            dimensions,
            interactive_elements,
        }
    }
}

/// Interactive element types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InteractiveElement {
    /// Tooltip on hover
    Tooltip,
    /// Zoom functionality
    Zoom,
    /// Pan functionality
    Pan,
    /// Selection functionality
    Selection,
}

impl fmt::Display for InteractiveElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InteractiveElement::Tooltip => write!(f, "Tooltip"),
            InteractiveElement::Zoom => write!(f, "Zoom"),
            InteractiveElement::Pan => write!(f, "Pan"),
            InteractiveElement::Selection => write!(f, "Selection"),
        }
    }
}