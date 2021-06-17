use thiserror::Error;

#[derive(Error, Debug)]
pub enum PDBError {
    #[error("parse error: {0}")]
    ParseError(String),

    #[error("Invalid value:\n\t{0}")]
    InvalidValue(String),

}