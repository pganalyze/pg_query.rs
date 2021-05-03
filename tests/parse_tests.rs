use pg_query::ast::Node;

#[test]
fn it_can_generate_an_ast() {
    let result =
        pg_query::parse("CREATE INDEX ix_test ON contacts.person (id, ssn) WHERE ssn IS NOT NULL;");
    assert!(result.is_ok());
    let result = result.unwrap();
    let el: &Node = &result[0];
    match *el {
        Node::IndexStmt(ref stmt) => {
            assert_eq!(stmt.idxname, Some("ix_test".to_string()), "idxname");
            let relation = stmt.relation.as_ref().expect("relation exists");
            assert_eq!(
                relation.schemaname,
                Some("contacts".to_string()),
                "schemaname"
            );
            assert_eq!(relation.relname, Some("person".to_string()), "relname");
            let params = stmt.index_params.as_ref().expect("index params");
            assert_eq!(2, params.len(), "Params length");
        }
        _ => panic!("Unexpected type"),
    }
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
