use yew::prelude::*;
use stylist::{style, yew::styled_component};
use ml_core::models::LearningPathway;
use skill_development::ml::LearnerProfile;
use impact_viz::core::{ImpactVizCore, VisualizationStyle};
use impact_viz::skill::SkillDevelopmentViz;
use std::collections::HashMap;

#[derive(Properties, PartialEq)]
pub struct ImpactPathwayProps {
    pub pathways: Vec<LearningPathway>,
    pub profile: LearnerProfile,
    pub community_needs: HashMap<String, f64>,
}

#[styled_component(ImpactPathway)]
pub fn impact_pathway(props: &ImpactPathwayProps) -> Html {
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
        height: 300px;
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

    let pathway_selector_style = style!(
        r#"
        display: flex;
        gap: 1rem;
        margin-bottom: 1rem;
        flex-wrap: wrap;
    "#
    ).unwrap();

    let pathway_button_style = style!(
        r#"
        background: var(--background-secondary);
        border: 1px solid var(--border-color);
        border-radius: 4px;
        padding: 0.75rem 1rem;
        cursor: pointer;
        font-size: 0.9rem;
        flex: 1;
        min-width: 200px;
        text-align: left;
        
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

    let scenario_container_style = style!(
        r#"
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
        gap: 1rem;
        margin-top: 1rem;
    "#
    ).unwrap();

    let scenario_card_style = style!(
        r#"
        background: var(--background-secondary);
        border-radius: 4px;
        padding: 1rem;
        border-left: 4px solid var(--primary);
    "#
    ).unwrap();

    // Create visualization
    let core_viz = ImpactVizCore::new();
    let skill_viz = SkillDevelopmentViz::new(Box::new(core_viz));
    
    // Learning pathways visualization
    let pathways_viz = skill_viz.visualize_learning_pathways(&props.pathways);

    html! {
        <div class={container_style}>
            <div class={title_style.clone()}>
                <span>{"Impact Pathways"}</span>
                <button class={pathway_button_style.clone()}>{"Explain This"}</button>
            </div>
            
            <div class={pathway_selector_style}>
                <button class={pathway_button_style.clone() + " active"}>
                    <div><strong>{"Systems Programming Pathway"}</strong></div>
                    <div style="font-size: 0.8rem; margin-top: 0.25rem;">{"Estimated time: 200 hours • Community impact: High"}</div>
                </button>
                <button class={pathway_button_style.clone()}>
                    <div><strong>{"Data Science Pathway"}</strong></div>
                    <div style="font-size: 0.8rem; margin-top: 0.25rem;">{"Estimated time: 180 hours • Community impact: Very High"}</div>
                </button>
                <button class={pathway_button_style.clone()}>
                    <div><strong>{"Community Leadership Pathway"}</strong></div>
                    <div style="font-size: 0.8rem; margin-top: 0.25rem;">{"Estimated time: 150 hours • Community impact: High"}</div>
                </button>
            </div>
            
            <div class={chart_container_style.clone()}>
                <div class={chart_title_style.clone()}>{"Learning Pathway Impact Projection"}</div>
                <div style="width: 100%; height: 100%; display: flex; align-items: center; justify-content: center;">
                    <div>{"Interactive Learning Pathway Visualization"}</div>
                </div>
            </div>
            
            <div class={scenario_container_style}>
                <div class={scenario_card_style.clone()}>
                    <h4>{"What if: Fast Track (50% more time)"}</h4>
                    <p>{"Community Impact: +40%"}</p>
                    <p>{"Completion Time: 100 hours"}</p>
                    <p>{"Skills Developed: Rust Programming, Systems Design"}</p>
                </div>
                
                <div class={scenario_card_style.clone()}>
                    <h4>{"What if: Community Focus"}</h4>
                    <p>{"Community Impact: +65%"}</p>
                    <p>{"Completion Time: 250 hours"}</p>
                    <p>{"Skills Developed: Rust Programming, Community Project Management"}</p>
                </div>
                
                <div class={scenario_card_style}>
                    <h4>{"What if: Balanced Approach"}</h4>
                    <p>{"Community Impact: +50%"}</p>
                    <p>{"Completion Time: 200 hours"}</p>
                    <p>{"Skills Developed: Rust Programming, Data Analysis, Leadership"}</p>
                </div>
            </div>
            
            <div style="margin-top: 1rem; padding: 1rem; background: var(--background-secondary); border-radius: 4px;">
                <h4>{"Pathway Recommendation"}</h4>
                <p><strong>{"Data Science Pathway"}</strong>{" aligns best with current community needs (+30% demand for data skills)"}</p>
                <p>{"Estimated community benefit: 85 impact points"}</p>
            </div>
            
            <div style="margin-top: 1rem; display: flex; gap: 1rem;">
                <button class={feedback_button_style.clone()}>{"Was this helpful?"}</button>
                <button class={feedback_button_style.clone()}>{"Suggest Improvements"}</button>
                <button class={feedback_button_style}>{"Community Validation"}</button>
            </div>
        </div>
    }
}