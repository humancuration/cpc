fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/skill_development.proto")?;
    Ok(())
}