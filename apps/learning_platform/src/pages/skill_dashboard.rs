use yew::prelude::*;
use stylist::{style, yew::styled_component};
use crate::contexts::{use_auth, use_skills};
use crate::services::skill_development::SkillDevelopmentService;
use crate::components::{progress_bar::ProgressBar, credential_badge::CredentialBadge, skill_visualization::SkillVisualization, navigation::Navigation, community_skill_landscape::CommunitySkillLandscape, impact_pathway::ImpactPathway, progress_narrative::ProgressNarrative, validation_badges::ValidationBadges, volunteer_connector::VolunteerConnector};
use gloo_timers::callback::Timeout;
use wasm_bindgen_futures::spawn_local;
use skill_development::ml::{LearnerProfile, CommunityData, LearningExperience};
use ml_core::models::LearningPathway;
use std::collections::HashMap;

#[styled_component(SkillDashboardPage)]
pub fn skill_dashboard_page() -> Html {
    let auth_ctx = use_auth();
    let skills_ctx = use_skills();
    let loading = use_state(|| true);
    
    // Load user skills when component mounts
    use_effect_with_deps(
        move |_| {
            if let Some(user_id) = &auth_ctx.user_id {
                let skills_ctx = skills_ctx.clone();
                let loading = loading.clone();
                let user_id = user_id.clone();
                
                spawn_local(async move {
                    let mut service = SkillDevelopmentService::new().await.unwrap();
                    match service.get_user_skill_progress(user_id).await {
                        Ok(skills) => {
                            skills_ctx.dispatch(crate::contexts::skill_context::SkillAction::SetSkills(skills));
                        }
                        Err(e) => {
                            skills_ctx.dispatch(crate::contexts::skill_context::SkillAction::SetError(Some(format!("Failed to load skills: {}", e))));
                        }
                    }
                    
                    match service.get_user_certifications(user_id.clone()).await {
                        Ok(certifications) => {
                            skills_ctx.dispatch(crate::contexts::skill_context::SkillAction::SetCertifications(certifications));
                        }
                        Err(e) => {
                            skills_ctx.dispatch(crate::contexts::skill_context::SkillAction::SetError(Some(format!("Failed to load certifications: {}", e))));
                        }
                    }
                    
                    loading.set(false);
                });
            }
        },
        auth_ctx.user_id.clone(),
    );

    let container_style = style!(
        r#"
        padding: 2rem;
        max-width: 1200px;
        margin: 0 auto;
    "#
    ).unwrap();

    let header_style = style!(
        r#"
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 2rem;
    "#
    ).unwrap();

    let grid_style = style!(
        r#"
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
        gap: 2rem;
    "#
    ).unwrap();

    let card_style = style!(
        r#"
        background: var(--surface);
        border-radius: 8px;
        padding: 1.5rem;
        box-shadow: 0 2px 8px rgba(0,0,0,0.1);
    "#
    ).unwrap();

    let title_style = style!(
        r#"
        margin-top: 0;
        margin-bottom: 1rem;
        font-size: 1.5rem;
        color: var(--text-primary);
    "#
    ).unwrap();

    let skills_list: Vec<Html> = skills_ctx.skills
        .iter()
        .map(|skill| {
            html! {
                <div class={card_style.clone()}>
                    <h3 class={title_style.clone()}>{&skill.skill_name}</h3>
                    <p>{"Progress: "}{skill.progress_percentage as i32}{"%"}</p>
                    <ProgressBar progress={skill.progress_percentage} />
                    <p>{"Current Level: "}{skill.current_level}</p>
                    <p>{"Target Level: "}{skill.target_level}</p>
                </div>
            }
        })
        .collect();

    let certifications_list: Vec<Html> = skills_ctx.certifications
        .iter()
        .map(|cert| {
            html! {
                <div class={card_style.clone()}>
                    <CredentialBadge credential={cert.clone()} />
                </div>
            }
        })
        .collect();

    // Create sample data for visualizations
    let mut current_skills = HashMap::new();
    current_skills.insert("Rust Programming".to_string(), 0.75);
    current_skills.insert("Data Analysis".to_string(), 0.65);
    current_skills.insert("Project Management".to_string(), 0.80);
    current_skills.insert("Public Speaking".to_string(), 0.55);
    
    let learning_history = vec![
        LearningExperience {
            skill: "Rust Programming".to_string(),
            time_taken: 120.0,
            satisfaction: 9,
            completion_date: chrono::Utc::now() - chrono::Duration::days(60),
        },
        LearningExperience {
            skill: "Data Analysis".to_string(),
            time_taken: 80.0,
            satisfaction: 8,
            completion_date: chrono::Utc::now() - chrono::Duration::days(30),
        },
    ];
    
    let profile = LearnerProfile {
        current_skills,
        learning_pace: 8,
        learning_styles: vec!["Visual".to_string(), "Hands-on".to_string()],
        available_time: 15.0,
        learning_goals: vec![
            "Master Systems Programming".to_string(),
            "Become Data Science Expert".to_string(),
            "Lead Community Projects".to_string(),
        ],
        learning_history,
    };
    
    let mut skill_distribution = HashMap::new();
    skill_distribution.insert("Programming".to_string(), vec![0.2, 0.3, 0.25, 0.15, 0.1]);
    skill_distribution.insert("Design".to_string(), vec![0.15, 0.25, 0.3, 0.2, 0.1]);
    skill_distribution.insert("Education".to_string(), vec![0.3, 0.25, 0.2, 0.15, 0.1]);
    
    let mut projected_needs = HashMap::new();
    projected_needs.insert("Healthcare".to_string(), 0.8);
    projected_needs.insert("Technology".to_string(), 0.9);
    projected_needs.insert("Education".to_string(), 0.75);
    
    let mut learning_resources = HashMap::new();
    learning_resources.insert("Online Courses".to_string(), vec!["Coursera".to_string(), "edX".to_string()]);
    learning_resources.insert("Mentorship".to_string(), vec!["Peer Mentoring".to_string(), "Expert Guidance".to_string()]);
    
    let mut demographics = HashMap::new();
    demographics.insert("Age 18-30".to_string(), 0.4);
    demographics.insert("Age 31-50".to_string(), 0.35);
    demographics.insert("Age 51+".to_string(), 0.25);
    
    let mut historical_trends = HashMap::new();
    historical_trends.insert("Programming Skills".to_string(), vec![0.1, 0.15, 0.25, 0.35, 0.45]);
    historical_trends.insert("Leadership Skills".to_string(), vec![0.2, 0.25, 0.3, 0.35, 0.4]);
    
    let community_data = CommunityData {
        skill_distribution,
        projected_needs,
        learning_resources,
        demographics,
        historical_trends,
    };
    
    let pathways = vec![
        LearningPathway {
            skills: vec!["Rust Programming".to_string(), "Systems Design".to_string(), "Performance Optimization".to_string()],
            estimated_time: 200.0,
            resources_needed: vec!["The Rust Programming Language Book".to_string(), "Online Compiler".to_string()],
        },
        LearningPathway {
            skills: vec!["Data Analysis".to_string(), "Machine Learning".to_string(), "Data Visualization".to_string()],
            estimated_time: 180.0,
            resources_needed: vec!["Python Course".to_string(), "Jupyter Notebooks".to_string(), "Kaggle Datasets".to_string()],
        },
        LearningPathway {
            skills: vec!["Project Management".to_string(), "Team Leadership".to_string(), "Community Organization".to_string()],
            estimated_time: 150.0,
            resources_needed: vec!["PMP Study Guide".to_string(), "Leadership Workshop".to_string()],
        },
    ];

    html! {
        <div class={container_style}>
            <Navigation />
            
            <div class={header_style}>
                <h1>{"Your Skill Dashboard"}</h1>
            </div>
            
            if *loading {
                <p>{"Loading your skills and certifications..."}</p>
            } else {
                <SkillVisualization profile={profile.clone()} community_data={community_data.clone()} pathways={pathways.clone()} />
                
                <CommunitySkillLandscape community_data={community_data.clone()} user_skills={profile.current_skills.clone()} />
                
                <ImpactPathway pathways={pathways.clone()} profile={profile.clone()} community_needs={community_data.projected_needs.clone()} />
                
                <ProgressNarrative skill_progress={skills_ctx.skills.clone()} community_impact_stories={vec!["Story 1".to_string(), "Story 2".to_string()]} />
                
                if let Some(first_skill) = skills_ctx.skills.first() {
                    <ValidationBadges
                        skill_progress={first_skill.clone()}
                        community_validations={vec!["Validation 1".to_string(), "Validation 2".to_string()]}
                        validators={HashMap::new()}
                    />
                }
                
                <VolunteerConnector
                    user_skills={skills_ctx.skills.clone()}
                    available_opportunities={vec![HashMap::new()]}
                    suggested_pathways={vec!["Pathway 1".to_string(), "Pathway 2".to_string()]}
                />
                
                <h2>{"Current Skills"}</h2>
                <div class={grid_style.clone()}>
                    {skills_list}
                </div>
                
                <h2>{"Certifications"}</h2>
                <div class={grid_style}>
                    {certifications_list}
                </div>
            }
        </div>
    }
}