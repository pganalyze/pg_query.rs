/// Error structure representing the basic error scenarios for `pg_query`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    ParseError(String),
    InvalidAst(String),
    InvalidJson(String),
}

/// Convenient Result alias for returning `pg_query::Error`.
pub type Result<T> = core::result::Result<T, Error>;
