use crate::domain::{Sheet, ChartSpec, CellAddress, CellValue};
use cpc_core::bi_visualization::{
    VisualizationService,
    ChartConfig,
    VisualizationTheme,
    SeriesConfig,
    DataSeries,
    TimeSeriesPoint,
};
use image::DynamicImage;

/// Service for generating charts from sheet data
pub struct ChartService;

impl ChartService {
    pub fn new() -> Self {
        Self
    }
    
    /// Generate a chart from sheet data
    pub fn generate_chart(
        &self,
        sheet: &Sheet,
        chart_spec: &ChartSpec,
    ) -> Result<DynamicImage, Box<dyn std::error::Error>> {
        // Transform sheet data to visualization format
        let data_series = self.transform_sheet_data(sheet, &chart_spec.data_range)?;
        
        // Create chart configuration
        let config = ChartConfig::new(
            chart_spec.chart_type.clone(),
            chart_spec.title.clone(),
            (chart_spec.options.width, chart_spec.options.height),
            VisualizationTheme::Spreadsheet,
            chart_spec.series_config.iter()
                .map(|sc| SeriesConfig::new(sc.name.clone(), sc.color.clone()))
                .collect(),
        );
        
        // Generate chart using BI Visualization Toolkit
        let chart_image = VisualizationService::generate_chart(config, data_series)?;
        
        Ok(chart_image)
    }
    
    /// Transform sheet data to visualization format
    fn transform_sheet_data(
        &self,
        sheet: &Sheet,
        range: &crate::domain::CellRange,
    ) -> Result<DataSeries, Box<dyn std::error::Error>> {
        // Extract column headers (assuming first row contains headers)
        let headers = self.extract_headers(sheet, range);
        
        // Extract data points
        let mut series_data = Vec::new();
        
        // For simplicity, we'll assume the first column contains labels/timestamps
        // and subsequent columns contain data series
        for row in range.start.row + 1..=range.end.row {
            // Extract label from first column
            let label_address = CellAddress::new(row, range.start.column);
            let label_value = if let Some(cell) = sheet.get_cell(&label_address) {
                match &cell.value {
                    CellValue::Text(s) => s.clone(),
                    CellValue::Number(n) => n.to_string(),
                    CellValue::DateTime(dt) => dt.format("%Y-%m-%d").to_string(),
                    _ => format!("Row {}", row),
                }
            } else {
                format!("Row {}", row)
            };
            
            // Extract data from subsequent columns
            for (col_offset, header) in headers.iter().enumerate().skip(1) {
                let col = range.start.column + col_offset as u32;
                if col <= range.end.column {
                    let value_address = CellAddress::new(row, col);
                    let value = if let Some(cell) = sheet.get_cell(&value_address) {
                        match &cell.value {
                            CellValue::Number(n) => *n,
                            CellValue::Text(s) => s.parse().unwrap_or(0.0),
                            _ => 0.0,
                        }
                    } else {
                        0.0
                    };
                    
                    // For time series data, we would need a proper timestamp
                    // For now, we'll use the row number as a simple proxy
                    let timestamp = row as f64;
                    series_data.push(TimeSeriesPoint::new(timestamp, value));
                }
            }
        }
        
        Ok(DataSeries::from_time_series("Sheet Data".to_string(), series_data))
    }
    
    /// Extract headers from the first row of a range
    fn extract_headers(&self, sheet: &Sheet, range: &crate::domain::CellRange) -> Vec<String> {
        let mut headers = Vec::new();
        
        for col in range.start.column..=range.end.column {
            let address = CellAddress::new(range.start.row, col);
            let header = if let Some(cell) = sheet.get_cell(&address) {
                match &cell.value {
                    CellValue::Text(s) => s.clone(),
                    CellValue::Number(n) => n.to_string(),
                    _ => format!("Column {}", col),
                }
            } else {
                format!("Column {}", col)
            };
            headers.push(header);
        }
        
        headers
    }
}