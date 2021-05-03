//! pg_query &emsp; [![Build Status]][actions] [![Latest Version]][crates.io] [![Docs Badge]][docs]
//! ===========
//!
//! [Build Status]: https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fpaupino%2Fpg_query%2Fbadge&label=build&logo=none
//! [actions]: https://actions-badge.atrox.dev/paupino/pg_query/goto
//! [Latest Version]: https://img.shields.io/crates/v/pg_query.svg
//! [crates.io]: https://crates.io/crates/pg_query
//! [Docs Badge]: https://docs.rs/pg_query/badge.svg
//! [docs]: https://docs.rs/pg_query
//!
//! PostgreSQL parser that uses the [actual PostgreSQL server source]((https://github.com/pganalyze/libpg_query)) to parse
//! SQL queries and return the internal PostgreSQL parse tree.
//!
//! Warning! This library is in early stages of development so any APIs exposed are subject to change.
//!
//! ## Getting started
//!
//! Add the following to your `Cargo.toml`
//!
//! ```toml
//! [dependencies]
//! pg_query = "0.1"
//! ```
//!
//! # Example: Parsing a query
//!
//! ```rust
//! use pg_query::ast::Node;
//!
//! let result = pg_query::parse("SELECT * FROM contacts");
//! assert!(result.is_ok());
//! let result = result.unwrap();
//! assert!(matches!(*&result[0], Node::SelectStmt(_)));
//! ```
//!

/// Generated structures representing the PostgreSQL AST.
pub mod ast;
mod bindings;
mod error;
mod query;

pub use error::*;
pub use query::*;
