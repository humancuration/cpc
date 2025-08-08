use yew::prelude::*;
use stylist::{style, yew::styled_component};
use crate::types::Course;
use std::collections::HashMap;

#[derive(Properties, PartialEq)]
pub struct CourseImpactPreviewProps {
    pub course: Course,
    pub user_profile: HashMap<String, f64>,
    pub community_needs: HashMap<String, f64>,
}

#[styled_component(CourseImpactPreview)]
pub fn course_impact_preview(props: &CourseImpactPreviewProps) -> Html {
    let container_style = style!(
        r#"
        background: var(--surface);
        border-radius: 8px;
        padding: 1.5rem;
        box-shadow: 0 2px 8px rgba(0,0,0,0.1);
        margin-bottom: 2rem;
        border-left: 4px solid var(--primary);
    "#
    ).unwrap();

    let title_style = style!(
        r#"
        margin-top: 0;
        margin-bottom: 1rem;
        font-size: 1.25rem;
        color: var(--text-primary);
        display: flex;
        justify-content: space-between;
        align-items: center;
    "#
    ).unwrap();

    let impact_metrics_style = style!(
        r#"
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
        gap: 1rem;
        margin: 1rem 0;
    "#
    ).unwrap();

    let metric_card_style = style!(
        r#"
        background: var(--background-secondary);
        border-radius: 4px;
        padding: 1rem;
        text-align: center;
    "#
    ).unwrap();

    let impact_bar_style = style!(
        r#"
        height: 8px;
        background: var(--background-secondary);
        border-radius: 4px;
        margin: 0.5rem 0;
        overflow: hidden;
    "#
    ).unwrap();

    let impact_fill_style = style!(
        r#"
        height: 100%;
        background: var(--primary);
        border-radius: 4px;
    "#
    ).unwrap();

    let slider_container_style = style!(
        r#"
        margin: 1rem 0;
        padding: 1rem;
        background: var(--background-secondary);
        border-radius: 4px;
    "#
    ).unwrap();

    let slider_style = style!(
        r#"
        width: 100%;
        height: 6px;
        background: var(--border-color);
        border-radius: 3px;
        position: relative;
        margin: 1rem 0;
    "#
    ).unwrap();

    let slider_handle_style = style!(
        r#"
        width: 20px;
        height: 20px;
        background: var(--primary);
        border-radius: 50%;
        position: absolute;
        top: 50%;
        transform: translate(-50%, -50%);
        cursor: pointer;
    "#
    ).unwrap();

    html! {
        <div class={container_style}>
            <div class={title_style.clone()}>
                <span>{"Course Impact Preview"}</span>
                <span style="font-size: 0.9rem; color: var(--text-secondary);">{"Community Benefit: High"}</span>
            </div>
            
            <div class={impact_metrics_style}>
                <div class={metric_card_style.clone()}>
                    <div>{"Community Benefit"}</div>
                    <div class={impact_bar_style}>
                        <div class={impact_fill_style} style="width: 85%;"></div>
                    </div>
                    <div style="font-weight: bold; color: var(--primary);">{"85/100"}</div>
                </div>
                
                <div class={metric_card_style.clone()}>
                    <div>{"Skill Development"}</div>
                    <div class={impact_bar_style}>
                        <div class={impact_fill_style} style="width: 90%;"></div>
                    </div>
                    <div style="font-weight: bold; color: var(--primary);">{"90/100"}</div>
                </div>
                
                <div class={metric_card_style.clone()}>
                    <div>{"Time Investment"}</div>
                    <div class={impact_bar_style}>
                        <div class={impact_fill_style} style="width: 60%; background: var(--secondary);"></div>
                    </div>
                    <div style="font-weight: bold; color: var(--secondary);">{"60/100"}</div>
                </div>
            </div>
            
            <div class={slider_container_style}>
                <h4>{"Time Investment vs. Impact"}</h4>
                <div>{"Adjust your time commitment to see how it affects community impact"}</div>
                <div class={slider_style}>
                    <div class={slider_handle_style} style="left: 60%;"></div>
                </div>
                <div style="display: flex; justify-content: space-between; font-size: 0.8rem; color: var(--text-secondary);">
                    <span>{"Less Time"}</span>
                    <span>{"Balanced"}</span>
                    <span>{"More Time"}</span>
                </div>
                <div style="margin-top: 1rem; text-align: center;">
                    <p><strong>{"Balanced Commitment"}</strong></p>
                    <p>{"Estimated 40 hours over 8 weeks"}</p>
                    <p>{"Community impact: 75 points"}</p>
                </div>
            </div>
            
            <div style="margin-top: 1rem; padding: 1rem; background: var(--background-secondary); border-radius: 4px;">
                <h4>{"Why This Course Matters"}</h4>
                <p>{"This course addresses a critical community need for data analysis skills. "}</p>
                <p>{"By completing this course, you'll be able to contribute to 3 ongoing community projects that need data insights."}</p>
            </div>
        </div>
    }
}