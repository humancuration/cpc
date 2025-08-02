use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{switch, AppRoute};
use crate::contexts::{AuthContextProvider, CourseContextProvider, ThemeContextProvider};

pub mod routes;
pub mod contexts;
pub mod services;
pub mod pages;
pub mod components;
pub mod types;
pub mod utils;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <ThemeContextProvider>
            <AuthContextProvider>
                <CourseContextProvider>
                    <BrowserRouter>
                        <main>
                            <Switch<AppRoute> render={switch} />
                        </main>
                    </BrowserRouter>
                </CourseContextProvider>
            </AuthContextProvider>
        </ThemeContextProvider>
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