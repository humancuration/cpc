use yew::prelude::*;
use stylist::{style, yew::styled_component};
use crate::components::progress_bar::ProgressBar;
use crate::contexts::use_courses;

#[styled_component(EnrollmentPage)]
pub fn enrollment_page() -> Html {
    let courses_ctx = use_courses();
    let enrollments = &courses_ctx.enrollments;

    let container_style = style!(
        r#"
        padding: 2rem;
        max-width: 1200px;
        margin: 0 auto;
    "#
    ).unwrap();

    let header_style = style!(
        r#"
        margin-bottom: 2rem;
    "#
    ).unwrap();

    let card_style = style!(
        r#"
        background: var(--surface);
        border-radius: 8px;
        padding: 1.5rem;
        margin-bottom: 1.5rem;
        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    "#
    ).unwrap();

    let title_style = style!(
        r#"
        margin-top: 0;
        margin-bottom: 1rem;
        display: flex;
        justify-content: space-between;
    "#
    ).unwrap();

    let status_style = style!(
        r#"
        padding: 0.25rem 0.5rem;
        border-radius: 4px;
        font-size: 0.875rem;
        font-weight: bold;

        &[data-status="0"] {
            background: #e3f2fd;
            color: #1976d2;
        }

        &[data-status="1"] {
            background: #fff8e1;
            color: #f57f17;
        }

        &[data-status="2"] {
            background: #e8f5e9;
            color: #388e3c;
        }

        &[data-status="3"] {
            background: #ffebee;
            color: #d32f2f;
        }
    "#
    ).unwrap();

    let enrollments_list: Vec<Html> = enrollments
        .values()
        .map(|enrollment| {
            // In a real app, we would look up the course details
            let course_title = format!("Course {}", &enrollment.course_id[..8]);
            let status_text = match enrollment.status {
                0 => "Enrolled",
                1 => "In Progress",
                2 => "Completed",
                3 => "Dropped",
                _ => "Unknown",
            };

            html! {
                <div class={card_style.clone()}>
                    <h3 class={title_style.clone()}>
                        {course_title}
                        <span class={status_style.clone()} data-status={enrollment.status.to_string()}>
                            {status_text}
                        </span>
                    </h3>
                    <ProgressBar progress={enrollment.progress} />
                    <p>{"Progress: "}{format!("{:.1}%", enrollment.progress)}</p>
                </div>
            }
        })
        .collect();

    html! {
        <div class={container_style}>
            <div class={header_style}>
                <h1>{"My Enrollments"}</h1>
            </div>
            if enrollments_list.is_empty() {
                <p>{"You are not enrolled in any courses yet."}</p>
            } else {
                {enrollments_list}
            }
        </div>
    }
}