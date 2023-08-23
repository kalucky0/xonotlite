#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TokenType {
    Exit,
    IntLiteral,
    Semicolon,
    OpenParen,
    CloseParen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<String>,
}

#[macro_export]
macro_rules! token {
    ($token_type:ident) => {
        Token {
            token_type: TokenType::$token_type,
            value: None,
        }
    };
    ($token_type:ident, $value:expr) => {
        Token {
            token_type: TokenType::$token_type,
            value: Some($value),
        }
    };
}
