use yew::prelude::*;
use plotters::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use gloo::timers::callback::Timeout;
use crate::types::SkillProgressData;

#[derive(Properties, PartialEq)]
pub struct ProgressVisualizerProps {
    pub skills: Vec<SkillProgressData>,
}

#[function_component(ProgressVisualizer)]
pub fn progress_visualizer(props: &ProgressVisualizerProps) -> Html {
    let canvas_ref = use_state(|| NodeRef::default());
    let canvas_ref_clone = canvas_ref.clone();
    let skills = props.skills.clone();

    // Draw the chart when the component is mounted
    use_effect_with((), move |_| {
        let canvas_ref = canvas_ref_clone.clone();
        let skills = skills.clone();
        
        // Use a timeout to ensure the canvas is rendered
        let timeout = Timeout::new(0, move || {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                draw_progress_chart(&canvas, &skills);
            }
        });
        
        timeout.forget();
        
        || {}
    });

    html! {
        <div class="card">
            <div class="card-header">
                <h2 class="card-title">{"Progress Visualization"}</h2>
            </div>
            <div class="card-body">
                <canvas 
                    ref={(*canvas_ref).clone()} 
                    width="600" 
                    height="400"
                    style="width: 100%; height: auto;"
                />
            </div>
        </div>
    }
}

fn draw_progress_chart(canvas: &HtmlCanvasElement, skills: &[SkillProgressData]) {
    // In a real implementation, we would use Plotters to draw the chart
    // For now, we'll just log to the console to show the concept
    web_sys::console::log_1(&format!("Drawing chart for {} skills", skills.len()).into());
    
    // This is where we would implement the actual chart drawing using Plotters
    // The implementation would involve:
    // 1. Creating a backend from the canvas context
    // 2. Setting up the chart area
    // 3. Drawing bars or lines for each skill's progress
    // 4. Adding labels and legends
}