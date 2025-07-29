//! Build script for the music player module

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Set environment variables for conditional compilation
    println!("cargo:rustc-env=MUSIC_PLAYER_MODULE_VERSION=0.1.0");
    
    // Create module metadata
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("module_metadata.rs");
    
    fs::write(
        &dest_path,
        r#"
        pub const MODULE_NAME: &str = "music-player";
        pub const MODULE_VERSION: &str = "0.1.0";
        pub const MODULE_DESCRIPTION: &str = "Full-featured music streaming platform with social features including timestamped comments, visualizers, and offline playback";
        "#
    ).unwrap();
    
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=MODULE.toml");
}