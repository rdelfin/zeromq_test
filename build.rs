use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    tonic_build::compile_protos("proto/data.proto")?;
    ::capnpc::CompilerCommand::new()
        .file("capnp/data.capnp")
        .run()
        .expect("compiling schema");
    Ok(())
}
