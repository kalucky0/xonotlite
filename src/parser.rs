use crate::{errors::Error, NodeExpr, NodeProg, NodeStmt, Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, cursor: 0 }
    }

    pub fn parse(&mut self) -> Result<NodeProg, Error> {
        let mut stmts = vec![];

        while self.peek().is_some() {
            stmts.push(self.parse_stmt()?);
        }

        Ok(NodeProg { stmts })
    }

    fn parse_expr(&mut self) -> Result<NodeExpr, Error> {
        let token = match self.next() {
            Some(token) => token,
            None => return Err(Error::ExpectedFoundNone("IntLiteral")),
        };

        match token.token_type {
            TokenType::IntLiteral => Ok(NodeExpr::IntLiteral(token.clone())),
            TokenType::Ident => Ok(NodeExpr::Ident(token.clone())),
            _ => Err(Error::UnexpectedTokenExpected(
                token.token_type,
                "IntLiteral",
            )),
        }
    }

    fn parse_stmt(&mut self) -> Result<NodeStmt, Error> {
        let token = match self.next() {
            Some(token) => token,
            None => return Err(Error::ExpectedFoundNone("Stmt")),
        };
        if token.token_type == TokenType::Exit {
            self.expect(TokenType::OpenParen, "(")?;
            let exit_node = match self.parse_expr() {
                Ok(expr) => NodeStmt::Exit(expr),
                Err(e) => return Err(e),
            };
            self.expect(TokenType::CloseParen, ")")?;
            self.expect(TokenType::Semicolon, ";")?;
            return Ok(exit_node);
        }
        if token.token_type == TokenType::Let {
            let ident = match self.parse_ident() {
                Ok(ident) => ident,
                Err(e) => return Err(e),
            };
            self.expect(TokenType::Eq, "=")?;
            let expr = match self.parse_expr() {
                Ok(expr) => expr,
                Err(e) => return Err(e),
            };
            self.expect(TokenType::Semicolon, ";")?;
            return Ok(NodeStmt::Let {
                ident,
                expr,
            });
        }
        Err(Error::UnexpectedTokenExpected(token.token_type, "Stmt"))
    }

    fn parse_ident(&mut self) -> Result<Token, Error> {
        let token = match self.next() {
            Some(token) => token,
            None => return Err(Error::ExpectedFoundNone("Ident")),
        };
        if token.token_type == TokenType::Ident {
            return Ok(token.clone());
        }
        Err(Error::UnexpectedTokenExpected(token.token_type, "Ident"))
    }

    fn expect(&mut self, token_type: TokenType, token_str: &'static str) -> Result<(), Error> {
        match self.next() {
            Some(token) => {
                if token.token_type != token_type {
                    return Err(Error::UnexpectedTokenExpected(token.token_type, token_str));
                }
                Ok(())
            }
            None => Err(Error::ExpectedFoundNone(token_str)),
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.cursor)
    }

    fn next(&mut self) -> Option<&Token> {
        self.cursor += 1;
        self.tokens.get(self.cursor - 1)
    }
}
