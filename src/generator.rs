use crate::{errors::Error, NodeExit};

pub struct Generator {
    root: NodeExit,
}

impl Generator {
    pub fn new(root: NodeExit) -> Self {
        Generator { root }
    }

    pub fn generate(&mut self) -> Result<String, Error> {
        let mut asm = String::new();
        asm.push_str("global _main\n_main:\n");
        asm.push_str("  mov rax, 0x02000001\n");
        asm.push_str("  mov rdi, ");
        match self.root.expr.int_literal.value {
            Some(ref value) => {
                asm.push_str(value);
            }
            None => {
                asm.push_str("0");
            }
        }
        asm.push_str("\n  syscall\n");
        Ok(asm)
    }
}
