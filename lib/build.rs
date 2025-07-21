fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .out_dir("src/grpc") // output directory
        .compile(
            &["protos/internal.proto"],
            &["protos"], // proto root
        )?;
    Ok(())
}