#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

use fs_extra::dir::CopyOptions;
use glob::glob;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

static SOURCE_DIRECTORY: &str = "libpg_query";
static LIBRARY_NAME: &str = "pg_query";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // return if proto file changes
    println!("cargo:rerun-if-changed=libpg_query/protobuf/pg_query.proto");

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let build_path = Path::new(".").join(SOURCE_DIRECTORY);
    let out_header_path = out_dir.join(LIBRARY_NAME).with_extension("h");
    let out_protobuf_path = out_dir.join("protobuf");
    let target = env::var("TARGET").unwrap();

    // Configure cargo through stdout
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
    let mut build = cc::Build::new();
    build
        .files(glob(out_dir.join("src/*.c").to_str().unwrap()).unwrap().map(|p| p.unwrap()))
        .files(glob(out_dir.join("src/postgres/*.c").to_str().unwrap()).unwrap().map(|p| p.unwrap()))
        .file(out_dir.join("vendor/protobuf-c/protobuf-c.c"))
        .file(out_dir.join("vendor/xxhash/xxhash.c"))
        .file(out_dir.join("protobuf/pg_query.pb-c.c"))
        .include(out_dir.join("."))
        .include(out_dir.join("./vendor"))
        .include(out_dir.join("./src/postgres/include"))
        .include(out_dir.join("./src/include"))
        .warnings(false); // Avoid unnecessary warnings, as they are already considered as part of libpg_query development
    if env::var("PROFILE").unwrap() == "debug" || env::var("DEBUG").unwrap() == "1" {
        build.define("USE_ASSERT_CHECKING", None);
    }
    if target.contains("windows") {
        build.include(out_dir.join("./src/postgres/include/port/win32"));
        if target.contains("msvc") {
            build.include(out_dir.join("./src/postgres/include/port/win32_msvc"));
        }
    }
    build.compile(LIBRARY_NAME);

    // Generate bindings for Rust
    bindgen::Builder::default()
        .header(out_header_path.to_str().ok_or("Invalid header path")?)
        .generate()
        .map_err(|_| "Unable to generate bindings")?
        .write_to_file(out_dir.join("bindings.rs"))?;

    // Only generate protobuf bindings if protoc is available
    let protoc_exists = Command::new("protoc").arg("--version").status().is_ok();
    // If the package is being built by docs.rs, we don't want to regenerate the protobuf bindings
    let is_built_by_docs_rs = env::var("DOCS_RS").is_ok();

    if !is_built_by_docs_rs && (env::var("REGENERATE_PROTOBUF").is_ok() || protoc_exists) {
        println!("generating protobuf bindings");
        // HACK: Set OUT_DIR to src/ so that the generated protobuf file is copied to src/protobuf.rs
        let src_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("src");
        env::set_var("OUT_DIR", &src_dir);

        prost_build::compile_protos(&[&out_protobuf_path.join(LIBRARY_NAME).with_extension("proto")], &[&out_protobuf_path])?;

        std::fs::rename(src_dir.join("pg_query.rs"), src_dir.join("protobuf.rs"))?;

        // Reset OUT_DIR to the original value
        env::set_var("OUT_DIR", &out_dir);
    } else {
        println!("skipping protobuf generation");
    }

    Ok(())
}
