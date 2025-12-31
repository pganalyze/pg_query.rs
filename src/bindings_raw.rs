//! Raw FFI bindings for PostgreSQL parse tree types.
//!
//! These bindings provide direct access to PostgreSQL's internal node structures,
//! allowing us to convert them to Rust types without going through protobuf serialization.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(clippy::all)]
#![allow(dead_code)]
include!(concat!(env!("OUT_DIR"), "/bindings_raw.rs"));
