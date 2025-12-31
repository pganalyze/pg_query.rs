#![allow(non_snake_case)]
#![cfg(test)]

use pg_query::ast::{Node, SelectStmt, InsertStmt, UpdateStmt, DeleteStmt, SetOperation, JoinType};
use pg_query::{parse_to_ast, deparse_ast};

#[macro_use]
mod support;

/// Test that parse_to_ast successfully parses a simple SELECT query
#[test]
fn it_parses_simple_select_to_ast() {
    let result = parse_to_ast("SELECT * FROM users").unwrap();
    assert_eq!(result.stmts.len(), 1);

    if let Node::SelectStmt(select) = &result.stmts[0].stmt {
        // Check from_clause contains users table
        assert_eq!(select.from_clause.len(), 1);
        if let Node::RangeVar(range_var) = &select.from_clause[0] {
            assert_eq!(range_var.relname, "users");
        } else {
            panic!("Expected RangeVar in from_clause");
        }

        // Check target_list contains *
        assert_eq!(select.target_list.len(), 1);
        if let Node::ResTarget(res_target) = &select.target_list[0] {
            assert!(res_target.val.is_some());
            if let Some(Node::ColumnRef(col_ref)) = &res_target.val {
                assert_eq!(col_ref.fields.len(), 1);
                assert!(matches!(&col_ref.fields[0], Node::AStar(_)));
            } else {
                panic!("Expected ColumnRef with AStar");
            }
        } else {
            panic!("Expected ResTarget in target_list");
        }
    } else {
        panic!("Expected SelectStmt");
    }
}

/// Test that parse_to_ast handles errors correctly
#[test]
fn it_handles_parse_errors() {
    let result = parse_to_ast("SELECT * FORM users");
    assert!(result.is_err());
}

/// Test parsing SELECT with WHERE clause
#[test]
fn it_parses_select_with_where_clause() {
    let result = parse_to_ast("SELECT id, name FROM users WHERE id = 1").unwrap();
    assert_eq!(result.stmts.len(), 1);

    if let Node::SelectStmt(select) = &result.stmts[0].stmt {
        assert!(select.where_clause.is_some());
        assert_eq!(select.target_list.len(), 2);
        assert_eq!(select.from_clause.len(), 1);
    } else {
        panic!("Expected SelectStmt");
    }
}

/// Test parsing INSERT statement
#[test]
fn it_parses_insert_to_ast() {
    let result = parse_to_ast("INSERT INTO users (name, email) VALUES ('test', 'test@example.com')").unwrap();
    assert_eq!(result.stmts.len(), 1);

    if let Node::InsertStmt(insert) = &result.stmts[0].stmt {
        // Check relation
        if let Some(rel) = &insert.relation {
            assert_eq!(rel.relname, "users");
        } else {
            panic!("Expected relation");
        }

        // Check columns
        assert_eq!(insert.cols.len(), 2);
    } else {
        panic!("Expected InsertStmt");
    }
}

/// Test parsing UPDATE statement
#[test]
fn it_parses_update_to_ast() {
    let result = parse_to_ast("UPDATE users SET name = 'bob' WHERE id = 1").unwrap();
    assert_eq!(result.stmts.len(), 1);

    if let Node::UpdateStmt(update) = &result.stmts[0].stmt {
        // Check relation
        if let Some(rel) = &update.relation {
            assert_eq!(rel.relname, "users");
        } else {
            panic!("Expected relation");
        }

        // Check target_list (SET clause)
        assert_eq!(update.target_list.len(), 1);

        // Check where_clause
        assert!(update.where_clause.is_some());
    } else {
        panic!("Expected UpdateStmt");
    }
}

/// Test parsing DELETE statement
#[test]
fn it_parses_delete_to_ast() {
    let result = parse_to_ast("DELETE FROM users WHERE id = 1").unwrap();
    assert_eq!(result.stmts.len(), 1);

    if let Node::DeleteStmt(delete) = &result.stmts[0].stmt {
        // Check relation
        if let Some(rel) = &delete.relation {
            assert_eq!(rel.relname, "users");
        } else {
            panic!("Expected relation");
        }

        // Check where_clause
        assert!(delete.where_clause.is_some());
    } else {
        panic!("Expected DeleteStmt");
    }
}

