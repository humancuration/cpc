//! Main entry point for the UI toolkit example application
//!
//! This module demonstrates how to use the UI toolkit components
//! in a complete Yew application.

#![cfg(feature = "demo")]

use yew::prelude::*;
use ui_toolkit::examples::app::App as ExampleApp;

/// Main application component
#[function_component(MainApp)]
fn main_app() -> Html {
    html! {
        <ExampleApp />
    }
}

fn main() {
    yew::Renderer::<MainApp>::new().render();
}