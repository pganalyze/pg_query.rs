#![allow(non_snake_case)]
#![cfg(test)]

use pg_query::{parse, parse_raw, Error};
use pg_query::protobuf::{a_const, node, ParseResult as ProtobufParseResult};

#[macro_use]
mod support;

// ============================================================================
// Helper functions
// ============================================================================

/// Helper to extract AConst from a SELECT statement's first target
fn get_first_const(result: &ProtobufParseResult) -> Option<&pg_query::protobuf::AConst> {
    let stmt = result.stmts.first()?;
    let raw_stmt = stmt.stmt.as_ref()?;
    let node = raw_stmt.node.as_ref()?;

    if let node::Node::SelectStmt(select) = node {
        let target = select.target_list.first()?;
        if let Some(node::Node::ResTarget(res_target)) = target.node.as_ref() {
            if let Some(val_node) = res_target.val.as_ref() {
                if let Some(node::Node::AConst(aconst)) = val_node.node.as_ref() {
                    return Some(aconst);
                }
            }
        }
    }
    None
}

// ============================================================================
// Basic parsing tests
// ============================================================================

/// Test that parse_raw successfully parses a simple SELECT query
#[test]
fn it_parses_simple_select() {
    let query = "SELECT 1";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf.stmts.len(), 1);
    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test that parse_raw handles syntax errors
#[test]
fn it_handles_parse_errors() {
    let query = "SELECT * FORM users";
    let raw_error = parse_raw(query).err().unwrap();
    let proto_error = parse(query).err().unwrap();

    assert!(matches!(raw_error, Error::Parse(_)));
    assert!(matches!(proto_error, Error::Parse(_)));
}

