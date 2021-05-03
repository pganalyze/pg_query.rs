#[test]
fn it_can_fingerprint_a_simple_statement() {
    let result = pg_query::fingerprint("SELECT * FROM contacts.person WHERE id IN (1, 2, 3, 4);");
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.hex, "643d2a3c294ab8a7");
}

#[test]
fn it_will_error_on_invalid_input() {
    let result = pg_query::fingerprint("CREATE RANDOM ix_test ON contacts.person;");
    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap(),
        pg_query::Error::ParseError("syntax error at or near \"RANDOM\"".into())
    );
}
