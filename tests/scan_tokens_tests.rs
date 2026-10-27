#![cfg(test)]

use pg_query::{scan, scan_tokens, Error};

#[test]
fn it_scans_a_minimal_query() {
    // mirrors the README example
    let tokens = scan_tokens("SELECT 1").unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 6);
}

#[test]
fn it_will_error_on_invalid_input() {
    let error = scan_tokens("SELECT 'unterminated").err().unwrap();
    assert!(matches!(error, Error::Scan(_)), "unexpected error: {:?}", error);
}

#[test]
fn it_returns_the_same_tokens_as_scan() {
    for sql in [
        "SELECT update AS left /* comment */ FROM between",
        "SELECT 1",
        "INSERT INTO test (a, b) VALUES ($1, $2)",
        "CREATE OR REPLACE FUNCTION foo() RETURNS int AS $$ BEGIN RETURN 1; END; $$ LANGUAGE plpgsql",
        "select /*;*/ 1; select \"2;\"",
    ] {
        assert_eq!(scan_tokens(sql).unwrap(), scan(sql).unwrap().tokens, "unexpected tokens for {:?}", sql);
    }
}

#[test]
fn it_finds_token_locations_and_keyword_kinds() {
    let tokens = scan_tokens("SELECT update AS left /* comment */ FROM between").unwrap();
    let formatted: Vec<String> = tokens.iter().map(|token| format!("{:?}", token)).collect();
    assert_eq!(
        formatted,
        vec![
            "ScanToken { start: 0, end: 6, token: Select, keyword_kind: ReservedKeyword }",
            "ScanToken { start: 7, end: 13, token: Update, keyword_kind: UnreservedKeyword }",
            "ScanToken { start: 14, end: 16, token: As, keyword_kind: ReservedKeyword }",
            "ScanToken { start: 17, end: 21, token: Left, keyword_kind: TypeFuncNameKeyword }",
            "ScanToken { start: 22, end: 35, token: CComment, keyword_kind: NoKeyword }",
            "ScanToken { start: 36, end: 40, token: From, keyword_kind: ReservedKeyword }",
            "ScanToken { start: 41, end: 48, token: Between, keyword_kind: ColNameKeyword }"
        ]
    );
}