/// Test parsing SELECT with JOIN
#[test]
fn it_parses_select_with_join() {
    let result = parse_to_ast("SELECT u.id, o.total FROM users u INNER JOIN orders o ON u.id = o.user_id").unwrap();
    assert_eq!(result.stmts.len(), 1);

    if let Node::SelectStmt(select) = &result.stmts[0].stmt {
        assert_eq!(select.from_clause.len(), 1);

        if let Node::JoinExpr(join) = &select.from_clause[0] {
            assert_eq!(join.jointype, JoinType::Inner);
            assert!(join.larg.is_some());
            assert!(join.rarg.is_some());
            assert!(join.quals.is_some());
        } else {
            panic!("Expected JoinExpr in from_clause");
        }
    } else {
        panic!("Expected SelectStmt");
    }
}

/// Test parsing UNION query
#[test]
fn it_parses_union_query() {
    let result = parse_to_ast("SELECT id FROM users UNION SELECT id FROM admins").unwrap();
    assert_eq!(result.stmts.len(), 1);

    if let Node::SelectStmt(select) = &result.stmts[0].stmt {
        assert_eq!(select.op, SetOperation::Union);
        assert!(select.larg.is_some());
        assert!(select.rarg.is_some());
    } else {
        panic!("Expected SelectStmt");
    }
}

/// Test round-trip: parse to AST then deparse back to SQL
#[test]
fn it_roundtrips_simple_select() {
    let original = "SELECT * FROM users";
    let ast = parse_to_ast(original).unwrap();
    let deparsed = deparse_ast(&ast).unwrap();
    assert_eq!(deparsed, original);
}

/// Test round-trip: SELECT with WHERE clause
#[test]
fn it_roundtrips_select_with_where() {
    let original = "SELECT id, name FROM users WHERE id = 1";
    let ast = parse_to_ast(original).unwrap();
    let deparsed = deparse_ast(&ast).unwrap();
    assert_eq!(deparsed, original);
}

/// Test round-trip: INSERT statement
#[test]
fn it_roundtrips_insert() {
    let original = "INSERT INTO users (name) VALUES ('test')";
    let ast = parse_to_ast(original).unwrap();
    let deparsed = deparse_ast(&ast).unwrap();
    assert_eq!(deparsed, original);
}

/// Test round-trip: UPDATE statement
#[test]
fn it_roundtrips_update() {
    let original = "UPDATE users SET name = 'bob' WHERE id = 1";
    let ast = parse_to_ast(original).unwrap();
    let deparsed = deparse_ast(&ast).unwrap();
    assert_eq!(deparsed, original);
}

/// Test round-trip: DELETE statement
#[test]
fn it_roundtrips_delete() {
    let original = "DELETE FROM users WHERE id = 1";
    let ast = parse_to_ast(original).unwrap();
    let deparsed = deparse_ast(&ast).unwrap();
    assert_eq!(deparsed, original);
}

/// Test round-trip: SELECT with JOIN
#[test]
fn it_roundtrips_join() {
    let original = "SELECT * FROM users u JOIN orders o ON u.id = o.user_id";
    let ast = parse_to_ast(original).unwrap();
    let deparsed = deparse_ast(&ast).unwrap();
    assert_eq!(deparsed, original);
}

/// Test round-trip: UNION query
#[test]
fn it_roundtrips_union() {
    let original = "SELECT id FROM users UNION SELECT id FROM admins";
    let ast = parse_to_ast(original).unwrap();
    let deparsed = deparse_ast(&ast).unwrap();
    assert_eq!(deparsed, original);
}

/// Test round-trip: complex SELECT
#[test]
fn it_roundtrips_complex_select() {
    let original = "SELECT u.id, u.name, count(*) AS order_count FROM users u LEFT JOIN orders o ON u.id = o.user_id WHERE u.active = true GROUP BY u.id, u.name HAVING count(*) > 0 ORDER BY order_count DESC LIMIT 10";
    let ast = parse_to_ast(original).unwrap();
    let deparsed = deparse_ast(&ast).unwrap();
    assert_eq!(deparsed, original);
}

/// Test round-trip: WITH clause (CTE)
#[test]
fn it_roundtrips_cte() {
    let original = "WITH active_users AS (SELECT * FROM users WHERE active = true) SELECT * FROM active_users";
    let ast = parse_to_ast(original).unwrap();
    let deparsed = deparse_ast(&ast).unwrap();
    assert_eq!(deparsed, original);
}

