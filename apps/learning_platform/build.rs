use std::io::Result;

fn main() -> Result<()> {
    // Compile the proto files
    tonic_build::compile_protos("../shared_packages/learning_core/proto/learning_platform.proto")?;
    Ok(())
}