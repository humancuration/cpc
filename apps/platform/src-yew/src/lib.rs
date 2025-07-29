use yew::prelude::*;
use yewdux::prelude::*;

pub mod components;
pub mod services;
pub mod types;
pub mod store;
pub mod graphql;

use yew_router::prelude::*;
use components::impact::impact_dashboard::ImpactDashboard;
use components::main_menu::MainMenu;
use routes::{Route, switch};
use store::Store;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <StoreProvider<Store> store={Store::default()}>
            <BrowserRouter>
                <div class="app">
                    <MainMenu />
                    <main>
                        <Switch<Route> render={Switch::render(switch)} />
                    </main>
                    <ImpactDashboard />
                </div>
            </BrowserRouter>
        </StoreProvider<Store>>
    }
}