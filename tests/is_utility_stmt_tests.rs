#![cfg(test)]

use pg_query::{is_utility_stmt, Error};

#[test]
fn it_will_error_on_invalid_input() {
    let error = is_utility_stmt("SELECT !").err().unwrap();
    assert_eq!(error, Error::IsUtility("syntax error at end of input".into()));
}

#[test]
fn it_detects_dml_statements_as_non_utility() {
    assert_eq!(is_utility_stmt("SELECT 1").unwrap(), vec![false]);
    assert_eq!(is_utility_stmt("INSERT INTO my_table VALUES(123)").unwrap(), vec![false]);
    assert_eq!(is_utility_stmt("UPDATE my_table SET foo = 123").unwrap(), vec![false]);
    assert_eq!(is_utility_stmt("DELETE FROM my_table").unwrap(), vec![false]);
}

#[test]
fn it_detects_utility_statements() {
    assert_eq!(is_utility_stmt("SHOW fsync").unwrap(), vec![true]);
    assert_eq!(is_utility_stmt("SET fsync = off").unwrap(), vec![true]);
    assert_eq!(is_utility_stmt("CREATE TABLE foo (a int)").unwrap(), vec![true]);
    assert_eq!(is_utility_stmt("DROP TABLE foo").unwrap(), vec![true]);
}

#[test]
fn it_returns_one_result_per_statement() {
    assert_eq!(is_utility_stmt("SELECT 1; SELECT 2;").unwrap(), vec![false, false]);
    assert_eq!(is_utility_stmt("SELECT 1; SHOW fsync;").unwrap(), vec![false, true]);
}
