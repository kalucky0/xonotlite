use crate::{errors::Error, NodeExit, NodeExpr, Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, cursor: 0 }
    }

    pub fn parse(&mut self) -> Result<NodeExit, Error> {
        let mut exit_node: Option<NodeExit> = Option::None;

        while let Some(token) = self.next() {
            if token.token_type == TokenType::Exit {
                self.expect(TokenType::OpenParen, "(")?;
                exit_node = match self.parse_expr() {
                    Ok(expr) => Some(NodeExit { expr }),
                    Err(e) => return Err(e),
                };
                self.expect(TokenType::CloseParen, ")")?;
                self.expect(TokenType::Semicolon, ";")?;
            }
        }

        exit_node.ok_or(Error::NoExitNode)
    }

    fn parse_expr(&mut self) -> Result<NodeExpr, Error> {
        let token = match self.next() {
            Some(token) => token,
            None => return Err(Error::UnexpectedTokenNone("IntLiteral")),
        };

        match token.token_type {
            TokenType::IntLiteral => Ok(NodeExpr {
                int_literal: token.clone(),
            }),
            _ => Err(Error::UnexpectedToken(token.token_type, "IntLiteral")),
        }
    }

    fn expect(&mut self, token_type: TokenType, token_str: &'static str) -> Result<(), Error> {
        match self.next() {
            Some(token) => {
                if token.token_type != token_type {
                    return Err(Error::UnexpectedToken(token.token_type, token_str));
                }
                Ok(())
            }
            None => Err(Error::UnexpectedTokenNone(token_str)),
        }
    }

    fn _peek(&self) -> Option<&Token> {
        self.tokens.get(self.cursor + 1)
    }

    fn next(&mut self) -> Option<&Token> {
        self.cursor += 1;
        self.tokens.get(self.cursor - 1)
    }
}
