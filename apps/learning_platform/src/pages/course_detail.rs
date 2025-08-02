use yew::prelude::*;
use stylist::{style, yew::styled_component};
use crate::components::module_accordion::ModuleAccordion;
use crate::contexts::use_courses;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CourseDetailProps {
    pub id: String,
}

#[styled_component(CourseDetailPage)]
pub fn course_detail_page(props: &CourseDetailProps) -> Html {
    let courses_ctx = use_courses();
    let course = courses_ctx.courses.get(&props.id).cloned();
    
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
        align-items: flex-start;
        margin-bottom: 2rem;
        gap: 2rem;
    "#
    ).unwrap();

    let content_style = style!(
        r#"
        flex: 1;
    "#
    ).unwrap();

    let sidebar_style = style!(
        r#"
        width: 300px;
        background: var(--surface);
        padding: 1.5rem;
        border-radius: 8px;
    "#
    ).unwrap();

    let button_style = style!(
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

    match course {
        Some(course) => {
            html! {
                <div class={container_style}>
                    <div class={header_style}>
                        <div class={content_style}>
                            <h1>{&course.title}</h1>
                            <p>{&course.description}</p>
                            
                            <h2>{"Course Content"}</h2>
                            {for course.modules.iter().map(|module| {
                                html! {
                                    <ModuleAccordion module={module.clone()} />
                                }
                            })}
                        </div>
                        <div class={sidebar_style}>
                            <h3>{"Course Info"}</h3>
                            <p>{"Creator: Unknown"}</p>
                            <p>{"Lessons: "}{course.modules.iter().map(|m| m.lessons.len()).sum::<usize>()}</p>
                            <button class={button_style}>{"Enroll in Course"}</button>
                        </div>
                    </div>
                </div>
            }
        }
        None => {
            html! {
                <div class={container_style}>
                    <h1>{"Course not found"}</h1>
                    <p>{"The course you're looking for doesn't exist."}</p>
                    <Link<AppRoute> to={AppRoute::CourseCatalog}>{"Back to Catalog"}</Link<AppRoute>>
                </div>
            }
        }
    }
}