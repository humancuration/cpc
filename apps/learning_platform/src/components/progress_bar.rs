use yew::prelude::*;
use stylist::{style, yew::styled_component};

#[derive(Properties, PartialEq)]
pub struct ProgressBarProps {
    pub progress: f32,
}

#[styled_component(ProgressBar)]
pub fn progress_bar(props: &ProgressBarProps) -> Html {
    let container_style = style!(
        r#"
        width: 100%;
        height: 12px;
        background: var(--surface);
        border-radius: 6px;
        overflow: hidden;
        margin: 1rem 0;
    "#
    ).unwrap();

    let filler_style = style!(
        r#"
        height: 100%;
        background: var(--primary);
        border-radius: 6px;
        transition: width 0.3s ease;
        position: relative;

        &[data-status="completed"] {
            background: var(--success);
        }

        &[data-status="in-progress"] {
            background: var(--warning);
        }

        &[data-status="enrolled"] {
            background: var(--primary);
        }
    "#
    ).unwrap();

    let progress_percentage = props.progress.min(100.0).max(0.0);
    
    let status = if progress_percentage >= 100.0 {
        "completed"
    } else if progress_percentage > 0.0 {
        "in-progress"
    } else {
        "enrolled"
    };

    html! {
        <div class={container_style}>
            <div 
                class={filler_style} 
                style={format!("width: {}%", progress_percentage)}
                data-status={status}
            >
            </div>
        </div>
    }
}