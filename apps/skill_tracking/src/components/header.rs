use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="header">
            <h1>{"Skill Development Tracker"}</h1>
            <p>{"Track your skills, learning paths, and certifications"}</p>
        </header>
    }
}