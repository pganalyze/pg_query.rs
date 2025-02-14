mod helpers;
use helpers::*;

use std::ffi::c_char;
use brunch::Bench;
use pg_query;
use pg_query::bindings::{pg_query_parse, pg_query_parse_protobuf};


brunch::benches!(
    Bench::new("pg_query_parse_protobuf")
        .run_seeded_with(c_seed, |query| {
            unsafe { pg_query_parse_protobuf(query.as_ptr() as *const c_char) }
        }),
);
