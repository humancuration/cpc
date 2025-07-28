//! Build script to generate gRPC code from proto files.

use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the output directory for generated code
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // Configure tonic-build to generate gRPC code
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir(out_dir)
        .compile(
            &["proto/consent_manager.proto"],
            &["proto"],
        )?;
    
    // Re-run if proto files change
    println!("cargo:rerun-if-changed=proto/consent_manager.proto");
    
    Ok(())
}