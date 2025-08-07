//! Shtairir Visual Editor - Main Entry Point
//!
//! This is the main entry point for the standalone Shtairir Visual Editor application.

fn main() {
    // For web targets, the application is started via WASM
    // This main function is only used for native targets or CLI operations
    #[cfg(not(target_arch = "wasm32"))]
    {
        println!("Shtairir Visual Editor");
        println!("This application is designed to run in a web browser.");
        println!("To build for web, use: wasm-pack build --target web");
    }
}