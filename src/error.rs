#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    ParseError(String),
}

pub type Result<T> = core::result::Result<T, Error>;
