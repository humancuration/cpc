fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir("src/grpc")
        .compile(
            &["../../shared_packages/skill_volunteering/proto/skill_volunteering.proto"],
            &["../../shared_packages/skill_volunteering/proto"],
        )?;
    Ok(())
}