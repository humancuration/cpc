use yew::prelude::*;
use stylist::{style, yew::styled_component};
use crate::components::credential_badge::CredentialBadge;

#[styled_component(CredentialPage)]
pub fn credential_page() -> Html {
    // In a real app, we would fetch credentials from context or API
    // For now, we'll create some mock data
    let credentials = vec![
        ("CERTIFICATE", "Rust Programming", "2023-05-15"),
        ("BADGE", "Web Development", "2023-06-20"),
        ("MICRO_DEGREE", "Data Science", "2023-08-10"),
    ];

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

    let grid_style = style!(
        r#"
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
        gap: 2rem;
    "#
    ).unwrap();

    html! {
        <div class={container_style}>
            <div class={header_style}>
                <h1>{"My Credentials"}</h1>
                <p>{"View and share your academic achievements"}</p>
            </div>
            <div class={grid_style}>
                {for credentials.iter().map(|(cred_type, course, date)| {
                    html! {
                        <CredentialBadge 
                            credential_type={cred_type.to_string()} 
                            course_name={course.to_string()} 
                            issued_date={date.to_string()} 
                        />
                    }
                })}
            </div>
        </div>
    }
}