#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

use fs_extra::dir::CopyOptions;
use glob::glob;
use std::env;
use std::path::{Path, PathBuf};

static SOURCE_DIRECTORY: &str = "libpg_query";
static LIBRARY_NAME: &str = "pg_query";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let build_path = Path::new(".").join(SOURCE_DIRECTORY);
    let out_header_path = out_dir.join(LIBRARY_NAME).with_extension("h");
    let out_raw_header_path = out_dir.join("pg_query_raw").with_extension("h");
    let out_protobuf_path = out_dir.join("protobuf");
    let target = env::var("TARGET").unwrap();

    // Configure cargo through stdout
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static={LIBRARY_NAME}");

    // Copy the relevant source files to the OUT_DIR
    let source_paths = vec![
        build_path.join(LIBRARY_NAME).with_extension("h"),
        build_path.join("pg_query_raw.h"),
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

    // Generate bindings for Rust (basic API)
    bindgen::Builder::default()
        .header(out_header_path.to_str().ok_or("Invalid header path")?)
        // Blocklist raw parse functions that are used via bindings_raw
        .blocklist_function("pg_query_parse_raw")
        .blocklist_function("pg_query_parse_raw_opts")
        .blocklist_function("pg_query_free_raw_parse_result")
        .blocklist_type("PgQueryRawParseResult")
        .generate()
        .map_err(|_| "Unable to generate bindings")?
        .write_to_file(out_dir.join("bindings.rs"))?;

    // Generate bindings for raw parse tree access (includes PostgreSQL internal types)
    let mut raw_builder = bindgen::Builder::default()
        .header(out_raw_header_path.to_str().ok_or("Invalid raw header path")?)
        .clang_arg(format!("-I{}", out_dir.display()))
        .clang_arg(format!("-I{}", out_dir.join("src/postgres/include").display()))
        .clang_arg(format!("-I{}", out_dir.join("src/include").display()));

    if target.contains("windows") {
        raw_builder = raw_builder.clang_arg(format!("-I{}", out_dir.join("src/postgres/include/port/win32").display()));
        if target.contains("msvc") {
            raw_builder = raw_builder.clang_arg(format!("-I{}", out_dir.join("src/postgres/include/port/win32_msvc").display()));
        }
    }

    raw_builder
        // Allowlist only the types we need for parse tree traversal
        .allowlist_type("List")
        .allowlist_type("ListCell")
        .allowlist_type("Node")
        .allowlist_type("NodeTag")
        .allowlist_type("RawStmt")
        .allowlist_type("SelectStmt")
        .allowlist_type("InsertStmt")
        .allowlist_type("UpdateStmt")
        .allowlist_type("DeleteStmt")
        .allowlist_type("MergeStmt")
        .allowlist_type("CreateStmt")
        .allowlist_type("AlterTableStmt")
        .allowlist_type("DropStmt")
        .allowlist_type("TruncateStmt")
        .allowlist_type("IndexStmt")
        .allowlist_type("ViewStmt")
        .allowlist_type("RangeVar")
        .allowlist_type("ColumnRef")
        .allowlist_type("ResTarget")
        .allowlist_type("A_Expr")
        .allowlist_type("FuncCall")
        .allowlist_type("TypeCast")
        .allowlist_type("TypeName")
        .allowlist_type("ColumnDef")
        .allowlist_type("Constraint")
        .allowlist_type("JoinExpr")
        .allowlist_type("SortBy")
        .allowlist_type("WindowDef")
        .allowlist_type("WithClause")
        .allowlist_type("CommonTableExpr")
        .allowlist_type("IntoClause")
        .allowlist_type("OnConflictClause")
        .allowlist_type("InferClause")
        .allowlist_type("Alias")
        .allowlist_type("A_Const")
        .allowlist_type("A_Star")
        .allowlist_type("A_Indices")
        .allowlist_type("A_Indirection")
        .allowlist_type("A_ArrayExpr")
        .allowlist_type("SubLink")
        .allowlist_type("BoolExpr")
        .allowlist_type("NullTest")
        .allowlist_type("BooleanTest")
        .allowlist_type("CaseExpr")
        .allowlist_type("CaseWhen")
        .allowlist_type("CoalesceExpr")
        .allowlist_type("MinMaxExpr")
        .allowlist_type("RowExpr")
        .allowlist_type("SetToDefault")
        .allowlist_type("MultiAssignRef")
        .allowlist_type("ParamRef")
        .allowlist_type("CollateClause")
        .allowlist_type("PartitionSpec")
        .allowlist_type("PartitionBoundSpec")
        .allowlist_type("PartitionRangeDatum")
        .allowlist_type("PartitionElem")
        .allowlist_type("CTESearchClause")
        .allowlist_type("CTECycleClause")
        .allowlist_type("RangeSubselect")
        .allowlist_type("RangeFunction")
        .allowlist_type("DefElem")
        .allowlist_type("IndexElem")
        .allowlist_type("SortGroupClause")
        .allowlist_type("GroupingSet")
        .allowlist_type("LockingClause")
        .allowlist_type("MergeWhenClause")
        .allowlist_type("TransactionStmt")
        .allowlist_type("VariableSetStmt")
        .allowlist_type("VariableShowStmt")
        .allowlist_type("ExplainStmt")
        .allowlist_type("CopyStmt")
        .allowlist_type("GrantStmt")
        .allowlist_type("RoleSpec")
        .allowlist_type("FunctionParameter")
        .allowlist_type("AlterTableCmd")
        .allowlist_type("AccessPriv")
        .allowlist_type("ObjectWithArgs")
        .allowlist_type("CreateFunctionStmt")
        .allowlist_type("CreateSchemaStmt")
        .allowlist_type("CreateSeqStmt")
        .allowlist_type("CreateTrigStmt")
        .allowlist_type("RuleStmt")
        .allowlist_type("CreateDomainStmt")
        .allowlist_type("CreateTableAsStmt")
        .allowlist_type("RefreshMatViewStmt")
        .allowlist_type("VacuumStmt")
        .allowlist_type("DoStmt")
        .allowlist_type("RenameStmt")
        .allowlist_type("NotifyStmt")
        .allowlist_type("ListenStmt")
        .allowlist_type("UnlistenStmt")
        .allowlist_type("PrepareStmt")
        .allowlist_type("ExecuteStmt")
        .allowlist_type("DeallocateStmt")
        .allowlist_type("FetchStmt")
        .allowlist_type("ClosePortalStmt")
        .allowlist_type("String")
        .allowlist_type("Integer")
        .allowlist_type("Float")
        .allowlist_type("Boolean")
        .allowlist_type("BitString")
        // Allowlist enums
        .allowlist_type("SetOperation")
        .allowlist_type("LimitOption")
        .allowlist_type("A_Expr_Kind")
        .allowlist_type("BoolExprType")
        .allowlist_type("SubLinkType")
        .allowlist_type("NullTestType")
        .allowlist_type("BoolTestType")
        .allowlist_type("MinMaxOp")
        .allowlist_type("JoinType")
        .allowlist_type("SortByDir")
        .allowlist_type("SortByNulls")
        .allowlist_type("CTEMaterialize")
        .allowlist_type("OnCommitAction")
        .allowlist_type("ObjectType")
        .allowlist_type("DropBehavior")
        .allowlist_type("OnConflictAction")
        .allowlist_type("GroupingSetKind")
        .allowlist_type("CmdType")
        .allowlist_type("TransactionStmtKind")
        .allowlist_type("ConstrType")
        .allowlist_type("DefElemAction")
        .allowlist_type("RoleSpecType")
        .allowlist_type("CoercionForm")
        .allowlist_type("VariableSetKind")
        .allowlist_type("LockClauseStrength")
        .allowlist_type("LockWaitPolicy")
        .allowlist_type("ViewCheckOption")
        .allowlist_type("DiscardMode")
        .allowlist_type("FetchDirection")
        .allowlist_type("FunctionParameterMode")
        .allowlist_type("AlterTableType")
        .allowlist_type("GrantTargetType")
        .allowlist_type("OverridingKind")
        .allowlist_type("PartitionStrategy")
        .allowlist_type("PartitionRangeDatumKind")
        // Allowlist raw parse functions
        .allowlist_function("pg_query_parse_raw")
        .allowlist_function("pg_query_parse_raw_opts")
        .allowlist_function("pg_query_free_raw_parse_result")
        .generate()
        .map_err(|_| "Unable to generate raw bindings")?
        .write_to_file(out_dir.join("bindings_raw.rs"))?;

    // Generate the protobuf definition
    prost_build::compile_protos(&[&out_protobuf_path.join(LIBRARY_NAME).with_extension("proto")], &[&out_protobuf_path])?;

    Ok(())
}
