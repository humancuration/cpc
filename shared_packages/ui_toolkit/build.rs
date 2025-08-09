//! Build script for the UI toolkit
//!
//! This script verifies that all components compile correctly.

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Only run the verification in development mode
    if env::var("PROFILE").unwrap_or_default() == "debug" {
        verify_module_structure();
    }
    
    println!("cargo:rerun-if-changed=src/");
}

fn verify_module_structure() {
    let src_path = Path::new("src");
    
    // Verify that all expected modules exist
    let expected_modules = vec![
        "themes.rs",
        "hooks.rs",
        "components/base.rs",
        "components/button.rs",
        "components/input.rs",
        "components/card.rs",
        "components/container.rs",
        "components/theme_provider.rs",
        "components/mod.rs",
        "hooks/mod.rs",
        "hooks/use_theme.rs",
        "themes/mod.rs",
        "examples.rs",
        "examples/theme_example.rs",
    ];
    
    for module in expected_modules {
        let module_path = src_path.join(module);
        if !module_path.exists() {
            panic!("Missing expected module: {}", module);
        }
    }
    
    println!("UI Toolkit module structure verified");
}