// Inside src/compiler.rs

use crate::ast::ASTNode;
use crate::tokens::{Instruction, Token};
use std::collections::HashMap; // 🌟 Need this for the Symbol Table!

pub struct Compiler {
    pub bytecode: Vec<Instruction>,
    symbol_table: HashMap<String, usize>, // 🌟 Maps variable names to memory slots
    next_slot: usize,                    // Tracks the next available memory slot
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            bytecode: Vec::new(),
            symbol_table: HashMap::new(),
            next_slot: 0,
        }
    }

    /// Helper function to look up or allocate a memory slot for a variable name
    fn get_or_allocate_slot(&mut self, name: &str) -> usize {
        if let Some(&slot) = self.symbol_table.get(name) {
            slot
        } else {
            let slot = self.next_slot;
            self.symbol_table.insert(name.to_string(), slot);
            self.next_slot += 1;
            slot
        }
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
                // 🌟 Dynamic lookup instead of character math!
                let slot = self.get_or_allocate_slot(name);
                self.bytecode.push(Instruction::Load(slot));
            }
            ASTNode::Assignment { name, value } => {
                self.compile(value);
                // 🌟 Dynamic slot assignment instead of character math!
                let slot = self.get_or_allocate_slot(name);
                self.bytecode.push(Instruction::Store(slot));
            }
            ASTNode::BinaryOp { left, op, right } => {
                self.compile(left);
                self.compile(right);
                match op {
                    Token::Plus => self.bytecode.push(Instruction::Add),
                    Token::Minus => self.bytecode.push(Instruction::Subtract),
                    Token::Star => self.bytecode.push(Instruction::Multiply),
                    Token::Slash => self.bytecode.push(Instruction::Divide),
                    _ => unreachable!(),
                }
            }
            ASTNode::Generate(expr) => {
                self.compile(expr);
                self.bytecode.push(Instruction::Print);
            }
        }
    }
}