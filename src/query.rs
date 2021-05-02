use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::bindings::*;
use crate::error::*;

#[derive(Debug, serde::Deserialize)]
struct ParseResult {
    version: u32,
    stmts: Vec<Stmt>,
}

#[derive(Debug, serde::Deserialize)]
struct Stmt {
    stmt: crate::ast::Node,
    stmt_len: u32,
}

pub fn parse(stmt: &str) -> Result<Vec<crate::ast::Node>> {
    unsafe {
        // Execute the query
        let c_str = CString::new(stmt).unwrap();
        let result = pg_query_parse(c_str.as_ptr() as *const c_char);

        // Capture any errors first
        if !result.error.is_null() {
            let error = &*result.error;
            let message = CStr::from_ptr(error.message).to_string_lossy().into();
            pg_query_free_parse_result(result);
            return Err(Error::ParseError(message));
        }

        // Parse the JSON into the AST
        let raw = CStr::from_ptr(result.parse_tree);
        println!("{:?}", raw);
        let parsed: ParseResult =
            serde_json::from_slice(raw.to_bytes()).map_err(|e| Error::InvalidAst(e.to_string()))?;
        pg_query_free_parse_result(result);
        Ok(parsed.stmts.into_iter().map(|s| s.stmt).collect())
    }
}
/*
pub fn normalize(stmt: &str) {}

pub fn parse_plpgsql(stmt: &str) {}

pub fn fingerprint(stmt: &str) {}
*/
