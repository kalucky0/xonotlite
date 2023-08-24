use crate::TokenType;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unknown identifier: `{0}`")]
    UnknownIdentifier(String),
    #[error("Unexpected token `{0:?}`")]
    UnexpectedToken(TokenType),
    #[error("Unexpected token `{0:?}`. Expected `{1}`")]
    UnexpectedTokenExpected(TokenType, &'static str),
    #[error("Expected `{0}`. Found none")]
    ExpectedFoundNone(&'static str),
    #[error("Unexpected node `{0}`")]
    UnexpectedNode(&'static str),
    #[error("Expected value for token `{0:?}`. Found none")]
    ExpectedValue(TokenType),
    #[error("Redeclaration of `{0}`")]
    Redeclaration(String),
    #[error("No exit node found")]
    NoExitNode,
}

#[macro_export]
macro_rules! exit {
    ($($arg:tt)*) => ({
        color_print::cprintln!("<s><r>error</>:</> {}", $($arg)*);
        std::process::exit(1);
    })
}
