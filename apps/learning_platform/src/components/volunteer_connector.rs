use yew::prelude::*;
use stylist::{style, yew::styled_component};
use crate::types::{SkillProgress, Course};
use std::collections::HashMap;

#[derive(Properties, PartialEq)]
pub struct VolunteerConnectorProps {
    pub user_skills: Vec<SkillProgress>,
    pub available_opportunities: Vec<HashMap<String, String>>, // opportunity data
    pub suggested_pathways: Vec<String>,
}

#[styled_component(VolunteerConnector)]
pub fn volunteer_connector(props: &VolunteerConnectorProps) -> Html {
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

    let opportunity_card_style = style!(
        r#"
        background: var(--background-secondary);
        border-radius: 8px;
        padding: 1rem;
        margin: 1rem 0;
        border-left: 4px solid var(--primary);
        
        &:hover {
            transform: translateY(-2px);
            box-shadow: 0 4px 12px rgba(0,0,0,0.1);
        }
    "#
    ).unwrap();

    let skill_match_style = style!(
        r#"
        display: inline-block;
        background: var(--primary);
        color: white;
        padding: 0.25rem 0.5rem;
        border-radius: 4px;
        font-size: 0.8rem;
        margin: 0.25rem;
    "#
    ).unwrap();

    let apply_button_style = style!(
        r#"
        background: var(--primary);
        color: white;
        border: none;
        padding: 0.75rem 1.5rem;
        border-radius: 4px;
        cursor: pointer;
        font-weight: bold;
        margin-top: 1rem;
        width: 100%;
        
        &:hover {
            background: var(--secondary);
        }
    "#
    ).unwrap();

    html! {
        <div class={container_style}>
            <div class={title_style.clone()}>
                <span>{"Apply What You're Learning"}</span>
                <span style="font-size: 0.9rem; color: var(--text-secondary);">{"Volunteer Opportunities"}</span>
            </div>
            
            <div style="margin: 1rem 0; padding: 1rem; background: var(--background-secondary); border-radius: 4px;">
                <h4>{"Perfect Match for Your Skills"}</h4>
                <p>{"You're ready to contribute to real community projects!"}</p>
            </div>
            
            <div class={opportunity_card_style.clone()}>
                <h3>{"Community Data Analysis Project"}</h3>
                <p>{"Help local non-profits understand their impact through data analysis and visualization."}</p>
                <div>
                    <span class={skill_match_style.clone()}>{"Rust Programming (90% match)"}</span>
                    <span class={skill_match_style.clone()}>{"Data Analysis (85% match)"}</span>
                </div>
                <div style="margin-top: 0.5rem; font-size: 0.9rem; color: var(--text-secondary);">
                    {"Estimated time: 20 hours • Impact: 75 community members"}
                </div>
                <button class={apply_button_style.clone()}>
                    {"Apply This Learning"}
                </button>
            </div>
            
            <div class={opportunity_card_style.clone()}>
                <h3>{"Education Technology Mentor"}</h3>
                <p>{"Mentor new learners in programming and data skills to help bridge the digital divide."}</p>
                <div>
                    <span class={skill_match_style.clone()}>{"Rust Programming (75% match)"}</span>
                    <span class={skill_match_style.clone()}>{"Teaching (60% match)"}</span>
                </div>
                <div style="margin-top: 0.5rem; font-size: 0.9rem; color: var(--text-secondary);">
                    {"Estimated time: 15 hours • Impact: 30 learners"}
                </div>
                <button class={apply_button_style.clone()}>
                    {"Apply This Learning"}
                </button>
            </div>
            
            <div class={opportunity_card_style}>
                <h3>{"Open Source Contribution"}</h3>
                <p>{"Contribute to cooperative technology projects that benefit communities worldwide."}</p>
                <div>
                    <span class={skill_match_style.clone()}>{"Rust Programming (80% match)"}</span>
                    <span class={skill_match_style}>{"Systems Design (70% match)"}</span>
                </div>
                <div style="margin-top: 0.5rem; font-size: 0.9rem; color: var(--text-secondary);">
                    {"Estimated time: 30 hours • Impact: Global cooperative network"}
                </div>
                <button class={apply_button_style}>
                    {"Apply This Learning"}
                </button>
            </div>
            
            <div style="margin-top: 1rem; padding: 1rem; background: var(--background-secondary); border-radius: 4px;">
                <h4>{"Suggested Learning Pathway"}</h4>
                <p>{"Complete 15 more hours of Data Visualization to unlock the 'Data Storyteller' volunteer role."}</p>
                <p>{"This role helps community organizations communicate their impact to funders and stakeholders."}</p>
            </div>
        </div>
    }
}