/// Test that parse_raw and parse produce equivalent results for simple SELECT
#[test]
fn it_matches_parse_for_simple_select() {
    let query = "SELECT 1";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test that parse_raw and parse produce equivalent results for SELECT with table
#[test]
fn it_matches_parse_for_select_from_table() {
    let query = "SELECT * FROM users";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    // Verify tables are extracted correctly
    let mut raw_tables = raw_result.tables();
    let mut proto_tables = proto_result.tables();
    raw_tables.sort();
    proto_tables.sort();
    assert_eq!(raw_tables, proto_tables);
    assert_eq!(raw_tables, vec!["users"]);
}

/// Test that parse_raw handles empty queries (comments only)
#[test]
fn it_handles_empty_queries() {
    let query = "-- just a comment";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf.stmts.len(), 0);
    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test that parse_raw parses multiple statements
#[test]
fn it_parses_multiple_statements() {
    let query = "SELECT 1; SELECT 2; SELECT 3";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf.stmts.len(), 3);
    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

// ============================================================================
// DML statement tests
// ============================================================================

/// Test parsing INSERT statement
#[test]
fn it_parses_insert() {
    let query = "INSERT INTO users (name) VALUES ('test')";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    // Verify the INSERT target table
    let mut raw_tables = raw_result.dml_tables();
    let mut proto_tables = proto_result.dml_tables();
    raw_tables.sort();
    proto_tables.sort();
    assert_eq!(raw_tables, proto_tables);
    assert_eq!(raw_tables, vec!["users"]);
}

/// Test parsing UPDATE statement
#[test]
fn it_parses_update() {
    let query = "UPDATE users SET name = 'bob' WHERE id = 1";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    // Verify the UPDATE target table
    let mut raw_tables = raw_result.dml_tables();
    let mut proto_tables = proto_result.dml_tables();
    raw_tables.sort();
    proto_tables.sort();
    assert_eq!(raw_tables, proto_tables);
    assert_eq!(raw_tables, vec!["users"]);
}

/// Test parsing DELETE statement
#[test]
fn it_parses_delete() {
    let query = "DELETE FROM users WHERE id = 1";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    // Verify the DELETE target table
    let mut raw_tables = raw_result.dml_tables();
    let mut proto_tables = proto_result.dml_tables();
    raw_tables.sort();
    proto_tables.sort();
    assert_eq!(raw_tables, proto_tables);
    assert_eq!(raw_tables, vec!["users"]);
}

// ============================================================================
// DDL statement tests
// ============================================================================

/// Test parsing CREATE TABLE
#[test]
fn it_parses_create_table() {
    let query = "CREATE TABLE test (id int, name text)";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    // Verify statement types match
    assert_eq!(raw_result.statement_types(), proto_result.statement_types());
    assert_eq!(raw_result.statement_types(), vec!["CreateStmt"]);
}

/// Test parsing DROP TABLE
#[test]
fn it_parses_drop_table() {
    let query = "DROP TABLE users";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    // Verify DDL tables match
    let mut raw_tables = raw_result.ddl_tables();
    let mut proto_tables = proto_result.ddl_tables();
    raw_tables.sort();
    proto_tables.sort();
    assert_eq!(raw_tables, proto_tables);
    assert_eq!(raw_tables, vec!["users"]);
}

/// Test parsing CREATE INDEX
#[test]
fn it_parses_create_index() {
    let query = "CREATE INDEX idx_users_name ON users (name)";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    // Verify statement types match
    assert_eq!(raw_result.statement_types(), proto_result.statement_types());
    assert_eq!(raw_result.statement_types(), vec!["IndexStmt"]);
}

// ============================================================================
// JOIN and complex SELECT tests
// ============================================================================

/// Test parsing SELECT with JOIN
#[test]
fn it_parses_join() {
    let query = "SELECT * FROM users u JOIN orders o ON u.id = o.user_id";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    // Verify tables are extracted correctly
    let mut raw_tables = raw_result.tables();
    let mut proto_tables = proto_result.tables();
    raw_tables.sort();
    proto_tables.sort();
    assert_eq!(raw_tables, proto_tables);
    assert_eq!(raw_tables, vec!["orders", "users"]);
}

/// Test parsing UNION query
#[test]
fn it_parses_union() {
    let query = "SELECT id FROM users UNION SELECT id FROM admins";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    // Verify tables from both sides of UNION
    let mut raw_tables = raw_result.tables();
    let mut proto_tables = proto_result.tables();
    raw_tables.sort();
    proto_tables.sort();
    assert_eq!(raw_tables, proto_tables);
    assert_eq!(raw_tables, vec!["admins", "users"]);
}

/// Test parsing WITH clause (CTE)
#[test]
fn it_parses_cte() {
    let query = "WITH active_users AS (SELECT * FROM users WHERE active = true) SELECT * FROM active_users";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    // Verify CTE names match
    assert_eq!(raw_result.cte_names, proto_result.cte_names);
    assert!(raw_result.cte_names.contains(&"active_users".to_string()));

    // Verify tables (should only include actual tables, not CTEs)
    let mut raw_tables = raw_result.tables();
    let mut proto_tables = proto_result.tables();
    raw_tables.sort();
    proto_tables.sort();
    assert_eq!(raw_tables, proto_tables);
    assert_eq!(raw_tables, vec!["users"]);
}

/// Test parsing subquery in SELECT
#[test]
fn it_parses_subquery() {
    let query = "SELECT * FROM users WHERE id IN (SELECT user_id FROM orders)";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    // Verify all tables are found
    let mut raw_tables = raw_result.tables();
    let mut proto_tables = proto_result.tables();
    raw_tables.sort();
    proto_tables.sort();
    assert_eq!(raw_tables, proto_tables);
    assert_eq!(raw_tables, vec!["orders", "users"]);
}

/// Test parsing aggregate functions
#[test]
fn it_parses_aggregates() {
    let query = "SELECT count(*), sum(amount), avg(price) FROM orders";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    // Verify functions are extracted correctly
    let mut raw_funcs = raw_result.functions();
    let mut proto_funcs = proto_result.functions();
    raw_funcs.sort();
    proto_funcs.sort();
    assert_eq!(raw_funcs, proto_funcs);
    assert!(raw_funcs.contains(&"count".to_string()));
    assert!(raw_funcs.contains(&"sum".to_string()));
    assert!(raw_funcs.contains(&"avg".to_string()));
}

/// Test parsing CASE expression
#[test]
fn it_parses_case_expression() {
    let query = "SELECT CASE WHEN x > 0 THEN 'positive' ELSE 'non-positive' END FROM t";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    // Verify table is found
    let raw_tables = raw_result.tables();
    let proto_tables = proto_result.tables();
    assert_eq!(raw_tables, proto_tables);
    assert_eq!(raw_tables, vec!["t"]);
}

/// Test parsing complex SELECT with multiple clauses
#[test]
fn it_parses_complex_select() {
    let query = "SELECT u.id, u.name, count(*) AS order_count FROM users u LEFT JOIN orders o ON u.id = o.user_id WHERE u.active = true GROUP BY u.id, u.name HAVING count(*) > 0 ORDER BY order_count DESC LIMIT 10";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    // Verify tables
    let mut raw_tables = raw_result.tables();
    let mut proto_tables = proto_result.tables();
    raw_tables.sort();
    proto_tables.sort();
    assert_eq!(raw_tables, proto_tables);
    assert_eq!(raw_tables, vec!["orders", "users"]);

    // Verify functions
    let mut raw_funcs = raw_result.functions();
    let mut proto_funcs = proto_result.functions();
    raw_funcs.sort();
    proto_funcs.sort();
    assert_eq!(raw_funcs, proto_funcs);
    assert!(raw_funcs.contains(&"count".to_string()));
}

// ============================================================================
// INSERT variations
// ============================================================================

/// Test parsing INSERT with ON CONFLICT
#[test]
fn it_parses_insert_on_conflict() {
    let query = "INSERT INTO users (id, name) VALUES (1, 'test') ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    // Verify DML tables
    let raw_tables = raw_result.dml_tables();
    let proto_tables = proto_result.dml_tables();
    assert_eq!(raw_tables, proto_tables);
    assert_eq!(raw_tables, vec!["users"]);
}

/// Test parsing INSERT with RETURNING
#[test]
fn it_parses_insert_returning() {
    let query = "INSERT INTO users (name) VALUES ('test') RETURNING id";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

// ============================================================================
// Literal value tests
// ============================================================================

/// Test parsing float with leading dot
#[test]
fn it_parses_floats_with_leading_dot() {
    let query = "SELECT .1";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    // Verify the float value
    let raw_const = get_first_const(&raw_result.protobuf).expect("should have const");
    let proto_const = get_first_const(&proto_result.protobuf).expect("should have const");
    assert_eq!(raw_const, proto_const);
}

/// Test parsing bit string in hex notation
#[test]
fn it_parses_bit_strings_hex() {
    let query = "SELECT X'EFFF'";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    // Verify the bit string value
    let raw_const = get_first_const(&raw_result.protobuf).expect("should have const");
    let proto_const = get_first_const(&proto_result.protobuf).expect("should have const");
    assert_eq!(raw_const, proto_const);
}

/// Test parsing real-world query with multiple joins
#[test]
fn it_parses_real_world_query() {
    let query = "
        SELECT memory_total_bytes, memory_free_bytes, memory_pagecache_bytes,
            (memory_swap_total_bytes - memory_swap_free_bytes) AS swap
        FROM snapshots s JOIN system_snapshots ON (snapshot_id = s.id)
        WHERE s.database_id = 1 AND s.collected_at BETWEEN '2021-01-01' AND '2021-12-31'
        ORDER BY collected_at";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    // Verify tables
    let mut raw_tables = raw_result.tables();
    let mut proto_tables = proto_result.tables();
    raw_tables.sort();
    proto_tables.sort();
    assert_eq!(raw_tables, proto_tables);
    assert_eq!(raw_tables, vec!["snapshots", "system_snapshots"]);
}

// ============================================================================
// A_Const value extraction tests
// ============================================================================

/// Test that parse_raw extracts integer values correctly and matches parse
#[test]
fn it_extracts_integer_const() {
    let query = "SELECT 42";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    let raw_const = get_first_const(&raw_result.protobuf).expect("Should have A_Const");
    let proto_const = get_first_const(&proto_result.protobuf).expect("Should have A_Const");

    assert_eq!(raw_const, proto_const);
    assert!(!raw_const.isnull);
    match &raw_const.val {
        Some(a_const::Val::Ival(int_val)) => {
            assert_eq!(int_val.ival, 42);
        }
        other => panic!("Expected Ival, got {:?}", other),
    }
}

/// Test that parse_raw extracts negative integer values correctly
#[test]
fn it_extracts_negative_integer_const() {
    let query = "SELECT -123";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test that parse_raw extracts string values correctly and matches parse
#[test]
fn it_extracts_string_const() {
    let query = "SELECT 'hello world'";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    let raw_const = get_first_const(&raw_result.protobuf).expect("Should have A_Const");
    let proto_const = get_first_const(&proto_result.protobuf).expect("Should have A_Const");

    assert_eq!(raw_const, proto_const);
    assert!(!raw_const.isnull);
    match &raw_const.val {
        Some(a_const::Val::Sval(str_val)) => {
            assert_eq!(str_val.sval, "hello world");
        }
        other => panic!("Expected Sval, got {:?}", other),
    }
}

/// Test that parse_raw extracts float values correctly and matches parse
#[test]
fn it_extracts_float_const() {
    let query = "SELECT 3.14159";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    let raw_const = get_first_const(&raw_result.protobuf).expect("Should have A_Const");
    let proto_const = get_first_const(&proto_result.protobuf).expect("Should have A_Const");

    assert_eq!(raw_const, proto_const);
    assert!(!raw_const.isnull);
    match &raw_const.val {
        Some(a_const::Val::Fval(float_val)) => {
            assert_eq!(float_val.fval, "3.14159");
        }
        other => panic!("Expected Fval, got {:?}", other),
    }
}

/// Test that parse_raw extracts boolean TRUE correctly and matches parse
#[test]
fn it_extracts_boolean_true_const() {
    let query = "SELECT TRUE";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    let raw_const = get_first_const(&raw_result.protobuf).expect("Should have A_Const");
    let proto_const = get_first_const(&proto_result.protobuf).expect("Should have A_Const");

    assert_eq!(raw_const, proto_const);
    assert!(!raw_const.isnull);
    match &raw_const.val {
        Some(a_const::Val::Boolval(bool_val)) => {
            assert!(bool_val.boolval);
        }
        other => panic!("Expected Boolval(true), got {:?}", other),
    }
}

/// Test that parse_raw extracts boolean FALSE correctly and matches parse
#[test]
fn it_extracts_boolean_false_const() {
    let query = "SELECT FALSE";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    let raw_const = get_first_const(&raw_result.protobuf).expect("Should have A_Const");
    let proto_const = get_first_const(&proto_result.protobuf).expect("Should have A_Const");

    assert_eq!(raw_const, proto_const);
    assert!(!raw_const.isnull);
    match &raw_const.val {
        Some(a_const::Val::Boolval(bool_val)) => {
            assert!(!bool_val.boolval);
        }
        other => panic!("Expected Boolval(false), got {:?}", other),
    }
}

/// Test that parse_raw extracts NULL correctly and matches parse
#[test]
fn it_extracts_null_const() {
    let query = "SELECT NULL";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    let raw_const = get_first_const(&raw_result.protobuf).expect("Should have A_Const");
    let proto_const = get_first_const(&proto_result.protobuf).expect("Should have A_Const");

    assert_eq!(raw_const, proto_const);
    assert!(raw_const.isnull);
    assert!(raw_const.val.is_none());
}

/// Test that parse_raw extracts bit string values correctly and matches parse
#[test]
fn it_extracts_bit_string_const() {
    let query = "SELECT B'1010'";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    let raw_const = get_first_const(&raw_result.protobuf).expect("Should have A_Const");
    let proto_const = get_first_const(&proto_result.protobuf).expect("Should have A_Const");

    assert_eq!(raw_const, proto_const);
    assert!(!raw_const.isnull);
    match &raw_const.val {
        Some(a_const::Val::Bsval(bit_val)) => {
            assert_eq!(bit_val.bsval, "b1010");
        }
        other => panic!("Expected Bsval, got {:?}", other),
    }
}

/// Test that parse_raw extracts hex bit string correctly and matches parse
#[test]
fn it_extracts_hex_bit_string_const() {
    let query = "SELECT X'FF'";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    let raw_const = get_first_const(&raw_result.protobuf).expect("Should have A_Const");
    let proto_const = get_first_const(&proto_result.protobuf).expect("Should have A_Const");

    assert_eq!(raw_const, proto_const);
    assert!(!raw_const.isnull);
    match &raw_const.val {
        Some(a_const::Val::Bsval(bit_val)) => {
            assert_eq!(bit_val.bsval, "xFF");
        }
        other => panic!("Expected Bsval, got {:?}", other),
    }
}

// ============================================================================
// ParseResult method equivalence tests
// ============================================================================

/// Test that tables() returns the same results for both parsers
#[test]
fn it_returns_tables_like_parse() {
    let query = "SELECT * FROM users";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    // Both should have the same tables
    let mut raw_tables = raw_result.tables();
    let mut proto_tables = proto_result.tables();
    raw_tables.sort();
    proto_tables.sort();
    assert_eq!(raw_tables, proto_tables);
    assert_eq!(raw_tables, vec!["users"]);
}

/// Test that functions() returns the same results for both parsers
#[test]
fn it_returns_functions_like_parse() {
    let query = "SELECT count(*), sum(amount) FROM orders";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    // Both should have the same functions
    let mut raw_funcs = raw_result.functions();
    let mut proto_funcs = proto_result.functions();
    raw_funcs.sort();
    proto_funcs.sort();
    assert_eq!(raw_funcs, proto_funcs);
    assert_eq!(raw_funcs, vec!["count", "sum"]);
}

/// Test that statement_types() returns the same results for both parsers
#[test]
fn it_returns_statement_types_like_parse() {
    let query = "SELECT 1; INSERT INTO t VALUES (1); UPDATE t SET x = 1; DELETE FROM t";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    // Full structural equality check
    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    assert_eq!(raw_result.statement_types(), proto_result.statement_types());
    assert_eq!(raw_result.statement_types(), vec!["SelectStmt", "InsertStmt", "UpdateStmt", "DeleteStmt"]);
}

// ============================================================================
// Advanced JOIN tests
// ============================================================================

/// Test LEFT JOIN
#[test]
fn it_parses_left_join() {
    let query = "SELECT * FROM users u LEFT JOIN orders o ON u.id = o.user_id";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    let mut raw_tables = raw_result.tables();
    raw_tables.sort();
    assert_eq!(raw_tables, vec!["orders", "users"]);
}

/// Test RIGHT JOIN
#[test]
fn it_parses_right_join() {
    let query = "SELECT * FROM users u RIGHT JOIN orders o ON u.id = o.user_id";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test FULL OUTER JOIN
#[test]
fn it_parses_full_outer_join() {
    let query = "SELECT * FROM users u FULL OUTER JOIN orders o ON u.id = o.user_id";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test CROSS JOIN
#[test]
fn it_parses_cross_join() {
    let query = "SELECT * FROM users CROSS JOIN products";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    let mut raw_tables = raw_result.tables();
    raw_tables.sort();
    assert_eq!(raw_tables, vec!["products", "users"]);
}

/// Test NATURAL JOIN
#[test]
fn it_parses_natural_join() {
    let query = "SELECT * FROM users NATURAL JOIN user_profiles";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test multiple JOINs
#[test]
fn it_parses_multiple_joins() {
    let query = "SELECT u.name, o.id, p.name FROM users u
                 JOIN orders o ON u.id = o.user_id
                 JOIN order_items oi ON o.id = oi.order_id
                 JOIN products p ON oi.product_id = p.id";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    let mut raw_tables = raw_result.tables();
    raw_tables.sort();
    assert_eq!(raw_tables, vec!["order_items", "orders", "products", "users"]);
}

/// Test JOIN with USING clause
#[test]
fn it_parses_join_using() {
    let query = "SELECT * FROM users u JOIN user_profiles p USING (user_id)";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test LATERAL JOIN
#[test]
fn it_parses_lateral_join() {
    let query = "SELECT * FROM users u, LATERAL (SELECT * FROM orders o WHERE o.user_id = u.id LIMIT 3) AS recent_orders";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    let mut raw_tables = raw_result.tables();
    raw_tables.sort();
    assert_eq!(raw_tables, vec!["orders", "users"]);
}

// ============================================================================
// Advanced subquery tests
// ============================================================================

/// Test correlated subquery
#[test]
fn it_parses_correlated_subquery() {
    let query = "SELECT * FROM users u WHERE EXISTS (SELECT 1 FROM orders o WHERE o.user_id = u.id)";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);

    let mut raw_tables = raw_result.tables();
    raw_tables.sort();
    assert_eq!(raw_tables, vec!["orders", "users"]);
}

/// Test NOT EXISTS subquery
#[test]
fn it_parses_not_exists_subquery() {
    let query = "SELECT * FROM users u WHERE NOT EXISTS (SELECT 1 FROM banned b WHERE b.user_id = u.id)";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test scalar subquery in SELECT
#[test]
fn it_parses_scalar_subquery() {
    let query = "SELECT u.name, (SELECT COUNT(*) FROM orders o WHERE o.user_id = u.id) AS order_count FROM users u";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test subquery in FROM clause
#[test]
fn it_parses_derived_table() {
    let query = "SELECT * FROM (SELECT id, name FROM users WHERE active = true) AS active_users";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test ANY/SOME subquery
#[test]
fn it_parses_any_subquery() {
    let query = "SELECT * FROM products WHERE price > ANY (SELECT avg_price FROM categories)";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test ALL subquery
#[test]
fn it_parses_all_subquery() {
    let query = "SELECT * FROM products WHERE price > ALL (SELECT price FROM discounted_products)";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

// ============================================================================
// Window function tests
// ============================================================================

/// Test basic window function
#[test]
fn it_parses_window_function() {
    let query = "SELECT name, salary, ROW_NUMBER() OVER (ORDER BY salary DESC) AS rank FROM employees";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test window function with PARTITION BY
#[test]
fn it_parses_window_function_partition() {
    let query = "SELECT department, name, salary, RANK() OVER (PARTITION BY department ORDER BY salary DESC) AS dept_rank FROM employees";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test window function with frame clause
#[test]
fn it_parses_window_function_frame() {
    let query = "SELECT date, amount, SUM(amount) OVER (ORDER BY date ROWS BETWEEN 2 PRECEDING AND CURRENT ROW) AS moving_sum FROM transactions";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test named window
#[test]
fn it_parses_named_window() {
    let query = "SELECT name, salary, SUM(salary) OVER w, AVG(salary) OVER w FROM employees WINDOW w AS (PARTITION BY department ORDER BY salary)";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test LAG and LEAD functions
#[test]
fn it_parses_lag_lead() {
    let query = "SELECT date, price, LAG(price, 1) OVER (ORDER BY date) AS prev_price, LEAD(price, 1) OVER (ORDER BY date) AS next_price FROM stock_prices";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

// ============================================================================
// CTE variations
// ============================================================================

/// Test multiple CTEs
#[test]
fn it_parses_multiple_ctes() {
    let query = "WITH
        active_users AS (SELECT * FROM users WHERE active = true),
        premium_users AS (SELECT * FROM active_users WHERE plan = 'premium')
        SELECT * FROM premium_users";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
    assert!(raw_result.cte_names.contains(&"active_users".to_string()));
    assert!(raw_result.cte_names.contains(&"premium_users".to_string()));
}

/// Test recursive CTE
#[test]
fn it_parses_recursive_cte() {
    let query = "WITH RECURSIVE subordinates AS (
        SELECT id, name, manager_id FROM employees WHERE id = 1
        UNION ALL
        SELECT e.id, e.name, e.manager_id FROM employees e INNER JOIN subordinates s ON e.manager_id = s.id
    ) SELECT * FROM subordinates";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test CTE with column list
#[test]
fn it_parses_cte_with_columns() {
    let query = "WITH regional_sales(region, total) AS (SELECT region, SUM(amount) FROM orders GROUP BY region) SELECT * FROM regional_sales";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test CTE with MATERIALIZED
#[test]
fn it_parses_cte_materialized() {
    let query = "WITH t AS MATERIALIZED (SELECT * FROM large_table WHERE x > 100) SELECT * FROM t";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

// ============================================================================
// Set operations
// ============================================================================

/// Test INTERSECT
#[test]
fn it_parses_intersect() {
    let query = "SELECT id FROM users INTERSECT SELECT user_id FROM orders";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test EXCEPT
#[test]
fn it_parses_except() {
    let query = "SELECT id FROM users EXCEPT SELECT user_id FROM banned_users";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test UNION ALL
#[test]
fn it_parses_union_all() {
    let query = "SELECT name FROM users UNION ALL SELECT name FROM admins";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test compound set operations
#[test]
fn it_parses_compound_set_operations() {
    let query = "(SELECT id FROM a UNION SELECT id FROM b) INTERSECT SELECT id FROM c";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

// ============================================================================
// GROUP BY variations
// ============================================================================

/// Test GROUP BY ROLLUP
#[test]
fn it_parses_group_by_rollup() {
    let query = "SELECT region, product, SUM(sales) FROM sales_data GROUP BY ROLLUP(region, product)";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test GROUP BY CUBE
#[test]
fn it_parses_group_by_cube() {
    let query = "SELECT region, product, SUM(sales) FROM sales_data GROUP BY CUBE(region, product)";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test GROUP BY GROUPING SETS
#[test]
fn it_parses_grouping_sets() {
    let query = "SELECT region, product, SUM(sales) FROM sales_data GROUP BY GROUPING SETS ((region), (product), ())";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

// ============================================================================
// DISTINCT and ORDER BY variations
// ============================================================================

/// Test DISTINCT ON
#[test]
fn it_parses_distinct_on() {
    let query = "SELECT DISTINCT ON (user_id) * FROM orders ORDER BY user_id, created_at DESC";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test ORDER BY with NULLS FIRST/LAST
#[test]
fn it_parses_order_by_nulls() {
    let query = "SELECT * FROM users ORDER BY last_login DESC NULLS LAST";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test FETCH FIRST
#[test]
fn it_parses_fetch_first() {
    let query = "SELECT * FROM users ORDER BY id FETCH FIRST 10 ROWS ONLY";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test OFFSET with FETCH
#[test]
fn it_parses_offset_fetch() {
    let query = "SELECT * FROM users ORDER BY id OFFSET 20 ROWS FETCH NEXT 10 ROWS ONLY";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

// ============================================================================
// Locking clauses
// ============================================================================

/// Test FOR UPDATE
#[test]
fn it_parses_for_update() {
    let query = "SELECT * FROM users WHERE id = 1 FOR UPDATE";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test FOR SHARE
#[test]
fn it_parses_for_share() {
    let query = "SELECT * FROM users WHERE id = 1 FOR SHARE";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test FOR UPDATE NOWAIT
#[test]
fn it_parses_for_update_nowait() {
    let query = "SELECT * FROM users WHERE id = 1 FOR UPDATE NOWAIT";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test FOR UPDATE SKIP LOCKED
#[test]
fn it_parses_for_update_skip_locked() {
    let query = "SELECT * FROM jobs WHERE status = 'pending' LIMIT 1 FOR UPDATE SKIP LOCKED";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

// ============================================================================
// Expression tests
// ============================================================================

/// Test COALESCE
#[test]
fn it_parses_coalesce() {
    let query = "SELECT COALESCE(nickname, name, 'Unknown') FROM users";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test NULLIF
#[test]
fn it_parses_nullif() {
    let query = "SELECT NULLIF(status, 'deleted') FROM records";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test GREATEST and LEAST
#[test]
fn it_parses_greatest_least() {
    let query = "SELECT GREATEST(a, b, c), LEAST(x, y, z) FROM t";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test IS NULL and IS NOT NULL
#[test]
fn it_parses_null_tests() {
    let query = "SELECT * FROM users WHERE deleted_at IS NULL AND email IS NOT NULL";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test IS DISTINCT FROM
#[test]
fn it_parses_is_distinct_from() {
    let query = "SELECT * FROM t WHERE a IS DISTINCT FROM b";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test BETWEEN
#[test]
fn it_parses_between() {
    let query = "SELECT * FROM events WHERE created_at BETWEEN '2023-01-01' AND '2023-12-31'";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test LIKE and ILIKE
#[test]
fn it_parses_like_ilike() {
    let query = "SELECT * FROM users WHERE name LIKE 'John%' OR email ILIKE '%@EXAMPLE.COM'";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test SIMILAR TO
#[test]
fn it_parses_similar_to() {
    let query = "SELECT * FROM products WHERE name SIMILAR TO '%(phone|tablet)%'";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test complex boolean expressions
#[test]
fn it_parses_complex_boolean() {
    let query = "SELECT * FROM users WHERE (active = true AND verified = true) OR (role = 'admin' AND NOT suspended)";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

// ============================================================================
// Type cast tests
// ============================================================================

/// Test PostgreSQL-style type cast
#[test]
fn it_parses_pg_type_cast() {
    let query = "SELECT '123'::integer, '2023-01-01'::date, 'true'::boolean";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test SQL-style CAST
#[test]
fn it_parses_sql_cast() {
    let query = "SELECT CAST('123' AS integer), CAST(created_at AS date) FROM t";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test array type cast
#[test]
fn it_parses_array_cast() {
    let query = "SELECT ARRAY[1, 2, 3]::text[]";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

// ============================================================================
// Array and JSON tests
// ============================================================================

/// Test array constructor
#[test]
fn it_parses_array_constructor() {
    let query = "SELECT ARRAY[1, 2, 3], ARRAY['a', 'b', 'c']";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test array subscript
#[test]
fn it_parses_array_subscript() {
    let query = "SELECT tags[1], matrix[1][2] FROM t";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test array slice
#[test]
fn it_parses_array_slice() {
    let query = "SELECT arr[2:4], arr[:3], arr[2:] FROM t";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test unnest
#[test]
fn it_parses_unnest() {
    let query = "SELECT unnest(ARRAY[1, 2, 3])";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test JSON operators
#[test]
fn it_parses_json_operators() {
    let query = "SELECT data->'name', data->>'email', data#>'{address,city}' FROM users";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test JSONB containment
#[test]
fn it_parses_jsonb_containment() {
    let query = "SELECT * FROM products WHERE metadata @> '{\"featured\": true}'";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

// ============================================================================
// DDL statements
// ============================================================================

/// Test CREATE TABLE with constraints
#[test]
fn it_parses_create_table_with_constraints() {
    let query = "CREATE TABLE orders (
        id SERIAL PRIMARY KEY,
        user_id INTEGER NOT NULL REFERENCES users(id),
        amount DECIMAL(10, 2) CHECK (amount > 0),
        status TEXT DEFAULT 'pending',
        created_at TIMESTAMP DEFAULT NOW(),
        UNIQUE (user_id, created_at)
    )";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test CREATE TABLE AS
#[test]
fn it_parses_create_table_as() {
    let query = "CREATE TABLE active_users AS SELECT * FROM users WHERE active = true";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test CREATE VIEW
#[test]
fn it_parses_create_view() {
    let query = "CREATE VIEW active_users AS SELECT id, name FROM users WHERE active = true";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test CREATE MATERIALIZED VIEW
#[test]
fn it_parses_create_materialized_view() {
    let query = "CREATE MATERIALIZED VIEW monthly_sales AS SELECT date_trunc('month', created_at) AS month, SUM(amount) FROM orders GROUP BY 1";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test ALTER TABLE ADD COLUMN
#[test]
fn it_parses_alter_table_add_column() {
    let query = "ALTER TABLE users ADD COLUMN email TEXT NOT NULL";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test ALTER TABLE DROP COLUMN
#[test]
fn it_parses_alter_table_drop_column() {
    let query = "ALTER TABLE users DROP COLUMN deprecated_field";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test ALTER TABLE ADD CONSTRAINT
#[test]
fn it_parses_alter_table_add_constraint() {
    let query = "ALTER TABLE orders ADD CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test CREATE INDEX with expression
#[test]
fn it_parses_create_index_expression() {
    let query = "CREATE INDEX idx_lower_email ON users (lower(email))";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test CREATE UNIQUE INDEX with WHERE
#[test]
fn it_parses_partial_unique_index() {
    let query = "CREATE UNIQUE INDEX idx_active_email ON users (email) WHERE active = true";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test CREATE INDEX CONCURRENTLY
#[test]
fn it_parses_create_index_concurrently() {
    let query = "CREATE INDEX CONCURRENTLY idx_name ON users (name)";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test TRUNCATE
#[test]
fn it_parses_truncate() {
    let query = "TRUNCATE TABLE logs, audit_logs RESTART IDENTITY CASCADE";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

// ============================================================================
// Transaction and utility statements
// ============================================================================

/// Test EXPLAIN
#[test]
fn it_parses_explain() {
    let query = "EXPLAIN SELECT * FROM users WHERE id = 1";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test EXPLAIN ANALYZE
#[test]
fn it_parses_explain_analyze() {
    let query = "EXPLAIN (ANALYZE, BUFFERS, FORMAT JSON) SELECT * FROM users";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test COPY
#[test]
fn it_parses_copy() {
    let query = "COPY users (id, name, email) FROM STDIN WITH (FORMAT csv, HEADER true)";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test PREPARE
#[test]
fn it_parses_prepare() {
    let query = "PREPARE user_by_id (int) AS SELECT * FROM users WHERE id = $1";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test EXECUTE
#[test]
fn it_parses_execute() {
    let query = "EXECUTE user_by_id(42)";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test DEALLOCATE
#[test]
fn it_parses_deallocate() {
    let query = "DEALLOCATE user_by_id";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

// ============================================================================
// Parameter placeholder tests
// ============================================================================

/// Test positional parameters
#[test]
fn it_parses_positional_params() {
    let query = "SELECT * FROM users WHERE id = $1 AND status = $2";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test parameters in INSERT
#[test]
fn it_parses_params_in_insert() {
    let query = "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

// ============================================================================
// Complex real-world queries
// ============================================================================

/// Test analytics query with window functions
#[test]
fn it_parses_analytics_query() {
    let query = "
        SELECT
            date_trunc('day', created_at) AS day,
            COUNT(*) AS daily_orders,
            SUM(amount) AS daily_revenue,
            AVG(amount) OVER (ORDER BY date_trunc('day', created_at) ROWS BETWEEN 6 PRECEDING AND CURRENT ROW) AS weekly_avg
        FROM orders
        WHERE created_at >= NOW() - INTERVAL '30 days'
        GROUP BY date_trunc('day', created_at)
        ORDER BY day";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test hierarchical query with recursive CTE
#[test]
fn it_parses_hierarchy_query() {
    let query = "
        WITH RECURSIVE category_tree AS (
            SELECT id, name, parent_id, 0 AS level, ARRAY[id] AS path
            FROM categories
            WHERE parent_id IS NULL
            UNION ALL
            SELECT c.id, c.name, c.parent_id, ct.level + 1, ct.path || c.id
            FROM categories c
            JOIN category_tree ct ON c.parent_id = ct.id
        )
        SELECT * FROM category_tree ORDER BY path";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test complex report query
#[test]
fn it_parses_complex_report_query() {
    let query = "
        WITH monthly_data AS (
            SELECT
                date_trunc('month', o.created_at) AS month,
                u.region,
                p.category,
                SUM(oi.quantity * oi.unit_price) AS revenue,
                COUNT(DISTINCT o.id) AS order_count,
                COUNT(DISTINCT o.user_id) AS customer_count
            FROM orders o
            JOIN users u ON o.user_id = u.id
            JOIN order_items oi ON o.id = oi.order_id
            JOIN products p ON oi.product_id = p.id
            WHERE o.created_at >= '2023-01-01' AND o.status = 'completed'
            GROUP BY 1, 2, 3
        )
        SELECT
            month,
            region,
            category,
            revenue,
            order_count,
            customer_count,
            revenue / NULLIF(order_count, 0) AS avg_order_value,
            SUM(revenue) OVER (PARTITION BY region ORDER BY month) AS cumulative_revenue
        FROM monthly_data
        ORDER BY month DESC, region, revenue DESC";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test query with multiple subqueries and CTEs
#[test]
fn it_parses_mixed_subqueries_and_ctes() {
    let query = "
        WITH high_value_customers AS (
            SELECT user_id FROM orders GROUP BY user_id HAVING SUM(amount) > 1000
        )
        SELECT u.*,
            (SELECT COUNT(*) FROM orders o WHERE o.user_id = u.id) AS total_orders,
            (SELECT MAX(created_at) FROM orders o WHERE o.user_id = u.id) AS last_order
        FROM users u
        WHERE u.id IN (SELECT user_id FROM high_value_customers)
            AND EXISTS (SELECT 1 FROM orders o WHERE o.user_id = u.id AND o.created_at > NOW() - INTERVAL '90 days')
        ORDER BY (SELECT SUM(amount) FROM orders o WHERE o.user_id = u.id) DESC
        LIMIT 100";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

// ============================================================================
// Complex INSERT tests
// ============================================================================

/// Test INSERT with multiple tuples
#[test]
fn it_parses_insert_multiple_rows() {
    let query = "INSERT INTO users (name, email, age) VALUES ('Alice', 'alice@example.com', 25), ('Bob', 'bob@example.com', 30), ('Charlie', 'charlie@example.com', 35)";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test INSERT ... SELECT
#[test]
fn it_parses_insert_select() {
    let query = "INSERT INTO archived_users (id, name, email) SELECT id, name, email FROM users WHERE deleted_at IS NOT NULL";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test INSERT ... SELECT with complex query
#[test]
fn it_parses_insert_select_complex() {
    let query = "INSERT INTO monthly_stats (month, user_count, order_count, total_revenue)
        SELECT date_trunc('month', created_at) AS month,
               COUNT(DISTINCT user_id),
               COUNT(*),
               SUM(amount)
        FROM orders
        WHERE created_at >= '2023-01-01'
        GROUP BY date_trunc('month', created_at)";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test INSERT with CTE
#[test]
fn it_parses_insert_with_cte() {
    let query = "WITH new_data AS (
        SELECT name, email FROM temp_imports WHERE valid = true
    )
    INSERT INTO users (name, email) SELECT name, email FROM new_data";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test INSERT with DEFAULT values
#[test]
fn it_parses_insert_default_values() {
    let query = "INSERT INTO users (name, created_at) VALUES ('test', DEFAULT)";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test INSERT with ON CONFLICT DO NOTHING
#[test]
fn it_parses_insert_on_conflict_do_nothing() {
    let query = "INSERT INTO users (id, name) VALUES (1, 'test') ON CONFLICT (id) DO NOTHING";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test INSERT with ON CONFLICT with WHERE clause
#[test]
fn it_parses_insert_on_conflict_with_where() {
    let query = "INSERT INTO users (id, name, updated_at) VALUES (1, 'test', NOW())
        ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name, updated_at = EXCLUDED.updated_at
        WHERE users.updated_at < EXCLUDED.updated_at";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test INSERT with multiple columns in ON CONFLICT
#[test]
fn it_parses_insert_on_conflict_multiple_columns() {
    let query = "INSERT INTO user_settings (user_id, key, value) VALUES (1, 'theme', 'dark')
        ON CONFLICT (user_id, key) DO UPDATE SET value = EXCLUDED.value";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test INSERT with RETURNING multiple columns
#[test]
fn it_parses_insert_returning_multiple() {
    let query = "INSERT INTO users (name, email) VALUES ('test', 'test@example.com') RETURNING id, created_at, name";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test INSERT with subquery in VALUES
#[test]
fn it_parses_insert_with_subquery_value() {
    let query = "INSERT INTO orders (user_id, total) VALUES ((SELECT id FROM users WHERE email = 'test@example.com'), 100.00)";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test INSERT with OVERRIDING
#[test]
fn it_parses_insert_overriding() {
    let query = "INSERT INTO users (id, name) OVERRIDING SYSTEM VALUE VALUES (1, 'test')";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

// ============================================================================
// Complex UPDATE tests
// ============================================================================

/// Test UPDATE with multiple columns
#[test]
fn it_parses_update_multiple_columns() {
    let query = "UPDATE users SET name = 'new_name', email = 'new@example.com', updated_at = NOW() WHERE id = 1";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test UPDATE with subquery in SET
#[test]
fn it_parses_update_with_subquery_set() {
    let query = "UPDATE orders SET total = (SELECT SUM(price * quantity) FROM order_items WHERE order_id = orders.id) WHERE id = 1";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test UPDATE with FROM clause (PostgreSQL-specific JOIN update)
#[test]
fn it_parses_update_from() {
    let query = "UPDATE orders o SET status = 'shipped', shipped_at = NOW()
        FROM shipments s
        WHERE o.id = s.order_id AND s.status = 'delivered'";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test UPDATE with FROM and multiple tables
#[test]
fn it_parses_update_from_multiple_tables() {
    let query = "UPDATE products p SET price = p.price * (1 + d.percentage / 100)
        FROM discounts d
        JOIN categories c ON d.category_id = c.id
        WHERE p.category_id = c.id AND d.active = true";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test UPDATE with CTE
#[test]
fn it_parses_update_with_cte() {
    let query = "WITH inactive_users AS (
        SELECT id FROM users WHERE last_login < NOW() - INTERVAL '1 year'
    )
    UPDATE users SET status = 'inactive' WHERE id IN (SELECT id FROM inactive_users)";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test UPDATE with RETURNING
#[test]
fn it_parses_update_returning() {
    let query = "UPDATE users SET name = 'updated' WHERE id = 1 RETURNING id, name, updated_at";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test UPDATE with complex WHERE clause
#[test]
fn it_parses_update_complex_where() {
    let query = "UPDATE orders SET status = 'cancelled'
        WHERE created_at < NOW() - INTERVAL '30 days'
        AND status = 'pending'
        AND NOT EXISTS (SELECT 1 FROM payments WHERE payments.order_id = orders.id)";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test UPDATE with row value comparison
#[test]
fn it_parses_update_row_comparison() {
    let query = "UPDATE users SET (name, email) = ('new_name', 'new@example.com') WHERE id = 1";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test UPDATE with CASE expression
#[test]
fn it_parses_update_with_case() {
    let query = "UPDATE products SET price = CASE
        WHEN category = 'electronics' THEN price * 0.9
        WHEN category = 'clothing' THEN price * 0.8
        ELSE price * 0.95
        END
        WHERE sale_active = true";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test UPDATE with array operations
#[test]
fn it_parses_update_array() {
    let query = "UPDATE users SET tags = array_append(tags, 'verified') WHERE id = 1";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

// ============================================================================
// Complex DELETE tests
// ============================================================================

/// Test DELETE with subquery in WHERE
#[test]
fn it_parses_delete_with_subquery() {
    let query = "DELETE FROM orders WHERE user_id IN (SELECT id FROM users WHERE status = 'deleted')";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test DELETE with USING clause (PostgreSQL-specific JOIN delete)
#[test]
fn it_parses_delete_using() {
    let query = "DELETE FROM order_items oi USING orders o
        WHERE oi.order_id = o.id AND o.status = 'cancelled'";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test DELETE with USING and multiple tables
#[test]
fn it_parses_delete_using_multiple_tables() {
    let query = "DELETE FROM notifications n
        USING users u, user_settings s
        WHERE n.user_id = u.id
        AND u.id = s.user_id
        AND s.key = 'email_notifications'
        AND s.value = 'false'";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test DELETE with CTE
#[test]
fn it_parses_delete_with_cte() {
    let query = "WITH old_orders AS (
        SELECT id FROM orders WHERE created_at < NOW() - INTERVAL '5 years'
    )
    DELETE FROM order_items WHERE order_id IN (SELECT id FROM old_orders)";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test DELETE with RETURNING
#[test]
fn it_parses_delete_returning() {
    let query = "DELETE FROM users WHERE id = 1 RETURNING id, name, email";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test DELETE with EXISTS
#[test]
fn it_parses_delete_with_exists() {
    let query = "DELETE FROM products p
        WHERE NOT EXISTS (SELECT 1 FROM order_items oi WHERE oi.product_id = p.id)
        AND p.created_at < NOW() - INTERVAL '1 year'";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test DELETE with complex boolean conditions
#[test]
fn it_parses_delete_complex_conditions() {
    let query = "DELETE FROM logs
        WHERE (level = 'debug' AND created_at < NOW() - INTERVAL '7 days')
        OR (level = 'info' AND created_at < NOW() - INTERVAL '30 days')
        OR (level IN ('warning', 'error') AND created_at < NOW() - INTERVAL '90 days')";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test DELETE with LIMIT (PostgreSQL extension)
#[test]
fn it_parses_delete_only() {
    let query = "DELETE FROM ONLY parent_table WHERE id = 1";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

// ============================================================================
// Combined DML with CTEs
// ============================================================================

/// Test data modification CTE (INSERT in CTE)
#[test]
fn it_parses_insert_cte_returning() {
    let query = "WITH inserted AS (
        INSERT INTO users (name, email) VALUES ('test', 'test@example.com') RETURNING id, name
    )
    SELECT * FROM inserted";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test UPDATE in CTE with final SELECT
#[test]
fn it_parses_update_cte_returning() {
    let query = "WITH updated AS (
        UPDATE users SET last_login = NOW() WHERE id = 1 RETURNING id, name, last_login
    )
    SELECT * FROM updated";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test DELETE in CTE with final SELECT
#[test]
fn it_parses_delete_cte_returning() {
    let query = "WITH deleted AS (
        DELETE FROM expired_sessions WHERE expires_at < NOW() RETURNING user_id
    )
    SELECT COUNT(*) FROM deleted";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}

/// Test chained CTEs with multiple DML operations
#[test]
fn it_parses_chained_dml_ctes() {
    let query = "WITH
        to_archive AS (
            SELECT id FROM users WHERE last_login < NOW() - INTERVAL '2 years'
        ),
        archived AS (
            INSERT INTO archived_users SELECT * FROM users WHERE id IN (SELECT id FROM to_archive) RETURNING id
        ),
        deleted AS (
            DELETE FROM users WHERE id IN (SELECT id FROM archived) RETURNING id
        )
        SELECT COUNT(*) as archived_count FROM deleted";
    let raw_result = parse_raw(query).unwrap();
    let proto_result = parse(query).unwrap();

    assert_eq!(raw_result.protobuf, proto_result.protobuf);
}
