use crate::ast::ASTNode;
use crate::tokens::{Instruction, Token};

pub struct Compiler {
    pub bytecode: Vec<Instruction>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            bytecode: Vec::new(),
        }
    }

    fn get_var_index(&self, name: &str) -> usize {
        let ch = name.chars().next().unwrap();
        (ch as usize) - ('a' as usize)
    }

    pub fn compile(&mut self, node: &ASTNode) {
        match node {
            ASTNode::Program(statements) => {
                let len = statements.len();
                for (i, stmt) in statements.iter().enumerate() {
                    self.compile(stmt);

                    if !matches!(stmt, ASTNode::Assignment { .. }) && i < len - 1 {
                        self.bytecode.push(Instruction::Pop);
                    }
                }
            }
            ASTNode::Number(val) => {
                self.bytecode.push(Instruction::Push(*val));
            }
            ASTNode::Variable(name) => {
                let idx = self.get_var_index(name);
                self.bytecode.push(Instruction::Load(idx));
            }
            ASTNode::Assignment { name, value } => {
                self.compile(value);
                let idx = self.get_var_index(name);
                self.bytecode.push(Instruction::Store(idx));
            }
            ASTNode::BinaryOp { left, op, right } => {
                self.compile(left);
                self.compile(right);
                match op {
                    Token::Plus => self.bytecode.push(Instruction::Add),
                    Token::Minus => self.bytecode.push(Instruction::Subtract),
                    _ => unreachable!(),
                }
            }
        }
    }
}
