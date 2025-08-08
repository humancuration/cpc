//! Example demonstrating the Unified Community Impact Dashboard
//!
//! This example shows how to use the dashboard components in a simple application.

use community_impact_dashboard::dashboard::UnifiedImpactDashboard;
use yew::prelude::*;

/// Example application component
#[function_component(ExampleApp)]
fn example_app() -> Html {
    html! {
        <div class="example-app">
            <h1>{"Unified Community Impact Dashboard Example"}</h1>
            <UnifiedImpactDashboard />
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<ExampleApp>::new().render();
}