use yew::prelude::*;
use stylist::{yew::styled_component, Style};

const STYLE_SHEET: &str = include_str!("styles.css");

#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet = Style::new(STYLE_SHEET).unwrap();

    html! {
        <div class={stylesheet}>
            <div class="container">
                <h1>{ "User Skill Profile" }</h1>
                // Skill list and add/remove functionality will go here
            </div>
        </div>
    }
}