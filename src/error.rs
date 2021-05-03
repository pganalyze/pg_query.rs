#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    ParseError(String),
    InvalidAst(String),
    InvalidJson(String),
}

pub type Result<T> = core::result::Result<T, Error>;
