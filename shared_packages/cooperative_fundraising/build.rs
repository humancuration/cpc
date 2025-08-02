fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compile the proto files
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(
            &["proto/cooperative_fundraising.proto"],
            &["proto"]
        )?;
    
    Ok(())
}