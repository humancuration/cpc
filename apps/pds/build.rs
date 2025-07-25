use std::env;

fn main() {
    // Set up build configuration for cross-platform deployment
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    
    match target_os.as_str() {
        "windows" => {
            println!("cargo:rustc-link-arg=/SUBSYSTEM:WINDOWS");
            // Add Windows-specific build configurations
        }
        "macos" => {
            // Add macOS-specific build configurations
            println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path/../Frameworks");
        }
        "linux" => {
            // Add Linux-specific build configurations
            println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
        }
        _ => {}
    }
    
    // Ensure we rebuild if environment changes
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_OS");
}