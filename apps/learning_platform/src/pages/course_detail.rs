use yew::prelude::*;
use stylist::{style, yew::styled_component};
use crate::components::{navigation::Navigation, course_impact_preview::CourseImpactPreview, personal_relevance::PersonalRelevanceIndicator};
use crate::contexts::use_courses;
use crate::types::Course;
use yew_router::prelude::*;
use std::collections::HashMap;
use skill_development::ml::LearnerProfile;

#[derive(Properties, PartialEq, Clone)]
pub struct CourseDetailProps {
    pub course_id: String,
}

#[styled_component(CourseDetailPage)]
pub fn course_detail_page(props: &CourseDetailProps) -> Html {
    let courses_ctx = use_courses();
    let course = courses_ctx.courses.get(&props.course_id).cloned();
    
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

    let course_header_style = style!(
        r#"
        background: var(--surface);
        border-radius: 8px;
        padding: 2rem;
        box-shadow: 0 2px 8px rgba(0,0,0,0.1);
        margin-bottom: 2rem;
    "#
    ).unwrap();

    let title_style = style!(
        r#"
        margin-top: 0;
        margin-bottom: 1rem;
        font-size: 2rem;
        color: var(--text-primary);
    "#
    ).unwrap();

    let description_style = style!(
        r#"
        color: var(--text-secondary);
        margin-bottom: 1.5rem;
        font-size: 1.1rem;
        line-height: 1.6;
    "#
    ).unwrap();

    let button_style = style!(
        r#"
        background: var(--primary);
        color: white;
        border: none;
        padding: 1rem 2rem;
        border-radius: 4px;
        cursor: pointer;
        font-weight: bold;
        font-size: 1.1rem;
        
        &:hover {
            background: var(--secondary);
        }
    "#
    ).unwrap();

    // Create sample data for visualizations
    let mut current_skills = HashMap::new();
    current_skills.insert("Rust Programming".to_string(), 0.75);
    current_skills.insert("Data Analysis".to_string(), 0.65);
    current_skills.insert("Project Management".to_string(), 0.80);
    
    let profile = LearnerProfile {
        current_skills,
        learning_pace: 8,
        learning_styles: vec!["Visual".to_string(), "Hands-on".to_string()],
        available_time: 15.0,
        learning_goals: vec![
            "Master Systems Programming".to_string(),
            "Become Data Science Expert".to_string(),
        ],
        learning_history: vec![],
    };
    
    let mut community_needs = HashMap::new();
    community_needs.insert("Data Analysis".to_string(), 0.9);
    community_needs.insert("Programming".to_string(), 0.85);
    community_needs.insert("Education".to_string(), 0.75);

    html! {
        <div class={container_style}>
            <Navigation />
            
            <div class={header_style}>
                <h1>{"Course Details"}</h1>
                <Link<AppRoute> to={AppRoute::CourseCatalog} classes={button_style.clone()}>
                    {"← Back to Catalog"}
                </Link<AppRoute>>
            </div>
            
            if let Some(course) = course {
                <div class={course_header_style}>
                    <h1 class={title_style}>{&course.title}</h1>
                    <p class={description_style}>{&course.description}</p>
                    <button class={button_style.clone()}>
                        {"Enroll in Course"}
                    </button>
                </div>
                
                <CourseImpactPreview 
                    course={course.clone()} 
                    user_profile={profile.current_skills.clone()} 
                    community_needs={community_needs.clone()} 
                />
                
                <PersonalRelevanceIndicator 
                    course={course} 
                    profile={profile} 
                    current_volunteer_activities={vec!["Community Data Project".to_string()]} 
                    expressed_values={vec!["Community Service".to_string(), "Education".to_string()]} 
                />
                
                <div style="background: var(--surface); border-radius: 8px; padding: 2rem; box-shadow: 0 2px 8px rgba(0,0,0,0.1); margin-top: 2rem;">
                    <h2>{"Course Modules"}</h2>
                    <p>{"This course contains 5 modules with 25 lessons covering all aspects of data analysis with Rust."}</p>
                    <button class={button_style}>
                        {"View Module Details"}
                    </button>
                </div>
            } else {
                <div>
                    <h2>{"Course not found"}</h2>
                    <p>{"The course you're looking for doesn't exist or has been removed."}</p>
                    <Link<AppRoute> to={AppRoute::CourseCatalog}>
                        {"← Back to Catalog"}
                    </Link<AppRoute>>
                </div>
            }
        </div>
    }
}