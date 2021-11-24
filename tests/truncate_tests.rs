#![allow(non_snake_case)]
#![cfg(test)]

use pg_query::parse;

#[test]
fn it_omits_target_list() {
    let query = "SELECT a, b, c, d, e, f FROM xyz WHERE a = b";
    let result = parse(query).unwrap();
    assert_eq!(result.truncate(40).unwrap(), "SELECT ... FROM xyz WHERE a = b")
}

#[test]
fn it_omits_CTE_definition() {
    let query = "WITH x AS (SELECT * FROM y) SELECT * FROM x";
    let result = parse(query).unwrap();
    assert_eq!(result.truncate(40).unwrap(), "WITH x AS (...) SELECT * FROM x")
}

#[test]
fn it_omits_WHERE_clause() {
    let query = "SELECT * FROM z WHERE a = b AND x = y";
    let result = parse(query).unwrap();
    assert_eq!(result.truncate(30).unwrap(), "SELECT * FROM z WHERE ...")
}

// attempt to subtract with overflow
#[test]
fn it_omits_INSERT_field_list() {
    let query = "INSERT INTO \"x\" (a, b, c, d, e, f) VALUES (?)";
    let result = parse(query).unwrap();
    assert_eq!(result.truncate(32).unwrap(), "INSERT INTO x (...) VALUES (?)")
}

#[test]
fn it_omits_comments() {
    let query = "SELECT $1 /* application:test */";
    let result = parse(query).unwrap();
    assert_eq!(result.truncate(100).unwrap(), "SELECT $1")
}

#[test]
fn it_falls_back_to_simple_truncation() {
    let query = "SELECT * FROM t";
    let result = parse(query).unwrap();
    assert_eq!(result.truncate(10).unwrap(), "SELECT ...")
}

#[test]
fn it_handles_problematic_cases() {
    let query = "SELECT CASE WHEN $2.typtype = ? THEN $2.typtypmod ELSE $1.atttypmod END";
    let result = parse(query).unwrap();
    assert_eq!(result.truncate(50).unwrap(), "SELECT ...")
}

// FailedAssertion("res_target->name != NULL", File: "src/pg_query_deparse.c", Line: 1587)
// #[test]
// fn it_omits_UPDATE_target_list() {
//     let query = "UPDATE x SET a = 1, c = 2, e = 'str'";
//     let result = parse(query).unwrap();
//     assert_eq!(result.truncate(30).unwrap(), "UPDATE x SET ... = ...")
// }

// attempt to subtract with overflow
// #[test]
// fn it_omits_ON_CONFLICT_target_list() {
//     let query = "INSERT INTO y(a) VALUES(1) ON CONFLICT DO UPDATE SET a = 123456789";
//     let result = parse(query).unwrap();
//     assert_eq!(result.truncate(65).unwrap(), "INSERT INTO y (a) VALUES (1) ON CONFLICT DO UPDATE SET ... = ...")
// }

#[test]
fn it_handles_GRANT() {
    let query = "GRANT SELECT (abc, def, ghj) ON TABLE t1 TO r1";
    let result = parse(query).unwrap();
    assert_eq!(result.truncate(35).unwrap(), "GRANT select (abc, def, ghj) ON ...")
}
