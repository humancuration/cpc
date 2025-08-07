//! Shtairir Visual Editor
//!
//! A web-based visual programming editor for creating and editing Shtairir workflows.
//! This crate provides both a standalone application and reusable components for
//! integration into other Shtairir-based applications.

// Only compile the Yew app for web targets
#[cfg(target_arch = "wasm32")]
mod app;
#[cfg(target_arch = "wasm32")]
mod components;
#[cfg(target_arch = "wasm32")]
mod models;
#[cfg(target_arch = "wasm32")]
mod registry;
#[cfg(target_arch = "wasm32")]
mod serializer;
#[cfg(target_arch = "wasm32")]
mod validator;

// Public API for integration
pub mod api {
    //! Public API for integrating the Shtairir editor into other applications
    
    #[cfg(target_arch = "wasm32")]
    pub use crate::models::{Graph, Node, Connection};
    
    #[cfg(target_arch = "wasm32")]
    pub use crate::serializer::Serializer;
    
    #[cfg(target_arch = "wasm32")]
    pub use crate::validator::{Validator, ValidationResult};
}

// When compiling for WASM, expose the Yew app
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Run the Shtairir Visual Editor application
/// 
/// This function initializes and starts the Yew application.
/// It should be called from JavaScript after loading the WebAssembly module.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    use yew::prelude::*;
    
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    
    yew::Renderer::<crate::app::App>::new().render();
    
    Ok(())
}