use crate::router::{switch, AppRoute};
use yew::prelude::*;
use yew::ServerRenderer;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct AppProps {
    pub route: String,
}

#[function_component(App)]
pub fn app(props: &AppProps) -> Html {
    html! {
        <StaticRouter location={props.route.clone()}>
            <Switch<AppRoute> render={switch} />
        </StaticRouter>
    }
}

pub async fn render_to_string_with_route(route: &str) -> String {
    let props = AppProps { route: route.to_string() };
    let renderer = ServerRenderer::<App>::with_props(props);
    renderer.render().await
}