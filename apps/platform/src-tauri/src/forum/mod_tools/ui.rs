use yew::prelude::*;

#[function_component(ModTools)]
pub fn mod_tools() -> Html {
    html! {
        <div>
            <h1>{ "Moderation Tools" }</h1>
            <p>{ "Here you will find tools for moderation." }</p>
        </div>
    }
}