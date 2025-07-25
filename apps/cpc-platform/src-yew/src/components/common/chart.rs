use yew::prelude::*;
use plotters::prelude::*;
use web_sys::HtmlCanvasElement;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ChartType {
    Line,
    Bar,
    Pie,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ChartProps {
    pub chart_type: ChartType,
    pub data: Vec<(String, f64)>,
    pub title: String,
    pub width: u32,
    pub height: u32,
}

#[function_component(Chart)]
pub fn chart(props: &ChartProps) -> Html {
    let canvas_ref = use_node_ref();
    let data = props.data.clone();
    let chart_type = props.chart_type.clone();
    let title = props.title.clone();
    let width = props.width;
    let height = props.height;

    use_effect_with_deps(
        move |(canvas_ref, data, chart_type, title, width, height)| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                let ctx = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<web_sys::CanvasRenderingContext2d>()
                    .unwrap();

                let backend = CanvasBackend::with_canvas_object(canvas).unwrap();
                let root = backend.into_drawing_area();
                root.fill(&WHITE).unwrap();

                match chart_type {
                    ChartType::Line => draw_line_chart(&root, &data, &title, *width, *height),
                    ChartType::Bar => draw_bar_chart(&root, &data, &title, *width, *height),
                    ChartType::Pie => draw_pie_chart(&root, &data, &title, *width, *height),
                }
            }
            || ()
        },
        (canvas_ref.clone(), data, chart_type, title, width, height),
    );

    html! {
        <div class="chart-container">
            <canvas 
                ref={canvas_ref} 
                width={props.width.to_string()} 
                height={props.height.to_string()}
            />
        </div>
    }
}

fn draw_line_chart(
    root: &DrawingArea<CanvasBackend, plotters::coord::Shift>,
    data: &[(String, f64)],
    title: &str,
    width: u32,
    height: u32,
) {
    let mut chart = ChartBuilder::on(root)
        .caption(title, ("sans-serif", 20))
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(
            0..data.len(),
            data.iter().map(|(_, v)| *v).fold(0.0, f64::min)..data.iter().map(|(_, v)| *v).fold(0.0, f64::max),
        )
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(LineSeries::new(
            data.iter().enumerate().map(|(i, (_, v))| (i, *v)),
            &BLUE,
        ))
        .unwrap();
}

fn draw_bar_chart(
    root: &DrawingArea<CanvasBackend, plotters::coord::Shift>,
    data: &[(String, f64)],
    title: &str,
    width: u32,
    height: u32,
) {
    // Implementation for bar chart
    // Similar to line chart but with bars
}

fn draw_pie_chart(
    root: &DrawingArea<CanvasBackend, plotters::coord::Shift>,
    data: &[(String, f64)],
    title: &str,
    width: u32,
    height: u32,
) {
    // Implementation for pie chart
    // Using plotters pie chart functionality
}