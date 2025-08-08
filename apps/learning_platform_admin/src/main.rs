use yew::prelude::*;
use yew_router::prelude::*;
use stylist::{style, yew::styled_component};

mod components;
mod pages;
mod services;

use components::{header::Header, footer::Footer};
use pages::{dashboard::DashboardPage, analytics::AnalyticsPage, settings::SettingsPage, not_found::NotFoundPage};

// Include the CSS
const STYLE: &str = include_str!("styles.css");

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Dashboard,
    #[at("/analytics")]
    Analytics,
    #[at("/settings")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[styled_component(App)]
fn app() -> Html {
    let app_style = style!(
        r#"
        .app-container {
            display: flex;
            flex-direction: column;
            min-height: 100vh;
            background-color: #f5f5f5;
        }
        
        .main-content {
            flex: 1;
            padding: 20px;
            margin-top: 60px; /* Account for fixed header */
        }
    "#
    ).unwrap();

    html! {
        <>
            <style>{ STYLE }</style>
            <div class={app_style}>
                <BrowserRouter>
                    <Header />
                    <main class="main-content">
                        <Switch<Route> render={switch} />
                    </main>
                    <Footer />
                </BrowserRouter>
            </div>
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Dashboard => html! { <DashboardPage /> },
        Route::Analytics => html! { <AnalyticsPage /> },
        Route::Settings => html! { <SettingsPage /> },
        Route::NotFound => html! { <NotFoundPage /> },
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}