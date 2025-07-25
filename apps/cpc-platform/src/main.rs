use yew::prelude::*;
use yew_router::prelude::*;
use cpc_platform::routes::{switch, AppRoute};

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<AppRoute> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}