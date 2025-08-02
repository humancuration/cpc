fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(false) // We only need the server
        .compile(
            &["../../shared_packages/grpc/learning_platform.proto"],
            &["../../shared_packages/grpc"],
        )?;
    Ok(())
}