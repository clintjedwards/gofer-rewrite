use std::{env, path::PathBuf, process::Command};

fn get_build_commit() -> String {
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .unwrap();
    String::from_utf8(output.stdout).unwrap()
}

fn main() {
    // Build protobuf files.
    tonic_build::configure()
        .file_descriptor_set_path(
            PathBuf::from(env::var("OUT_DIR").unwrap()).join("reflection.bin"),
        )
        .out_dir("src/proto")
        .compile(
            &[
                "src/proto/gofer.proto",
                "src/proto/gofer_transport.proto",
                "src/proto/gofer_message.proto",
            ],
            &["src/proto"],
        )
        .expect("failed compiling protos");

    // Set binary specific compile time variables.
    println!("cargo:rustc-env=BUILD_SEMVER={}", env!("CARGO_PKG_VERSION"));
    println!("cargo:rustc-env=BUILD_COMMIT={}", get_build_commit());
}
