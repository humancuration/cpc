use yew::prelude::*;
use stylist::{style, yew::styled_component};
use crate::components::tip_form::TipForm;

#[styled_component(TippingPage)]
pub fn tipping_page() -> Html {
    let container_style = style!(
        r#"
        padding: 2rem;
        max-width: 800px;
        margin: 0 auto;
    "#
    ).unwrap();

    let header_style = style!(
        r#"
        margin-bottom: 2rem;
        text-align: center;
    "#
    ).unwrap();

    let card_style = style!(
        r#"
        background: var(--surface);
        border-radius: 8px;
        padding: 2rem;
        box-shadow: 0 2px 10px rgba(0,0,0,0.1);
    "#
    ).unwrap();

    html! {
        <div class={container_style}>
            <div class={header_style}>
                <h1>{"Support Educators"}</h1>
                <p>{"Show your appreciation by tipping educators for their excellent work"}</p>
            </div>
            <div class={card_style}>
                <TipForm />
            </div>
        </div>
    }
}