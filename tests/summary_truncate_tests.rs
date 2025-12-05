#![allow(non_snake_case)]
#![cfg(test)]

use pg_query::summary;

#[test]
fn it_omits_target_list() {
    let query = "SELECT a, b, c, d, e, f FROM xyz WHERE a = b";
    let result = summary(query, 40).unwrap();
    assert_eq!(result.truncated_query, "SELECT ... FROM xyz WHERE a = b")
}

#[test]
fn it_omits_CTE_definition() {
    let query = "WITH x AS (SELECT * FROM y) SELECT * FROM x";
    let result = summary(query, 40).unwrap();
    assert_eq!(result.truncated_query, "WITH x AS (...) SELECT * FROM x")
}

#[test]
fn it_omits_WHERE_clause() {
    let query = "SELECT * FROM z WHERE a = b AND x = y";
    let result = summary(query, 30).unwrap();
    assert_eq!(result.truncated_query, "SELECT * FROM z WHERE ...")
}

#[test]
fn it_omits_INSERT_field_list() {
    let query = "INSERT INTO \"x\" (a, b, c, d, e, f) VALUES ($1)";
    let result = summary(query, 32).unwrap();
    assert_eq!(result.truncated_query, "INSERT INTO x (...) VALUES ($1)")
}

#[test]
fn it_omits_comments() {
    let query = "SELECT $1 /* application:test */";
    let result = summary(query, 100).unwrap();
    assert_eq!(result.truncated_query, "SELECT $1")
}

#[test]
fn it_falls_back_to_simple_truncation() {
    let query = "SELECT * FROM t";
    let result = summary(query, 10).unwrap();
    assert_eq!(result.truncated_query, "SELECT ...")
}

#[test]
fn it_handles_problematic_cases() {
    let query = "SELECT CASE WHEN $2.typtype = $1 THEN $2.typtypmod ELSE $1.atttypmod END";
    let result = summary(query, 50).unwrap();
    assert_eq!(result.truncated_query, "SELECT ...")
}

#[test]
fn it_omits_UPDATE_target_list() {
    let query = "UPDATE x SET a = 1, c = 2, e = 'str'";
    let result = summary(query, 30).unwrap();
    assert_eq!(result.truncated_query, "UPDATE x SET ... = ...")
}

#[test]
fn it_omits_ON_CONFLICT_target_list() {
    let query = "INSERT INTO y(a) VALUES(1) ON CONFLICT DO UPDATE SET a = 123456789";
    let result = summary(query, 64).unwrap();
    assert_eq!(result.truncated_query, "INSERT INTO y (a) VALUES (1) ON CONFLICT DO UPDATE SET ... = ...")
}

#[test]
fn it_omits_ON_CONFLICT_target_list_2() {
    let query = r#"
        INSERT INTO foo (a, b, c, d) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19,$20,$21,$22,$23,$24,$25,$26,$27,$28,$29)
        ON CONFLICT (id)
        DO UPDATE SET (a, b, c, d) = (excluded.a,excluded.b,excluded.c,case when foo.d = excluded.d then excluded.d end)
    "#;
    let result = summary(query, 100).unwrap();
    assert_eq!(result.truncated_query, "INSERT INTO foo (a, b, c, d) VALUES (...) ON CONFLICT (id) DO UPDATE SET ... = ...");
}

#[test]
fn it_handles_GRANT() {
    let query = "GRANT SELECT (abc, def, ghj) ON TABLE t1 TO r1";
    let result = summary(query, 35).unwrap();
    assert_eq!(result.truncated_query, "GRANT select (abc, def, ghj) ON ...")
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
    let result = summary(query, 100).unwrap();
    assert_eq!(result.truncated_query, "WITH activity AS (...) SELECT FROM pganalyze.get_stat_progress_vacuum() v JOIN activity a USING (...");
}

// If we truncate the index expression in the future this would remove (lower(d) || upper(d)) first
#[test]
fn it_handles_CREATE_INDEX() {
    let query = "CREATE INDEX testidx ON test USING btree ((lower(d) || upper(d)), a, (b+c))";
    let result = summary(query, 60).unwrap();
    assert_eq!(result.truncated_query, "CREATE INDEX testidx ON test USING btree ((lower(d) || up...");
}

#[test]
fn char_truncate_works() {
    let query = "WITH \"原チコ氏にはす腹腹腹腹腹腹腹腹腹腹腹\" AS (SELECT) SELECT w";
    let result = summary(query, 21).unwrap();
    let output = "WITH \"原チコ氏にはす腹腹腹腹腹...";
    assert_eq!(result.truncated_query, output);
}

#[test]
#[should_panic(
    expected = "byte index 22 is not a char boundary; it is inside 'は' (bytes 21..24) of `WITH \"原チコ氏にはす腹腹腹腹腹腹腹腹腹腹腹\" AS (SELECT) SELECT w`"
)]
fn byte_truncate_fails() {
    let query = "WITH \"原チコ氏にはす腹腹腹腹腹腹腹腹腹腹腹\" AS (SELECT) SELECT w".to_string();
    query[0..=21].to_string();
}
