#[test]
fn it_works() {
    pg_query::parse("CREATE INDEX ix_test ON contacts.person (id, ssn) WHERE ssn IS NOT NULL;");
}
