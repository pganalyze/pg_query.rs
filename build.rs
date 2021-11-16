#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

use std::env;
use std::fs::{self};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let build_dir = out_dir.join("libpg_query");
    let src_dir = PathBuf::from("./lib/libpg_query").canonicalize().unwrap();
    println!(
        "cargo:rerun-if-changed={}",
        build_dir.join("pg_query.h").display()
    );

    // Copy the files over
    eprintln!("Copying {} -> {}", src_dir.display(), build_dir.display());
    let changed = copy_dir(&src_dir, &build_dir).expect("Copy failed");

    // Generate the protobuf definition
    prost_build::compile_protos(
        &["./lib/libpg_query/protobuf/pg_query.proto"],
        &["./lib/libpg_query/protobuf"],
    ).expect("protobuf generation failed");

    // Now compile the C library.
    // We try to optimize the build a bit by only rebuilding if the directory tree has a detected change
    if changed {
        let mut make = Command::new("make");
        make.env_remove("PROFILE").arg("-C").arg(&build_dir);
        if env::var("PROFILE").unwrap() == "debug" {
            make.arg("DEBUG=1");
        }
        let status = make
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .unwrap();
        assert!(status.success());
    }

    // Also generate bindings
    let bindings = bindgen::Builder::default()
        .header(build_dir.join("pg_query.h").to_str().unwrap())
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-search=native={}", build_dir.display());
    println!("cargo:rustc-link-lib=static=pg_query");
}

fn copy_dir<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> std::io::Result<bool> {
    let mut stack = vec![PathBuf::from(from.as_ref())];

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    let mut changed = false;
    while let Some(working_path) = stack.pop() {
        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            eprintln!("{}", path.display());
            if path.is_dir() {
                stack.push(path);
            } else if let Some(filename) = path.file_name() {
                let dest_path = dest.join(filename);
                if dest_path.exists() {
                    if let Ok(source) = path.metadata() {
                        if let Ok(dest) = dest_path.metadata() {
                            if source.len() == dest.len() {
                                if let Ok(smtime) = source.modified() {
                                    if let Ok(dmtime) = dest.modified() {
                                        if smtime == dmtime {
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                fs::copy(&path, &dest_path)?;
                changed = true;
            }
        }
    }

    Ok(changed)
}
