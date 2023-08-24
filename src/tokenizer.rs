use crate::{errors::Error, token, Token, TokenType};
use std::{iter::Peekable, str::Chars};

pub struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl Tokenizer<'_> {
    pub fn new(source: &str) -> Tokenizer {
        Tokenizer {
            chars: source.chars().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = vec![];
        let mut buffer = String::new();

        while let Some(c) = self.next() {
            if c.is_alphabetic() {
                buffer.push(c);
                while let Some(&c) = self.peek() {
                    if !c.is_alphanumeric() {
                        break;
                    }
                    buffer.push(c);
                    self.next();
                }

                if buffer == "exit" {
                    tokens.push(token!(Exit));
                    buffer.clear();
                } else if buffer == "let" {
                    tokens.push(token!(Let));
                    buffer.clear();
                } else {
                    tokens.push(token!(Ident, buffer.clone()));
                    buffer.clear();
                }
            } else if c.is_numeric() {
                buffer.push(c);
                while let Some(&c) = self.peek() {
                    if !c.is_numeric() {
                        break;
                    }
                    buffer.push(c);
                    self.next();
                }
                tokens.push(token!(IntLiteral, buffer.clone()));
                buffer.clear();
            } else if c == ';' {
                tokens.push(token!(Semicolon));
            } else if c == '(' {
                tokens.push(token!(OpenParen));
            } else if c == ')' {
                tokens.push(token!(CloseParen));
            } else if c == '=' {
                tokens.push(token!(Eq));
            } else if c.is_whitespace() {
                continue;
            }
        }
        Ok(tokens)
    }

    fn next(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }
}
