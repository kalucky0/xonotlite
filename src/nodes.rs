use crate::Token;

pub struct NodeExpr {
    pub int_literal: Token,
}

pub struct NodeExit {
    pub expr: NodeExpr,
}