use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::bindings::*;
use crate::error::*;

pub fn parse(stmt: &str) -> Result<()> {
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
        println!("{:?}", CStr::from_ptr(result.parse_tree));
        pg_query_free_parse_result(result);
        Ok(())
    }
}
/*
pub fn normalize(stmt: &str) {}

pub fn parse_plpgsql(stmt: &str) {}

pub fn fingerprint(stmt: &str) {}
*/
