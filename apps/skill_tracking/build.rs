use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::compile_protos("../../shared_packages/skill_development/proto/skill_development.proto")?;
    Ok(())
}