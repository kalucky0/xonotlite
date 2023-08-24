use crate::Token;

#[derive(Debug)]
pub enum NodeExpr {
    IntLiteral(Token),
    Ident(Token),
}

#[derive(Debug)]
pub enum NodeStmt {
    Exit(NodeExpr),
    Let {
        ident: Token,
        expr: NodeExpr,
    },
}

#[derive(Debug)]
pub struct NodeProg {
    pub stmts: Vec<NodeStmt>,
}
