use crate::TokenType;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unknown identifier: `{0}`")]
    UnknownIdentifier(String),
    #[error("Unexpected token `{0:?}`. Expected `{1}`")]
    UnexpectedToken(TokenType, &'static str),
    #[error("Expected `{0}`. Found none")]
    UnexpectedTokenNone(&'static str),
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
