#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

use fs_extra::dir::CopyOptions;
use glob::glob;
use heck::ToUpperCamelCase;
use prost::Message;
use prost_types::field_descriptor_proto::Type;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::env;
use std::path::{Path, PathBuf};

static SOURCE_DIRECTORY: &str = "libpg_query";
static LIBRARY_NAME: &str = "pg_query";

type Cardinality = prost_types::field_descriptor_proto::Label;

struct Edge {
    field: String,
    message: usize,
    cardinality: Cardinality,
}

/// Represents a directed labeled multigraph of Message types. Each vertex represents a message
/// type. An edge A->B is a tuple (field_name: String, type: FieldType), that states that
/// Message A has a field (with name equal to `field_name`) of Message type B.
struct MessageGraph {
    messages: HashMap<String, usize>,

    /// For each vertex A, the list of edges from A to other vertices, and a set of vertices B such that there is at least one edge B->A
    edges: Vec<(String, Vec<Edge>, BTreeSet<usize>)>,
}

impl MessageGraph {
    fn new() -> Self {
        Self { messages: HashMap::new(), edges: Vec::new() }
    }

    /// Get the ID for a given `type_name` if it exists, or generate a new one if it doesn't
    fn id_for(&mut self, type_name: &str) -> usize {
        if let Some(id) = self.messages.get(type_name) {
            *id
        } else {
            let id = self.edges.len();
            self.edges.push((type_name.to_string(), Vec::new(), BTreeSet::new()));
            self.messages.insert(type_name.to_string(), id);
            id
        }
    }

    /// Parse protobuf files and populate the graph with its Messages and corresponding edges
    fn make(&mut self, fds: prost_types::FileDescriptorSet) {
        for fd in fds.file {
            let package = fd.package().to_string();
            for msg in fd.message_type {
                let full_name = format!(".{}.{}", package, msg.name());
                let id = self.id_for(&full_name);

                // We use this to check for duplicate fields
                let mut fields: HashSet<String> = HashSet::new();

                if msg.name() != "Node" && msg.name() != "A_Const" {
                    for field in &msg.field {
                        if field.r#type() != Type::Message {
                            continue;
                        }

                        if field.oneof_index.is_some() {
                            panic!("No support for enums: field {} of message {}", field.name(), msg.name());
                        }

                        if !fields.insert(field.name().to_string()) {
                            panic!("Duplicate field: {}", field.name());
                        }

                        let message_id = self.id_for(field.type_name());
                        self.edges[id].1.push(Edge { field: field.name().to_string(), message: message_id, cardinality: field.label() });
                        self.edges[message_id].2.insert(id);
                    }
                }
            }
        }
    }

    /// Set `filter[x] = true` for all vertices `x` with a path to vertex `id`
    fn filter_incoming(&self, id: usize, filter: &mut Vec<bool>) {
        if !filter[id] {
            filter[id] = true;
            for nb in self.edges[id].2.iter() {
                self.filter_incoming(*nb, filter);
            }
        }
    }

    /// Generate code for `unpack` impls for all Message types
    fn write(&self, buf: &mut String) {
        let mut filter = vec![false; self.messages.len()];
        self.filter_incoming(*self.messages.get(".pg_query.Node").unwrap(), &mut filter);
        for (id, (name, edges, _incoming)) in self.edges.iter().enumerate() {
            let filtered = filter[id];
            let short_name = &name[name.rfind(".").unwrap() + 1..].to_upper_camel_case();
            if short_name == "Node" || short_name == "ParseResult" || short_name == "ScanResult" || short_name == "ScanToken" {
                continue;
            }

            buf.push_str(&format!("impl<'a> Unpack<'a> for protobuf::{} {{\n", short_name));
            if filtered && edges.iter().any(|e| filter[e.message]) {
                buf.push_str("    fn unpack(&'a self, vec: &mut VecDeque<NodeRef<'a>>) {\n");
                for edge in edges.iter() {
                    if filter[edge.message] {
                        match edge.cardinality {
                            Cardinality::Repeated => buf.push_str(&format!("        self.{}.iter().for_each(|n| n.unpack(vec));\n", edge.field)),
                            Cardinality::Required => buf.push_str(&format!("        vec.push_back(self.{});\n", edge.field)),
                            Cardinality::Optional => {
                                buf.push_str(&format!("        if let Some(ref e) = self.{} {{ e.unpack(vec); }}\n", edge.field))
                            }
                        }
                    }
                }
                buf.push_str("    }\n}\n\n");
            } else {
                buf.push_str("    fn unpack(&'a self, _vec: &mut VecDeque<NodeRef<'a>>) { }\n}\n");
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    // Generate the protobuf definition
    let mut config = prost_build::Config::new();
    let fds_path = out_dir.join("./file_descriptor_set.bin");
    config.file_descriptor_set_path(fds_path.clone());
    config.compile_protos(&[&out_protobuf_path.join(LIBRARY_NAME).with_extension("proto")], &[&out_protobuf_path])?;

    let mut buf = String::new();
    let fds = prost_types::FileDescriptorSet::decode(std::fs::read(fds_path)?.as_slice())?;
    let mut graph = MessageGraph::new();
    graph.make(fds);
    graph.write(&mut buf);
    std::fs::write(out_dir.join("./unpack.rs"), buf)?;

    prost_build::compile_protos(&[&out_protobuf_path.join(LIBRARY_NAME).with_extension("proto")], &[&out_protobuf_path])?;

    Ok(())
}
