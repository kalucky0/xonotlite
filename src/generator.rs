use crate::{errors::Error, NodeExpr, NodeProg, NodeStmt};
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
};

#[derive(Debug, Clone)]
struct Var {
    loc: isize,
}

pub struct Generator {
    prog: NodeProg,
    stack_size: Cell<isize>,
    vars: RefCell<HashMap<String, Var>>,
}

impl Generator {
    pub fn new(prog: NodeProg) -> Self {
        Generator {
            prog,
            stack_size: Cell::new(0),
            vars: RefCell::new(HashMap::new()),
        }
    }

    pub fn generate(&self) -> Result<String, Error> {
        let mut asm = String::new();
        asm.push_str("global _main\n_main:\n");

        for stmt in &self.prog.stmts {
            asm.push_str(&self.gen_stmt(stmt)?);
        }

        asm.push_str("  mov rax, 0x02000001\n");
        asm.push_str("  mov rdi, 0");
        asm.push_str("\n  syscall\n");
        Ok(asm)
    }

    fn gen_stmt(&self, stmt: &NodeStmt) -> Result<String, Error> {
        match stmt {
            NodeStmt::Let { ident, expr } => {
                let ident = match ident.value {
                    Some(ref value) => value,
                    None => return Err(Error::ExpectedValue(ident.token_type)),
                };
                if self.var_exists(ident) {
                    return Err(Error::Redeclaration(ident.to_string()));
                }
                self.insert_var(ident);
                self.gen_expr(expr)
            }
            NodeStmt::Exit(expr) => {
                let mut asm = String::new();
                asm.push_str(&self.gen_expr(expr)?);
                asm.push_str("  mov rax, 0x02000001\n");
                asm.push_str(&self.pop("rdi"));
                asm.push_str("  syscall\n");
                Ok(asm)
            }
        }
    }

    fn gen_expr(&self, expr: &NodeExpr) -> Result<String, Error> {
        match expr {
            NodeExpr::IntLiteral(token) => {
                let mut asm = String::new();
                asm.push_str("  mov rax, ");
                match token.value {
                    Some(ref value) => asm.push_str(value),
                    None => asm.push('0'),
                }
                asm.push('\n');
                asm.push_str(&self.push("rax"));
                Ok(asm)
            }
            NodeExpr::Ident(token) => {
                let ident = match token.value {
                    Some(ref value) => value,
                    None => return Err(Error::ExpectedValue(token.token_type)),
                };
                let vars = self.vars.borrow();
                let var = match vars.get(ident) {
                    Some(var) => var,
                    None => return Err(Error::UnknownIdentifier(ident.to_string())),
                };
                let mut offset = String::new();
                offset.push_str("QWORD [rsp + ");
                let loc = self.stack_size.get() - var.loc - 8;
                offset.push_str(&loc.to_string());
                offset.push(']');
                Ok(self.push(&offset))
            }
        }
    }

    fn push(&self, reg: &str) -> String {
        let mut asm = String::new();
        asm.push_str("  push ");
        asm.push_str(reg);
        asm.push('\n');
        let stack_size = self.stack_size.get();
        self.stack_size.set(stack_size + 8);
        asm
    }

    fn pop(&self, reg: &str) -> String {
        let mut asm = String::new();
        asm.push_str("  pop ");
        asm.push_str(reg);
        asm.push('\n');
        let stack_size = self.stack_size.get();
        self.stack_size.set(stack_size - 8);
        asm
    }

    fn insert_var(&self, ident: &str) {
        let mut vars = self.vars.borrow_mut();
        vars.insert(
            ident.to_string(),
            Var {
                loc: self.stack_size.get(),
            },
        );
    }

    fn var_exists(&self, ident: &str) -> bool {
        let vars = self.vars.borrow();
        vars.contains_key(ident)
    }
}
