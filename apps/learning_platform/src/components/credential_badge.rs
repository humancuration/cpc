use yew::prelude::*;
use stylist::{style, yew::styled_component};

#[derive(Properties, PartialEq)]
pub struct CredentialBadgeProps {
    pub credential_type: String,
    pub course_name: String,
    pub issued_date: String,
}

#[styled_component(CredentialBadge)]
pub fn credential_badge(props: &CredentialBadgeProps) -> Html {
    let badge_style = style!(
        r#"
        background: linear-gradient(135deg, var(--surface), #f0f4f8);
        border-radius: 12px;
        padding: 1.5rem;
        text-align: center;
        box-shadow: 0 4px 12px rgba(0,0,0,0.05);
        border: 1px solid var(--border);
        transition: transform 0.2s, box-shadow 0.2s;

        &:hover {
            transform: translateY(-4px);
            box-shadow: 0 6px 16px rgba(0,0,0,0.1);
        }
    "#
    ).unwrap();

    let icon_style = style!(
        r#"
        width: 96px;
        height: 96px;
        margin: 0 auto 1rem;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 48px;
        background: var(--primary);
        color: white;
        border-radius: 50%;
    "#
    ).unwrap();

    let type_style = style!(
        r#"
        font-weight: bold;
        color: var(--primary);
        margin: 0 0 0.5rem;
        font-size: 1.1rem;
        text-transform: uppercase;
        letter-spacing: 1px;
    "#
    ).unwrap();

    let course_style = style!(
        r#"
        font-weight: bold;
        margin: 0 0 0.5rem;
        font-size: 1.2rem;
    "#
    ).unwrap();

    let date_style = style!(
        r#"
        color: var(--text-secondary);
        font-size: 0.9rem;
        margin: 0;
    "#
    ).unwrap();

    let get_icon = |cred_type: &str| -> &str {
        match cred_type.to_uppercase().as_str() {
            "CERTIFICATE" => "📜",
            "BADGE" => "🥇",
            "MICRO_DEGREE" => "🎓",
            "DEGREE" => "📘",
            _ => "🏅",
        }
    };

    html! {
        <div class={badge_style}>
            <div class={icon_style}>
                {get_icon(&props.credential_type)}
            </div>
            <div class={type_style}>{&props.credential_type}</div>
            <div class={course_style}>{&props.course_name}</div>
            <p class={date_style}>{"Issued: "}{&props.issued_date}</p>
            <p class={date_style}>{"Verification code: VC-XXXXXX"}</p>
        </div>
    }
}