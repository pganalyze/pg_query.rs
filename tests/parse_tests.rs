#[test]
fn it_can_generate_an_ast() {
    let result =
        pg_query::parse("CREATE INDEX ix_test ON contacts.person (id, ssn) WHERE ssn IS NOT NULL;");
    println!("{:?}", result.unwrap());
    //assert!(result.is_ok());
}

#[test]
fn it_will_error_on_invalid_input() {
    let result = pg_query::parse("CREATE RANDOM ix_test ON contacts.person;");
    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap(),
        pg_query::Error::ParseError("syntax error at or near \"RANDOM\"".into())
    );
}
