#![allow(non_snake_case)]
#![cfg(test)]

use pg_query::{parse, DeparseOptions};

#[cfg(test)]
use regex::Regex;

#[test]
fn it_deparses_with_pretty_print_options() {
    let result = parse("SELECT a, b FROM t WHERE x = 1", 0).unwrap();
    let out = pg_query::deparse(&result.protobuf, DeparseOptions { pretty_print: true, trailing_newline: true, ..Default::default() }).unwrap();
    assert_eq!(
        out,
        "SELECT a, b
FROM t
WHERE x = 1
"
    );

    let long = "SELECT column_one, column_two, column_three, column_four, column_five FROM my_table";
    let result = parse(long, 0).unwrap();
    let out = pg_query::deparse(
        &result.protobuf,
        DeparseOptions { pretty_print: true, indent_size: 2, max_line_length: 20, commas_start_of_line: true, ..Default::default() },
    )
    .unwrap();
    assert_eq!(
        out,
        "SELECT
  column_one
  , column_two
  , column_three
  , column_four
  , column_five
FROM my_table"
    );
}

#[test]
fn it_parses_pg18_returning_aliases() {
    // Postgres 18 RETURNING OLD/NEW aliases (new in PG18)
    let result = parse("INSERT INTO t (a) VALUES (1) RETURNING OLD AS old_row, NEW AS new_row", 0);
    assert!(result.is_ok(), "PG18 RETURNING alias syntax should parse: {:?}", result.err());
}

#[test]
fn it_roundtrips_comments() {
    let query = "SELECT 1 -- cast to string\n";
    let comments = pg_query::deparse_comments_for_query(query).unwrap();
    assert_eq!(comments.len(), 1);
    assert_eq!(comments[0].text, "-- cast to string");

    let parsed = parse(query, 0).unwrap();
    let out = pg_query::deparse(&parsed.protobuf, DeparseOptions { comments, ..Default::default() }).unwrap();
    assert!(out.contains("-- cast to string"), "actual: {}", out);
}

fn assert_deparse(input: &str, output: &str) {
    let result = parse(input, 0).unwrap();
    assert_eq!(result.deparse(Default::default()).unwrap(), output);
}

fn oneline(query: &str) -> String {
    let query = Regex::new(r"\s+").unwrap().replace_all(query, " ");
    let query = Regex::new(r"\( ").unwrap().replace_all(&query, "(");
    let query = Regex::new(r" \)").unwrap().replace_all(&query, ")");
    query.trim().trim_end_matches(';').to_string()
}

#[test]
fn it_deparses_SELECT() {
    let query = "SELECT a AS b FROM x WHERE y = 5 AND z = y";
    assert_deparse(query, query);
}

#[test]
fn it_deparses_SELECT_with_empty_target_list() {
    let query = "SELECT FROM x WHERE y = 5 AND z = y";
    assert_deparse(query, query);
}

#[test]
fn it_deparses_SELECT_with_schema() {
    let query = "SELECT a AS b FROM public.x WHERE y = 5 AND z = y";
    assert_deparse(query, query);
}

#[test]
fn it_deparses_SELECT_with_DISTINCT() {
    let query = "SELECT DISTINCT a, b, * FROM c WHERE d = e";
    assert_deparse(query, query);
}

#[test]
fn it_deparses_SELECT_with_DISTINCT_ON() {
    let query = "SELECT DISTINCT ON (a) a, b FROM c";
    assert_deparse(query, query);
}

#[test]
fn it_deparses_SELECT_with_INTO() {
    let query = "SELECT * INTO films_recent FROM films WHERE date_prod >= '2002-01-01'";
    assert_deparse(query, query);
}

#[test]
fn it_deparses_SELECT_value_function() {
    let query = "SELECT current_timestamp";
    assert_deparse(query, query);
}

#[test]
fn it_deparses_SELECT_value_function_with_precision() {
    let query = "SELECT current_time(2)";
    assert_deparse(query, query);
}

#[test]
fn it_deparses_SELECT_complex() {
    let query = "SELECT memory_total_bytes, memory_swap_total_bytes - memory_swap_free_bytes AS swap, date_part($1, s.collected_at) AS collected_at FROM snapshots s JOIN system_snapshots ON snapshot_id = s.id WHERE s.database_id = $2 AND s.collected_at >= $3 AND s.collected_at <= $4 ORDER BY collected_at ASC";
    assert_deparse(query, query);
}

#[test]
fn it_deparses_SELECT_ORDER_BY_with_NULLS_FIRST() {
    let query = "SELECT * FROM a ORDER BY x ASC NULLS FIRST";
    assert_deparse(query, query);
}

#[test]
fn it_deparses_SELECT_ORDER_BY_with_NULLS_LAST() {
    let query = "SELECT * FROM a ORDER BY x ASC NULLS LAST";
    assert_deparse(query, query);
}

#[test]
fn it_deparses_SELECT_ORDER_BY_with_COLLATE() {
    let query = r#"SELECT * FROM a ORDER BY x COLLATE "tr_TR" DESC NULLS LAST"#;
    assert_deparse(query, query);
}

#[test]
fn it_deparses_SELECT_text_with_COLLATE() {
    let query = r#"SELECT 'foo' COLLATE "tr_TR""#;
    assert_deparse(query, query);
}

#[test]
fn it_deparses_SELECT_UNION_or_UNION_ALL() {
    let query = "WITH kodsis AS (SELECT * FROM application), kodsis2 AS (SELECT * FROM application) SELECT * FROM kodsis UNION SELECT * FROM kodsis ORDER BY id DESC";
    assert_deparse(query, query);
}

