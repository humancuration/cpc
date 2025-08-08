use yew::prelude::*;
use stylist::{style, yew::styled_component};
use skill_development::ml::CommunityData;
use impact_viz::core::{ImpactVizCore, VisualizationStyle};
use impact_viz::skill::SkillDevelopmentViz;
use std::collections::HashMap;

#[derive(Properties, PartialEq)]
pub struct CommunitySkillLandscapeProps {
    pub community_data: CommunityData,
    pub user_skills: HashMap<String, f64>,
}

#[styled_component(CommunitySkillLandscape)]
pub fn community_skill_landscape(props: &CommunitySkillLandscapeProps) -> Html {
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

    let chart_container_style = style!(
        r#"
        width: 100%;
        height: 350px;
        position: relative;
        background: var(--background-secondary);
        border-radius: 4px;
        display: flex;
        align-items: center;
        justify-content: center;
        margin-bottom: 1rem;
        overflow: hidden;
    "#
    ).unwrap();

    let chart_title_style = style!(
        r#"
        position: absolute;
        top: 10px;
        left: 10px;
        font-weight: bold;
        color: var(--text-primary);
        z-index: 10;
    "#
    ).unwrap();

    let filter_container_style = style!(
        r#"
        display: flex;
        gap: 1rem;
        margin-bottom: 1rem;
        flex-wrap: wrap;
    "#
    ).unwrap();

    let filter_button_style = style!(
        r#"
        background: var(--background-secondary);
        border: 1px solid var(--border-color);
        border-radius: 4px;
        padding: 0.5rem 1rem;
        cursor: pointer;
        font-size: 0.9rem;
        
        &:hover {
            background: var(--primary);
            color: white;
        }
        
        &.active {
            background: var(--primary);
            color: white;
        }
    "#
    ).unwrap();

    // Create visualization
    let core_viz = ImpactVizCore::new();
    let skill_viz = SkillDevelopmentViz::new(Box::new(core_viz));
    
    // Community skill landscape visualization
    let landscape_viz = skill_viz.visualize_skill_landscape(&props.community_data);
    
    // Skill needs mapping visualization
    let needs_viz = skill_viz.visualize_skill_needs_mapping(&props.community_data);

    html! {
        <div class={container_style}>
            <div class={title_style.clone()}>
                <span>{"Community Skill Landscape"}</span>
                <button class={filter_button_style.clone()}>{"Explain This"}</button>
            </div>
            
            <div class={filter_container_style}>
                <button class={filter_button_style.clone() + " active"}>{"All Skills"}</button>
                <button class={filter_button_style.clone()}>{"Technology"}</button>
                <button class={filter_button_style.clone()}>{"Education"}</button>
                <button class={filter_button_style.clone()}>{"Healthcare"}</button>
                <button class={filter_button_style.clone()}>{"Community Development"}</button>
            </div>
            
            <div class={chart_container_style.clone()}>
                <div class={chart_title_style.clone()}>{"Skill Distribution Heatmap"}</div>
                <div style="width: 100%; height: 100%; display: flex; align-items: center; justify-content: center;">
                    <div>{"Interactive Skill Distribution Heatmap Visualization"}</div>
                </div>
            </div>
            
            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-top: 1rem;">
                <div class={chart_container_style.clone()}>
                    <div class={chart_title_style.clone()}>{"Skill Gap Analysis"}</div>
                    <div style="width: 100%; height: 100%; display: flex; align-items: center; justify-content: center;">
                        <div>{"Skill Gap Analysis Chart"}</div>
                    </div>
                </div>
                
                <div class={chart_container_style}>
                    <div class={chart_title_style}>{"Your Position in Community"}</div>
                    <div style="width: 100%; height: 100%; display: flex; align-items: center; justify-content: center;">
                        <div>{"Personal Skill Position Visualization"}</div>
                    </div>
                </div>
            </div>
            
            <div style="margin-top: 1rem; padding: 1rem; background: var(--background-secondary); border-radius: 4px;">
                <h4>{"Community Skill Insights"}</h4>
                <p>{"High-demand skills in your community: Programming (+25% demand), Data Analysis (+30%), Community Organization (+20%)"}</p>
                <p>{"Skills where you can make the biggest impact: "}<strong>{"Data Analysis (90% match to your skills)"}</strong></p>
            </div>
            
            <div style="margin-top: 1rem; display: flex; gap: 1rem;">
                <button class={feedback_button_style.clone()}>{"Was this helpful?"}</button>
                <button class={feedback_button_style.clone()}>{"Suggest Improvements"}</button>
                <button class={feedback_button_style}>{"Community Validation"}</button>
            </div>
        </div>
    }
}