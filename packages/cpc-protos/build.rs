fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .compile(
            &["protos/auth.proto", "protos/user.proto", "protos/impact.proto"],
            &["protos"],
        )?;
    Ok(())
}