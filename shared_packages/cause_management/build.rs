//! Build script for cause_management
//!
//! This build script generates the gRPC code from the proto files.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Re-run if the proto file changes
    println!("cargo:rerun-if-changed=../protos/cpay.proto");
    
    // Compile the proto file
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(&["../protos/cpay.proto"], &["../protos"])?;
    
    Ok(())
}