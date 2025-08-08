use yew::prelude::*;
use ui_toolkit::components::button::Button;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let onclick = Callback::from(|_| web_sys::console::log_1(&"Button clicked!".into()));
    
    html! {
        <div>
            <h1>{"Welcome to CPC Web Portal"}</h1>
            <Button label="Click me!" on_click={onclick} variant="primary" />
        </div>
    }
}