#[test]
fn it_deparses_SELECT_UNION_with_ORDER() {
    let query = "SELECT id, name FROM table1 UNION (SELECT id, name FROM table2 ORDER BY name) ORDER BY id ASC";
    assert_deparse(query, query);
}

#[test]
fn it_deparses_SELECT_EXCEPT() {
    let query = "SELECT a FROM kodsis EXCEPT SELECT a FROM application";
    assert_deparse(query, query);
}

#[test]
fn it_deparses_SELECT_INTERSECT() {
    let query = "SELECT 'a' INTERSECT SELECT 'b'";
    assert_deparse(query, query);
}

#[test]
fn it_deparses_SELECT_with_specific_column_alias() {
    let query = "SELECT * FROM (VALUES ('anne', 'smith'), ('bob', 'jones'), ('joe', 'blow')) names(first, last)";
    assert_deparse(query, query);
}

#[test]
fn it_deparses_SELECT_with_LIKE_filter() {
    let query = "SELECT * FROM users WHERE name LIKE 'postgresql:%';";
    assert_deparse(query, &oneline(query));
}

#[test]
fn it_deparses_SELECT_with_NOT_LIKE_filter() {
    let query = "SELECT * FROM users WHERE name NOT LIKE 'postgresql:%';";
    assert_deparse(query, &oneline(query));
}

#[test]
fn it_deparses_SELECT_with_ILIKE_filter() {
    let query = "SELECT * FROM users WHERE name ILIKE 'postgresql:%';";
    assert_deparse(query, &oneline(query));
}

#[test]
fn it_deparses_SELECT_with_NOT_ILIKE_filter() {
    let query = "SELECT * FROM users WHERE name NOT ILIKE 'postgresql:%';";
    assert_deparse(query, &oneline(query));
}

#[test]
fn it_deparses_SELECT_simple_WITH() {
    let query = "WITH t AS (SELECT random() AS x FROM generate_series(1, 3)) SELECT * FROM t";
    assert_deparse(query, query);
}

#[test]
fn it_deparses_SELECT_complex_WITH() {
    // Taken from http://www.postgresql.org/docs/9.1/static/queries-with.html
    let query = "
        WITH RECURSIVE search_graph(id, link, data, depth, path, cycle) AS (
          SELECT g.id, g.link, g.data, 1,
            ARRAY[ROW(g.f1, g.f2)],
            false
          FROM graph g
        UNION ALL
          SELECT g.id, g.link, g.data, sg.depth + 1,
            path || ROW(g.f1, g.f2),
            ROW(g.f1, g.f2) = ANY(path)
          FROM graph g, search_graph sg
          WHERE g.id = sg.link AND NOT cycle
        )
        SELECT id, data, link FROM search_graph;
    ";
    assert_deparse(query, &oneline(query));
}

#[test]
fn it_deparses_SELECT_OVERLAY() {
    let query = "SELECT OVERLAY(m.name PLACING \'******\' FROM 3 FOR 6) AS tc_kimlik FROM tb_test m";
    assert_deparse(query, query);
}

#[test]
fn it_deparses_SELECT_SUM() {
    let query = "SELECT sum(price_cents) FROM products";
    assert_deparse(query, query);
}

#[test]
fn it_deparses_SELECT_ARRAY() {
    let query = "SELECT ARRAY(SELECT id FROM products)::bigint[]";
    assert_deparse(query, query);
}

#[test]
fn it_deparses_SELECT_LATERAL() {
    let query = "SELECT m.name AS mname, pname FROM manufacturers m, LATERAL get_product_names(m.id) pname";
    assert_deparse(query, query);
}

#[test]
fn it_deparses_SELECT_LATERAL_JOIN() {
    let query = "
        SELECT m.name AS mname, pname
        FROM manufacturers m LEFT JOIN LATERAL get_product_names(m.id) pname ON true
    ";
    assert_deparse(query, &oneline(query));
}

#[test]
fn it_deparses_SELECT_LATERAL_JOIN_with_nested_query() {
    let query = "
        SELECT *
        FROM tb_test_main mh
        JOIN LATERAL (
            SELECT ftnrm.* FROM test ftnrm WHERE ftnrm.hizmet_id = mh.id
            UNION ALL
            SELECT ftarc.* FROM test.test2 ftarc WHERE ftarc.hizmet_id = mh.id
        ) ft ON true
    ";
    assert_deparse(query, &oneline(query));
}

#[test]
fn it_deparses_SELECT_CROSS_JOIN() {
    let query = "SELECT x, y FROM a CROSS JOIN b";
    assert_deparse(query, query);
}

#[test]
fn it_deparses_SELECT_NATURAL_JOIN() {
    let query = "SELECT x, y FROM a NATURAL JOIN b";
    assert_deparse(query, query);
}

#[test]
fn it_deparses_SELECT_LEFT_JOIN() {
    let query = "SELECT x, y FROM a LEFT JOIN b ON 1 > 0";
    assert_deparse(query, query);
}

#[test]
fn it_deparses_SELECT_RIGHT_JOIN() {
    let query = "SELECT x, y FROM a RIGHT JOIN b ON 1 > 0";
    assert_deparse(query, query);
}

#[test]
fn it_deparses_SELECT_FULL_JOIN() {
    let query = "SELECT x, y FROM a FULL JOIN b ON 1 > 0";
    assert_deparse(query, query);
}

#[test]
fn it_deparses_SELECT_JOIN_with_USING() {
    let query = "SELECT x, y FROM a JOIN b USING (z)";
    assert_deparse(query, query);
}

// There are many more Ruby tests but we probably don't need to implement
// them here since we're just passing the protobuf back to C
// https://github.com/pganalyze/pg_query/tree/main/spec/lib/pg_query/deparse_spec.rb
