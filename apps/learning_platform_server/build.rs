fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(false) // We only need the server
        .compile(
            &[
                "../../shared_packages/grpc/learning_platform.proto",
                "proto/learning_platform_server.proto",
                "proto/health.proto"
            ],
            &[
                "../../shared_packages/grpc",
                "proto"
            ],
        )?;
    Ok(())
}