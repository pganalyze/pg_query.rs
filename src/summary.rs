use std::ffi::{CStr, CString};

use prost::Message;

use crate::bindings::*;
use crate::error::*;
use crate::protobuf;
use crate::summary_result::SummaryResult;

/// Parses the given SQL statement and provides a summary of it.
///
/// # Example
///
/// ```rust
/// use pg_query::{Node, NodeEnum, NodeRef};
///
/// let result = pg_query::summary("SELECT * FROM contacts", 0, -1);
/// assert!(result.is_ok());
/// let result = result.unwrap();
/// assert_eq!(result.tables(), vec!["contacts"]);
/// ```
pub fn summary(statement: &str, parser_options: i32, truncate_limit: i32) -> Result<SummaryResult> {
    let input = CString::new(statement)?;
    let result = unsafe { pg_query_summary(input.as_ptr(), parser_options, truncate_limit) };
    let parse_result = if !result.error.is_null() {
        let message = unsafe { CStr::from_ptr((*result.error).message) }.to_string_lossy().to_string();
        Err(Error::Parse(message))
    } else {
        let data = unsafe { std::slice::from_raw_parts(result.summary.data as *const u8, result.summary.len as usize) };
        let stderr = unsafe { CStr::from_ptr(result.stderr_buffer) }.to_string_lossy().to_string();
        protobuf::SummaryResult::decode(data).map_err(Error::Decode).map(|result| SummaryResult::new(result, stderr))
    };
    unsafe { pg_query_free_summary_parse_result(result) };
    parse_result
}
