//! Main entry point for the Finance-Sheets web application
//!
//! This module initializes the Yew application and sets up the main routes.

fn main() {
    // Initialize the logger
    wasm_logger::init(wasm_logger::Config::default());
    
    // Start the Yew application
    yew::Renderer::<finance_sheets::App>::new().render();
}