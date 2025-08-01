//! Build script for CPay Core
//! 
//! This script generates Rust code from protobuf definitions for gRPC services.

use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure the proto compilation
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(
            &["proto/cpay.proto"], 
            &["proto"]
        )?;
    
    // Re-run if proto files change
    println!("cargo:rerun-if-changed=proto/cpay.proto");
    
    Ok(())
}