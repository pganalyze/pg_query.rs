use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
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

    // Generate the AST first
    generate_ast(&build_dir, &out_dir).expect("AST generation");

    // Now compile and generate bindings
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

fn generate_ast(build_dir: &PathBuf, out_dir: &PathBuf) -> std::io::Result<()> {
    let srcdata_dir = build_dir.join("srcdata");

    // Common out dir
    let out_file = File::create(out_dir.join("ast.rs"))?;
    let mut out_file = BufWriter::new(out_file);

    // Type defs first
    let type_defs = File::open(srcdata_dir.join("typedefs.json"))?;
    let type_defs = BufReader::new(type_defs);
    let type_defs: Vec<TypeDef> = serde_json::from_reader(type_defs)?;
    make_aliases(&type_defs, &mut out_file)?;

    // Enums
    let enum_defs = File::open(srcdata_dir.join("enum_defs.json"))?;
    let enum_defs = BufReader::new(enum_defs);
    let enum_defs: HashMap<String, HashMap<String, Enum>> = serde_json::from_reader(enum_defs)?;
    make_enums(&enum_defs, &mut out_file)?;

    // Structs
    let struct_defs = File::open(srcdata_dir.join("struct_defs.json"))?;
    let struct_defs = BufReader::new(struct_defs);
    let struct_defs: HashMap<String, HashMap<String, Struct>> =
        serde_json::from_reader(struct_defs)?;
    make_primitives(&struct_defs, &mut out_file)?;
    make_nodes(&struct_defs, &mut out_file)?;
    Ok(())
}

fn make_aliases(type_defs: &Vec<TypeDef>, out: &mut BufWriter<File>) -> std::io::Result<()> {
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
    }
    Ok(())
}

fn make_enums(
    enum_defs: &HashMap<String, HashMap<String, Enum>>,
    out: &mut BufWriter<File>,
) -> std::io::Result<()> {
    let sections = vec![
        "nodes/parsenodes",
        "nodes/primnodes",
        "nodes/lockoptions",
        "nodes/nodes",
    ];
    for section in sections {
        for (name, def) in &enum_defs[section] {
            write!(out, "#[derive(Debug, serde::Deserialize)]\n")?;
            write!(out, "pub enum {} {{\n", name)?;
            // This enum has duplicate values - I don't think these are really necessary
            let ignore_value = name.eq("PartitionRangeDatumKind");

            for value in &def.values {
                if let Some(comment) = &value.comment {
                    write!(out, "    {}\n", comment)?;
                }
                if let Some(name) = &value.name {
                    if ignore_value {
                        write!(out, "    {},\n", name)?;
                    } else {
                        if let Some(v) = &value.value {
                            write!(out, "    {} = {},\n", name, *v)?;
                        } else {
                            write!(out, "    {},\n", name)?;
                        }
                    }
                }
            }
            write!(out, "}}\n\n")?;
        }
    }
    Ok(())
}

fn make_primitives(
    struct_defs: &HashMap<String, HashMap<String, Struct>>,
    out: &mut BufWriter<File>,
) -> std::io::Result<()> {
    for (name, def) in &struct_defs["nodes/primnodes"] {
        write!(out, "#[derive(Debug, serde::Deserialize)]\n")?;
        write!(out, "pub struct {} {{\n", name)?;
        for field in &def.fields {
            let (name, c_type) = match (&field.name, &field.c_type) {
                (&Some(ref name), &Some(ref c_type)) => (name, c_type),
                _ => continue,
            };

            if name == "type" {
                continue;
            }
            if is_reserved(&name) {
                write!(out, "    #[serde(rename = \"{}\")]\n", name)?;
                write!(out, "    pub {}_: {},\n", name, c_to_rust_type(c_type))?;
            } else {
                write!(out, "    pub {}: {},\n", name, c_to_rust_type(c_type))?;
            }
        }
        write!(out, "}}\n")?;
    }
    Ok(())
}

fn make_nodes(
    struct_defs: &HashMap<String, HashMap<String, Struct>>,
    out: &mut BufWriter<File>,
) -> std::io::Result<()> {
    write!(out, "#[derive(Debug, serde::Deserialize)]\n")?;
    write!(out, "pub enum Node {{\n")?;

    for (name, def) in &struct_defs["nodes/parsenodes"] {
        write!(out, "    {} {{\n", name)?;

        for field in &def.fields {
            let (name, c_type) = match (&field.name, &field.c_type) {
                (&Some(ref name), &Some(ref c_type)) => (name, c_type),
                _ => continue,
            };

            if name == "type" {
                continue;
            }
            if is_reserved(&name) {
                write!(out, "        #[serde(rename = \"{}\")]\n", name)?;
                write!(out, "        {}_: {},\n", name, c_to_rust_type(c_type))?;
            } else {
                write!(out, "        {}: {},\n", name, c_to_rust_type(c_type))?;
            }
        }

        write!(out, "    }},\n")?;
    }
    write!(out, "}}\n")?;
    Ok(())
}

fn is_reserved(variable: &str) -> bool {
    match variable {
        "abstract" | "become" | "box" | "do" | "final" | "macro" | "override" | "priv" | "try"
        | "typeof" | "unsized" | "virtual" | "yield" => true,
        _ => false,
    }
}

