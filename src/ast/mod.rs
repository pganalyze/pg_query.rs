//! Native Rust AST types for PostgreSQL parse trees.
//!
//! This module provides ergonomic Rust types that wrap the PostgreSQL parse tree
//! structure. These types make it easier to work with parsed SQL queries without
//! the complexity of deeply nested protobuf Option<Box<Node>> wrappers.
//!
//! # Example
//!
//! ```rust
//! use pg_query::ast::Node;
//!
//! let result = pg_query::parse_to_ast("SELECT * FROM users WHERE id = 1").unwrap();
//! for stmt in &result.stmts {
//!     match &stmt.stmt {
//!         Node::SelectStmt(select) => {
//!             // Access fields more directly
//!             for table in &select.from_clause {
//!                 if let Node::RangeVar(rv) = table {
//!                     println!("Table: {}", rv.relname);
//!                 }
//!             }
//!         }
//!         _ => {}
//!     }
//! }
//! ```

mod nodes;
mod convert;

pub use nodes::*;
