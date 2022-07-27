//! Rust pg_query &emsp; [![Build Status]][actions] [![Latest Version]][crates.io] [![Docs Badge]][docs]
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
//! pg_query = "0.7"
//! ```
//!
//! # Example: Parsing a query
//!
//! ```rust
//! use pg_query::NodeRef;
//!
//! let result = pg_query::parse("SELECT * FROM contacts");
//! assert!(result.is_ok());
//! let result = result.unwrap();
//! assert_eq!(result.tables(), vec!["contacts"]);
//! assert!(matches!(result.protobuf.nodes()[0].0, NodeRef::SelectStmt(_)));
//! ```
//!

mod bindings;
mod error;
mod node_enum;
mod node_mut;
mod node_ref;
mod node_structs;
mod parse_result;
mod query;
mod truncate;

pub use error::*;
pub use node_enum::*;
pub use node_mut::*;
pub use node_ref::*;
pub use node_structs::*;
pub use query::*;
pub use truncate::*;

pub mod protobuf {
    include!(concat!(env!("OUT_DIR"), "/pg_query.rs"));
}

pub use protobuf::Node;

// From Postgres source: src/include/storage/lockdefs.h
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LockMode {
    NoLock = 0,                   // NoLock is not a lock mode, but a flag value meaning "don't get a lock"
    AccessShareLock = 1,          // SELECT
    RowShareLock = 2,             // SELECT FOR UPDATE/FOR SHARE
    RowExclusiveLock = 3,         // INSERT, UPDATE, DELETE
    ShareUpdateExclusiveLock = 4, // VACUUM (non-FULL), ANALYZE, CREATE INDEX CONCURRENTLY
    ShareLock = 5,                // CREATE INDEX (WITHOUT CONCURRENTLY)
    ShareRowExclusiveLock = 6,    // like EXCLUSIVE MODE, but allows ROW SHARE
    ExclusiveLock = 7,            // blocks ROW SHARE/SELECT...FOR UPDATE
    AccessExclusiveLock = 8,      // ALTER TABLE, DROP TABLE, VACUUM FULL, and unqualified LOCK TABLE
}

// From Postgres source: src/include/catalog/pg_trigger.h
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TriggerType {
    Row = 1,
    Before = 2,
    Insert = 4,
    Delete = 8,
    Update = 16,
    Truncate = 32,
    Instead = 64,
}