/// Test round-trip: CREATE TABLE
#[test]
fn it_roundtrips_create_table() {
    let original = "CREATE TABLE test (id integer, name text)";
    let ast = parse_to_ast(original).unwrap();
    let deparsed = deparse_ast(&ast).unwrap();
    // pg_query uses "int" instead of "integer" in its canonical form
    assert_eq!(deparsed, "CREATE TABLE test (id int, name text)");
}

/// Test round-trip: DROP TABLE
#[test]
fn it_roundtrips_drop_table() {
    let original = "DROP TABLE users";
    let ast = parse_to_ast(original).unwrap();
    let deparsed = deparse_ast(&ast).unwrap();
    assert_eq!(deparsed, original);
}

/// Test round-trip: CREATE INDEX
#[test]
fn it_roundtrips_create_index() {
    let original = "CREATE INDEX idx_users_name ON users (name)";
    let ast = parse_to_ast(original).unwrap();
    let deparsed = deparse_ast(&ast).unwrap();
    // pg_query adds explicit "USING btree" in its canonical form
    assert_eq!(deparsed, "CREATE INDEX idx_users_name ON users USING btree (name)");
}

/// Test that the AST types are ergonomic (no deep Option<Box<Node>> unwrapping)
#[test]
fn ast_types_are_ergonomic() {
    let result = parse_to_ast("SELECT id FROM users WHERE active = true").unwrap();

    // Direct pattern matching without .as_ref().unwrap() chains
    if let Node::SelectStmt(select) = &result.stmts[0].stmt {
        // Direct access to from_clause vector
        for table in &select.from_clause {
            if let Node::RangeVar(rv) = table {
                assert_eq!(rv.relname, "users");
            }
        }

        // Direct access to target_list
        for target in &select.target_list {
            if let Node::ResTarget(rt) = target {
                if let Some(Node::ColumnRef(cr)) = &rt.val {
                    // Can access fields directly
                    assert!(!cr.fields.is_empty());
                }
            }
        }
    }
}

/// Test parsing multiple statements
#[test]
fn it_parses_multiple_statements() {
    let result = parse_to_ast("SELECT 1; SELECT 2; SELECT 3").unwrap();
    assert_eq!(result.stmts.len(), 3);

    for stmt in &result.stmts {
        assert!(matches!(&stmt.stmt, Node::SelectStmt(_)));
    }
}

/// Test parsing empty query (comment only)
#[test]
fn it_parses_empty_query() {
    let result = parse_to_ast("-- just a comment").unwrap();
    assert_eq!(result.stmts.len(), 0);
}

/// Test round-trip: subquery in SELECT
#[test]
fn it_roundtrips_subquery() {
    let original = "SELECT * FROM users WHERE id IN (SELECT user_id FROM orders)";
    let ast = parse_to_ast(original).unwrap();
    let deparsed = deparse_ast(&ast).unwrap();
    assert_eq!(deparsed, original);
}

/// Test round-trip: aggregate functions
#[test]
fn it_roundtrips_aggregates() {
    let original = "SELECT count(*), sum(amount), avg(price) FROM orders";
    let ast = parse_to_ast(original).unwrap();
    let deparsed = deparse_ast(&ast).unwrap();
    assert_eq!(deparsed, original);
}

/// Test round-trip: CASE expression
#[test]
fn it_roundtrips_case_expression() {
    let original = "SELECT CASE WHEN x > 0 THEN 'positive' ELSE 'non-positive' END FROM t";
    let ast = parse_to_ast(original).unwrap();
    let deparsed = deparse_ast(&ast).unwrap();
    assert_eq!(deparsed, original);
}

/// Test round-trip: INSERT with RETURNING
#[test]
fn it_roundtrips_insert_returning() {
    let original = "INSERT INTO users (name) VALUES ('test') RETURNING id";
    let ast = parse_to_ast(original).unwrap();
    let deparsed = deparse_ast(&ast).unwrap();
    assert_eq!(deparsed, original);
}

/// Test round-trip: UPDATE with FROM
#[test]
fn it_roundtrips_update_from() {
    let original = "UPDATE users SET name = o.name FROM other_users o WHERE users.id = o.id";
    let ast = parse_to_ast(original).unwrap();
    let deparsed = deparse_ast(&ast).unwrap();
    assert_eq!(deparsed, original);
}

/// Test round-trip: DELETE with USING
#[test]
fn it_roundtrips_delete_using() {
    let original = "DELETE FROM users USING orders WHERE users.id = orders.user_id";
    let ast = parse_to_ast(original).unwrap();
    let deparsed = deparse_ast(&ast).unwrap();
    assert_eq!(deparsed, original);
}
