use std::ffi::{c_char, CString};
use brunch::Bench;
use pg_query;
use pg_query::bindings::{pg_query_parse, pg_query_parse_protobuf};

fn build_query(table_references: i32) -> String {
    let mut query = "SELECT * FROM t".to_string();
    for i in 0..table_references {
        query = format!("{query} JOIN t{i} ON t.id = t{i}.t_id AND t{i}.k IN (1, 2, 3, 4) AND t{i}.f IN (SELECT o FROM p WHERE q = 'foo')");
    }
    query
}

fn seed() -> String {
    build_query(100)
}

fn c_seed() -> CString {
    CString::new(seed()).unwrap()
}

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

    Bench::new("pg_query_parse_protobuf")
        .run_seeded_with(c_seed, |query| {
            unsafe { pg_query_parse_protobuf(query.as_ptr() as *const c_char) }
        }),

    /*
    Bench::new("pg_query_raw_parse")
        .run_seeded_with(seed, |query| {
        }),
    */

    // This was entirely for my own curiousity. -@duckinator
    /*
    Bench::new("pg_query::parse (uses parse_protobuf)")
        .run_seeded_with(seed, |query| {
            pg_query::parse(&query).unwrap()
        }),
    */
);
