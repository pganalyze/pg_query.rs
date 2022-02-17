#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed=./libpg_query/Makefile"); // Includes version number
    println!("cargo:rustc-link-search=native=./libpg_query");
    println!("cargo:rustc-link-lib=static=pg_query");

    // Compile the C library.
    let mut make = Command::new("make");
    make.current_dir("libpg_query");
    if env::var("PROFILE").unwrap() == "debug" {
        make.arg("DEBUG=1");
    }
    let status = make.stdin(Stdio::null()).stdout(Stdio::inherit()).stderr(Stdio::inherit()).status().unwrap();
    assert!(status.success());

    // Generate bindings for Rust
    let bindings = bindgen::Builder::default().header("./libpg_query/pg_query.h").generate().expect("Unable to generate bindings");
    bindings.write_to_file(out_dir.join("bindings.rs")).expect("Couldn't write bindings!");

    // Generate the protobuf definition
    prost_build::compile_protos(&["./libpg_query/protobuf/pg_query.proto"], &["./libpg_query/protobuf"]).expect("protobuf generation failed");
}
