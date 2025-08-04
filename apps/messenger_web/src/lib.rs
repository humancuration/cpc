use yew::prelude::*;
use yew_router::prelude::*;

pub mod components;
pub mod pages;
pub mod services;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/conversation/:id")]
    Conversation { id: String },
    #[at("/thread/:id")]
    Thread { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <main>
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <pages::Home /> },
        Route::Conversation { id } => html! { <pages::Conversation id={id} /> },
        Route::Thread { id } => html! { <pages::Thread id={id} /> },
        Route::NotFound => html! { <pages::NotFound /> },
    }
}

// Required for WASM
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}