use plotters::prelude::*;
use crate::domain::analytics::TimeSeriesPoint;
use std::io::Cursor;

pub struct ChartService;

impl ChartService {
    pub fn new() -> Self {
        Self
    }
    
    pub fn generate_time_series_chart(
        &self,
        data: &[TimeSeriesPoint],
        title: &str,
        width: u32,
        height: u32,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut buffer = vec![0; (width * height * 3) as usize];
        let root = BitMapBackend::with_buffer(&mut buffer, (width, height)).into_drawing_area();
        
        root.fill(&WHITE)?;
        
        let mut chart = ChartBuilder::on(&root)
            .caption(title, ("sans-serif", 20))
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(
                data.iter().map(|p| p.timestamp).min().unwrap()..data.iter().map(|p| p.timestamp).max().unwrap(),
                0..data.iter().map(|p| p.value).max().unwrap_or(10),
            )?;
        
        chart.configure_mesh().draw()?;
        
        chart.draw_series(LineSeries::new(
            data.iter().map(|p| (p.timestamp, p.value)),
            &RED,
        ))?;
        
        root.present()?;
        
        Ok(buffer)
    }
}