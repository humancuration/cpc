use crate::router::{switch, AppRoute};
use yew::prelude::*;
use yew::ServerRenderer;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<AppRoute> render={switch} />
        </BrowserRouter>
    }
}

pub async fn render_to_string() -> String {
    let renderer = ServerRenderer::<App>::new();
    renderer.render().await
}