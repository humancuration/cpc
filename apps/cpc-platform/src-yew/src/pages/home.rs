use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <div class="home-page">
            <h1>{"Welcome to CPC Platform"}</h1>
            <p>{"Scan products to see their impact."}</p>
        </div>
    }
}