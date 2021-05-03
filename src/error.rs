#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    ParseError(String),
    NormalizeError(String),
    InvalidAst(String),
}

pub type Result<T> = core::result::Result<T, Error>;
