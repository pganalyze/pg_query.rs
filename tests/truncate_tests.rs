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

#[test]
fn it_omits_INSERT_field_list() {
    let query = "INSERT INTO \"x\" (a, b, c, d, e, f) VALUES ($1)";
    let result = parse(query).unwrap();
    assert_eq!(result.truncate(32).unwrap(), "INSERT INTO x (...) VALUES (...)")
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
    let query = "SELECT CASE WHEN $2.typtype = $1 THEN $2.typtypmod ELSE $1.atttypmod END";
    let result = parse(query).unwrap();
    assert_eq!(result.truncate(50).unwrap(), "SELECT ...")
}

#[test]
fn it_omits_UPDATE_target_list() {
    let query = "UPDATE x SET a = 1, c = 2, e = 'str'";
    let result = parse(query).unwrap();
    assert_eq!(result.truncate(30).unwrap(), "UPDATE x SET ... = ...")
}

#[test]
fn it_omits_ON_CONFLICT_target_list() {
    let query = "INSERT INTO y(a) VALUES(1) ON CONFLICT DO UPDATE SET a = 123456789";
    let result = parse(query).unwrap();
    assert_eq!(result.truncate(65).unwrap(), "INSERT INTO y (a) VALUES (1) ON CONFLICT DO UPDATE SET ... = ...")
}

#[test]
fn it_omits_ON_CONFLICT_target_list_2() {
    let query = r#"
        INSERT INTO foo (a, b, c, d) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19,$20,$21,$22,$23,$24,$25,$26,$27,$28,$29)
        ON CONFLICT (id)
        DO UPDATE SET (a, b, c, d) = (excluded.a,excluded.b,excluded.c,case when foo.d = excluded.d then excluded.d end)
    "#;
    let result = parse(query).unwrap();
    let truncated = result.truncate(100).unwrap();
    assert_eq!(truncated, "INSERT INTO foo (a, b, c, d) VALUES (...) ON CONFLICT (id) DO UPDATE SET ... = ...");
}

#[test]
fn it_handles_GRANT() {
    let query = "GRANT SELECT (abc, def, ghj) ON TABLE t1 TO r1";
    let result = parse(query).unwrap();
    assert_eq!(result.truncate(35).unwrap(), "GRANT select (abc, def, ghj) ON ...")
}

#[test]
fn it_does_not_segfault_on_target_list_from_CTE_already_removed_from_possible_truncations() {
    let query = r#"
        WITH activity AS (
            SELECT pid, COALESCE(a.usename, '') AS usename
            FROM pganalyze.get_stat_activity() a
        )
        SELECT
        FROM pganalyze.get_stat_progress_vacuum() v
        JOIN activity a USING (pid)
    "#;
    let result = parse(query).unwrap();
    let truncated = result.truncate(100).unwrap();
    assert_eq!(truncated, "WITH activity AS (...) SELECT FROM pganalyze.get_stat_progress_vacuum() v JOIN activity a USING (...");
}

// If we truncate the index expression in the future this would remove (lower(d) || upper(d)) first
#[test]
fn it_handles_CREATE_INDEX() {
    let query = "CREATE INDEX testidx ON test USING btree ((lower(d) || upper(d)), a, (b+c))";
    let result = parse(query).unwrap();
    assert_eq!(result.truncate(60).unwrap(), "CREATE INDEX testidx ON test USING btree ((lower(d) || up...");
}

#[test]
fn char_truncate_works() {
    let query = "WITH \"原チコ氏にはす腹腹腹腹腹腹腹腹腹腹腹\" AS (SELECT) SELECT w";
    let result = parse(query).unwrap();
    let output = "WITH \"原チコ氏にはす腹腹腹腹腹...";
    assert_eq!(result.truncate(21).unwrap(), output);
}

#[test]
#[should_panic(
    expected = "byte index 22 is not a char boundary; it is inside 'は' (bytes 21..24) of `WITH \"原チコ氏にはす腹腹腹腹腹腹腹腹腹腹腹\" AS (SELECT) SELECT w`"
)]
fn byte_truncate_fails() {
    let query = "WITH \"原チコ氏にはす腹腹腹腹腹腹腹腹腹腹腹\" AS (SELECT) SELECT w".to_string();
    query[0..=21].to_string();
}
