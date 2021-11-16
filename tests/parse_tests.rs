use pg_query::{Node, Nodes};

#[test]
fn it_can_generate_a_create_index_ast() {
    let result =
        pg_query::parse("CREATE INDEX ix_test ON contacts.person (id, ssn) WHERE ssn IS NOT NULL;");
    assert!(result.is_ok());
    let result = result.unwrap();
    let el: &Node = &result[0];
    match el.node {
        Some(Nodes::IndexStmt(ref stmt)) => {
            assert_eq!(stmt.idxname, "ix_test".to_string(), "idxname");
            let relation = stmt.relation.as_ref().expect("relation exists");
            assert_eq!(
                relation.schemaname,
                "contacts".to_string(),
                "schemaname"
            );
            assert_eq!(relation.relname, "person".to_string(), "relname");
            assert_eq!(2, stmt.index_params.len(), "Params length");
        }
        _ => panic!("Unexpected type"),
    }
}

#[test]
fn it_can_generate_a_create_table_ast() {
    let result =
        pg_query::parse("CREATE TABLE contacts.person(id serial primary key, name text not null);");
    assert!(result.is_ok());
    let result = result.unwrap();
    let el: &Node = &result[0];
    match el.node {
        Some(Nodes::CreateStmt(ref stmt)) => {
            let relation = stmt.relation.as_ref().expect("relation exists");
            assert_eq!(
                relation.schemaname,
                "contacts".to_string(),
                "schemaname"
            );
            assert_eq!(relation.relname, "person".to_string(), "relname");
            assert_eq!(2, stmt.table_elts.len(), "Columns length");
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
        pg_query::Error::Parse("syntax error at or near \"RANDOM\"".into())
    );
}
