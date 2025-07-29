use serde::{Deserialize, Serialize};
use uuid::Uuid;
use cpc_core::bi_visualization::ChartType;
use super::cell::CellAddress;

/// Chart specification for visualizing spreadsheet data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartSpec {
    pub id: Uuid,
    pub title: String,
    pub chart_type: ChartType,
    pub data_range: CellRange,
    pub series_config: Vec<SeriesConfig>,
    pub options: ChartOptions,
}

/// Cell range in a spreadsheet
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CellRange {
    pub start: CellAddress,
    pub end: CellAddress,
}

/// Series configuration for chart data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesConfig {
    pub name: String,
    pub color: String,
    pub data_column: u32,
}

/// Chart options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartOptions {
    pub show_legend: bool,
    pub show_grid: bool,
    pub show_labels: bool,
    pub width: u32,
    pub height: u32,
}

impl Default for ChartOptions {
    fn default() -> Self {
        Self {
            show_legend: true,
            show_grid: true,
            show_labels: true,
            width: 800,
            height: 600,
        }
    }
}

impl ChartSpec {
    pub fn new(
        title: String,
        chart_type: ChartType,
        data_range: CellRange,
        series_config: Vec<SeriesConfig>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            chart_type,
            data_range,
            series_config,
            options: ChartOptions::default(),
        }
    }
    
    pub fn update_title(&mut self, title: String) {
        self.title = title;
    }
    
    pub fn update_chart_type(&mut self, chart_type: ChartType) {
        self.chart_type = chart_type;
    }
    
    pub fn update_data_range(&mut self, data_range: CellRange) {
        self.data_range = data_range;
    }
    
    pub fn update_series_config(&mut self, series_config: Vec<SeriesConfig>) {
        self.series_config = series_config;
    }
    
    pub fn update_options(&mut self, options: ChartOptions) {
        self.options = options;
    }
}