mod helpers;
use helpers::*;

use std::ffi::c_char;
use brunch::Bench;
use pg_query;
use pg_query::bindings::{pg_query_parse, pg_query_parse_protobuf};


// pub fn pg_query_parse_protobuf(input: *const ::std::os::raw::c_char) -> PgQueryProtobufParseResult;
// pub fn pg_query_parse_protobuf_opts(input: *const ::std::os::raw::c_char, parser_options: ::std::os::raw::c_int) -> PgQueryProtobufParseResult;

// pub fn pg_query_parse(input: *const ::std::os::raw::c_char) -> PgQueryParseResult;
// pub fn pg_query_parse_opts(input: *const ::std::os::raw::c_char, parser_options: ::std::os::raw::c_int) -> PgQueryParseResult;

// pg_query_raw_parse ?

brunch::benches!(
    Bench::new("pg_query_parse")
        .run_seeded_with(c_seed, |query| {
            unsafe { pg_query_parse(query.as_ptr() as *const c_char) }
            //let result = pg_query::parse(&query);
            //assert!(result.is_ok());
        }),
);
