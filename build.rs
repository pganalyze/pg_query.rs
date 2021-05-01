use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let build_dir = out_dir.join("libpg_query");
    println!(
        "cargo:rerun-if-changed={}",
        build_dir.join("pg_query.h").display()
    );

    // Copy the files over
    run_command(
        Command::new("cp")
            .arg("-R")
            //.arg("-n")
            .arg("./lib/libpg_query")
            .arg(&out_dir),
    );

    let mut make = Command::new("make");
    make.env_remove("PROFILE").arg("-C").arg(&build_dir);
    if env::var("PROFILE").unwrap() == "debug" {
        make.arg("DEBUG=1");
    }
    run_command(&mut make);

    println!("cargo:rustc-link-search=native={}", build_dir.display());
    println!("cargo:rustc-link-lib=static=pg_query");

    let bindings = bindgen::Builder::default()
        .header(build_dir.join("pg_query.h").to_str().unwrap())
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn run_command(command: &mut Command) {
    let status = command
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap();
    assert!(status.success());
}
