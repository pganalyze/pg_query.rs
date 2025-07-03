#![allow(non_snake_case)]
#![cfg(test)]

#[cfg(test)]
use itertools::sorted;

use pg_query::summary;
use pg_query::FilterColumn;

#[test]
fn it_finds_unqualified_names() {
    let result = summary("SELECT * FROM x WHERE y = $1 AND z = 1", 0, -1).unwrap();
    let filter_columns: Vec<FilterColumn> = sorted(result.filter_columns).collect();
    assert_eq!(filter_columns, vec![
        FilterColumn { schema: None, table: None, column: "y".to_string() },
        FilterColumn { schema: None, table: None, column: "z".to_string() },
    ]);
}

#[test]
fn it_finds_qualified_names() {
    let result = summary("SELECT * FROM x WHERE x.y = $1 AND x.z = 1", 0, -1).unwrap();
    let filter_columns: Vec<FilterColumn> = sorted(result.filter_columns).collect();
    assert_eq!(filter_columns, [
        FilterColumn { schema: None, table: Some("x".into()), column: "y".into() },
        FilterColumn { schema: None, table: Some("x".into()), column: "z".into() },
    ]);
}

#[test]
fn it_traverses_into_ctes() {
    let result = summary("WITH a AS (SELECT * FROM x WHERE x.y = $1 AND x.z = 1) SELECT * FROM a WHERE b = 5", 0, -1).unwrap();
    let filter_columns: Vec<FilterColumn> = sorted(result.filter_columns).collect();
    assert_eq!(filter_columns, [
        FilterColumn { schema: None, table: None, column: "b".into() },
        FilterColumn { schema: None, table: Some("x".into()), column: "y".into() },
        FilterColumn { schema: None, table: Some("x".into()), column: "z".into() },
    ]);
}

#[test]
fn it_recognizes_boolean_tests() {
    let result = summary("SELECT * FROM x WHERE x.y IS TRUE AND x.z IS NOT FALSE", 0, -1).unwrap();
    let filter_columns: Vec<FilterColumn> = sorted(result.filter_columns).collect();
    assert_eq!(filter_columns, [
        FilterColumn { schema: None, table: Some("x".into()), column: "y".into() },
        FilterColumn { schema: None, table: Some("x".into()), column: "z".into() },
    ]);
}

#[test]
fn it_recognizes_null_tests() {
    let result = summary("SELECT * FROM x WHERE x.y IS NULL AND x.z IS NOT NULL", 0, -1).unwrap();
    let filter_columns: Vec<FilterColumn> = sorted(result.filter_columns).collect();
    assert_eq!(filter_columns, [
        FilterColumn { schema: None, table: Some("x".into()), column: "y".into() },
        FilterColumn { schema: None, table: Some("x".into()), column: "z".into() },
    ]);
}

#[test]
fn it_finds_coalesce_argument_names() {
    let result = summary("SELECT * FROM x WHERE x.y = COALESCE(z.a, z.b)", 0, -1).unwrap();
    let filter_columns: Vec<FilterColumn> = sorted(result.filter_columns).collect();
    assert_eq!(filter_columns, [
        FilterColumn { schema: None, table: Some("x".into()), column: "y".into() },
        FilterColumn { schema: None, table: Some("z".into()), column: "a".into() },
        FilterColumn { schema: None, table: Some("z".into()), column: "b".into() },
    ]);
}

#[test]
fn it_finds_unqualified_names_in_union_query() {
    let result = summary("SELECT * FROM x where y = $1 UNION SELECT * FROM x where z = $2", 0, -1).unwrap();
    let filter_columns: Vec<FilterColumn> = sorted(result.filter_columns).collect();
    assert_eq!(filter_columns, [
        FilterColumn { schema: None, table: None, column: "y".into() },
        FilterColumn { schema: None, table: None, column: "z".into() },
    ]);
}

#[test]
fn it_finds_unqualified_names_in_union_all_query() {
    let result = summary("SELECT * FROM x where y = $1 UNION ALL SELECT * FROM x where z = $2", 0, -1).unwrap();
    let filter_columns: Vec<FilterColumn> = sorted(result.filter_columns).collect();
    assert_eq!(filter_columns, [
        FilterColumn { schema: None, table: None, column: "y".into() },
        FilterColumn { schema: None, table: None, column: "z".into() },
    ]);
}

#[test]
fn it_finds_unqualified_names_in_except_query() {
    let result = summary("SELECT * FROM x where y = $1 EXCEPT SELECT * FROM x where z = $2", 0, -1).unwrap();
    let filter_columns: Vec<FilterColumn> = sorted(result.filter_columns).collect();
    assert_eq!(filter_columns, [
        FilterColumn { schema: None, table: None, column: "y".into() },
        FilterColumn { schema: None, table: None, column: "z".into() },
    ]);
}

#[test]
fn it_finds_unqualified_names_in_except_all_query() {
    let result = summary("SELECT * FROM x where y = $1 EXCEPT ALL SELECT * FROM x where z = $2", 0, -1).unwrap();
    let filter_columns: Vec<FilterColumn> = sorted(result.filter_columns).collect();
    assert_eq!(filter_columns, [
        FilterColumn { schema: None, table: None, column: "y".into() },
        FilterColumn { schema: None, table: None, column: "z".into() },
    ]);
}

#[test]
fn it_finds_unqualified_names_in_intersect_query() {
    let result = summary("SELECT * FROM x where y = $1 INTERSECT SELECT * FROM x where z = $2", 0, -1).unwrap();
    let filter_columns: Vec<FilterColumn> = sorted(result.filter_columns).collect();
    assert_eq!(filter_columns, [
        FilterColumn { schema: None, table: None, column: "y".into() },
        FilterColumn { schema: None, table: None, column: "z".into() },
    ]);
}

#[test]
fn it_finds_unqualified_names_in_intersect_all_query() {
    let result = summary("SELECT * FROM x where y = $1 INTERSECT ALL SELECT * FROM x where z = $2", 0, -1).unwrap();
    let filter_columns: Vec<FilterColumn> = sorted(result.filter_columns).collect();
    assert_eq!(filter_columns, [
        FilterColumn { schema: None, table: None, column: "y".into() },
        FilterColumn { schema: None, table: None, column: "z".into() },
    ]);
}

#[test]
fn it_ignores_target_list_columns() {
    let result = summary("SELECT a, y, z FROM x WHERE x.y = $1 AND x.z = 1", 0, -1).unwrap();
    let filter_columns: Vec<FilterColumn> = sorted(result.filter_columns).collect();
    assert_eq!(filter_columns, [
        FilterColumn { schema: None, table: Some("x".into()), column: "y".into() },
        FilterColumn { schema: None, table: Some("x".into()), column: "z".into() },
    ]);
}

#[test]
fn it_ignores_order_by_columns() {
    let result = summary("SELECT * FROM x WHERE x.y = $1 AND x.z = 1 ORDER BY a, b", 0, -1).unwrap();
    let filter_columns: Vec<FilterColumn> = sorted(result.filter_columns).collect();
    assert_eq!(filter_columns, [
        FilterColumn { schema: None, table: Some("x".into()), column: "y".into() },
        FilterColumn { schema: None, table: Some("x".into()), column: "z".into() },
    ]);
}
