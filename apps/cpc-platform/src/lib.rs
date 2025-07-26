pub mod business_tools;
pub mod impact;
pub mod api;
pub mod context;
pub mod components;
pub mod routes;
pub mod stores;
pub mod styles;
pub mod utils;

use yew::prelude::*;
use crate::business_tools::BusinessToolsApp;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <BusinessToolsApp />
        </main>
    }
}
