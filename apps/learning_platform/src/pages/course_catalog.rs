use yew::prelude::*;
use stylist::{style, yew::styled_component};
use crate::components::course_card::CourseCard;
use crate::contexts::use_courses;

#[styled_component(CourseCatalogPage)]
pub fn course_catalog_page() -> Html {
    let courses_ctx = use_courses();
    let courses = &courses_ctx.courses;

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

    let courses_list: Vec<Html> = courses
        .values()
        .map(|course| {
            html! {
                <CourseCard course={course.clone()} />
            }
        })
        .collect();

    html! {
        <div class={container_style}>
            <div class={header_style}>
                <h1>{"Course Catalog"}</h1>
                <div>
                    <input type="text" placeholder="Search courses..." />
                </div>
            </div>
            <div class={grid_style}>
                {courses_list}
            </div>
        </div>
    }
}