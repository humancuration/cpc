use yew::prelude::*;
use stylist::{style, yew::styled_component};
use skill_development::ml::{LearnerProfile, CommunityData, LearningExperience};
use impact_viz::core::{ImpactVizCore, VisualizationStyle};
use impact_viz::skill::SkillDevelopmentViz;
use ml_core::models::LearningPathway;
use std::collections::HashMap;

#[derive(Properties, PartialEq)]
pub struct SkillVisualizationProps {
    pub profile: LearnerProfile,
    pub community_data: CommunityData,
    pub pathways: Vec<LearningPathway>,
}

#[styled_component(SkillVisualization)]
pub fn skill_visualization(props: &SkillVisualizationProps) -> Html {
    let container_style = style!(
        r#"
        background: var(--surface);
        border-radius: 8px;
        padding: 1.5rem;
        box-shadow: 0 2px 8px rgba(0,0,0,0.1);
        margin-bottom: 2rem;
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

    let chart_container_style = style!(
        r#"
        width: 100%;
        height: 300px;
        position: relative;
        background: var(--background-secondary);
        border-radius: 4px;
        display: flex;
        align-items: center;
        justify-content: center;
        margin-bottom: 1rem;
    "#
    ).unwrap();

    let chart_title_style = style!(
        r#"
        position: absolute;
        top: 10px;
        left: 10px;
        font-weight: bold;
        color: var(--text-primary);
    "#
    ).unwrap();

    let feedback_style = style!(
        r#"
        display: flex;
        gap: 0.5rem;
        align-items: center;
    "#
    ).unwrap();

    let feedback_button_style = style!(
        r#"
        background: var(--background-secondary);
        border: 1px solid var(--border-color);
        border-radius: 4px;
        padding: 0.25rem 0.5rem;
        cursor: pointer;
        font-size: 0.8rem;
        
        &:hover {
            background: var(--primary);
            color: white;
        }
    "#
    ).unwrap();

    // Create visualization
    let core_viz = ImpactVizCore::new();
    let skill_viz = SkillDevelopmentViz::new(Box::new(core_viz));
    
    // Individual growth visualization
    let growth_viz = skill_viz.visualize_individual_growth(&props.profile);
    
    // Community skill landscape visualization
    let landscape_viz = skill_viz.visualize_skill_landscape(&props.community_data);
    
    // Learning pathways visualization
    let pathways_viz = skill_viz.visualize_learning_pathways(&props.pathways);

    let on_feedback_click = Callback::from(|_e: MouseEvent| {
        // In a real implementation, this would open a feedback modal
        web_sys::console::log_1(&"Feedback button clicked".into());
    });

    html! {
        <div class={container_style}>
            <h3 class={title_style.clone()}>
                {"Your Skill Growth"}
                <div class={feedback_style.clone()}>
                    <button class={feedback_button_style.clone()} onclick={on_feedback_click.clone()}>{"Was this helpful?"}</button>
                    <button class={feedback_button_style.clone()} onclick={on_feedback_click.clone()}>{"Suggest Improvements"}</button>
                </div>
            </h3>
            <div class={chart_container_style.clone()}>
                <div class={chart_title_style.clone()}>{"Individual Growth Visualization"}</div>
                <div>
                    <pre>{&growth_viz.data.json_data}</pre>
                </div>
            </div>
            
            <h3 class={title_style.clone()}>
                {"Community Skill Landscape"}
                <div class={feedback_style.clone()}>
                    <button class={feedback_button_style.clone()} onclick={on_feedback_click.clone()}>{"Was this helpful?"}</button>
                    <button class={feedback_button_style.clone()} onclick={on_feedback_click.clone()}>{"Suggest Improvements"}</button>
                </div>
            </h3>
            <div class={chart_container_style.clone()}>
                <div class={chart_title_style.clone()}>{"Community Landscape Visualization"}</div>
                <div>
                    <pre>{&landscape_viz.data.json_data}</pre>
                </div>
            </div>
            
            <h3 class={title_style.clone()}>
                {"Learning Pathways"}
                <div class={feedback_style}>
                    <button class={feedback_button_style.clone()} onclick={on_feedback_click}>{"Was this helpful?"}</button>
                    <button class={feedback_button_style}>{"Suggest Improvements"}</button>
                </div>
            </h3>
            <div class={chart_container_style}>
                <div class={chart_title_style}>{"Learning Pathways Visualization"}</div>
                <div>
                    <pre>{&pathways_viz.data.json_data}</pre>
                </div>
            </div>
        </div>
    }
}