fn c_to_rust_type(c_type: &str) -> &str {
    match c_type {
        // Primitive mappings
        "uint32" => "u32",
        "uint64" => "u64",
        "bits32" => "bits32", // Alias
        "bool" => "bool",
        "int" => "i32",
        "long" => "i64",
        "int32" => "i32",
        "char*" => "Option<String>", // Make all strings optional
        "int16" => "i16",
        "char" => "char",
        "double" => "f64",

        // Vec
        "List*" => "Option<Vec<Node>>",

        // Box<Node>
        "Node*" => "Option<Box<Node>>",
        "Bitmapset*" => "Box<Node>",
        "CollateClause*" => "Box<Node>",
        "CreateStmt" => "Box<Node>",
        "FuncCall*" => "Box<Node>",
        "GrantStmt*" => "Box<Node>",
        "Index" => "Box<Node>",
        "InferClause*" => "Box<Node>",
        "ObjectWithArgs*" => "Box<Node>",
        "Oid" => "uuid::Uuid",
        "OnConflictClause*" => "Box<Node>",
        "PartitionSpec*" => "Box<Node>",
        "PartitionBoundSpec*" => "Box<Node>",
        "Query*" => "Box<Node>",
        "RoleSpec*" => "Box<Node>",
        "SelectStmt*" => "Box<Node>",
        "TableSampleClause*" => "Box<Node>",
        "TypeName*" => "Box<Node>",
        "Value*" => "Box<Node>",
        "VariableSetStmt*" => "Box<Node>",
        "WindowDef*" => "Box<Node>",
        "WithClause*" => "Box<Node>",

        // Other fields
        "A_Expr_Kind" => "A_Expr_Kind",
        "AclMode" => "AclMode",
        "AggSplit" => "AggSplit",
        "Alias*" => "Option<Alias>",
        "AlterSubscriptionType" => "AlterSubscriptionType",
        "AlterTableType" => "AlterTableType",
        "AlterTSConfigType" => "AlterTSConfigType",
        "AttrNumber" => "AttrNumber",
        "BoolExprType" => "BoolExprType",
        "BoolTestType" => "BoolTestType",
        "CmdType" => "CmdType",
        "CoercionContext" => "CoercionContext",
        "CoercionForm" => "CoercionForm",
        "ConstrType" => "ConstrType",
        "Cost" => "Cost",
        "CTEMaterialize" => "CTEMaterialize",
        "Datum" => "Datum",
        "DefElemAction" => "DefElemAction",
        "DiscardMode" => "DiscardMode",
        "DropBehavior" => "DropBehavior",
        "Expr" => "Expr",
        "Expr*" => "Option<Expr>",
        "FetchDirection" => "FetchDirection",
        "FromExpr*" => "Option<FromExpr>",
        "FuncExpr*" => "Option<FuncExpr>",
        "FunctionParameterMode" => "FunctionParameterMode",
        "GrantTargetType" => "GrantTargetType",
        "GroupingSetKind" => "GroupingSetKind",
        "ImportForeignSchemaType" => "ImportForeignSchemaType",
        "IntoClause*" => "Option<IntoClause>",
        "JoinType" => "JoinType",
        "LimitOption" => "LimitOption",
        "LockClauseStrength" => "LockClauseStrength",
        "LockWaitPolicy" => "LockWaitPolicy",
        "MinMaxOp" => "MinMaxOp",
        "NullTestType" => "NullTestType",
        "ObjectType" => "ObjectType",
        "OnCommitAction" => "OnCommitAction",
        "OnConflictAction" => "OnConflictAction",
        "OnConflictExpr*" => "Option<OnConflictExpr>",
        "OverridingKind" => "OverridingKind",
        "ParamKind" => "ParamKind",
        "PartitionRangeDatumKind" => "PartitionRangeDatumKind",
        "QuerySource" => "QuerySource",
        "RangeVar*" => "Option<RangeVar>",
        "ReindexObjectType" => "ReindexObjectType",
        "RoleSpecType" => "RoleSpecType",
        "RoleStmtType" => "RoleStmtType",
        "RowCompareType" => "RowCompareType",
        "RTEKind" => "RTEKind",
        "SetOperation" => "SetOperation",
        "SortByDir" => "SortByDir",
        "SortByNulls" => "SortByNulls",
        "SQLValueFunctionOp" => "SQLValueFunctionOp",
        "SubLinkType" => "SubLinkType",
        "SubTransactionId" => "SubTransactionId",
        "TableFunc*" => "Option<TableFunc>",
        "TransactionStmtKind" => "TransactionStmtKind",
        "Value" => "Value", // TODO: Implement this one
        "VariableSetKind" => "VariableSetKind",
        "ViewCheckOption" => "ViewCheckOption",
        "WCOKind" => "WCOKind",
        "XmlExprOp" => "XmlExprOp",
        "XmlOptionType" => "XmlOptionType",

        // This is a sanity check. We want this list to be exhaustive
        unexpected => panic!("Unexpected type: {}", unexpected),
    }
}
