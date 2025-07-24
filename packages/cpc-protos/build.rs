fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .compile(
            &[
                "proto/job_service.proto",
                "proto/file_hosting.proto",
                "proto/social_features.proto",
                "protos/impact.proto"
            ],
            &["proto", "protos"],
        )?;
    Ok(())
}