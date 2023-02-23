#![allow(non_snake_case)]
#![cfg(test)]

use pg_query::{normalize, Error};

#[test]
fn it_normalizes_simple_query() {
    let result = normalize("SELECT 1").unwrap();
    assert_eq!(result, "SELECT $1");
}

#[test]
fn it_normalizes_IN() {
    let result = normalize("SELECT 1 FROM x WHERE y = 12561 AND z = '124' AND b IN (1, 2, 3)").unwrap();
    assert_eq!(result, "SELECT $1 FROM x WHERE y = $2 AND z = $3 AND b IN ($4, $5, $6)");
}

#[test]
fn it_errors_on_invalid_input() {
    let error = normalize("CREATE RANDOM ix_test ON contacts.person;").err().unwrap();
    assert_eq!(error, Error::Parse("syntax error at or near \"RANDOM\"".into()));
}

#[test]
fn it_normalizes_subselects() {
    let result = normalize("SELECT 1 FROM x WHERE y = (SELECT 123 FROM a WHERE z = 'bla')").unwrap();
    assert_eq!(result, "SELECT $1 FROM x WHERE y = (SELECT $2 FROM a WHERE z = $3)");
}

#[test]
fn it_normalizes_ANY() {
    let result = normalize("SELECT * FROM x WHERE y = ANY(array[1, 2])").unwrap();
    assert_eq!(result, "SELECT * FROM x WHERE y = ANY(array[$1, $2])");

    let result = normalize("SELECT * FROM x WHERE y = ANY(SELECT 1)").unwrap();
    assert_eq!(result, "SELECT * FROM x WHERE y = ANY(SELECT $1)");
}

#[test]
fn it_normalizes_complicated_strings() {
    let result = normalize("SELECT U&'d\\0061t\\+000061' FROM x").unwrap();
    assert_eq!(result, "SELECT $1 FROM x");

    let result = normalize("SELECT u&'d\\0061t\\+000061'    FROM x").unwrap();
    assert_eq!(result, "SELECT $1    FROM x");

    let result = normalize("SELECT * FROM x WHERE z NOT LIKE E'abc'AND TRUE").unwrap();
    assert_eq!(result, "SELECT * FROM x WHERE z NOT LIKE $1AND $2");

    let result = normalize("SELECT U&'d\\0061t\\+000061'-- comment\nFROM x").unwrap();
    assert_eq!(result, "SELECT $1-- comment\nFROM x");
}

#[test]
fn it_normalizes_COPY() {
    let result = normalize("COPY (SELECT * FROM t WHERE id IN ('1', '2')) TO STDOUT").unwrap();
    assert_eq!(result, "COPY (SELECT * FROM t WHERE id IN ($1, $2)) TO STDOUT");
}

#[test]
fn it_normalizes_SET() {
    let result = normalize("SET test=123").unwrap();
    assert_eq!(result, "SET test=$1");

    let result = normalize("SET CLIENT_ENCODING = UTF8").unwrap();
    assert_eq!(result, "SET CLIENT_ENCODING = $1");
}

#[test]
fn it_does_not_error_on_DEALLOCATE() {
    let result = normalize("DEALLOCATE bla; SELECT 1").unwrap();
    assert_eq!(result, "DEALLOCATE bla; SELECT $1");
}

#[test]
fn it_normalizes_EXPLAIN() {
    let result = normalize("EXPLAIN SELECT x FROM y WHERE z = 1").unwrap();
    assert_eq!(result, "EXPLAIN SELECT x FROM y WHERE z = $1");
}

#[test]
fn it_normalizes_DECLARE_CURSOR() {
    let result = normalize("DECLARE cursor_b CURSOR FOR SELECT * FROM databases WHERE id = 23").unwrap();
    assert_eq!(result, "DECLARE cursor_b CURSOR FOR SELECT * FROM databases WHERE id = $1");
}

#[test]
fn it_unwraps_and_normalizes_immediately_invoked_functions() {
    let query = "
        CREATE OR REPLACE FUNCTION pg_temp.testfunc(OUT response t, OUT sequelize_caught_exception text)
        RETURNS RECORD AS $func_08a0ae3001ba4697bd3a1a677c6dab12$
        BEGIN
            INSERT INTO t (columns)
            VALUES ('non-normalized-values-here')
            RETURNING * INTO response;
            EXCEPTION WHEN unique_violation THEN GET STACKED DIAGNOSTICS sequelize_caught_exception = PG_EXCEPTION_DETAIL;
        END $func_08a0ae3001ba4697bd3a1a677c6dab12$ LANGUAGE plpgsql;

        SELECT (testfunc.response).*, testfunc.sequelize_caught_exception FROM pg_temp.testfunc();

        DROP FUNCTION IF EXISTS pg_temp.testfunc()
    ";
    let result = pg_query::unwrap_immediately_invoked_function(query).unwrap();
    let normalized_result = pg_query::normalize(&result).unwrap();
    assert_eq!(
        normalized_result,
        "INSERT INTO t (columns)
            VALUES ($1)
            RETURNING *"
    );
    pg_query::parse(&normalized_result).unwrap();
}
