#![allow(non_snake_case)]
#![cfg(test)]

#[cfg(test)]
use itertools::sorted;

use pg_query::parse;

#[test]
fn it_finds_unqualified_names() {
    let result = parse("SELECT * FROM x WHERE y = $1 AND z = 1").unwrap();
    let filter_columns: Vec<(Option<String>, String)> = sorted(result.filter_columns).collect();
    assert_eq!(filter_columns, [(None, "y".into()), (None, "z".into())]);
}

#[test]
fn it_finds_qualified_names() {
    let result = parse("SELECT * FROM x WHERE x.y = $1 AND x.z = 1").unwrap();
    let filter_columns: Vec<(Option<String>, String)> = sorted(result.filter_columns).collect();
    assert_eq!(filter_columns, [(Some("x".into()), "y".into()), (Some("x".into()), "z".into())]);
}

#[test]
fn it_traverses_into_ctes() {
    let result = parse("WITH a AS (SELECT * FROM x WHERE x.y = $1 AND x.z = 1) SELECT * FROM a WHERE b = 5").unwrap();
    let filter_columns: Vec<(Option<String>, String)> = sorted(result.filter_columns).collect();
    assert_eq!(filter_columns, [(None, "b".into()), (Some("x".into()), "y".into()), (Some("x".into()), "z".into())]);
}

#[test]
fn it_recognizes_boolean_tests() {
    let result = parse("SELECT * FROM x WHERE x.y IS TRUE AND x.z IS NOT FALSE").unwrap();
    let filter_columns: Vec<(Option<String>, String)> = sorted(result.filter_columns).collect();
    assert_eq!(filter_columns, [(Some("x".into()), "y".into()), (Some("x".into()), "z".into())]);
}

#[test]
fn it_recognizes_null_tests() {
    let result = parse("SELECT * FROM x WHERE x.y IS NULL AND x.z IS NOT NULL").unwrap();
    let filter_columns: Vec<(Option<String>, String)> = sorted(result.filter_columns).collect();
    assert_eq!(filter_columns, [(Some("x".into()), "y".into()), (Some("x".into()), "z".into())]);
}

#[test]
fn it_finds_coalesce_argument_names() {
    let result = parse("SELECT * FROM x WHERE x.y = COALESCE(z.a, z.b)").unwrap();
    let filter_columns: Vec<(Option<String>, String)> = sorted(result.filter_columns).collect();
    assert_eq!(filter_columns, [(Some("x".into()), "y".into()), (Some("z".into()), "a".into()), (Some("z".into()), "b".into())]);
}

#[test]
fn it_finds_unqualified_names_in_union_query() {
    let result = parse("SELECT * FROM x where y = $1 UNION SELECT * FROM x where z = $2").unwrap();
    let filter_columns: Vec<(Option<String>, String)> = sorted(result.filter_columns).collect();
    assert_eq!(filter_columns, [(None, "y".into()), (None, "z".into())]);
}

#[test]
fn it_finds_unqualified_names_in_union_all_query() {
    let result = parse("SELECT * FROM x where y = $1 UNION ALL SELECT * FROM x where z = $2").unwrap();
    let filter_columns: Vec<(Option<String>, String)> = sorted(result.filter_columns).collect();
    assert_eq!(filter_columns, [(None, "y".into()), (None, "z".into())]);
}

#[test]
fn it_finds_unqualified_names_in_except_query() {
    let result = parse("SELECT * FROM x where y = $1 EXCEPT SELECT * FROM x where z = $2").unwrap();
    let filter_columns: Vec<(Option<String>, String)> = sorted(result.filter_columns).collect();
    assert_eq!(filter_columns, [(None, "y".into()), (None, "z".into())]);
}

#[test]
fn it_finds_unqualified_names_in_except_all_query() {
    let result = parse("SELECT * FROM x where y = $1 EXCEPT ALL SELECT * FROM x where z = $2").unwrap();
    let filter_columns: Vec<(Option<String>, String)> = sorted(result.filter_columns).collect();
    assert_eq!(filter_columns, [(None, "y".into()), (None, "z".into())]);
}

#[test]
fn it_finds_unqualified_names_in_intersect_query() {
    let result = parse("SELECT * FROM x where y = $1 INTERSECT SELECT * FROM x where z = $2").unwrap();
    let filter_columns: Vec<(Option<String>, String)> = sorted(result.filter_columns).collect();
    assert_eq!(filter_columns, [(None, "y".into()), (None, "z".into())]);
}

#[test]
fn it_finds_unqualified_names_in_intersect_all_query() {
    let result = parse("SELECT * FROM x where y = $1 INTERSECT ALL SELECT * FROM x where z = $2").unwrap();
    let filter_columns: Vec<(Option<String>, String)> = sorted(result.filter_columns).collect();
    assert_eq!(filter_columns, [(None, "y".into()), (None, "z".into())]);
}

#[test]
fn it_ignores_target_list_columns() {
    let result = parse("SELECT a, y, z FROM x WHERE x.y = $1 AND x.z = 1").unwrap();
    let filter_columns: Vec<(Option<String>, String)> = sorted(result.filter_columns).collect();
    assert_eq!(filter_columns, [(Some("x".into()), "y".into()), (Some("x".into()), "z".into())]);
}

#[test]
fn it_ignores_order_by_columns() {
    let result = parse("SELECT * FROM x WHERE x.y = $1 AND x.z = 1 ORDER BY a, b").unwrap();
    let filter_columns: Vec<(Option<String>, String)> = sorted(result.filter_columns).collect();
    assert_eq!(filter_columns, [(Some("x".into()), "y".into()), (Some("x".into()), "z".into())]);
}
