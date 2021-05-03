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

pub struct Fingerprint {
    pub value: u64,
    pub hex: String,
}

pub fn parse(stmt: &str) -> Result<Vec<crate::ast::Node>> {
    unsafe {
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
        let parsed: ParseResult =
            serde_json::from_slice(raw.to_bytes()).map_err(|e| Error::InvalidAst(e.to_string()))?;
        pg_query_free_parse_result(result);
        Ok(parsed.stmts.into_iter().map(|s| s.stmt).collect())
    }
}

pub fn normalize(stmt: &str) -> Result<String> {
    unsafe {
        let c_str = CString::new(stmt).unwrap();
        let result = pg_query_normalize(c_str.as_ptr() as *const c_char);

        // Capture any errors first
        if !result.error.is_null() {
            let error = &*result.error;
            let message = CStr::from_ptr(error.message).to_string_lossy().into();
            pg_query_free_normalize_result(result);
            return Err(Error::ParseError(message));
        }

        // Parse the query back
        let raw = CStr::from_ptr(result.normalized_query);
        let owned = raw.to_string_lossy().to_string();
        pg_query_free_normalize_result(result);
        Ok(owned)
    }
}

pub fn fingerprint(stmt: &str) -> Result<Fingerprint> {
    unsafe {
        let c_str = CString::new(stmt).unwrap();
        let result = pg_query_fingerprint(c_str.as_ptr() as *const c_char);

        // Capture any errors first
        if !result.error.is_null() {
            let error = &*result.error;
            let message = CStr::from_ptr(error.message).to_string_lossy().into();
            pg_query_free_fingerprint_result(result);
            return Err(Error::ParseError(message));
        }

        // Parse the fingerprint
        let raw = CStr::from_ptr(result.fingerprint_str);
        let owned = Fingerprint {
            value: result.fingerprint,
            hex: raw.to_string_lossy().to_string(),
        };
        pg_query_free_fingerprint_result(result);
        Ok(owned)
    }
}

pub fn parse_plpgsql(stmt: &str) -> Result<serde_json::Value> {
    unsafe {
        let c_str = CString::new(stmt).unwrap();
        let result = pg_query_parse_plpgsql(c_str.as_ptr() as *const c_char);

        // Capture any errors first
        if !result.error.is_null() {
            let error = &*result.error;
            let message = CStr::from_ptr(error.message).to_string_lossy().into();
            pg_query_free_plpgsql_parse_result(result);
            return Err(Error::ParseError(message));
        }

        // Parse the pglpsql tree
        let raw = CStr::from_ptr(result.plpgsql_funcs);
        let owned = serde_json::from_str(&raw.to_string_lossy())
            .map_err(|e| Error::InvalidJson(e.to_string()))?;
        pg_query_free_plpgsql_parse_result(result);
        Ok(owned)
    }
}
