use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    prost_build::compile_protos(&["proto/data.proto"], &["proto/"])?;
    Ok(())
}
