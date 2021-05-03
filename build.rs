#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

use convert_case::{Case, Casing};

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Write};
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

    // Generate the AST first
    generate_ast(&build_dir, &out_dir).expect("AST generation");

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

#[derive(serde::Deserialize)]
pub struct Struct {
    pub fields: Vec<Field>,
    pub comment: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct Field {
    pub name: Option<String>,
    pub c_type: Option<String>,
    pub comment: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct Enum {
    pub values: Vec<EnumValue>,
    pub comment: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct EnumValue {
    pub name: Option<String>,
    pub comment: Option<String>,
    pub value: Option<u32>,
}

#[derive(serde::Deserialize)]
pub struct TypeDef {
    new_type_name: String,
    source_type: String,
    comment: Option<String>,
}

fn generate_ast(build_dir: &Path, out_dir: &Path) -> std::io::Result<()> {
    let srcdata_dir = build_dir.join("srcdata");
    assert!(
        srcdata_dir.exists(),
        "srcdata_dir did not exist: {}",
        srcdata_dir.display()
    );

    // Common out dir
    let out_file = File::create(out_dir.join("ast.rs"))?;
    let mut out_file = BufWriter::new(out_file);

    // Keep track of types for type resolution
    let mut type_resolver = TypeResolver::new();

    // Read in all "Node" types as this helps generating struct vs node
    let node_types = File::open(srcdata_dir.join("nodetypes.json"))?;
    let node_types = BufReader::new(node_types);
    let node_types: Vec<String> = serde_json::from_reader(node_types)?;
    for ty in node_types.iter() {
        type_resolver.add_node(ty);
    }
    let node_types = node_types.into_iter().collect();

    // Generate type aliases first
    let type_defs = File::open(srcdata_dir.join("typedefs.json"))?;
    let type_defs = BufReader::new(type_defs);
    let type_defs: Vec<TypeDef> = serde_json::from_reader(type_defs)?;
    for ty in type_defs.iter() {
        type_resolver.add_alias(&ty.new_type_name, &ty.source_type);
    }
    make_aliases(&mut out_file, &type_defs, &node_types, &mut type_resolver)?;

    // Enums
    let enum_defs = File::open(srcdata_dir.join("enum_defs.json"))?;
    let enum_defs = BufReader::new(enum_defs);
    let enum_defs: HashMap<String, HashMap<String, Enum>> = serde_json::from_reader(enum_defs)?;
    for map in enum_defs.values() {
        for ty in map.keys() {
            type_resolver.add_type(ty);
        }
    }
    make_enums(&mut out_file, &enum_defs)?;

    // Structs
    let struct_defs = File::open(srcdata_dir.join("struct_defs.json"))?;
    let struct_defs = BufReader::new(struct_defs);
    let struct_defs: HashMap<String, HashMap<String, Struct>> =
        serde_json::from_reader(struct_defs)?;
    for map in struct_defs.values() {
        for ty in map.keys() {
            if !type_resolver.contains(ty) {
                type_resolver.add_type(ty);
            }
        }
    }

    // Finally make the nodes and the primitives
    make_nodes(&mut out_file, &struct_defs, &node_types, &type_resolver)?;
    Ok(())
}

fn make_aliases(
    out: &mut BufWriter<File>,
    type_defs: &[TypeDef],
    node_types: &HashSet<String>,
    type_resolver: &mut TypeResolver,
) -> std::io::Result<()> {
    const IGNORE: [&str; 5] = [
        "BlockId",
        "ExpandedObjectHeader",
        "Name",
        "ParamListInfo",
        "VacAttrStatsP",
    ];

    for def in type_defs {
        // TODO: Look into what these are actually for
        if IGNORE.iter().any(|e| def.new_type_name.eq(e)) {
            continue;
        }
        if node_types.contains(&def.source_type) {
            // Type alias won't work, so just ignore this and replace the type
            type_resolver.add_node(&def.new_type_name);
            continue;
        }
        if let Some(comment) = &def.comment {
            writeln!(out, "{}", comment)?;
        }
        let ty = match &def.source_type[..] {
            "char" => "char",
            "double" => "f64",
            "int16" => "i16",
            "signed int" => "i32",
            "uint32" => "u32",
            "unsigned int" => "u32",
            "uintptr_t" => "usize",

            "BlockIdData" => "BlockIdData",
            "NameData" => "NameData",
            "Oid" => "Oid",
            "OpExpr" => "OpExpr",
            "ParamListInfoData" => "ParamListInfoData",
            "regproc" => "regproc",
            "TransactionId" => "TransactionId",
            "VacAttrStats" => "VacAttrStats",

            unexpected => panic!("Unrecognized type for alias: {}", unexpected),
        };
        writeln!(out, "pub type {} = {};", def.new_type_name, ty)?;
        type_resolver.add_type(&def.new_type_name);
    }
    Ok(())
}

fn make_enums(
    out: &mut BufWriter<File>,
    enum_defs: &HashMap<String, HashMap<String, Enum>>,
) -> std::io::Result<()> {
    const SECTIONS: [&str; 4] = [
        "nodes/parsenodes",
        "nodes/primnodes",
        "nodes/lockoptions",
        "nodes/nodes",
    ];
    for section in &SECTIONS {
        let map = &enum_defs[*section];
        let mut map = map.iter().collect::<Vec<_>>();
        map.sort_by_key(|x| x.0);

        for (name, def) in map {
            writeln!(out, "#[derive(Debug, serde::Deserialize)]")?;
            writeln!(out, "pub enum {} {{", name)?;
            // This enum has duplicate values - I don't think these are really necessary
            let ignore_value = name.eq("PartitionRangeDatumKind");

            for value in &def.values {
                if let Some(comment) = &value.comment {
                    writeln!(out, "    {}", comment)?;
                }
                if let Some(name) = &value.name {
                    if ignore_value {
                        writeln!(out, "    {},", name)?;
                    } else if let Some(v) = &value.value {
                        writeln!(out, "    {} = {},", name, *v)?;
                    } else {
                        writeln!(out, "    {},", name)?;
                    }
                }
            }
            writeln!(out, "}}")?;
            writeln!(out)?;
        }
    }
    Ok(())
}

fn make_nodes(
    out: &mut BufWriter<File>,
    struct_defs: &HashMap<String, HashMap<String, Struct>>,
    node_types: &HashSet<String>,
    type_resolver: &TypeResolver,
) -> std::io::Result<()> {
    const SECTIONS: [&str; 2] = ["nodes/parsenodes", "nodes/primnodes"];

    writeln!(out, "#[derive(Debug, serde::Deserialize)]")?;
    writeln!(out, "pub enum Node {{")?;

    for section in &SECTIONS {
        let map = &struct_defs[*section];
        let mut map = map.iter().collect::<Vec<_>>();
        map.sort_by_key(|x| x.0);

        for (name, def) in map {
            // Only generate node types
            if !node_types.contains(name) {
                // We panic here since all structs are nodes for our purposes
                panic!("Unexpected struct `{}` (not a node).", name);
            }
            // If no fields just generate an empty variant
            if def.fields.is_empty() {
                writeln!(out, "    {},", name)?;
                continue;
            }

            // Generate with a passable struct
            writeln!(out, "    {}({}),", name, name)?;
        }
    }

    // Also do "Value" type nodes. These are generated differently.
    writeln!(out, "    // Value nodes")?;
    let values = &struct_defs["nodes/value"];
    let mut values = values.iter().collect::<Vec<_>>();
    values.sort_by_key(|x| x.0);

    for (name, def) in values {
        if def.fields.is_empty() {
            writeln!(out, "    {},", name)?;
            continue;
        }
        // Only one
        let field = &def.fields[0];
        writeln!(out, "    {} {{", name)?;
        writeln!(
            out,
            "        #[serde(rename = \"{}\")]",
            field.name.as_ref().unwrap(),
        )?;
        writeln!(
            out,
            "        value: {}",
            type_resolver.resolve(field.c_type.as_ref().unwrap())
        )?;
        writeln!(out, "    }},")?;
    }

    writeln!(out, "}}")?;

    // Generate the structs
    for section in &SECTIONS {
        let map = &struct_defs[*section];
        let mut map = map.iter().collect::<Vec<_>>();
        map.sort_by_key(|x| x.0);

        for (name, def) in map {
            writeln!(out)?;
            writeln!(out, "#[derive(Debug, serde::Deserialize)]")?;
            writeln!(out, "pub struct {} {{", name)?;

            for field in &def.fields {
                let (name, c_type) = match (&field.name, &field.c_type) {
                    (&Some(ref name), &Some(ref c_type)) => (name, c_type),
                    _ => continue,
                };

                // These are meta data fields and have no real use
                if name == "type" || name == "xpr" {
                    continue;
                }

                if is_reserved(&name) {
                    writeln!(
                        out,
                        "    #[serde(rename = \"{}\"{})]",
                        name,
                        if type_resolver.is_primitive(c_type) {
                            ", default"
                        } else {
                            ""
                        }
                    )?;
                    writeln!(out, "    pub {}_: {},", name, type_resolver.resolve(c_type))?;
                } else {
                    // Figure out what the snake case version of the name is
                    let cleaned = name.to_case(Case::Snake);
                    if cleaned.eq(name) {
                        if type_resolver.is_primitive(c_type) {
                            writeln!(out, "    #[serde(default)]",)?;
                        }
                        writeln!(out, "    pub {}: {},", name, type_resolver.resolve(c_type))?;
                    } else {
                        writeln!(
                            out,
                            "    #[serde(rename = \"{}\"{})]",
                            name,
                            if type_resolver.is_primitive(c_type) {
                                ", default"
                            } else {
                                ""
                            }
                        )?;
                        writeln!(
                            out,
                            "    pub {}: {},",
                            cleaned,
                            type_resolver.resolve(c_type)
                        )?;
                    }
                }
            }

            writeln!(out, "}}")?;
        }
    }

    Ok(())
}

fn is_reserved(variable: &str) -> bool {
    matches!(
        variable,
        "abstract"
            | "become"
            | "box"
            | "do"
            | "final"
            | "macro"
            | "override"
            | "priv"
            | "try"
            | "typeof"
            | "unsized"
            | "virtual"
            | "yield"
    )
}

struct TypeResolver {
    aliases: HashMap<String, bool>, // bool = primitive
    primitive: HashMap<&'static str, &'static str>,
    nodes: HashSet<String>,
    types: HashSet<String>,
}

impl TypeResolver {
    pub fn new() -> Self {
        let mut primitive = HashMap::new();
        primitive.insert("uint32", "u32");
        primitive.insert("uint64", "u64");
        primitive.insert("bits32", "bits32"); // Alias
        primitive.insert("bool", "bool");
        primitive.insert("int", "i32");
        primitive.insert("long", "i64");
        primitive.insert("int32", "i32");
        primitive.insert("char*", "Option<String>"); // Make all strings optional
        primitive.insert("int16", "i16");
        primitive.insert("char", "char");
        primitive.insert("double", "f64");
        primitive.insert("signed int", "i32");
        primitive.insert("unsigned int", "u32");
        primitive.insert("uintptr_t", "usize");

        // Similar to primitives
        primitive.insert("List*", "Option<Vec<Node>>");
        primitive.insert("Node*", "Option<Box<Node>>");
        primitive.insert("Expr*", "Option<Box<Node>>");

        // TODO: Bitmapset is defined in bitmapset.h and is roughly equivalent to a vector of u32's.
        //       It'll do for now.
        primitive.insert("Bitmapset*", "Option<Vec<u32>>");

        TypeResolver {
            primitive,

            aliases: HashMap::new(),
            nodes: HashSet::new(),
            types: HashSet::new(),
        }
    }

    pub fn add_alias(&mut self, ty: &str, target: &str) {
        self.aliases
            .insert(ty.to_string(), self.primitive.contains_key(target));
    }

    pub fn add_node(&mut self, ty: &str) {
        self.nodes.insert(ty.to_string());
    }

    pub fn add_type(&mut self, ty: &str) {
        self.types.insert(ty.to_string());
    }

    pub fn contains(&self, ty: &str) -> bool {
        self.aliases.contains_key(ty)
            || self.primitive.contains_key(ty)
            || self.nodes.contains(ty)
            || self.types.contains(ty)
    }

    pub fn is_primitive(&self, ty: &str) -> bool {
        self.primitive.contains_key(ty) || self.aliases.get(ty).copied().unwrap_or_default()
    }

    pub fn resolve(&self, c_type: &str) -> String {
        if let Some(ty) = self.primitive.get(c_type) {
            return ty.to_string();
        }
        if let Some(ty) = c_type.strip_suffix('*') {
            if let Some(primitive) = self.aliases.get(c_type) {
                if *primitive {
                    return format!("Option<{}>", ty);
                } else {
                    return format!("Option<Box<{}>>", ty);
                }
            }

            if self.nodes.contains(ty) || self.types.contains(ty) {
                return format!("Option<Box<{}>>", ty);
            }
        } else {
            if let Some(primitive) = self.aliases.get(c_type) {
                if *primitive {
                    return c_type.to_string();
                } else {
                    return format!("Box<{}>", c_type);
                }
            }

            if self.nodes.contains(c_type) || self.types.contains(c_type) {
                return format!("Box<{}>", c_type);
            }
        }

        // SHOULD be unreachable
        // let mut expected = String::new();
        // for ty in self.types.keys() {
        //     expected.push_str(ty);
        //     expected.push(',');
        // }
        unreachable!("Unexpected type: {}", c_type)
    }
}
