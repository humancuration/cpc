use yew::prelude::*;
use yew_router::prelude::*;

mod visualization_client;
mod visual_scripting;
mod dashboard_nodes;
mod execution;
mod node_handlers;
mod visual_components;
mod graphql;
mod caching;
mod websocket;

use visualization_client::VisualizationClient;
use visual_scripting::VisualScriptingView;

// Include the CSS
const STYLE: &str = include_str!("styles.css");

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/visual-scripting")]
    VisualScripting,
    #[at("/visualizations")]
    Visualizations,
    #[at("/settings")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <style>{ STYLE }</style>
            <BrowserRouter>
                <div class="app-container">
                    <Header />
                    <main class="main-content">
                        <Switch<Route> render={switch} />
                    </main>
                    <Footer />
                </div>
            </BrowserRouter>
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomeView /> },
        Route::VisualScripting => html! { <VisualScriptingView /> },
        Route::Visualizations => html! { <VisualizationsView /> },
        Route::Settings => html! { <SettingsView /> },
        Route::NotFound => html! { <NotFoundView /> },
    }
}

#[function_component(Header)]
fn header() -> Html {
    html! {
        <header class="app-header">
            <div class="logo">
                <img src="/logo.png" alt="CPC Logo" />
                <h1>{"CPC Dashboard"}</h1>
            </div>
            <nav class="main-nav">
                <ul>
                    <li><Link<Route> to={Route::Home}>{"Home"}</Link<Route>></li>
                    <li><Link<Route> to={Route::VisualScripting}>{"Visual Scripting"}</Link<Route>></li>
                    <li><Link<Route> to={Route::Visualizations}>{"Visualizations"}</Link<Route>></li>
                    <li><Link<Route> to={Route::Settings}>{"Settings"}</Link<Route>></li>
                </ul>
            </nav>
            <div class="user-profile">
                <span class="user-name">{"User"}</span>
                <div class="user-avatar">{"U"}</div>
            </div>
        </header>
    }
}

#[function_component(Footer)]
fn footer() -> Html {
    html! {
        <footer class="app-footer">
            <div class="footer-content">
                <p>{"© 2023 CPC Cooperative. All rights reserved."}</p>
                <div class="footer-links">
                    <a href="/privacy">{"Privacy Policy"}</a>
                    <a href="/terms">{"Terms of Service"}</a>
                    <a href="/help">{"Help"}</a>
                </div>
            </div>
        </footer>
    }
}

#[function_component(HomeView)]
fn home_view() -> Html {
    html! {
        <div class="home-view">
            <div class="hero">
                <h2>{"Welcome to CPC Dashboard"}</h2>
                <p>{"Create, visualize, and analyze your data with our powerful dashboard tools."}</p>
                <div class="cta-buttons">
                    <Link<Route> classes="btn btn-primary" to={Route::VisualScripting}>
                        {"Start Visual Scripting"}
                    </Link<Route>>
                    <Link<Route> classes="btn btn-secondary" to={Route::Visualizations}>
                        {"View Visualizations"}
                    </Link<Route>>
                </div>
            </div>
            
            <div class="features">
                <div class="feature-card">
                    <h3>{"Visual Scripting"}</h3>
                    <p>{"Create complex data workflows with our intuitive visual scripting interface."}</p>
                </div>
                <div class="feature-card">
                    <h3>{"Real-time Visualizations"}</h3>
                    <p>{"Monitor your data in real-time with interactive charts and graphs."}</p>
                </div>
                <div class="feature-card">
                    <h3>{"Dashboard Integration"}</h3>
                    <p>{"Seamlessly integrate with other CPC applications and services."}</p>
                </div>
            </div>
        </div>
    }
}

#[function_component(VisualizationsView)]
fn visualizations_view() -> Html {
    html! {
        <div class="visualizations-view">
            <h2>{"Visualizations"}</h2>
            <div class="visualization-grid">
                <div class="visualization-card">
                    <div class="visualization-preview"></div>
                    <div class="visualization-info">
                        <h3>{"Sample Visualization"}</h3>
                        <p>{"Created on: 2023-01-01"}</p>
                        <button class="btn btn-primary">{"View"}</button>
                    </div>
                </div>
                <div class="visualization-card">
                    <div class="visualization-preview"></div>
                    <div class="visualization-info">
                        <h3>{"Another Visualization"}</h3>
                        <p>{"Created on: 2023-01-02"}</p>
                        <button class="btn btn-primary">{"View"}</button>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[function_component(SettingsView)]
fn settings_view() -> Html {
    html! {
        <div class="settings-view">
            <h2>{"Settings"}</h2>
            <div class="settings-form">
                <div class="form-group">
                    <label for="api-url">{"API Gateway URL"}</label>
                    <input type="text" id="api-url" value="http://localhost:3001" />
                </div>
                <div class="form-group">
                    <label for="default-layout">{"Default Dashboard Layout"}</label>
                    <select id="default-layout">
                        <option value="grid">{"Grid"}</option>
                        <option value="tabs">{"Tabs"}</option>
                        <option value="freeform">{"Freeform"}</option>
                    </select>
                </div>
                <div class="form-group">
                    <label for="cache-interval">{"Cache Refresh Interval (seconds)"}</label>
                    <input type="number" id="cache-interval" value="60" min="10" max="3600" />
                </div>
                <div class="form-actions">
                    <button class="btn btn-primary">{"Save Settings"}</button>
                    <button class="btn btn-secondary">{"Reset to Defaults"}</button>
                </div>
            </div>
        </div>
    }
}

#[function_component(NotFoundView)]
fn not_found_view() -> Html {
    html! {
        <div class="not-found-view">
            <h2>{"404 - Page Not Found"}</h2>
            <p>{"The page you are looking for does not exist."}</p>
            <Link<Route> classes="btn btn-primary" to={Route::Home}>
                {"Go to Home"}
            </Link<Route>>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}