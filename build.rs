use lcm_gen::LcmGen;
use std::{error, path::PathBuf};

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut lcm_source_dir: PathBuf = env!("CARGO_MANIFEST_DIR").into();
    lcm_source_dir.push("lcm");
    println!("cargo:rerun-if-changed={}", lcm_source_dir.display());

    prost_build::compile_protos(&["proto/data.proto"], &["proto/"])?;
    ::capnpc::CompilerCommand::new()
        .file("capnp/data.capnp")
        .run()
        .expect("compiling schema");
    LcmGen::new().add_directory(lcm_source_dir).run();
    Ok(())
}
