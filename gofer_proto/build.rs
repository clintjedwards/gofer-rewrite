use std::{env, path::PathBuf};

fn main() {
    // Build protobuf files.
    tonic_build::configure()
        .file_descriptor_set_path(
            PathBuf::from(env::var("OUT_DIR").unwrap()).join("reflection.bin"),
        )
        .out_dir("src")
        .compile(
            &[
                "src/gofer.proto",
                "src/gofer_transport.proto",
                "src/gofer_message.proto",
            ],
            &["src"],
        )
        .expect("failed compiling protos");
}
