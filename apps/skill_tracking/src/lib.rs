mod components;
mod services;
mod types;

use yew::prelude::*;

use components::header::Header;
use components::main_dashboard::MainDashboard;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="app">
                <Header />
                <MainDashboard />
            </div>
        }
    }
}