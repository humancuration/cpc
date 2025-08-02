use std::io::Result;

fn main() -> Result<()> {
    // Generate gRPC code from proto files
    tonic_build::configure()
        .build_server(false) // We only need the client
        .compile(
            &["../../shared_packages/cooperative_fundraising/proto/cooperative_fundraising.proto"],
            &["../../shared_packages/cooperative_fundraising/proto"],
        )?;
    
    Ok(())
}