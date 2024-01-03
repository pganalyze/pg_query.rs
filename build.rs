#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

use fs_extra::dir::CopyOptions;
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

static SOURCE_DIRECTORY: &str = "libpg_query";
static LIBRARY_NAME: &str = "pg_query";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let build_path = Path::new(".").join(SOURCE_DIRECTORY);
    let makefile_path = build_path.join("Makefile");
    let out_header_path = out_dir.join(LIBRARY_NAME).with_extension("h");
    let out_protobuf_path = out_dir.join("protobuf");

    // Configure cargo through stdout
    println!("cargo:rerun-if-changed={}", makefile_path.display()); // Includes version number
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static={LIBRARY_NAME}");

    // Copy the relevant source files to the OUT_DIR
    let source_paths = vec![
        build_path.join(LIBRARY_NAME).with_extension("h"),
        build_path.join("Makefile"),
        build_path.join("src"),
        build_path.join("protobuf"),
        build_path.join("vendor"),
    ];

    let copy_options = CopyOptions { overwrite: true, ..CopyOptions::default() };

    fs_extra::copy_items(&source_paths, &out_dir, &copy_options)?;

    // Compile the C library.
    let mut make = Command::new("make");

    make.env_remove("PROFILE").arg("-C").arg(&out_dir).arg("build");

    if env::var("PROFILE").unwrap() == "debug" {
        make.arg("DEBUG=1");
    }

    let status = make.stdin(Stdio::null()).stdout(Stdio::inherit()).stderr(Stdio::inherit()).status()?;

    if !status.success() {
        return Err("libpg_query compilation failed".into());
    }

    // Generate bindings for Rust
    bindgen::Builder::default()
        .header(out_header_path.to_str().ok_or("Invalid header path")?)
        .generate()
        .map_err(|_| "Unable to generate bindings")?
        .write_to_file(out_dir.join("bindings.rs"))?;

    // Generate the protobuf definition
    let mut config = prost_build::Config::new();
    config.recursion_limit("ParseResult", 1000);
    config.compile_protos(&[&out_protobuf_path.join(LIBRARY_NAME).with_extension("proto")], &[&out_protobuf_path])?;

    Ok(())
}
