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
    stmt_len: Option<u32>,
}

/// Represents the resulting fingerprint containing both the raw integer form as well as the
/// corresponding 16 character hex value.
pub struct Fingerprint {
    pub value: u64,
    pub hex: String,
}

/// Parses the given SQL statement into the given abstract syntax tree.
///
/// # Example
///
/// ```rust
/// use pg_query::ast::Node;
///
/// let result = pg_query::parse("SELECT * FROM contacts");
/// assert!(result.is_ok());
/// let result = result.unwrap();
/// let el: &Node = &result[0];
/// assert!(matches!(*el, Node::SelectStmt(_)));
/// ```
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

/// Normalizes the given SQL statement, returning a parametized version.
///
/// # Example
///
/// ```rust
/// let result = pg_query::normalize("SELECT * FROM contacts WHERE name='Paul'");
/// assert!(result.is_ok());
/// let result = result.unwrap();
/// assert_eq!(result, "SELECT * FROM contacts WHERE name=$1");
/// ```
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

/// Fingerprints the given SQL statement. Useful for comparing parse trees across different implementations
/// of `libpg_query`.
///
/// # Example
///
/// ```rust
/// let result = pg_query::fingerprint("SELECT * FROM contacts WHERE name='Paul'");
/// assert!(result.is_ok());
/// let result = result.unwrap();
/// assert_eq!(result.hex, "0e2581a461ece536");
/// ```
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

/// An experimental API which parses a PLPGSQL function. This currently returns the raw JSON structure.
///
/// # Example
///
/// ```rust
/// let result = pg_query::parse_plpgsql(
///         " \
///         CREATE OR REPLACE FUNCTION cs_fmt_browser_version(v_name varchar, v_version varchar) \
///         RETURNS varchar AS $$ \
///         BEGIN \
///             IF v_version IS NULL THEN \
///                 RETURN v_name; \
///             END IF; \
///             RETURN v_name || '/' || v_version; \
///         END; \
///         $$ LANGUAGE plpgsql;",
///     );
/// assert!(result.is_ok());
/// ```
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
