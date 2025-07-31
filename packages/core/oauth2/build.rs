//! Build script to generate gRPC code from proto files

use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the output directory for generated code
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // Create the generated directory if it doesn't exist
    let generated_dir = out_dir.join("generated");
    std::fs::create_dir_all(&generated_dir)?;
    
    // Compile the proto file
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir(generated_dir)
        .compile(&["proto/oauth.proto"], &["proto"])?;
    
    println!("cargo:rerun-if-changed=proto/oauth.proto");
    
    Ok(())
}